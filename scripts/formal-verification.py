#!/usr/bin/env python3
"""Run pinned Rust proofs; never turn missing tools into success."""
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
PROOF_DIR = ROOT / "docs/formal-verification"
KANI_VERSION = "0.68.0"
CREUSOT_VERSION = "0.15.0"
POLICY_SOURCE = ROOT / "crates/pqc_scan_core/src/policy/aggregation.rs"
MUTATIONS = {
    "omit_completeness": (" || !inspection_complete || !inventory_exhaustive", ""),
    "forget_failure": ("Decision::Fail => has_fail = true,", "Decision::Fail => {},"),
    "skip_last": ("while i < decisions.len() {",
                  "while i < decisions.len() && i != decisions.len() - 1 {"),
}
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


def prepare_creusot_project(directory, source):
    """Compile the real production file, with isolated proof-only dependencies."""
    directory.mkdir()
    directory = directory.resolve()
    manifest = (PROOF_DIR / "Cargo.toml").read_text()
    original = '"../../crates/pqc_scan_core/src/policy/aggregation.rs"'
    if manifest.count(original) != 1:
        raise ValueError("proof manifest no longer selects the production kernel")
    relative = os.path.relpath(source, directory)
    (directory / "Cargo.toml").write_text(manifest.replace(original, json.dumps(relative)) +
        '\n[[test]]\nname = "contract_witnesses"\npath = "witnesses.rs"\n')
    # Independent concrete counterexamples distinguish a bad mutant from solver incompleteness.
    (directory / "witnesses.rs").write_text(f"#[path = {json.dumps(relative)}]\nmod aggregation;\n" + """
use aggregation::{aggregate_decision, Decision::*};
#[test]
fn omit_completeness() {
    assert_eq!(aggregate_decision(&[Pass], false, true), Indeterminate);
    assert_eq!(aggregate_decision(&[Pass], true, false), Indeterminate);
}
#[test]
fn forget_failure() {
    assert_eq!(aggregate_decision(&[Pass, Fail], true, true), Fail);
}
#[test]
fn skip_last() {
    assert_eq!(aggregate_decision(&[Pass, Indeterminate], true, true), Indeterminate);
}
""")
    for name in ("Cargo.lock", "rust-toolchain.toml", "why3find.json"):
        shutil.copyfile(PROOF_DIR / name, directory / name)


def creusot_goals(output):
    """Require the selected goal, nonempty output, and no unproved goal summaries."""
    goals = re.findall(r"^  Goal ([^:]+): (.+)$", output, re.MULTILINE)
    proved = bool(goals) and all(re.fullmatch(r"✔ \([1-9][0-9]*\)", status)
                                  for _, status in goals)
    return {"proved": proved and any(name == "vc_aggregate_decision" for name, _ in goals),
            "goals": dict(goals)}


def mutate(source, name):
    old, new = MUTATIONS[name]
    if source.count(old) != 1:
        raise ValueError(f"mutation anchor changed: {name}")
    return source.replace(old, new)


def solver_outcomes(path):
    """Why3find writes consecutive JSON objects (not a JSON array)."""
    remaining = path.read_text() if path.is_file() else ""
    counts = {}
    decoder = json.JSONDecoder()
    while remaining.strip():
        result, end = decoder.raw_decode(remaining.lstrip())
        key = result["result"]
        counts[key] = counts.get(key, 0) + 1
        remaining = remaining.lstrip()[end:]
    return counts


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
                and path.suffix in {".toml", ".lock", ".json"}):
            selected[name] = hashlib.sha256(path.read_bytes()).hexdigest()
    return selected


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mode", choices=("all", "kani", "creusot"), default="all")
    parser.add_argument("--out-dir", type=pathlib.Path)
    parser.add_argument("--timeout", type=float, default=900, help="seconds per command")
    parser.add_argument("--assessment", action="store_true",
                        help="also attempt the experimental symbolic unknown-identifier proof")
    args = parser.parse_args()
    if args.assessment and args.mode not in {"all", "kani"}:
        parser.error("--assessment requires --mode kani or --mode all")
    if args.timeout <= 0:
        parser.error("--timeout must be positive")
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    out = (args.out_dir or ROOT / "target/formal-verification" / stamp).resolve()
    # No stale successful artifacts may survive into a new run.
    out.mkdir(parents=True, exist_ok=False)
    initial = source_manifest()
    report = {
        "schema_version": 2, "started_utc": stamp, "mode": args.mode,
        "source_revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "source_files": initial, "steps": {},
        "selected_harnesses": list(HARNESSES + (ASSESSMENT_HARNESSES if args.assessment else ())),
        "experimental_assessment": {"status": "not_run", "reason": "unknown identifiers; opt in with --assessment"},
        "kani": {"status": "not_run"},
        "creusot": {"status": "not_run"},
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

    if args.mode in {"all", "creusot"}:
        project = out / "creusot"
        prepare_creusot_project(project, POLICY_SOURCE)
        ready = version("creusot-version", ["cargo", "creusot", "version"],
                        rf"(?m)^cargo-creusot {re.escape(CREUSOT_VERSION)}$", project)
        if ready:
            output = (out / "creusot-version.log").read_text()
            pins = (r"^Rust toolchain nightly-2026-08-03$", r"^Why3 platform, version 1\.8\.2\+git$",
                    r"^why3find v1\.3\.0\+dev$", r"^z3 5\.3\.2$")
            ready = all(re.search(pin, output, re.MULTILINE) for pin in pins)
            if not ready:
                report["steps"]["creusot-version"].update(status="failed", reason="dependency version mismatch")
        if ready:
            env = dict(os.environ, CARGO_TARGET_DIR=str(PROOF_DIR / "target/creusot"))

            def prove(name, directory):
                command = ["cargo", "creusot", "--no-cache", "--why3find-arg=--goals",
                           "--why3find-arg=--log-prover-results",
                           f"--why3find-arg={directory / 'solver-results.json'}", "--", "--locked"]
                ok = step(name, command, directory, env)
                goals = creusot_goals((out / f"{name}.log").read_text())
                report["steps"][name]["verification"] = goals
                report["steps"][name]["solver_outcomes"] = solver_outcomes(directory / "solver-results.json")
                generated = directory / "verif/pqc_scan_policy_proof_rlib/aggregate_decision.coma"
                report["steps"][name]["coma_generated"] = generated.is_file()
                if ok and (not goals["proved"] or not generated.is_file()
                           or not report["steps"][name]["solver_outcomes"].get("Valid")):
                    report["steps"][name].update(status="failed", reason="expected proof goals missing or unproved")
                    ok = False
                save()
                return ok

            ok = prove("creusot-proof", project)
            ok = step("creusot-witnesses", ["cargo", "test", "--locked", "--test", "contract_witnesses"],
                      project, env) and ok
            report["creusot"]["mutations"] = {}
            if ok:
                source = POLICY_SOURCE.read_text()
                for name in MUTATIONS:
                    mutant = out / f"mutant_{name}.rs"
                    mutant.write_text(mutate(source, name))
                    directory = out / f"creusot-{name}"
                    prepare_creusot_project(directory, mutant)
                    prove(f"creusot-{name}", directory)
                    result = report["steps"][f"creusot-{name}"]
                    step(f"witness-{name}", ["cargo", "test", "--locked", "--test", "contract_witnesses",
                                             name, "--", "--exact"], directory, env)
                    witness = report["steps"][f"witness-{name}"]
                    witness_failed = (witness["status"] == "failed" and witness["exit_code"] == 101
                                      and f"test {name} ... FAILED" in (out / f"witness-{name}.log").read_text())
                    # Compiler/install errors, missing goals and process timeouts are not negative controls.
                    rejected = (result["status"] == "failed" and result["exit_code"] not in {None, 0}
                                and result["coma_generated"]
                                and result["verification"]["goals"].get("vc_aggregate_decision", "").startswith("✘ ")
                                and witness_failed)
                    report["creusot"]["mutations"][name] = {"status": "passed" if rejected else "failed",
                                                               "expected": "proof rejection"}
                    ok = rejected and ok
            report["creusot"]["status"] = "passed" if ok else "failed"
        else:
            report["creusot"]["reason"] = "pinned Creusot/dependencies unavailable; see version step"
        save()

    report["source_unchanged"] = initial == source_manifest()
    requested = [report[key]["status"] for key in ("kani", "creusot") if args.mode in {"all", key}]
    report["status"] = "passed" if all(s == "passed" for s in requested) and report["source_unchanged"] else "failed"
    save()
    print(f"results: {out / 'results.json'}", flush=True)
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
