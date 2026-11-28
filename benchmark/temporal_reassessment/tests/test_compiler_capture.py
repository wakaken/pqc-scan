import copy
import json
import os
from pathlib import Path
import stat
import subprocess
import sys
from unittest.mock import patch

from test_execution import PrivateFixture
import test_adapter
import adapter
import boundary as b
import capture_rustc as capture


class CompilerCaptureTests(PrivateFixture):
    def fixture(self):
        product = test_adapter.AdapterPlanTests.product(self)
        envelope = test_adapter.AdapterPlanTests.envelope(self, product)
        compiler = self.write('fake-compiler', ('#!' + str(Path(sys.executable).resolve()) + ' -I\n' + '''import json, os, sys
if '-vV' in sys.argv:
    print('synthetic compiler version probe')
    sys.exit(0)
fd = next((int(arg.split('=', 1)[1]) for arg in sys.argv if arg.startswith('--job-fd=')), None)
print(json.dumps(dict(argv=sys.argv[1:], cwd=os.getcwd(),
    jobToken=os.read(fd, 1).decode() if fd is not None else None,
    secretPreserved=os.environ.get('API_TOKEN') == 'synthetic-secret-marker')))
sys.exit(17)
''').encode())
        compiler.chmod(0o700)
        envelope['compiler'] = b.ref(compiler)
        for row in envelope['crateInvocations']:
            row['argv'][0] = str(compiler)
        with patch.object(b, 'ROOT', product):
            plan = adapter.build_plan(envelope, b.mkdir(self.root / 'adapter'), envelope['sourceBindings'])
        manifest = Path(plan['cwd']) / 'crates' / 'pqc_scan_core'
        environment = dict(plan['environment'], CARGO_MANIFEST_DIR=str(manifest), CARGO_MANIFEST_PATH=str(manifest / 'Cargo.toml'),
                           CARGO_PKG_NAME='pqc_scan_core', API_TOKEN='synthetic-secret-marker',
                           CARGO_REGISTRIES_PRIVATE_TOKEN='synthetic-cargo-secret-marker', CARGO_FEATURE_PRIVATE='secret-feature-marker')
        argv = [plan['compilerCapture']['script']['path'], compiler.as_posix(), '--crate-name', 'pqc_scan_core',
                str(manifest / 'src/lib.rs'), '--emit=dep-info,metadata,link']
        return product, envelope, plan, environment, argv

    def invoke(self, plan, environment, argv, **kwargs):
        return subprocess.run(argv, cwd=plan['cwd'], env=environment, capture_output=True, timeout=10, **kwargs)

    def records(self, plan):
        return sorted(Path(plan['compilerCapture']['recordsDirectory']).iterdir())

    def test_transparent_exec_preserves_raw_argv_jobserver_descriptor_and_exit_status(self):
        product, envelope, plan, environment, argv = self.fixture()
        read_fd, write_fd = os.pipe()
        try:
            os.write(write_fd, b'J')
            os.close(write_fd)
            write_fd = None
            argv += ['--job-fd=' + str(read_fd)]
            completed = self.invoke(plan, environment, argv, pass_fds=(read_fd,))
        finally:
            os.close(read_fd)
            if write_fd is not None:
                os.close(write_fd)
        self.assertEqual(completed.returncode, 17, completed.stderr)
        actual = json.loads(completed.stdout)
        self.assertEqual(actual['argv'], argv[2:])
        self.assertEqual(actual['cwd'], plan['cwd'])
        self.assertEqual(actual['jobToken'], 'J')
        self.assertTrue(actual['secretPreserved'])
        records = self.records(plan)
        self.assertEqual(len(records), 1)
        raw = records[0].read_bytes()
        recorded = json.loads(raw)
        self.assertEqual(recorded['argv'], argv[1:])
        self.assertEqual(recorded['cwd'], plan['cwd'])
        self.assertEqual(set(recorded['environment']), set(environment) & capture.ENVIRONMENT_KEYS)
        self.assertNotIn(b'synthetic-secret-marker', raw)
        self.assertNotIn(b'synthetic-cargo-secret-marker', raw)
        self.assertNotIn(b'secret-feature-marker', raw)
        self.assertNotIn('exitCode', recorded)
        self.assertEqual(stat.S_IMODE(records[0].stat().st_mode), 0o600)
        self.assertEqual(stat.S_IMODE(records[0].parent.stat().st_mode), 0o700)

    def test_version_probe_and_external_dependency_pass_through_without_first_party_record(self):
        product, envelope, plan, environment, argv = self.fixture()
        probe = argv[:2] + ['-vV']
        completed = self.invoke(plan, environment, probe)
        self.assertEqual(completed.returncode, 0, completed.stderr)
        self.assertEqual(self.records(plan), [])
        environment['CARGO_MANIFEST_DIR'] = '/external/dependency'
        environment['CARGO_MANIFEST_PATH'] = '/external/dependency/Cargo.toml'
        completed = self.invoke(plan, environment, argv)
        self.assertEqual(completed.returncode, 17, completed.stderr)
        self.assertEqual(self.records(plan), [])

    def test_wrong_compiler_changed_compiler_profile_or_ambient_hook_rejected(self):
        product, envelope, plan, environment, argv = self.fixture()
        variants = [('wrong-compiler', None), ('CARGO_PROFILE_DEV_DEBUG', '2'), ('CARGO_PROFILE_TEST_DEBUG', None),
                    ('RUSTC_WORKSPACE_WRAPPER', '/ambient/wrapper'), ('RUSTFLAGS', '--cfg injected'),
                    ('PYTHONPATH', '/ambient/python'), ('PQC_SCAN_OSS_REPLAY_GENERATED_AT', 'synthetic-private-clock')]
        for key, value in variants:
            bad_environment = copy.deepcopy(environment)
            bad_argv = list(argv)
            if key == 'wrong-compiler':
                bad_argv[1] = '/wrong/compiler'
            elif value is None:
                del bad_environment[key]
            else:
                bad_environment[key] = value
            result = self.invoke(plan, bad_environment, bad_argv)
            self.assertEqual(result.returncode, 125, (key, result.stderr))
            self.assertEqual(result.stderr, b'E_COMPILER_CAPTURE\n')
            self.assertEqual(self.records(plan), [])
        compiler = Path(envelope['compiler']['path'])
        compiler.write_bytes(compiler.read_bytes() + b'\n')
        self.assertEqual(self.invoke(plan, environment, argv).returncode, 125)
        self.assertEqual(self.records(plan), [])

    def test_changed_capture_script_is_rejected_before_compiler_exec(self):
        product, envelope, plan, environment, argv = self.fixture()
        script = Path(plan['compilerCapture']['script']['path'])
        script.chmod(0o700)
        script.write_bytes(script.read_bytes() + b'\n')
        result = self.invoke(plan, environment, argv)
        self.assertEqual(result.returncode, 125)
        self.assertEqual(result.stdout, b'')
        self.assertEqual(self.records(plan), [])

    def test_records_outside_plan_and_symlink_records_or_source_are_rejected(self):
        product, envelope, plan, environment, argv = self.fixture()
        outside = b.mkdir(self.root / 'outside-records')
        wrong = dict(environment, **{capture.RECORDS_ENV: str(outside)})
        self.assertEqual(self.invoke(plan, wrong, argv).returncode, 125)
        self.assertEqual(list(outside.iterdir()), [])
        records = Path(plan['compilerCapture']['recordsDirectory'])
        records.rmdir()
        records.symlink_to(outside, target_is_directory=True)
        self.assertEqual(self.invoke(plan, environment, argv).returncode, 125)
        self.assertEqual(list(outside.iterdir()), [])
        records.unlink()
        records.mkdir(mode=0o700)
        source = Path(argv[4])
        external = self.write('external.rs', source.read_bytes())
        source.unlink()
        source.symlink_to(external)
        self.assertEqual(self.invoke(plan, environment, argv).returncode, 125)
        self.assertEqual(self.records(plan), [])

    def test_unbound_first_party_source_and_records_inside_input_root_are_rejected(self):
        product, envelope, plan, environment, argv = self.fixture()
        source = Path(argv[4])
        source.write_bytes(source.read_bytes() + b' changed')
        self.assertEqual(self.invoke(plan, environment, argv).returncode, 125)
        config_path = Path(plan['compilerCapture']['configuration']['path'])
        config = json.loads(config_path.read_bytes())
        config['storageRoot'] = str(product)
        config_path.write_text(json.dumps(config))
        self.assertEqual(self.invoke(plan, environment, argv).returncode, 125)
        self.assertEqual(self.records(plan), [])
