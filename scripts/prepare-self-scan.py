#!/usr/bin/env python3
"""Prepare a byte-preserving product-only CI scan input. Never execute inputs."""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path, PurePosixPath
import sys

REPO = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('self_scan_source', REPO/'tests/protocol/container_runner.py')
source = importlib.util.module_from_spec(spec)
spec.loader.exec_module(source)

MAX_FILE_BYTES = 2 * 1024**2
MAX_TOTAL_BYTES = 64 * 1024**2
MAX_FILES = 4096
EXCLUDED_COMPONENTS = {'.git', 'node_modules', 'target', 'dist', 'docs'}


def tree_files(directory):
    def fail(error):
        raise error

    if directory.is_symlink() or not directory.is_dir():
        raise ValueError('Missing or linked source directory')
    for parent, folders, files in os.walk(directory, followlinks=False, onerror=fail):
        if any((Path(parent)/folder).is_symlink() for folder in folders):
            raise ValueError('Linked source directory')
        for name in files:
            yield Path(parent)/name


def product_files(root):
    """Enumerate only closed product locations, never traverse the test corpus."""
    names = ['Cargo.toml', 'Cargo.lock']
    if not all((root/name).is_file() for name in names):
        raise ValueError('Required product inputs are missing')

    def add(file):
        names.append(file.relative_to(root).as_posix())

    if (root/'build.rs').exists() or (root/'build.rs').is_symlink():
        add(root/'build.rs')
    crates = root/'crates'
    if crates.is_symlink():
        raise ValueError('Linked source directory')
    for crate in sorted(crates.iterdir()):
        if crate.is_symlink():
            raise ValueError('Linked source directory')
        if not crate.is_dir():
            continue
        if not (crate/'Cargo.toml').is_file():
            raise ValueError('Crate manifest missing')
        add(crate/'Cargo.toml')
        if (crate/'build.rs').exists() or (crate/'build.rs').is_symlink():
            add(crate/'build.rs')
        for file in tree_files(crate/'src'):
            add(file)
    rules = root/'rules/default'
    if rules.is_symlink() or rules.parent.is_symlink():
        raise ValueError('Linked rule directory')
    for file in sorted(rules.glob('*.yml')):
        add(file)
    names = sorted(set(names))
    if any(part.lower() in EXCLUDED_COMPONENTS for name in names for part in PurePosixPath(name).parts):
        raise ValueError('Product path would be excluded by the scanner')
    if not any('/src/' in name and name.endswith('.rs') for name in names) \
            or not any(name.startswith('rules/default/') for name in names):
        raise ValueError('Product source or rules missing')
    if len(names) > MAX_FILES:
        raise ValueError('Product input count limit exceeded')
    return names


def prepare(root, out, expectations):
    root = Path(root).resolve(strict=True)
    out = Path(out).resolve()
    if out.is_relative_to(root) or root.is_relative_to(out):
        raise ValueError('Scan input must be outside the repository')
    if any(part.lower() in EXCLUDED_COMPONENTS for part in out.parts):
        raise ValueError('Scan input would be excluded by the scanner')
    if out.exists():
        raise ValueError('Refusing to overwrite an existing input directory')
    names = product_files(root)
    policy_bytes = source.source_bytes(root, str(PurePosixPath(expectations)), limit=8*1024**2)
    policy = json.loads(policy_bytes)
    if not isinstance(policy, dict) or set(policy) != {'version', 'files'} or policy['version'] != 1 \
            or isinstance(policy['version'], bool) \
            or not isinstance(policy['files'], list) \
            or any(not isinstance(file, dict) or not isinstance(file.get('path'), str) for file in policy['files']):
        raise ValueError('Invalid expectation policy')
    # Intersect existing allowances with copied paths. Never learn from scan output
    # or replace stale source/result hashes. The existing projection validates them.
    scoped_policy = {'version': 1, 'files': [file for file in policy['files'] if file['path'] in names]}
    out.mkdir(parents=True, mode=0o700)
    input_root = out/'input'
    input_root.mkdir()
    files = []
    total = 0
    for name in names:
        data = source.source_bytes(root, name, limit=MAX_FILE_BYTES)
        total += len(data)
        if total > MAX_TOTAL_BYTES:
            raise ValueError('Product input byte limit exceeded')
        destination = input_root/name
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes(data)
        destination.chmod(0o444)
        files.append({'path': name, 'sha256': hashlib.sha256(data).hexdigest(), 'bytes': len(data)})
    if names != product_files(root) or any(
        hashlib.sha256(source.source_bytes(root, item['path'], limit=MAX_FILE_BYTES)).hexdigest() != item['sha256']
        for item in files
    ):
        raise ValueError('Product sources changed during preparation')
    manifest = {
        'schemaVersion': 1,
        'scope': 'product-source',
        'included': ['Cargo.toml', 'Cargo.lock', 'build.rs (if present)',
                     'crates/*/Cargo.toml', 'crates/*/build.rs (if present)', 'crates/*/src/**', 'rules/default/*.yml'],
        'limitations': 'Colocated source test modules remain byte-identical; no cfg stripping or source rewriting.',
        'fileCount': len(files), 'byteCount': total, 'files': files,
        'expectationsSourceSha256': hashlib.sha256(policy_bytes).hexdigest(),
        'scopedExpectationFiles': len(scoped_policy['files']),
    }
    (out/'expectations.json').write_text(json.dumps(scoped_policy, indent=2)+'\n')
    # Written last: an incomplete preparation cannot declare its inputs complete.
    (out/'manifest.json').write_text(json.dumps(manifest, indent=2)+'\n')
    return manifest


def verify(root, out, diagnostics):
    root, out = Path(root).resolve(strict=True), Path(out).resolve(strict=True)
    manifest = json.loads(source.source_bytes(out, 'manifest.json', limit=2*1024**2))
    if not isinstance(manifest, dict) or manifest.get('schemaVersion') != 1 \
            or manifest.get('scope') != 'product-source' or not isinstance(manifest.get('files'), list):
        raise ValueError('Invalid product manifest')
    files = manifest['files']
    if not files or len(files) > MAX_FILES or any(not isinstance(item, dict)
            or not isinstance(item.get('path'), str) or type(item.get('bytes')) is not int for item in files):
        raise ValueError('Invalid product manifest files')
    names = sorted(item['path'] for item in files)
    copied = sorted(file.relative_to(out/'input').as_posix() for file in tree_files(out/'input'))
    if names != product_files(root) or copied != names or manifest.get('fileCount') != len(files):
        raise ValueError('Product input population changed')
    total = 0
    for item in files:
        for folder, relative in [(root, item['path']), (out, 'input/'+item['path'])]:
            data = source.source_bytes(folder, relative, limit=MAX_FILE_BYTES)
            if len(data) != item.get('bytes') or hashlib.sha256(data).hexdigest() != item.get('sha256'):
                raise ValueError('Product input bytes changed')
        total += item['bytes']
    if total != manifest.get('byteCount') or total > MAX_TOTAL_BYTES:
        raise ValueError('Product input byte count mismatch')
    path = Path(diagnostics).absolute()
    diagnostic_bytes = source.source_bytes(path.parent, path.name, limit=8*1024**2)
    report = json.loads(diagnostic_bytes)
    if not isinstance(report, dict) or report.get('schema_version') != '1' \
            or not isinstance(report.get('coverage'), dict):
        raise ValueError('Invalid scanner diagnostics')
    coverage = report['coverage']
    # Inventory + unchanged input tree + exact selected/read/analyzed counts guard
    # against hidden ignore rules, missing files and unreadable/partial inspections.
    expected = {'selected_files': len(files), 'read_ok_files': len(files), 'analyzed_files': len(files),
                'read_failed_files': 0, 'partial_files': 0, 'analysis_failed_files': 0,
                'unsupported_files': 0, 'not_applicable_files': 0, 'unknown_files': 0,
                'excluded_enumerated_files': 0, 'enumeration_failed_entries': 0}
    if coverage.get('status') != 'complete_within_scope' or any(
            type(coverage.get(key)) is not int or coverage[key] != value for key, value in expected.items()):
        raise ValueError('Scanner did not completely read the declared product inputs')
    result = {'schemaVersion': 1, 'scope': 'product-source', 'verifiedFiles': len(files),
              'verifiedBytes': total, 'manifestSha256': hashlib.sha256(source.source_bytes(out, 'manifest.json')).hexdigest(),
              'diagnosticsSha256': hashlib.sha256(diagnostic_bytes).hexdigest()}
    (out/'verification.json').write_text(json.dumps(result, indent=2)+'\n')
    return result


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path.cwd())
    parser.add_argument('--out-dir', type=Path, required=True)
    parser.add_argument('--expectations', default='docs/quality/self-scan/SELF-SCAN-EXPECTATIONS.json')
    parser.add_argument('--verify', action='store_true')
    parser.add_argument('--diagnostics', type=Path)
    args = parser.parse_args(argv)
    try:
        if args.verify:
            if args.diagnostics is None:
                raise ValueError('Diagnostics required for verification')
            result = verify(args.root, args.out_dir, args.diagnostics)
        else:
            if args.diagnostics is not None:
                raise ValueError('Diagnostics require verification mode')
            manifest = prepare(args.root, args.out_dir, args.expectations)
    except (ValueError, OSError):
        phase = 'verification' if args.verify else 'preparation'
        print(f'Product self-scan {phase} failed; no complete scan is asserted.', file=sys.stderr)
        return 1
    if args.verify:
        print(f"Verified all {result['verifiedFiles']} product files were read and analyzed.")
        return 0
    print(f"Prepared {manifest['fileCount']} product files ({manifest['byteCount']} bytes); input inventory written.")
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
