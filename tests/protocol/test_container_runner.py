"""Independent filesystem and real cgroup enforcement checks."""
import importlib.util
import io
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('container_runner', HERE / 'container_runner.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)


def archive(name, kind=None, data=b'public result'):
    stream = io.BytesIO()
    with tarfile.open(fileobj=stream, mode='w') as tar:
        entry = tarfile.TarInfo(name)
        if kind is not None:
            entry.type = kind
            entry.linkname = '/outside'
        else:
            entry.size = len(data)
        tar.addfile(entry, io.BytesIO(data) if kind is None else None)
    return stream.getvalue()


class HostBoundaryTests(unittest.TestCase):
    def test_only_regular_bounded_relative_artifacts_are_extracted(self):
        with tempfile.TemporaryDirectory() as temp:
            out = Path(temp) / 'safe'
            runner.extract_artifacts(archive('./nested/result.json'), out, max_bytes=100)
            self.assertEqual((out / 'nested/result.json').read_bytes(), b'public result')
            for name, kind in [('../outside', None), ('/outside', None), ('C:/outside',None), ('..\\outside',None),
                               ('link', tarfile.SYMTYPE), ('hardlink', tarfile.LNKTYPE),
                               ('pipe', tarfile.FIFOTYPE)]:
                with self.subTest(name=name, kind=kind):
                    with self.assertRaises(ValueError):
                        runner.extract_artifacts(archive(name, kind), Path(temp) / 'rejected')
            with self.assertRaises(ValueError):
                runner.extract_artifacts(archive('large', data=b'x'*101), Path(temp)/'oversize', max_bytes=100)
            self.assertFalse((Path(temp)/'outside').exists())

    def test_artifact_total_is_checked_before_writing_and_export_is_bounded(self):
        with tempfile.TemporaryDirectory() as temp:
            stream = io.BytesIO()
            with tarfile.open(fileobj=stream, mode='w') as tar:
                for name in ['one','two']:
                    entry = tarfile.TarInfo(name); entry.size=60
                    tar.addfile(entry,io.BytesIO(b'x'*60))
            out = Path(temp)/'evidence'
            with self.assertRaises(ValueError):
                runner.extract_artifacts(stream.getvalue(),out,max_bytes=100)
            self.assertFalse((out/'one').exists())
        import sys
        with self.assertRaises(ValueError):
            runner.bounded_output([sys.executable,'-c',"print('x'*1000)"],limit=64)
        with self.assertRaises(subprocess.TimeoutExpired):
            runner.bounded_output([sys.executable,'-c','import time; time.sleep(5)'],timeout=.1)

    def test_snapshot_excludes_host_metadata_and_rejects_parent_symlinks(self):
        with tempfile.TemporaryDirectory() as temp:
            base = Path(temp); root = base/'repository'; root.mkdir()
            subprocess.run(['git', 'init', '-q', str(root)], check=True)
            source = root/'crates/example/src'; source.mkdir(parents=True)
            (source/'lib.rs').write_text('// public fixture\n')
            (root/'.env').write_text('SYNTHETIC_SECRET=must-not-copy')
            (root/'.git/config-canary').write_text('must-not-copy')
            subprocess.run(['git', '-C', str(root), 'add', 'crates'], check=True)
            snapshot = base/'snapshot'
            bindings = runner.snapshot(root, snapshot)
            self.assertEqual(set(bindings), {'crates/example/src/lib.rs'})
            self.assertFalse((snapshot/'.env').exists())
            self.assertFalse((snapshot/'.git').exists())
            source.rename(root/'original')
            source.symlink_to(root/'original', target_is_directory=True)
            with self.assertRaises(ValueError):
                runner.snapshot(root, base/'rejected')


@unittest.skipUnless(os.environ.get('PQC_RUN_CONTAINER_TESTS') == '1', 'explicit Docker resource tests')
class KernelBoundaryTests(unittest.TestCase):
    def image(self):
        return runner.image_id(Path(os.environ['PQC_PROTOCOL_IMAGE_FILE']))

    def exercise(self, case, **limits):
        with tempfile.TemporaryDirectory() as temp:
            evidence = os.environ.get('PQC_PROTOCOL_EVIDENCE_DIR')
            out = Path(evidence)/case if evidence else Path(temp)/'evidence'
            result = runner.execute(self.image(), out, case=case, **limits)
            self.assertEqual(result['exit'], 0)
            self.assertTrue(result['containerRemoved'])
            return json.loads((out/'checks.json').read_text()), result

    def test_source_root_environment_and_network_boundaries(self):
        os.environ['PQC_SYNTHETIC_SECRET'] = 'must-not-inherit'
        try:
            checks, result = self.exercise('boundary')
        finally:
            os.environ.pop('PQC_SYNTHETIC_SECRET')
        for key in ['sourceReadonly', 'rootReadonly', 'emptyEnvironment', 'loopbackWorks', 'externalNetworkDenied']:
            self.assertTrue(checks[key], key)
        self.assertEqual(result['container']['HostConfig']['NetworkMode'], 'none')

    def test_memory_is_bounded_across_child_processes(self):
        checks, _ = self.exercise('memory', memory=64*1024**2, scratch=16*1024**2)
        self.assertGreater(checks['oomKills'], 0)
        self.assertEqual(checks['memoryMax'], 64*1024**2)
        self.assertEqual(checks['swapMax'], 0)

    def test_aggregate_scratch_limit_spans_multiple_files(self):
        checks, _ = self.exercise('disk', memory=128*1024**2, scratch=16*1024**2)
        self.assertTrue(checks['aggregateDiskDenied'])
        self.assertGreater(checks['filesWritten'], 1)
        self.assertLessEqual(checks['scratchCapacity'], 16*1024**2)

    def test_pid_limit_applies_to_descendants(self):
        checks, _ = self.exercise('pids', memory=128*1024**2, scratch=16*1024**2, pids=16)
        self.assertTrue(checks['forkDenied'])
        self.assertLess(checks['children'], 16)

    def test_create_response_failure_removes_daemon_created_container(self):
        from unittest import mock
        original = runner.subprocess.check_output
        created = []
        def lose_create_response(command, *args, **kwargs):
            data = original(command, *args, **kwargs)
            if command[:2] == ['docker','create']:
                created.append(data.strip())
                raise subprocess.TimeoutExpired(command,kwargs.get('timeout',30))
            return data
        with tempfile.TemporaryDirectory() as temp:
            evidence = os.environ.get('PQC_PROTOCOL_EVIDENCE_DIR')
            out = Path(evidence)/'create-response' if evidence else Path(temp)/'evidence'
            try:
                with mock.patch.object(runner.subprocess,'check_output',side_effect=lose_create_response):
                    with self.assertRaises(subprocess.TimeoutExpired):
                        runner.execute(self.image(),out,case='boundary')
                self.assertEqual(len(created),1)
                record = json.loads((out/'HOST-RESULTS.json').read_text())
                self.assertTrue(record['containerRemoved'])
                check = subprocess.run(['docker','inspect',created[0]],capture_output=True)
                self.assertNotEqual(check.returncode,0)
            finally:
                # Also clean the deliberately failing old implementation during RED.
                for container in created:
                    subprocess.run(['docker','rm','-f',container],capture_output=True)

    def test_timeout_removes_the_entire_container(self):
        with tempfile.TemporaryDirectory() as temp:
            evidence = os.environ.get('PQC_PROTOCOL_EVIDENCE_DIR')
            out = Path(evidence)/'timeout' if evidence else Path(temp)/'evidence'
            with self.assertRaises(subprocess.TimeoutExpired):
                runner.execute(self.image(), out, case='timeout', timeout=1)
            record = json.loads((out/'HOST-RESULTS.json').read_text())
            self.assertTrue(record['containerRemoved'])
            self.assertNotEqual(record['exit'], 0)
            check = subprocess.run(['docker', 'inspect', record['containerId']], capture_output=True)
            self.assertNotEqual(check.returncode, 0)


if __name__ == '__main__':
    unittest.main()
