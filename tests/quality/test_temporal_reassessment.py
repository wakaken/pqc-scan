"""Run reassessment admission regressions with code and storage separated."""
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
ADMISSION_TIMEOUT_SECONDS = 1800


class ReassessmentInstallationTests(unittest.TestCase):
    def test_installed_admission_regressions_use_external_private_storage(self):
        with tempfile.TemporaryDirectory(prefix='pqc-reassessment-tests-') as temporary:
            storage = Path(temporary).resolve()
            storage.chmod(0o700)
            environment = {**os.environ, 'PYTHONDONTWRITEBYTECODE': '1',
                           'PQC_SCAN_REASSESSMENT_ROOT': str(ROOT),
                           'PQC_SCAN_REASSESSMENT_STORAGE': str(storage)}
            try:
                result = subprocess.run([sys.executable, '-B', '-m', 'unittest', 'discover',
                                         '-s', 'tests', '-v'], cwd=ROOT / 'benchmark/temporal_reassessment',
                                        env=environment, capture_output=True,
                                        timeout=ADMISSION_TIMEOUT_SECONDS, check=False)
            except subprocess.TimeoutExpired as error:
                progress = (error.stderr or b'').decode(errors='replace')[-16384:]
                self.fail(f'Admission regressions exceeded {ADMISSION_TIMEOUT_SECONDS}s; last progress:\n{progress}')
            self.assertEqual(result.returncode, 0, result.stderr.decode(errors='replace'))


if __name__ == '__main__':
    unittest.main()
