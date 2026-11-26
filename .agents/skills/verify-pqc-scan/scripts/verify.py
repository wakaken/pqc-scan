#!/usr/bin/env python3
"""Drive pqc-scan with synthetic inputs and retain attributable evidence."""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import platform
import signal
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[4]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def source():
    revision = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    names = subprocess.check_output(
        ["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard"], cwd=ROOT
    ).decode().split("\0")
    hashes = {}
    for name in sorted(set(names) - {""}):
        path = ROOT / name
        if path.is_symlink():
            hashes[name] = hashlib.sha256(os.readlink(path).encode()).hexdigest()
        elif path.is_file():
            hashes[name] = sha(path)
        else:
            hashes[name] = None
    status = subprocess.check_output(["git", "status", "--porcelain=v1", "-z"], cwd=ROOT).decode()
    return dict(revision=revision, status=status, files=hashes,
                sha256=hashlib.sha256(json.dumps(hashes, sort_keys=True).encode()).hexdigest())


def doctor(binary):
    for command in (["rustc", "--version"], ["cargo", "--version"]):
        subprocess.run(command, cwd=ROOT, check=True)
    if not (ROOT / "rules/default/jwt.yml").is_file():
        raise RuntimeError("Default rules are unavailable")
    if binary:
        subprocess.run([str(binary), "--version"], cwd=ROOT, check=True)
        print(json.dumps(dict(binary=str(binary), sha256=sha(binary))))
    print("Doctor passed. No scanner process or server is required.")


def smoke():
    base = ROOT / "target/verification"
    base.mkdir(parents=True, exist_ok=True)
    run = Path(tempfile.mkdtemp(prefix="smoke-", dir=base))
    scratch = Path(tempfile.mkdtemp(prefix="pqc-verify-"))
    manifest = dict(schema_version=1, mode="smoke", status="running", source=source(),
                    platform=platform.platform(), started_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                    configuration=dict(features="default", profile="debug", threads=1),
                    commands=[], assertions=[], artifacts={}, scratch=str(scratch))
    env = dict(os.environ, CARGO_TARGET_DIR=str(run / "cargo-default"))

    def save():
        (run / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")

    def command(name, argv, expected=0):
        record = dict(name=name, argv=[str(x) for x in argv], cwd=str(ROOT), expected_exit=expected,
                      cargo_target_dir=env["CARGO_TARGET_DIR"], exit_code=None)
        manifest["commands"].append(record)
        save()
        with (run / f"{name}.stdout.log").open("wb") as out, (run / f"{name}.stderr.log").open("wb") as err:
            process = subprocess.Popen(record["argv"], cwd=ROOT, env=env, stdout=out, stderr=err, start_new_session=True)
            try:
                record["exit_code"] = process.wait(timeout=1200)
            finally:
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                process.wait()
        save()
        if record["exit_code"] != expected:
            raise RuntimeError(f"{name} exited {record['exit_code']}; expected {expected}. Read retained logs.")

    def require(ok, name):
        manifest["assertions"].append(dict(name=name, passed=bool(ok)))
        save()
        if not ok:
            raise RuntimeError(name)

    def read(path):
        return json.loads(path.read_text())

    try:
        command("rustc-version", ["rustc", "--version"])
        command("cargo-version", ["cargo", "--version"])
        command("build", ["cargo", "build", "--locked", "--bin", "pqc-scan"])
        binary = run / "cargo-default/debug/pqc-scan"
        manifest["binary"] = dict(path=str(binary), sha256=sha(binary))
        command("version", [binary, "--version"])
        command("help", [binary, "--help"])
        command("scan-help", [binary, "scan", "--help"])
        command("rules-list", [binary, "rules", "list", "--rules-dir", ROOT / "rules"])
        positive, negative = scratch / "positive", scratch / "negative"
        positive.mkdir()
        negative.mkdir()
        (positive / "app.rs").write_text('fn main() { let algorithm = "RS256"; }\n')
        (negative / "plain.rs").write_text('fn main() { let answer = 42; }\n')
        shutil.copytree(positive, run / "inputs/positive")
        shutil.copytree(negative, run / "inputs/negative")
        fixture_hashes = {str(path.relative_to(scratch)): sha(path) for path in scratch.rglob("*") if path.is_file()}
        for label, fixture in (("en", positive), ("ja", positive), ("negative", negative)):
            output = run / "reports" / label
            command(f"scan-{label}", [binary, "scan", fixture, "--rules-dir", ROOT / "rules", "--out-dir", output,
                    "--threads", "1", "--format", "all", "--diagnostics", "--inventory-json", "--cyclonedx-17",
                    "--repository-scope", "verification/synthetic", "--lang", "ja" if label == "ja" else "en"])
            report = read(output / "report.json")
            findings = report["findings"]
            require(report["summary"]["scanned_files"] == 1, f"{label}: one selected fixture scanned")
            require(any(f["rule_id"] == "JWT_RS256" and f["location"]["file"] == "app.rs" and
                        f["location"]["line"] == 1 for f in findings) if label != "negative" else not findings,
                    f"{label}: expected positive location or zero negative findings")
            ids = {f["finding_id"] for f in findings}
            sarif = read(output / "report.sarif")
            results = sarif["runs"][0]["results"]
            require(sarif["version"] == "2.1.0" and len(results) == len(findings), f"{label}: SARIF finding count")
            require(sorted(r["ruleId"] for r in results) == sorted(f["rule_id"] for f in findings),
                    f"{label}: SARIF rule identity")
            inventory = read(output / "scan.inventory.v1.json")
            require({x["legacy_id"] for x in inventory["legacy_associations"]} == ids,
                    f"{label}: native inventory legacy references")
            cbom = read(output / "cbom.json")
            require(isinstance(cbom, list) and (any(x["algorithm"] == "RS256" and x["location"] == "app.rs:1"
                    and x["component"] == "app.rs" for x in cbom) if label != "negative" else not cbom),
                    f"{label}: CBOM algorithm and source location")
            for name in ("cbom.cdx-1.7.json", "dependency-sbom.cdx-1.7.json"):
                cdx = read(output / name)
                require(cdx["bomFormat"] == "CycloneDX" and cdx["specVersion"] == "1.7",
                        f"{label}: {name} version")
            require(all((output / name).stat().st_size for name in ("report.html", "report.rules.html", "report.md", "diagnostics.json")),
                    f"{label}: human reports and diagnostics retained")
        en = read(run / "reports/en/report.json")
        ja = read(run / "reports/ja/report.json")
        require(en["findings"] == ja["findings"], "Language preserves machine finding facts and identity")
        for label, option in (("invalid-language", ["--lang", "fr"]), ("invalid-limit", ["--max-file-size-mib", "0"])):
            output = scratch / label
            command(label, [binary, "scan", negative, "--out-dir", output, *option], expected=1)
            require(not output.exists(), f"{label}: rejected before output creation")
        require(fixture_hashes == {str(path.relative_to(scratch)): sha(path) for path in scratch.rglob("*") if path.is_file()},
                "Scanner left synthetic input files unchanged")
        require(sha(binary) == manifest["binary"]["sha256"], "Driven binary remained unchanged")
        manifest["source_after"] = source()
        require(manifest["source"]["sha256"] == manifest["source_after"]["sha256"], "Source remained unchanged during smoke")
        manifest["status"] = "passed"
    except BaseException as error:
        manifest["status"] = "failed"
        manifest["failure"] = f"{type(error).__name__}: {error}"
        raise
    finally:
        shutil.rmtree(scratch)
        manifest["scratch_removed"] = not scratch.exists()
        for path in sorted(run.rglob("*")):
            if path.is_file() and "cargo-default" not in path.relative_to(run).parts and path.name != "manifest.json":
                manifest["artifacts"][str(path.relative_to(run))] = sha(path)
        save()
        print(f"Evidence retained at {run}", flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=("doctor", "smoke"))
    parser.add_argument("--binary", type=Path, help="Existing CLI to inspect in doctor mode")
    args = parser.parse_args()
    if args.mode == "doctor":
        doctor(args.binary.resolve() if args.binary else None)
    else:
        smoke()


if __name__ == "__main__":
    main()
