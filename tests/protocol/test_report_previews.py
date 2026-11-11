"""Readiness checks must tolerate kernel FD entries disappearing mid-read."""
import importlib.util
import os
from pathlib import Path
import tempfile
import unittest
from unittest import mock

spec = importlib.util.spec_from_file_location('report_previews',Path(__file__).with_name('report-previews.py'))
previews = importlib.util.module_from_spec(spec); spec.loader.exec_module(previews)


class ReadinessTests(unittest.TestCase):
    def test_closed_descriptor_does_not_hide_the_owned_listener(self):
        with tempfile.TemporaryDirectory() as temp:
            proc=Path(temp); descriptors=proc/'123/fd';descriptors.mkdir(parents=True)
            (proc/'net').mkdir();(proc/'net/tcp').write_text('header\n0: 0100007F:1F90 00000000:0000 0A 0:0 00:0 0 0 0 222\n')
            (descriptors/'3').symlink_to('socket:[111]');(descriptors/'4').symlink_to('socket:[222]')
            original=os.readlink
            def closes_before_read(path):
                path=Path(path)
                if path.name=='3': path.unlink(missing_ok=True)
                return original(path)
            with mock.patch.object(previews.os,'readlink',side_effect=closes_before_read):
                self.assertTrue(previews.linux_listener_ready(123,8080,proc))
            (descriptors/'4').unlink()
            self.assertFalse(previews.linux_listener_ready(123,8080,proc))
            self.assertFalse(previews.linux_listener_ready(999,8080,proc))


if __name__=='__main__':unittest.main()
