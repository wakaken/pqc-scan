import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time
import unittest
from unittest.mock import patch

import run


def wait_for(path, seconds=3):
    deadline = time.monotonic() + seconds
    while not path.exists() and time.monotonic() < deadline:
        time.sleep(.005)
    if not path.exists():
        raise AssertionError(f"Child did not create {path.name}")


def descendant_command(root, ending):
    child = (
        "from pathlib import Path;import time;"
        f"Path({str(root / 'ready')!r}).write_text('ready');"
        "time.sleep(.4);"
        f"Path({str(root / 'effect')!r}).write_text('escaped')"
    )
    leader = (
        "import os,signal,subprocess,sys,time;from pathlib import Path;"
        f"Path({str(root / 'leader')!r}).write_text(str(os.getpid()));"
        f"subprocess.Popen([sys.executable,'-c',{child!r}]);"
        f"ready=Path({str(root / 'ready')!r});"
        "deadline=time.monotonic()+3\n"
        "while not ready.exists() and time.monotonic()<deadline:time.sleep(.005)\n"
        "assert ready.exists()\n"
        + ending
    )
    return [sys.executable, "-c", leader]


class MeasurementSafetyTests(unittest.TestCase):
    def test_existing_empty_directory_remains_supported(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            sample = run.measure([sys.executable, '-c', "print('kept')"], root, 3)
            self.assertEqual(sample['exitCode'], 0)
            self.assertEqual((root / 'stdout.log').read_text(), 'kept\n')

    def test_reused_directory_preserves_both_logs_and_does_not_execute(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            command = [sys.executable, '-c', "import sys;print('first');print('error',file=sys.stderr)"]
            run.measure(command, root, 3)
            before = {name: (root / name).read_bytes() for name in ['stdout.log', 'stderr.log']}
            marker = root / 'reran'
            with self.assertRaises((ValueError, FileExistsError)):
                run.measure([sys.executable, '-c', f"from pathlib import Path;Path({str(marker)!r}).touch()"], root, 3)
            self.assertFalse(marker.exists())
            self.assertEqual({name: (root / name).read_bytes() for name in before}, before)

    def test_partial_prior_logs_are_preserved(self):
        for name in ['stdout.log', 'stderr.log']:
            with self.subTest(name=name), tempfile.TemporaryDirectory() as tmp:
                root = Path(tmp)
                (root / name).write_bytes(b'prior evidence')
                with self.assertRaises((ValueError, FileExistsError)):
                    run.measure([sys.executable, '-c', "print('replacement')"], root, 3)
                self.assertEqual((root / name).read_bytes(), b'prior evidence')

    def test_invalid_timeout_rejected_before_execution_or_output_creation(self):
        for timeout in [0, -1, float('inf'), -float('inf'), float('nan')]:
            with self.subTest(timeout=timeout), tempfile.TemporaryDirectory() as tmp:
                output = Path(tmp) / 'logs'
                with self.assertRaises(ValueError):
                    run.measure([sys.executable, '-c', "print('must not run')"], output, timeout)
                self.assertFalse(output.exists())

    @unittest.skipUnless(os.name == 'posix', 'POSIX process groups required')
    def test_descendant_side_effect_positive_control(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            sample = subprocess.run(descendant_command(root, "raise SystemExit(0)"), check=False)
            self.assertEqual(sample.returncode, 0)
            wait_for(root / 'effect')
            self.assertEqual((root / 'effect').read_text(), 'escaped')

    @unittest.skipUnless(os.name == 'posix', 'POSIX process groups required')
    def test_successful_and_failed_leader_exit_stop_descendants(self):
        for code in [0, 7]:
            with self.subTest(code=code), tempfile.TemporaryDirectory() as tmp:
                root = Path(tmp)
                sample = run.measure(descendant_command(root, f"raise SystemExit({code})"), root / 'logs', 3)
                self.assertEqual(sample['exitCode'], code)
                self.assertFalse(sample['timedOut'])
                self.assertTrue((root / 'ready').exists())
                time.sleep(.5)
                self.assertFalse((root / 'effect').exists())

    @unittest.skipUnless(os.name == 'posix', 'POSIX process groups required')
    def test_timeout_stops_ready_descendant_and_reaps_leader(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            clock = time.monotonic
            def deadline_after_ready():
                return clock() + (120 if (root / 'ready').exists() else 0)
            with patch.object(run.time, 'monotonic', side_effect=deadline_after_ready):
                sample = run.measure(descendant_command(root, "time.sleep(20)"), root / 'logs', 60)
            self.assertTrue(sample['timedOut'])
            self.assertNotEqual(sample['exitCode'], 0)
            self.assertTrue((root / 'ready').exists())
            time.sleep(.5)
            self.assertFalse((root / 'effect').exists())
            with self.assertRaises(ChildProcessError):
                os.waitpid(int((root / 'leader').read_text()), os.WNOHANG)

    @unittest.skipUnless(os.name == 'posix', 'POSIX process groups required')
    def test_keyboard_interrupt_stops_descendant_and_reaps_leader(self):
        def interrupt(signum, frame):
            raise KeyboardInterrupt

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            previous = signal.signal(signal.SIGUSR1, interrupt)
            try:
                with self.assertRaises(KeyboardInterrupt):
                    run.measure(descendant_command(root, "os.kill(os.getppid(),signal.SIGUSR1);time.sleep(20)"), root / 'logs', 3)
                self.assertTrue((root / 'ready').exists())
                time.sleep(.5)
                self.assertFalse((root / 'effect').exists())
                with self.assertRaises(ChildProcessError):
                    os.waitpid(int((root / 'leader').read_text()), os.WNOHANG)
            finally:
                signal.signal(signal.SIGUSR1, previous)
                if (root / 'leader').exists():
                    pid = int((root / 'leader').read_text())
                    try:
                        os.killpg(pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                    try:
                        os.waitpid(pid, 0)
                    except ChildProcessError:
                        pass


if __name__ == '__main__':
    unittest.main()
