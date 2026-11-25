"""Exercise execution/evidence failures, not verifier internals."""
import importlib.util
import pathlib
import sys
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("formal_runner", ROOT / "scripts/formal-verification.py")
runner = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(runner)


class RunnerTests(unittest.TestCase):
    def run_command(self, command, timeout=5):
        with tempfile.TemporaryDirectory() as tmp:
            return runner.execute(command, ROOT, pathlib.Path(tmp) / "run.log", timeout)

    def test_success_requires_zero_exit(self):
        result = self.run_command([sys.executable, "-c", "print('checked')"])
        self.assertEqual((result["status"], result["exit_code"]), ("passed", 0))

    def test_nonzero_is_failed(self):
        result = self.run_command([sys.executable, "-c", "raise SystemExit(7)"])
        self.assertEqual((result["status"], result["exit_code"]), ("failed", 7))

    def test_missing_tool_is_not_run(self):
        result = self.run_command(["/no/such/formal-verifier"])
        self.assertEqual((result["status"], result["exit_code"]), ("not_run", None))

    def test_timeout_is_distinct(self):
        result = self.run_command([sys.executable, "-c", "import time; time.sleep(10)"], 0.05)
        self.assertEqual(result["status"], "timeout")

    def test_manifest_detects_source_changes_and_includes_new_proofs(self):
        import subprocess
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            subprocess.run(["git", "init", "-q", str(root)], check=True)
            source = root / "crates/demo/src/lib.rs"
            source.parent.mkdir(parents=True)
            source.write_text("pub fn decision() -> bool { false }\n")
            proof = root / "docs/formal-verification/why3find.json"
            proof.parent.mkdir(parents=True)
            proof.write_text('{"provers": ["z3"]}\n')
            config = proof.with_name("Cargo.toml")
            config.write_text('[workspace]\n')
            old_root, runner.ROOT = runner.ROOT, root
            try:
                initial = runner.source_manifest()
                self.assertIn("crates/demo/src/lib.rs", initial)
                self.assertIn("docs/formal-verification/why3find.json", initial)
                self.assertIn("docs/formal-verification/Cargo.toml", initial)
                source.write_text("pub fn decision() -> bool { true }\n")
                self.assertNotEqual(initial, runner.source_manifest())
            finally:
                runner.ROOT = old_root

    def test_creusot_rejects_empty_missing_and_unproved_goals(self):
        good = "  Goal vc_len_Decision: ✔ (1)\n  Goal vc_aggregate_decision: ✔ (1)\n"
        self.assertTrue(runner.creusot_goals(good)["proved"])
        for output in ("", "Proved (empty) ✔", "  Goal vc_len_Decision: ✔ (1)\n",
                       good.replace("✔ (1)", "✘ (0/1)", 1), good + "  Goal new_contract: ✘ (0/1)\n",
                       good.replace("✔ (1)", "✔ (0)")):
            self.assertFalse(runner.creusot_goals(output)["proved"], output)

    def test_creusot_project_uses_original_source_and_locked_dependencies(self):
        with tempfile.TemporaryDirectory() as tmp:
            directory = pathlib.Path(tmp) / "proof"
            runner.prepare_creusot_project(directory, runner.POLICY_SOURCE)
            import json
            import re
            path = re.search(r'^path = (".*")$', (directory / "Cargo.toml").read_text(), re.MULTILINE)
            self.assertEqual((directory / json.loads(path[1])).resolve(), runner.POLICY_SOURCE)
            self.assertEqual((directory / "Cargo.lock").read_bytes(), (runner.PROOF_DIR / "Cargo.lock").read_bytes())
            self.assertFalse((directory / "aggregation.rs").exists())
            with self.assertRaises(FileExistsError):
                runner.prepare_creusot_project(directory, runner.POLICY_SOURCE)

    def test_mutations_retain_contracts_and_require_unique_anchors(self):
        source = runner.POLICY_SOURCE.read_text()
        contracts = [line for line in source.splitlines() if "cfg_attr(creusot" in line]
        for name in runner.MUTATIONS:
            mutant = runner.mutate(source, name)
            self.assertNotEqual(source, mutant)
            self.assertEqual(contracts, [line for line in mutant.splitlines() if "cfg_attr(creusot" in line])
            for bad in ("", source + source):
                with self.assertRaises(ValueError):
                    runner.mutate(bad, name)

    def test_solver_timeouts_are_preserved_as_timeouts(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = pathlib.Path(tmp) / "solver.json"
            self.assertEqual(runner.solver_outcomes(path), {})
            path.write_text('{"result":"Timeout"}\n{\n"result":"Valid"\n}\n{"result":"Unknown"}')
            self.assertEqual(runner.solver_outcomes(path), {"Timeout": 1, "Valid": 1, "Unknown": 1})

    def test_creusot_mode_rejects_explicit_kani_proof_request(self):
        import subprocess
        with tempfile.TemporaryDirectory() as tmp:
            out = pathlib.Path(tmp) / "results"
            result = subprocess.run([sys.executable, str(ROOT / "scripts/formal-verification.py"),
                "--mode", "creusot", "--assessment", "--out-dir", str(out)], capture_output=True, text=True)
            self.assertEqual(result.returncode, 2)
            self.assertIn("--assessment requires", result.stderr)
            self.assertFalse(out.exists())

    def test_retired_lean_mode_is_rejected_before_running_tools(self):
        import subprocess
        with tempfile.TemporaryDirectory() as tmp:
            out = pathlib.Path(tmp) / "results"
            result = subprocess.run([sys.executable, str(ROOT / "scripts/formal-verification.py"),
                "--mode", "lean", "--out-dir", str(out)], capture_output=True, text=True)
            self.assertEqual(result.returncode, 2)
            self.assertIn("invalid choice", result.stderr)
            self.assertFalse(out.exists())

    def test_default_selects_both_rust_verifiers_and_reports_missing_versions(self):
        import json
        import os
        import subprocess
        with tempfile.TemporaryDirectory() as tmp:
            directory = pathlib.Path(tmp)
            cargo = directory / "cargo"
            cargo.write_text(f"#!{sys.executable}\n" + "import sys\n" +
                "if sys.argv[1:] == ['kani', '--version']: print('Kani Rust Verifier 0.0.0')\n" +
                "elif sys.argv[1:] == ['creusot', 'version']: print('cargo-creusot 0.0.0')\n" +
                "else: raise SystemExit(99)\n")
            cargo.chmod(0o755)
            out = directory / "results"
            env = dict(os.environ, PATH=str(directory) + os.pathsep + os.environ["PATH"])
            result = subprocess.run([sys.executable, str(ROOT / "scripts/formal-verification.py"),
                "--out-dir", str(out)], env=env, capture_output=True, text=True)
            self.assertEqual(result.returncode, 1)
            report = json.loads((out / "results.json").read_text())
            self.assertEqual(report["schema_version"], 2)
            self.assertEqual(report["mode"], "all")
            self.assertEqual(set(report["steps"]), {"kani-version", "creusot-version"})
            self.assertEqual(report["status"], "failed")
            for lane in ("kani", "creusot"):
                self.assertEqual(report[lane]["status"], "not_run")
                self.assertEqual(report["steps"][f"{lane}-version"]["status"], "failed")
            self.assertNotIn("lean", report)
            self.assertNotIn("comparison", report)


if __name__ == "__main__":
    unittest.main()
