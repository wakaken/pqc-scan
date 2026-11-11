"""OS-enforced boundary tests; invoked separately from the Rust binaries."""
import importlib.util
import os
from pathlib import Path
import platform
import subprocess
import sys
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('protocol_isolation', HERE / 'isolation.py')
isolation = importlib.util.module_from_spec(spec)
spec.loader.exec_module(isolation)

@unittest.skipUnless(platform.system() == 'Darwin', 'macOS sandbox-exec policy')
class IsolationTests(unittest.TestCase):
    def test_readonly_source_private_home_empty_environment_and_loopback_only(self):
        with tempfile.TemporaryDirectory() as temp:
            base = Path(temp).resolve()
            source, scratch, private = [base / name for name in ['source', 'scratch', 'private-home']]
            for directory in [source, scratch, private]:
                directory.mkdir()
            (source / 'fixture').write_text('public fixture')
            (private / 'canary').write_text('synthetic private home data')
            code = '''
import os, pathlib, socket, sys
source, scratch, private = map(pathlib.Path, sys.argv[1:])
assert source.joinpath('fixture').read_text() == 'public fixture'
assert 'PQC_SYNTHETIC_SECRET' not in os.environ
assert pathlib.Path(os.environ['HOME']).resolve() == scratch / 'home'
for path, mode in [(private / 'canary', 'r'), (source / 'modified', 'w')]:
    try:
        with path.open(mode) as stream:
            stream.read() if mode == 'r' else stream.write('unexpected')
    except PermissionError:
        pass
    else:
        raise AssertionError('sandbox allowed forbidden access')
with socket.socket() as listener:
    listener.bind(('127.0.0.1', 0)); listener.listen()
    with socket.create_connection(listener.getsockname(), timeout=1) as client:
        conn, _ = listener.accept(); conn.close()
with socket.socket() as external:
    external.settimeout(1)
    try:
        external.connect(('192.0.2.1', 9))
    except PermissionError:
        pass
    else:
        raise AssertionError('external connect was not denied by policy')
scratch.joinpath('result').write_text('boundary checks passed')
'''
            env = isolation.environment(scratch)
            profile = isolation.profile(source, scratch, [Path(sys.prefix)])
            old = os.environ.get('PQC_SYNTHETIC_SECRET')
            os.environ['PQC_SYNTHETIC_SECRET'] = 'must-not-be-inherited'
            try:
                result = isolation.run([sys.executable, '-c', code, str(source), str(scratch), str(private)],
                                       source, env, profile, timeout=15, capture_output=True, text=True)
            finally:
                if old is None:
                    os.environ.pop('PQC_SYNTHETIC_SECRET', None)
                else:
                    os.environ['PQC_SYNTHETIC_SECRET'] = old
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual((scratch / 'result').read_text(), 'boundary checks passed')
            self.assertFalse((source / 'modified').exists())

if __name__ == '__main__':
    unittest.main()
