import pathlib
import sys
import tempfile
import unittest

import run


class MeasurementTests(unittest.TestCase):
    def test_actual_child_records_success_wall_cpu_and_per_child_rss(self):
        with tempfile.TemporaryDirectory(prefix="pqc023-child-") as name:
            sample = run.measure([sys.executable, "-c", "print('synthetic')"], pathlib.Path(name), 5)
            self.assertEqual(sample["exitCode"], 0)
            self.assertFalse(sample["timedOut"])
            self.assertGreater(sample["wallSeconds"], 0)
            if hasattr(__import__('os'), 'wait4'):
                self.assertGreaterEqual(sample["cpuSeconds"], 0)
                self.assertGreater(sample["peakRssBytes"], 0)
            self.assertEqual((pathlib.Path(name) / "stdout.log").read_text(), "synthetic\n")

    @unittest.skipUnless(hasattr(__import__('os'), 'wait4') and sys.platform in {'darwin', 'linux'}, 'Per-child RSS unavailable')
    def test_small_second_child_does_not_inherit_previous_child_peak_rss(self):
        with tempfile.TemporaryDirectory(prefix="pqc023-rss-") as name:
            root = pathlib.Path(name)
            large = run.measure([sys.executable, "-c", "data = bytearray(32 * 1024 * 1024); print(len(data))"], root / "large", 5)
            small = run.measure([sys.executable, "-c", "print('small')"], root / "small", 5)
            self.assertEqual((large["exitCode"], small["exitCode"]), (0, 0))
            self.assertLess(small["peakRssBytes"], large["peakRssBytes"])

    def test_timeout_is_failed_sample_not_successful_fast_run(self):
        with tempfile.TemporaryDirectory(prefix="pqc023-timeout-") as name:
            sample = run.measure([sys.executable, "-c", "import time; time.sleep(20)"], pathlib.Path(name), 0.1)
            self.assertTrue(sample["timedOut"])
            self.assertNotEqual(sample["exitCode"], 0)

    def test_workloads_have_fixed_sizes_digests_and_do_not_alias_source(self):
        repo = pathlib.Path(__file__).resolve().parents[2]
        data, _ = run.load_truth(repo / "tests/quality")
        with tempfile.TemporaryDirectory(prefix="pqc023-workloads-") as name:
            root = pathlib.Path(name)
            a = root / "a"
            b = root / "b"
            one = run.materialize(data, repo, a, 1)
            two = run.materialize(data, repo, b, 2)
            self.assertEqual(two["files"], 2 * one["files"])
            self.assertEqual(two["bytes"], 2 * one["bytes"])
            self.assertNotEqual(one["sha256"], two["sha256"])
            same = run.materialize(data, repo, root / "same", 1)
            self.assertEqual(one, same)
            first = next(p for p in a.rglob('*') if p.is_file())
            first.write_bytes(b'changed')
            run.validate_corpus(data, repo)

    def test_environment_binds_embedded_renderer_source_assets(self):
        env = run.environment(pathlib.Path(sys.executable).resolve())
        for asset in ['crates/pqc_scan_report/src/compact.js', 'crates/pqc_scan_report/src/compact.css']:
            self.assertIn(asset, env["runtimeSourceBindings"])
            self.assertEqual(env["runtimeSourceBindings"][asset], run.bytes_digest(run.REPO / asset))

    def test_distribution_keeps_all_samples_and_nulls_unavailable_resources(self):
        values = [{"wallSeconds": x, "cpuSeconds": None, "peakRssBytes": None} for x in [1, 3, 2]]
        result = run.distributions(values)
        self.assertEqual(result["wallSeconds"], {"min": 1, "median": 2, "max": 3, "samples": [1, 3, 2]})
        self.assertIsNone(result["cpuSeconds"])


if __name__ == "__main__":
    unittest.main()
