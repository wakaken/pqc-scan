"""macOS file/environment/network boundary, with finite per-process limits.

This is not a hostile-code VM: no memory limit, aggregate disk quota or process-tree memory
accounting is provided. OS read-only source rules and no-egress apply to children.
"""
import json
import os
from pathlib import Path
import resource
import signal
import subprocess
import sys


def environment(scratch, toolchain=None):
    scratch = Path(scratch).resolve()
    for name in ['home', 'tmp', 'cargo']:
        (scratch / name).mkdir(exist_ok=True)
    paths = [str(Path(sys.executable).resolve().parent), '/usr/bin', '/bin', '/usr/sbin', '/sbin']
    if toolchain:
        paths.insert(0, str(Path(toolchain).resolve().parent))
    return {'PATH': os.pathsep.join(paths), 'HOME': str(scratch / 'home'),
            'TMPDIR': str(scratch / 'tmp'), 'CARGO_HOME': str(scratch / 'cargo'),
            'CARGO_NET_OFFLINE': 'true', 'LC_ALL': 'C', 'RUST_TEST_THREADS': '2'}


def profile(source, scratch, read_paths=(), write_paths=()):
    def subpath(path):
        return '(subpath ' + json.dumps(str(Path(path).resolve())) + ')'
    reads = [source, scratch, '/System', '/usr', '/bin', '/sbin',
             '/opt/homebrew/Cellar', '/opt/homebrew/opt', '/Library/Developer', '/Applications/Xcode.app', '/private/etc', '/dev',
             *read_paths]
    writes = [scratch, *write_paths]
    return '\n'.join([
        '(version 1)', '(deny default)',
        '(allow process* signal sysctl-read mach-lookup)',
        '(allow file-read* (literal "/"))',
        '(allow file-read-metadata)',
        '(allow file-read* ' + ' '.join(map(subpath, reads)) + ')',
        '(allow file-write* ' + ' '.join(map(subpath, writes)) + ' (literal "/dev/null"))',
        '(allow network-bind (local ip "localhost:*"))',
        '(allow network-inbound (local ip "localhost:*"))',
        '(allow network-outbound (remote ip "localhost:*"))',
    ])


def limits():
    for key, cap in [(resource.RLIMIT_CPU, 900),
                     (resource.RLIMIT_FSIZE, 1024**3), (resource.RLIMIT_NOFILE, 1024),
                     (resource.RLIMIT_NPROC, 1024), (resource.RLIMIT_CORE, 0)]:
        _, hard = resource.getrlimit(key)
        value = cap if hard == resource.RLIM_INFINITY else min(cap, hard)
        resource.setrlimit(key, (value, value))


def run(command, cwd, env, policy, timeout, **kwargs):
    process = subprocess.Popen(['/usr/bin/sandbox-exec', '-p', policy, *map(str, command)],
                               cwd=cwd, env=env, start_new_session=True, preexec_fn=limits,
                               **{k: v for k, v in kwargs.items() if k != 'capture_output'},
                               **({'stdout': subprocess.PIPE, 'stderr': subprocess.PIPE}
                                  if kwargs.get('capture_output') else {}))
    try:
        stdout, stderr = process.communicate(timeout=timeout)
        return subprocess.CompletedProcess(command, process.returncode, stdout, stderr)
    finally:
        # Reap descendants on timeout as well as on ordinary completion.
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        process.wait()
