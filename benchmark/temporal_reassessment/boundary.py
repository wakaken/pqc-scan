import importlib.util
import importlib.metadata
import json
import os
from pathlib import Path
import stat
import sys

sys.dont_write_bytecode = True

CODE_DIR = Path(__file__).resolve().parent
ROOT = Path(os.environ.get('PQC_SCAN_REASSESSMENT_ROOT', str(CODE_DIR.parents[1])))
OWN = Path(os.environ.get('PQC_SCAN_REASSESSMENT_STORAGE', str(CODE_DIR)))
sys.path[:0] = [str(ROOT / 'tests/quality'), str(ROOT / 'benchmark')]
import artifact_preflight as approved

require = approved.require
Rejected = approved.Rejected
MAX = approved.MAX_ARTIFACT
TOTAL = approved.MAX_TOTAL
FORMATS = ('report.json', 'report.html', 'report.rules.html', 'report.md', 'report.sarif',
           'cbom.json', 'dependency-sbom.json', 'diagnostics.json', 'scan.inventory.v1.json',
           'cbom.cdx-1.7.json', 'dependency-sbom.cdx-1.7.json')


def digest(value):
    return approved.canonical_digest(value)


def ref(path, limit=MAX):
    return approved.checked_file(Path(path), limit=limit)


def read(binding, limit=MAX):
    return approved.checked_file(binding['path'], binding, limit=limit, collect=True)


def document(binding):
    return approved.decode(read(binding, approved.MAX_METADATA))


def private_directory(path):
    path = Path(path)
    require(path.is_absolute() and path.is_relative_to(OWN), 'E_WRITE_OWNERSHIP')
    fd = approved._open(path, directory=True)
    try:
        require(stat.S_IMODE(os.fstat(fd).st_mode) == 0o700, 'E_PRIVATE_DIRECTORY')
    finally:
        os.close(fd)
    return path


def require_external_storage():
    require(OWN.is_absolute() and not OWN.is_relative_to(ROOT), 'E_STORAGE_INSIDE_PRODUCT_ROOT')
    return private_directory(OWN)


def mkdir(path):
    path = Path(path)
    require(path.is_absolute() and path.is_relative_to(OWN) and path != OWN, 'E_WRITE_OWNERSHIP')
    fd = approved._open(path.parent, directory=True)
    try:
        os.mkdir(path.name, mode=0o700, dir_fd=fd)
    finally:
        os.close(fd)
    return private_directory(path)


def create(path):
    path = Path(path)
    require(path.is_absolute() and path.is_relative_to(OWN) and path != OWN, 'E_WRITE_OWNERSHIP')
    fd = approved._open(path.parent, directory=True)
    try:
        result = os.open(path.name, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=fd)
        return os.fdopen(result, 'wb')
    finally:
        os.close(fd)


def publish(path, value):
    raw = (json.dumps(value, sort_keys=True, indent=2, ensure_ascii=False, allow_nan=False) + '\n').encode()
    with create(path) as stream:
        stream.write(raw)
        stream.flush()
        os.fsync(stream.fileno())
    return ref(path)


def import_auditors():
    import oss
    import external
    spec = importlib.util.spec_from_file_location('execution_approved_output_audits', ROOT / 'benchmark/audit_outputs.py')
    audit = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(audit)
    return oss, external, audit


def source_bindings(root=ROOT):
    root = Path(root)
    paths = [root / name for name in ('Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml')]
    for folder in ('crates', 'rules', 'schemas'):
        paths.extend(path for path in (root / folder).rglob('*') if path.is_file() or path.is_symlink())
    return {str(path.relative_to(root)): ref(path)['sha256'] for path in sorted(paths)}


def verify_source(expected, root=ROOT):
    require(source_bindings(root) == expected, 'E_FINAL_SOURCE_CHANGED')


def harness_bindings():
    own = CODE_DIR
    paths = list(own.glob('*.py')) + list(own.glob('*.rs')) + list((own / 'tests').glob('*.py'))
    paths += list((ROOT / 'benchmark').rglob('*.py')) + list((ROOT / 'tests/quality').rglob('*.py'))
    paths += [ROOT / 'benchmark/protocol.json', ROOT / 'benchmark/audit-requirements.txt']
    return {str(path): ref(path)['sha256'] for path in sorted(paths)}


def validation_environment():
    packages = {name: importlib.metadata.version(name) for name in
                ('jsonschema', 'referencing', 'jsonschema-specifications', 'attrs', 'rpds-py')}
    requirements = ROOT / 'benchmark/audit-requirements.txt'
    expected = dict(line.strip().split('==', 1) for line in read(ref(requirements)).decode().splitlines() if line.strip())
    require(packages == expected, 'E_AUDIT_PACKAGE_PINS')
    return dict(requirements=ref(requirements), python=ref(Path(sys.executable).resolve()), pythonVersion=sys.version, platform=sys.platform,
                packages=packages, assertionsEnabled=__debug__, performanceObservations=False)
