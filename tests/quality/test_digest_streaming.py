import hashlib
import io
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from evaluate import bytes_digest


class DigestStreamingTests(unittest.TestCase):
    def test_large_and_empty_files_preserve_sha256_contract(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "input"
            for data in [b"", b"abc", bytes(range(256)) * 12345]:
                path.write_bytes(data)
                self.assertEqual(bytes_digest(path), hashlib.sha256(data).hexdigest())

    def test_digest_reads_without_requesting_an_unbounded_buffer(self):
        class BoundedReader(io.BytesIO):
            def read(self, size=-1):
                if size <= 0 or size > 1024 * 1024:
                    raise AssertionError("Digest requested an unbounded read")
                return super().read(size)
        data = b"x" * (3 * 1024 * 1024 + 7)
        with patch.object(Path, "open", return_value=BoundedReader(data)):
            self.assertEqual(bytes_digest(Path("input")), hashlib.sha256(data).hexdigest())
