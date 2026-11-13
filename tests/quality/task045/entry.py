"""Run the fixed TASK-045 sample inside the shared offline container boundary."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import time

sys.path.insert(0, '/source/tests/protocol')
sys.path.insert(0, '/source/tests/quality')
from container_entry import boundary, EVIDENCE, SCRATCH, write
from task045.study import apply, classify, summarize

ROOT = SCRATCH/'mutation-source'
COMMANDS = []


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def command(args, name, timeout):
    start = time.monotonic()
    with (EVIDENCE/name).open('w') as out, (EVIDENCE/(name+'.stderr')).open('w') as err:
        proc = subprocess.Popen(args, cwd=ROOT, stdout=out, stderr=err, start_new_session=True)
        try:
            status = proc.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            os.killpg(proc.pid, signal.SIGKILL)
            proc.wait()
            status = 'timeout'
    COMMANDS.append({'command':args, 'cwd':str(ROOT), 'exit':status, 'log':name,
                     'timeoutSeconds':timeout, 'wallSeconds':time.monotonic()-start})
    write('commands.json', COMMANDS)
    return status


def evaluate(package, label, timeout):
    selection = ['--workspace'] if package == 'workspace' else ['-p', package]
    args = ['cargo', '+1.93.1', 'test', *selection, '--all-features', '--locked',
            '--offline', '--no-run', '--message-format=json']
    build = command(args, label+'-build.jsonl', timeout)
    if build != 0:
        return classify(build, None, '', (EVIDENCE/(label+'-build.jsonl')).read_text() +
                        (EVIDENCE/(label+'-build.jsonl.stderr')).read_text())
    binaries = {}
    for line in (EVIDENCE/(label+'-build.jsonl')).read_text().splitlines():
        row = json.loads(line)
        if row.get('reason') == 'compiler-artifact' and row.get('executable') and row.get('profile',{}).get('test'):
            path = row['executable']
            binaries[path] = sha(path)
    if not binaries:
        raise RuntimeError('no test executables')
    write(label+'-binaries.json', binaries)
    combined = []
    outcome = 'missed'
    for index, path in enumerate(sorted(binaries)):
        name = f'{label}-test-{index:02}.log'
        code = command([path, '--test-threads=2'], name, timeout)
        log = (EVIDENCE/name).read_text()
        if sha(path) != binaries[path]:
            raise RuntimeError('test executable changed during execution')
        # Empty test targets do not establish a surviving mutant, but can coexist
        # with actual tests in the package. Still reject their abnormal exit.
        if code == 0 and 'test result: ok. 0 passed; 0 failed;' in log:
            continue
        state = classify(0, code, log)
        combined.append(log)
        if state == 'caught':
            outcome = 'caught'
            break
        if state == 'timeout':
            return state
    if not combined:
        raise RuntimeError('all test targets were empty')
    return outcome


def controls():
    """Exercise every classifier state with real small Rust test executables."""
    folder = SCRATCH/'controls'
    folder.mkdir()
    fixtures = {
        'missed': '#[test] fn control() { assert_eq!(2 + 2, 4); }',
        'caught': '#[test] fn control() { assert_eq!(2 + 2, 5); }',
        'unviable': '#[test] fn control() { DOES_NOT_EXIST; }',
        'timeout': '#[test] fn control() { loop { std::thread::sleep(std::time::Duration::from_secs(1)); } }',
    }
    results = {}
    for expected, source in fixtures.items():
        path = folder/(expected+'.rs')
        path.write_text(source+'\n')
        binary = folder/expected
        build = command(['rustc', '+1.93.1', '--test', str(path), '-o', str(binary)],
                        'control-'+expected+'-build.log', 30)
        test = None
        log = ''
        if build == 0:
            name = 'control-'+expected+'-test.log'
            test = command([str(binary)], name, .2 if expected == 'timeout' else 10)
            log = (EVIDENCE/name).read_text()
        actual = classify(build, test, log,
                          (EVIDENCE/('control-'+expected+'-build.log.stderr')).read_text())
        results[expected] = {'source':source, 'sourceSha256':sha(path),
                             'buildExit':build, 'testExit':test, 'actual':actual}
        if actual != expected:
            raise RuntimeError('classifier control failed')
    write('controls.json', results)


def main():
    write('checks.json', boundary())
    shutil.copytree('/source', ROOT, ignore=shutil.ignore_patterns('target'))
    for path in ROOT.rglob('*'):
        if path.is_file():
            path.chmod(0o644)
    (ROOT/'target').symlink_to('/scratch/test-target', target_is_directory=True)
    (SCRATCH/'test-target').mkdir()
    cargo = SCRATCH/'cargo'
    cargo.mkdir()
    for name in ('registry', 'git'):
        cache = Path('/usr/local/cargo')/name
        if cache.exists():
            (cargo/name).symlink_to(cache, target_is_directory=True)
    command(['rustc','-Vv'], 'rustc.log', 30)
    command(['cargo','-V'], 'cargo.log', 30)
    rows = json.loads((ROOT/'tests/quality/task045/mutations.json').read_text())
    controls()
    if evaluate('workspace', 'baseline-workspace', 700) != 'missed':
        raise RuntimeError('unmutated workspace baseline must pass')
    results = []
    start = time.monotonic()
    for row in rows:
        if time.monotonic()-start > 1200:
            raise RuntimeError('sample budget exhausted; unfinished rows remain')
        path = ROOT/row['path']
        original = path.read_bytes()
        try:
            before, after = apply(ROOT, row)
            package = row['path'].split('/')[1]
            outcome = evaluate(package, row['id'], 90)
            primary = outcome
            if outcome == 'missed':
                outcome = evaluate('workspace', row['id']+'-workspace', 90)
            results.append(dict(row, primaryOutcome=primary, outcome=outcome,
                                beforeSha256=before, afterSha256=after))
            write('results.json', {'mutations':results, **summarize(results)})
        finally:
            path.write_bytes(original)
            if path.read_bytes() != original:
                raise RuntimeError('mutation restoration failed')
    # Rebuild and rerun the unmutated sample to catch stale-binary/cache mistakes.
    if evaluate('workspace', 'restored-workspace', 180) != 'missed':
        raise RuntimeError('restored workspace baseline did not pass')
    source_hashes = {str(p.relative_to(ROOT)):sha(p) for p in ROOT.glob('crates/**/src/**/*.rs')}
    if any(sha(Path('/source')/name) != digest for name,digest in source_hashes.items()):
        raise RuntimeError('scratch product source not restored')
    write('restored-source.json', source_hashes)


if __name__ == '__main__':
    main()
