import hashlib
import json
import os
from pathlib import Path
import stat
import sys
import uuid

RECORDS_ENV = 'PQC_TEMPORAL_COMPILER_RECORDS'
ENVIRONMENT_KEYS = frozenset('''CARGO_BIN_NAME CARGO_CRATE_NAME CARGO_MANIFEST_DIR CARGO_MANIFEST_PATH
CARGO_PKG_AUTHORS CARGO_PKG_DESCRIPTION CARGO_PKG_HOMEPAGE CARGO_PKG_LICENSE CARGO_PKG_LICENSE_FILE
CARGO_PKG_NAME CARGO_PKG_README CARGO_PKG_REPOSITORY CARGO_PKG_RUST_VERSION CARGO_PKG_VERSION
CARGO_PKG_VERSION_MAJOR CARGO_PKG_VERSION_MINOR CARGO_PKG_VERSION_PATCH CARGO_PKG_VERSION_PRE
CARGO_PRIMARY_PACKAGE CARGO_INCREMENTAL CARGO_PROFILE_DEV_DEBUG CARGO_PROFILE_TEST_DEBUG CARGO_TARGET_DIR
PATH RUSTUP_HOME RUSTUP_TOOLCHAIN RUSTC RUSTDOC OUT_DIR HOST TARGET OPT_LEVEL PROFILE DEBUG NUM_JOBS
LD_LIBRARY_PATH DYLD_FALLBACK_LIBRARY_PATH'''.split())
FORBIDDEN_ENVIRONMENT = frozenset('''RUSTC_WORKSPACE_WRAPPER RUSTFLAGS CARGO_ENCODED_RUSTFLAGS
RUSTDOCFLAGS CARGO_ENCODED_RUSTDOCFLAGS RUSTC_BOOTSTRAP LD_PRELOAD DYLD_INSERT_LIBRARIES
PYTHONPATH PYTHONHOME PQC_SCAN_OSS_REPLAY_DETECTOR_SECOND PQC_SCAN_OSS_REPLAY_GENERATED_AT'''.split())
MAX_RECORD_BYTES = 262144


class CaptureRejected(Exception):
    pass


def require(condition):
    if not condition:
        raise CaptureRejected('E_COMPILER_CAPTURE')


def directory(path, *, private=False):
    path = Path(path)
    require(path.is_absolute() and '..' not in path.parts and '.' not in path.parts)
    descriptor = os.open('/', os.O_RDONLY | os.O_DIRECTORY)
    try:
        for part in path.parts[1:]:
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=descriptor)
            os.close(descriptor)
            descriptor = child
        if private:
            require(stat.S_IMODE(os.fstat(descriptor).st_mode) == 0o700)
        return descriptor
    except BaseException:
        os.close(descriptor)
        raise


def file_bytes(path, *, limit=MAX_RECORD_BYTES):
    path = Path(path)
    parent = directory(path.parent)
    try:
        descriptor = os.open(path.name, os.O_RDONLY | os.O_NOFOLLOW, dir_fd=parent)
    finally:
        os.close(parent)
    with os.fdopen(descriptor, 'rb') as stream:
        require(stat.S_ISREG(os.fstat(stream.fileno()).st_mode))
        raw = stream.read(limit + 1)
    require(len(raw) <= limit)
    return raw


def checked_binding(binding, *, limit=MAX_RECORD_BYTES):
    raw = file_bytes(binding['path'], limit=limit)
    require(len(raw) == binding['bytes'] and hashlib.sha256(raw).hexdigest() == binding['sha256'])
    return raw


def capture_configuration(script):
    raw = file_bytes(script.parent / 'capture-config.json')
    config = json.loads(raw)
    require(config['kind'] == 'temporal-rustc-capture-configuration-v1' and
            config['script']['path'] == str(script) and config['recordsDirectory'] == str(script.parent / 'compiler-records') and
            Path(config['storageRoot']).is_absolute() and script.parent.is_relative_to(Path(config['storageRoot'])) and
            not Path(config['storageRoot']).is_relative_to(Path(config['productRoot'])) and
            not Path(config['recordsDirectory']).is_relative_to(Path(config['overlayRoot'])))
    descriptor = directory(script.parent, private=True)
    os.close(descriptor)
    descriptor = directory(config['storageRoot'], private=True)
    os.close(descriptor)
    descriptor = directory(config['recordsDirectory'], private=True)
    os.close(descriptor)
    checked_binding(config['script'])
    return config, hashlib.sha256(raw).hexdigest()


def invocation_record(config, config_hash, argv, environment, cwd):
    manifest = environment.get('CARGO_MANIFEST_DIR')
    if manifest not in config['manifestDirectories']:
        return None
    sources = [arg for arg in argv[1:] if arg.endswith('.rs')]
    if not sources:
        return None
    require(len(sources) == 1)
    source = Path(os.path.abspath(Path(cwd) / sources[0]))
    root = Path(config['overlayRoot'])
    require(Path(cwd).is_absolute() and Path(cwd).is_relative_to(root) and
            source.is_relative_to(Path(manifest)) and source.is_relative_to(root))
    relative = source.relative_to(root).as_posix()
    require(relative in config['sourceBindings'])
    raw = file_bytes(source, limit=16777216)
    require(hashlib.sha256(raw).hexdigest() == config['sourceBindings'][relative])
    if 'CARGO_MANIFEST_PATH' in environment:
        require(environment['CARGO_MANIFEST_PATH'] == str(Path(manifest) / 'Cargo.toml'))
    row = dict(kind='temporal-rustc-invocation-capture-v1', captureConfigurationSHA256=config_hash,
               captureSourceSHA256=config['script']['sha256'], argv=argv, cwd=cwd,
               environment={key: environment[key] for key in sorted(ENVIRONMENT_KEYS) if key in environment},
               sourcePath=str(source), manifestDirectory=manifest)
    require(len(json.dumps(row).encode()) <= MAX_RECORD_BYTES)
    return row


def main():
    script = Path(os.path.abspath(__file__))
    config, config_hash = capture_configuration(script)
    compiler = config['compiler']
    argv = sys.argv[1:]
    require(argv and argv[0] == compiler['path'] and os.environ.get('RUSTC') == compiler['path'] and
            os.environ.get('RUSTC_WRAPPER') == str(script) and os.environ.get(RECORDS_ENV) == config['recordsDirectory'] and
            all(not os.environ.get(key) for key in FORBIDDEN_ENVIRONMENT) and
            all(os.environ.get(key) == value for key, value in config['profileEnvironment'].items()))
    checked_binding(compiler, limit=134217728)
    row = invocation_record(config, config_hash, argv, os.environ, os.getcwd())
    if row is not None:
        descriptor = directory(config['recordsDirectory'], private=True)
        try:
            name = 'invocation-' + uuid.uuid4().hex + '.json'
            output = os.open(name, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=descriptor)
            with os.fdopen(output, 'wb') as stream:
                stream.write((json.dumps(row, sort_keys=True, ensure_ascii=True) + '\n').encode())
                stream.flush()
                os.fsync(stream.fileno())
        finally:
            os.close(descriptor)
    os.execv(compiler['path'], argv)


if __name__ == '__main__':
    try:
        main()
    except (CaptureRejected, OSError, ValueError, KeyError, TypeError):
        sys.stderr.write('E_COMPILER_CAPTURE\n')
        sys.exit(125)
