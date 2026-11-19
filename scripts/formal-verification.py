#!/usr/bin/env python3
"""Run pinned proofs and model comparisons; never turn missing tools into success."""
import argparse
import datetime
import hashlib
import json
import os
import pathlib
import re
import shutil
import signal
import subprocess
import time

ROOT = pathlib.Path(__file__).resolve().parents[1]
LEAN_DIR = ROOT / "docs/formal-verification"
KANI_VERSION = "0.68.0"
THEOREMS = (
    "policy_correct", "pass_iff", "incomplete_never_pass", "indeterminate_never_pass",
    "fail_dominates", "complete_iff", "failure_never_complete",
)
HARNESSES = (
    "policy::proofs::policy_aggregation_contract",
    "diagnostic::proofs::coverage_classification_contract",
    *(f"assessment::proofs::{name}_vulnerable" for name in
      ("rsa", "ec", "dh", "dsa", "ecdh", "ecdsa", "ed25519", "x25519")),
    *(f"assessment::proofs::{name}_needs_parameters" for name in ("ml_kem", "ml_dsa", "slh_dsa")),
)
ASSESSMENT_HARNESSES = ("assessment::proofs::short_unknown_identifiers_stay_unknown",)


def execute(command, cwd, log, timeout, env=None):
    """Kill the entire child group on timeout, including solver descendants."""
    started = time.monotonic()
    result = {"command": command, "cwd": str(cwd), "log": str(log), "exit_code": None}
    log.parent.mkdir(parents=True, exist_ok=True)
    with log.open("wb") as stream:
        try:
            process = subprocess.Popen(command, cwd=cwd, stdout=stream, stderr=stream,
                                       env=env, start_new_session=True)
        except OSError as error:
            stream.write(str(error).encode())
            result["status"] = "not_run"
        else:
            try:
                result["exit_code"] = process.wait(timeout=timeout)
                result["status"] = "passed" if process.returncode == 0 else "failed"
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
                result["status"] = "timeout"
                result["exit_code"] = process.returncode
    result["seconds"] = round(time.monotonic() - started, 3)
    result["log_sha256"] = hashlib.sha256(log.read_bytes()).hexdigest()
    return result


def compare_vectors(a, b):
    if not a.is_file() or not b.is_file():
        return {"equal": False, "reason": "missing vectors"}
    left, right = a.read_bytes(), b.read_bytes()
    return {"equal": bool(left) and left == right, "rows": len(left.splitlines()),
            "rust_sha256": hashlib.sha256(left).hexdigest(),
            "lean_sha256": hashlib.sha256(right).hexdigest()}


def check_lean_axioms(output):
    allowed = {"propext", "Quot.sound", "Classical.choice"}
    for name in THEOREMS:
        line = next((line for line in output.splitlines() if line.startswith(f"'PQC.{name}' ")), "")
        if "does not depend on any axioms" in line:
            continue
        match = re.fullmatch(r"'PQC\." + name + r"' depends on axioms: \[(.*)\]", line)
        if not match or not set(filter(None, match[1].split(", "))) <= allowed:
            return False
    return True


def source_manifest():
    files = subprocess.check_output(
        ["git", "ls-files", "-c", "-o", "--exclude-standard"], cwd=ROOT, text=True
    ).splitlines()
    selected = {}
    for name in sorted(set(files)):
        path = ROOT / name
        if (name.startswith("crates/") and path.suffix in {".rs", ".toml"}
                or name in {"Cargo.toml", "Cargo.lock", "rust-toolchain.toml",
                            "scripts/formal-verification.py",
                            ".github/workflows/formal-verification.yml"}
                or name.startswith("tests/formal-verification/") and path.suffix == ".py"
                or name.startswith("docs/formal-verification/")
                and (path.suffix == ".lean" or path.name == "lean-toolchain")):
            selected[name] = hashlib.sha256(path.read_bytes()).hexdigest()
    return selected


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mode", choices=("all", "kani", "lean"), default="all")
    parser.add_argument("--out-dir", type=pathlib.Path)
    parser.add_argument("--timeout", type=float, default=900, help="seconds per command")
    parser.add_argument("--assessment", action="store_true",
                        help="also attempt the experimental symbolic unknown-identifier proof")
    args = parser.parse_args()
    if args.assessment and args.mode == "lean":
        parser.error("--assessment requires --mode kani or --mode all")
    if args.timeout <= 0:
        parser.error("--timeout must be positive")
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    out = (args.out_dir or ROOT / "target/formal-verification" / stamp).resolve()
    # No stale successful artifacts may survive into a new run.
    out.mkdir(parents=True, exist_ok=False)
    initial = source_manifest()
    report = {
        "schema_version": 1, "started_utc": stamp, "mode": args.mode,
        "source_revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "source_files": initial, "steps": {},
        "selected_harnesses": list(HARNESSES + (ASSESSMENT_HARNESSES if args.assessment else ())),
        "experimental_assessment": {"status": "not_run", "reason": "unknown identifiers; opt in with --assessment"},
        "kani": {"status": "not_run"}, "lean": {"status": "not_run"},
    }

    def save():
        (out / "results.json").write_text(json.dumps(report, indent=2) + "\n")

    def step(name, command, cwd=ROOT, env=None):
        result = execute(command, cwd, out / f"{name}.log", args.timeout, env)
        report["steps"][name] = result
        save()
        print(f"{name}: {result['status']}", flush=True)
        return result["status"] == "passed"

    def version(name, command, pattern, cwd=ROOT):
        if not step(name, command, cwd):
            return False
        if not re.search(pattern, (out / f"{name}.log").read_text()):
            report["steps"][name]["status"] = "failed"
            report["steps"][name]["reason"] = "tool version mismatch"
            save()
            return False
        return True

    save()
    if args.mode in {"all", "kani"}:
        kani_ok = version("kani-version", ["cargo", "kani", "--version"],
                          rf"Kani Rust Verifier {re.escape(KANI_VERSION)}\b")
        if kani_ok:
            selected = HARNESSES + (ASSESSMENT_HARNESSES if args.assessment else ())
            assessment_results = []
            for harness in selected:
                name = harness.split("::")[-1]
                ok = step(name, ["cargo", "kani", "-p", "pqc_scan_core", "--lib",
                                 "--harness", harness, "--exact", "--output-format", "terse"])
                if ok and "1 successfully verified harnesses, 0 failures, 1 total" not in (out / f"{name}.log").read_text():
                    report["steps"][name]["status"] = "failed"
                    report["steps"][name]["reason"] = "expected harness was not verified"
                    ok = False
                kani_ok = ok and kani_ok
                if harness in ASSESSMENT_HARNESSES:
                    assessment_results.append(report["steps"][name]["status"])
            report["kani"]["status"] = "passed" if kani_ok else "failed"
            if assessment_results:
                report["experimental_assessment"] = {"status": "passed" if all(s == "passed" for s in assessment_results) else "failed",
                                        "step_statuses": assessment_results}
        else:
            report["kani"]["reason"] = "pinned Kani unavailable; see version step"
        save()

    if args.mode in {"all", "lean"}:
        lean = shutil.which("lean") or str(pathlib.Path.home() / ".elan/bin/lean")
        lean_version = (LEAN_DIR / "lean-toolchain").read_text().strip().split(":v")[-1]
        ready = version("lean-version", [lean, "--version"], rf"version {re.escape(lean_version)},", LEAN_DIR)
        ready = version("rust-version", ["rustc", "--version"], r"rustc 1\.93\.1\b") and ready
        if ready:
            code = "\n".join(re.sub(r"--[^\n]*", "", p.read_text()) for p in LEAN_DIR.glob("*.lean"))
            if re.search(r"\b(sorry|admit|axiom|unsafe|native_decide)\b", code):
                report["lean"] = {"status": "failed", "reason": "forbidden proof escape"}
            else:
                ok = step("lean-proof", [lean, "-DwarningAsError=true", "-o", str(out / "Model.olean"), "Model.lean"], LEAN_DIR)
                if ok and not check_lean_axioms((out / "lean-proof.log").read_text()):
                    report["steps"]["lean-proof"]["status"] = "failed"
                    report["steps"]["lean-proof"]["reason"] = "axiom audit failed"
                    ok = False
                if ok:
                    env = dict(os.environ, PQC_FORMAL_VECTORS_DIR=str(out))
                    ok = step("rust-vectors", ["cargo", "test", "-p", "pqc_scan_core", "--lib",
                                              "formal::write_lean_vectors", "--locked", "--", "--exact"], env=env)
                    lean_env = dict(os.environ, LEAN_PATH=str(out))
                    model_ok = step("lean-vectors", [lean, "-DwarningAsError=true", "--run", "Vectors.lean"], LEAN_DIR, lean_env)
                    if model_ok:
                        shutil.copyfile(out / "lean-vectors.log", out / "lean.tsv")
                    comparison = compare_vectors(out / "rust.tsv", out / "lean.tsv")
                    report["comparison"] = comparison
                    ok = ok and model_ok and comparison["equal"] and comparison.get("rows") == 10662
                report["lean"]["status"] = "passed" if ok else "failed"
        else:
            report["lean"]["reason"] = "pinned Lean/Rust unavailable; see version steps"
        save()

    report["source_unchanged"] = initial == source_manifest()
    requested = [report[key]["status"] for key in ("kani", "lean") if args.mode in {"all", key}]
    report["status"] = "passed" if all(s == "passed" for s in requested) and report["source_unchanged"] else "failed"
    save()
    print(f"results: {out / 'results.json'}", flush=True)
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
