"""Real child processes verify exit classification and group termination."""
import json
from pathlib import Path
import sys
import tempfile
import unittest
from bounded_process import execute


class BoundedProcessTests(unittest.TestCase):
    def run_code(self,code,**options):
        with tempfile.TemporaryDirectory() as tmp:
            result=execute([sys.executable,'-c',code],Path(tmp)/'logs',timeout=options.get('timeout',2),cpu_seconds=options.get('cpu_seconds',2))
            result['stdoutText']=(Path(tmp)/'logs/stdout.log').read_text()
            return result

    def test_exit_and_timeout_are_separate_and_keep_successful_output(self):
        self.assertEqual(self.run_code("print('kept')")['status'],'completed')
        for code,status in [("raise SystemExit(42)",'panic'),("raise SystemExit(86)",'invariant'),("raise SystemExit(1)",'error'),("import os,signal;os.kill(os.getpid(),signal.SIGTERM)",'signal')]:
            with self.subTest(status=status):self.assertEqual(self.run_code(code)['status'],status)
        stopped=self.run_code("import time;print('kept',flush=True);time.sleep(20)",timeout=.1)
        self.assertEqual(stopped['status'],'timeout');self.assertIn('kept',stopped['stdoutText']);self.assertTrue(stopped['groupTerminationSent'])

    def test_descendant_cannot_keep_running_after_parent_timeout(self):
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp);marker=root/'escaped'
            code="import subprocess,sys,time;subprocess.Popen([sys.executable,'-c',"+repr("import time;from pathlib import Path;time.sleep(.5);Path("+repr(str(marker))+").write_text('escaped')")+"]);time.sleep(20)"
            result=execute([sys.executable,'-c',code],root/'logs',timeout=.1,cpu_seconds=2)
            import time;time.sleep(.6)
            self.assertEqual(result['status'],'timeout');self.assertFalse(marker.exists())

    def test_invalid_limits_are_rejected_before_execution(self):
        with tempfile.TemporaryDirectory() as tmp:
            for timeout in [0,-1,float('inf')]:
                with self.assertRaises(ValueError):execute(['does-not-exist'],Path(tmp)/'invalid',timeout=timeout,cpu_seconds=1)
