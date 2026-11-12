#!/usr/bin/env python3
"""Repeatable local/CI baseline runner; Python standard library only."""
import argparse
import datetime
import json
import os
import pathlib
import platform
import shutil
import signal
import statistics
import subprocess
import sys
import tempfile
import time

from evaluate import (bytes_digest, canonical, check_coverage, digest, load_truth,
                      order_digest, safe_relative, score, semantic_digest, validate_corpus)
from taxonomy import select_corpus

REPO = pathlib.Path(__file__).resolve().parents[2]


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False, allow_nan=False) + "\n")


def measure(command, directory, timeout):
    directory.mkdir(parents=True, exist_ok=True)
    start = time.monotonic()
    timed_out, usage = False, None
    with (directory / "stdout.log").open("wb") as stdout, (directory / "stderr.log").open("wb") as stderr:
        proc = subprocess.Popen(command, cwd=REPO, stdout=stdout, stderr=stderr,
                                start_new_session=os.name == "posix")
        if hasattr(os, "wait4"):
            while True:
                pid, status, observed = os.wait4(proc.pid, os.WNOHANG)
                if pid:
                    usage = observed
                    proc.returncode = os.waitstatus_to_exitcode(status)
                    break
                if time.monotonic() - start >= timeout:
                    timed_out = True
                    try:
                        os.killpg(proc.pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
                    _, status, usage = os.wait4(proc.pid, 0)
                    proc.returncode = os.waitstatus_to_exitcode(status)
                    break
                time.sleep(0.005)
        else:
            try:
                proc.wait(timeout=timeout)
            except subprocess.TimeoutExpired:
                timed_out = True
                proc.kill()
                proc.wait()
    rss = None
    if usage is not None and sys.platform in {"darwin", "linux"}:
        rss = usage.ru_maxrss * (1 if sys.platform == "darwin" else 1024)
    return dict(command=[str(x) for x in command], exitCode=proc.returncode, timedOut=timed_out,
                wallSeconds=time.monotonic() - start,
                cpuSeconds=usage.ru_utime + usage.ru_stime if usage else None,
                peakRssBytes=rss, resourceMethod="wait4 per child" if usage else "unavailable",
                rssNativeUnit="bytes" if sys.platform == "darwin" else "KiB" if sys.platform == "linux" else "unavailable")


def materialize(data, repo, root, copies):
    root.mkdir(parents=True, exist_ok=False)
    bindings = {}
    for index in range(copies):
        for file in data["files"]:
            name = file["path"] if copies == 1 else f"copy-{index:03}/" + file["path"]
            target = root / safe_relative(name)
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(repo / file["source"], target)
            if bytes_digest(target) != file["sha256"]:
                raise ValueError("Materialized input digest mismatch")
            bindings[name] = dict(sha256=file["sha256"], bytes=target.stat().st_size)
    return dict(files=len(bindings), bytes=sum(f["bytes"] for f in bindings.values()),
                sha256=digest(bindings), bindings=bindings)


def distributions(samples):
    output = {}
    for field in ["wallSeconds", "cpuSeconds", "peakRssBytes"]:
        values = [s[field] for s in samples]
        output[field] = (dict(min=min(values), median=statistics.median(values), max=max(values), samples=values)
                         if all(x is not None for x in values) else None)
    return output


def scan(binary, root, directory, expected_files, canaries, timeout):
    command = [str(binary), "scan", str(root), "--format", "all", "--diagnostics",
               "--threads", "1", "--rules-dir", str(REPO / "rules"), "--out-dir", str(directory)]
    sample = measure(command, directory, timeout)
    write_json(directory / "measurement.json", sample)
    if sample["exitCode"] != 0 or sample["timedOut"]:
        raise RuntimeError("Scanner failed or timed out; retain measurement/logs")
    for path in directory.iterdir():
        if path.is_file() and any(c.encode() in path.read_bytes() for c in canaries):
            raise ValueError("Synthetic private-body canary survived output masking")
    report = json.loads((directory / "report.json").read_text())
    diagnostics = json.loads((directory / "diagnostics.json").read_text())
    coverage = check_coverage(diagnostics, expected_files)
    if report["summary"]["scanned_files"] != expected_files:
        raise ValueError("CLI report did not read all fixture files")
    sample.update(semanticDigest=semantic_digest(report), orderDigest=order_digest(report),
                  coverage=coverage, scope=diagnostics["scope"],
                  diagnosticCodes=sorted({d["code"] for d in diagnostics["diagnostics"]}),
                  diagnosticDigest=digest({k: v for k, v in diagnostics.items() if k != "generated_at"}),
                  findingCount=len(report["findings"]), outputBytes=sum(p.stat().st_size for p in directory.iterdir() if p.is_file()))
    write_json(directory / "measurement.json", sample)
    return sample, report


def command_text(command):
    result = subprocess.run(command, cwd=REPO, capture_output=True, text=True, check=False)
    return dict(command=command, exitCode=result.returncode, stdout=result.stdout.strip())


def environment(binary):
    files = [REPO / "Cargo.toml", REPO / "Cargo.lock", REPO / "rust-toolchain.toml"]
    for root, patterns in [(REPO / "crates", ["*.rs", "*.js", "*.css", "Cargo.toml"]), (REPO / "rules", ["*.yml"])]:
        for pattern in patterns:
            files.extend(root.rglob(pattern))
    runtime = {str(f.relative_to(REPO)): bytes_digest(f) for f in sorted(set(files))}
    harness = {str(f.relative_to(REPO)): bytes_digest(f) for f in sorted((REPO / "tests/quality").rglob('*'))
               if f.is_file() and '__pycache__' not in f.parts}
    hardware = {}
    if sys.platform == "darwin":
        for key in ["machdep.cpu.brand_string", "hw.memsize", "hw.ncpu"]:
            hardware[key] = command_text(["sysctl", "-n", key])
    elif sys.platform == "linux":
        for name in ["meminfo", "cpuinfo"]:
            text = pathlib.Path('/proc', name).read_text()
            hardware[name] = [line for line in text.splitlines() if line.startswith(('MemTotal:', 'model name', 'Hardware'))][:1]
    return dict(scannerVersion=command_text([str(binary), "--version"]),
                head=command_text(["git", "rev-parse", "HEAD"]),
                branch=command_text(["git", "branch", "--show-current"]),
                worktree=command_text(["git", "status", "--short"]),
                rust=command_text(["rustc", "-Vv"]), cargo=command_text(["cargo", "-V"]),
                python=sys.version, platform=platform.platform(), machine=platform.machine(),
                cpuCount=os.cpu_count(), hardware=hardware,
                binary=str(binary), binarySha256=bytes_digest(binary),
                buildProfile="caller supplied; use --release for published performance baseline",
                features="workspace defaults, no extra features requested",
                runtimeSourceBindings=runtime, runtimeSourceDigest=digest(runtime),
                harnessBindings=harness, harnessDigest=digest(harness),
                clockUtc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                cachePolicy="first measured run after materialization, then warm; OS cold cache unmeasured",
                wallTimer="monotonic, 5 ms child-completion polling; launch/report writing included")


def run(args):
    binary = pathlib.Path(args.binary).resolve(strict=True)
    out = pathlib.Path(args.out_dir).resolve()
    if not out.is_relative_to(REPO / "target") or out == REPO / "target":
        raise ValueError("Output must be a new directory under repository target/")
    if out.exists() and any(out.iterdir()):
        raise ValueError("Refusing to overwrite prior evidence")
    out.mkdir(parents=True, exist_ok=True)
    corpus_dir = select_corpus(REPO, getattr(args, 'corpus_dir', None))
    data, labels = load_truth(corpus_dir)
    validate_corpus(data, REPO)
    result = dict(version=1, environment=environment(binary),
                  corpusVersion=data["corpusVersion"], corpusSha256=bytes_digest(corpus_dir / "corpus.json"),
                  truthSha256=bytes_digest(corpus_dir / "truth.json"),
                  budgets=dict(status="unapproved; descriptive baseline only", enforced=False))
    result["environment"]["buildProfile"] = args.build_profile
    if data.get('annotationVersion'):
        from taxonomy import utc_time
        truth = json.loads((corpus_dir/'truth.json').read_text())
        if utc_time(result['environment']['clockUtc']) < utc_time(truth['freeze']['timestampUtc']):
            raise ValueError('Measurement timestamp precedes truth freeze')
    if corpus_dir != REPO/'tests/quality':
        result['corpusDirectory'] = str(corpus_dir.relative_to(REPO))
    write_json(out / "environment.json", result["environment"])
    with tempfile.TemporaryDirectory(prefix="pqc023-input-") as tmp:
        root = pathlib.Path(tmp)
        corpus_root = root / "accuracy"
        result["corpusInput"] = materialize(data, REPO, corpus_root, 1)
        accuracy_samples, scores = [], []
        for index in range(args.repeats):
            sample, report = scan(binary, corpus_root, out / f"accuracy-{index}", len(data["files"]), data["canaries"], args.timeout)
            accuracy_samples.append(sample)
            scores.append(score(data, labels, report["findings"]))
        result["accuracy"] = scores[0]
        result["repeatability"] = dict(repeats=args.repeats,
            semanticStable=len({s["semanticDigest"] for s in accuracy_samples}) == 1,
            diagnosticStable=len({s["diagnosticDigest"] for s in accuracy_samples}) == 1,
            rawOrderStable=len({s["orderDigest"] for s in accuracy_samples}) == 1,
            scoreStable=len({digest(s) for s in scores}) == 1, samples=accuracy_samples)
        if not all(result["repeatability"][k] for k in ["semanticStable", "diagnosticStable", "scoreStable"]):
            write_json(out / "summary.json", result)
            raise ValueError("Semantic/diagnostic/accuracy repeatability failed")
        result["benchmarks"] = {}
        if not args.skip_benchmarks:
            for name, copies in [("small", 1), ("medium", 8), ("large", 32), ("pathological", 0)]:
                bench_root = root / name
                if copies:
                    info = materialize(data, REPO, bench_root, copies)
                    recipe = f"{copies} deterministic complete corpus copies; not accuracy truth"
                else:
                    bench_root.mkdir()
                    content = 'const data = "' + 'RS256 ' * 10000 + '";\n'
                    (bench_root / "repeated.js").write_text(content)
                    info = dict(files=1, bytes=len(content.encode()), sha256=digest({"repeated.js": dict(sha256=bytes_digest(bench_root / 'repeated.js'), bytes=len(content.encode()))}))
                    recipe = 'one JS string with 10000 repeated RSA JWT tokens, intentionally bounded/truncated'
                samples = []
                for index in range(args.warm_samples + 1):
                    sample, _ = scan(binary, bench_root, out / f"{name}-{index}", info["files"], data["canaries"], args.timeout)
                    sample["cacheClass"] = "first-measured" if index == 0 else "warm"
                    samples.append(sample)
                if name == "pathological" and 'detector_truncated' not in samples[0]["diagnosticCodes"]:
                    raise ValueError("Pathological case did not demonstrate bounded truncation")
                result["benchmarks"][name] = dict(recipe=recipe, input=info, samples=samples,
                    warmDistribution=distributions(samples[1:]), firstMeasured=samples[0])
                print(f"Measured {name}: {info['files']} files / {info['bytes']} bytes", flush=True)
        write_json(out / "summary.json", result)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", default=str(REPO / "target/release/pqc-scan"))
    parser.add_argument("--out-dir", default=str(REPO / "target/task023/baseline"))
    parser.add_argument("--repeats", type=int, default=3)
    parser.add_argument("--warm-samples", type=int, default=3)
    parser.add_argument("--timeout", type=float, default=60)
    parser.add_argument("--build-profile", choices=["release", "debug", "unspecified"], default="unspecified")
    parser.add_argument("--skip-benchmarks", action="store_true")
    parser.add_argument("--corpus-dir", type=pathlib.Path, help="Optional frozen corpus under tests/quality; default is TASK-023")
    args = parser.parse_args()
    if args.repeats < 2 or args.warm_samples < 3 or not 0 < args.timeout <= 60:
        parser.error("Need >=2 accuracy repeats, >=3 warm samples and 0 < timeout <=60 seconds")
    try:
        summary = run(args)
    except (ValueError, RuntimeError, OSError, KeyError) as error:
        print(f"Baseline failed: {error}", file=sys.stderr)
        return 1
    print(f"Baseline saved. Semantic stable: {summary['repeatability']['semanticStable']}. Budgets unapproved.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
