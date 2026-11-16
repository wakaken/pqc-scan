#!/usr/bin/env python3
"""Offline task-051 dependency/notice assessment; never builds or updates crates."""
import argparse
from datetime import datetime, timezone
from functools import lru_cache
import gzip
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import re
import subprocess
import tarfile
import tomllib

VARIANTS = {
    'static': [],
    'network': ['probe-tls', 'probe-ssh', 'probe-ssh-kex'],
}
TARGET = 'aarch64-apple-darwin'
REGISTRY = 'registry+https://github.com/rust-lang/crates.io-index'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def encoded(value):
    return (json.dumps(value, sort_keys=True, indent=2)+'\n').encode()


def identity(package):
    return package['name'], package['version'], package.get('source')


def closure(metadata):
    """Match packaging's normal/build closure, retaining resolved feature facts."""
    packages = {p['id']: p for p in metadata['packages']}
    nodes = {n['id']: n for n in metadata['resolve']['nodes']}
    roots = [p['id'] for p in packages.values() if p['name'] == 'pqc_scan_cli']
    if len(roots) != 1:
        raise ValueError('expected one CLI package')
    selected, todo = set(), roots[:]
    while todo:
        ident = todo.pop()
        if ident in selected:
            continue
        selected.add(ident)
        todo.extend(d['pkg'] for d in nodes[ident]['deps']
                    if any(k['kind'] in (None, 'build') for k in d['dep_kinds']))
    return [(packages[i], nodes[i]) for i in sorted(selected)]


def audit_findings(report, exit_code):
    """Reject tool errors/partial JSON instead of interpreting them as no findings."""
    vulnerabilities = report['vulnerabilities']
    entries = vulnerabilities['list']
    warnings = report['warnings']
    if (type(entries) is not list or type(warnings) is not dict or
            type(vulnerabilities['count']) is not int or
            vulnerabilities['count'] != len(entries) or
            type(vulnerabilities['found']) is not bool or
            vulnerabilities['found'] != bool(entries) or
            exit_code != (1 if entries else 0) or
            report['settings']['ignore'] != []):
        raise ValueError('failed, inconsistent or filtered audit')
    result = []
    for kind, items in [('vulnerability', entries), *sorted(warnings.items())]:
        if type(items) is not list:
            raise ValueError('invalid audit warnings')
        for item in items:
            advisory = item['advisory']
            package = item['package']
            if (not re.fullmatch(r'RUSTSEC-\d{4}-\d{4}', advisory['id']) or
                    advisory['package'] != package['name'] or not package['version'] or
                    package.get('source') != REGISTRY):
                raise ValueError('invalid advisory identity/source')
            result.append({'kind': kind, 'package': {k: package[k] for k in ('name', 'version', 'source')},
                           'advisory': advisory, 'versions': item.get('versions'),
                           'affected': item.get('affected'),
                           'action': 'Review affected use and patched ranges; propose upgrade/replacement and rerun feature, detector and package checks before publication.',
                           'exception': {'accepted': False, 'maintainer_decision': None,
                                         'expires_at': None, 'reason': 'No exception requested or granted.'}})
    return result


@lru_cache(maxsize=512)
def registry_files(directory, checksum):
    """Bind collected registry files to the actual locked .crate archive."""
    archive = directory.parents[2]/'cache'/directory.parent.name/(directory.name+'.crate')
    if archive.is_symlink() or not archive.is_file() or archive.stat().st_size > 32*1024**2:
        raise ValueError('missing or oversized registry archive')
    data = archive.read_bytes()
    if sha(data) != checksum:
        raise ValueError('registry archive checksum mismatch')
    with gzip.GzipFile(fileobj=io.BytesIO(data)) as stream:
        decoded = stream.read(128*1024**2+1)
    if len(decoded) > 128*1024**2:
        raise ValueError('registry archive decoded limit exceeded')
    files = {}
    with tarfile.open(fileobj=io.BytesIO(decoded), mode='r:') as stream:
        for index, member in enumerate(stream):
            if index >= 20000:
                raise ValueError('registry archive member limit exceeded')
            if not member.isfile():
                continue
            prefix = directory.name+'/'
            if not member.name.startswith(prefix) or member.name in files or member.size > 32*1024**2:
                raise ValueError('invalid registry archive member')
            with stream.extractfile(member) as content:
                files[member.name] = sha(content.read())
    return {name.removeprefix(directory.name+'/'): digest for name, digest in files.items()}


def package_record(root, package, node, lock, packaging, output):
    key = identity(package)
    locked = [p for p in lock['package'] if identity(p) == key]
    if len(locked) != 1 or not package.get('license'):
        raise ValueError('missing exact lock/license identity')
    directory = Path(package['manifest_path']).parent.resolve()
    checksum, vcs = None, None
    if package['source'] is not None:
        if package['source'] != REGISTRY:
            raise ValueError('unreviewed dependency source')
        checksum = locked[0]['checksum']
        archive_files = registry_files(directory, checksum)
        if archive_files.get('Cargo.toml') != sha(packaging.local_bytes(directory, 'Cargo.toml', 1024*1024)):
            raise ValueError('registry manifest differs from archive')
        vcs_path = directory/'.cargo_vcs_info.json'
        if vcs_path.exists():
            vcs_bytes = packaging.local_bytes(directory, '.cargo_vcs_info.json', 4096)
            if archive_files.get('.cargo_vcs_info.json') != sha(vcs_bytes):
                raise ValueError('registry VCS record differs from archive')
            vcs = json.loads(vcs_bytes)
    elif not directory.is_relative_to(root):
        raise ValueError('local package outside source')
    notices = []
    paths = packaging.license_files(directory)
    if package['source'] is not None:
        expected = {name for name in archive_files
                    if not any(part in ('.git', 'target', '.cargo') for part in Path(name).parts[:-1])
                    and re.fullmatch(r'(LICENSE|LICENCE|NOTICE|COPYING|COPYRIGHT)([._-].*)?',
                                     Path(name).name, re.IGNORECASE)}
        if expected != {path.relative_to(directory).as_posix() for path in paths}:
            raise ValueError('registry notice file set differs from archive')
    if package['source'] is None:
        paths = sorted(set(paths + [root/'LICENSE']))
    for path in paths:
        base = directory if path.is_relative_to(directory) else root
        name = path.relative_to(base).as_posix()
        data = packaging.local_bytes(base, name, 1024*1024)
        if package['source'] is not None and archive_files.get(name) != sha(data):
            raise ValueError('registry notice file checksum mismatch')
        notices.append({'data': data, 'source_relative_path': name,
                        'kind': 'package_license_or_notice', 'review_required': False})
    if not notices:
        notices = packaging.reviewed_notice_files(root, package)
    records = []
    for item in notices:
        data = item['data']; digest = sha(data)
        name = 'notices/'+digest+'.txt'
        (output/name).write_bytes(data)
        records.append({k: v for k, v in item.items() if k != 'data'} |
                       {'sha256': digest, 'path': name})
    return {'name': key[0], 'version': key[1], 'source': key[2],
            'registry_checksum': checksum, 'vcs': vcs, 'features': sorted(node['features']),
            'license_expression': package['license'], 'notices': records,
            'notice_review_required': any(n['review_required'] for n in records)}


def database_binding(preparation):
    record = json.loads((preparation/'database.json').read_text())
    db = preparation/'advisory-db'
    files = {p.relative_to(db).as_posix(): sha(p.read_bytes()) for p in sorted(db.rglob('*'))
             if p.is_file() and '.git' not in p.relative_to(db).parts}
    digest = sha(json.dumps(files, sort_keys=True, separators=(',', ':')).encode())
    commit = subprocess.check_output(['git', '-c', 'safe.directory='+str(db), '-C', str(db),
                                      'rev-parse', 'HEAD'], text=True).strip()
    if (not files or files != record['files'] or digest != record['files_sha256'] or
            commit != record['commit'] or not record['retrieved_at']):
        raise ValueError('database snapshot binding mismatch')
    return record


def license_markdown(review):
    lines = ['# Dependency license and notice review', '',
             'Source: `'+review['source_head']+'`; target: `'+TARGET+'`.', '',
             'Normal/build dependency closure and resolved features match the packaging metadata method. '
             'This is not exact binary linkage, legal certification or publication clearance.', '',
             'Each package below maps to declared license expressions and exact notice bytes in '
             '`dependency-review.json`. Hash-named files under `notices/` retain those bytes. '
             'Expressions are preserved, including legacy slash notation; no license choice is silently made.', '',
             '## Variant inventory', '',
             '| Variant | Packages | Notice reviews |', '|---|---:|---:|']
    union = {}
    for variant, info in review['variants'].items():
        lines.append(f"| {variant} | {len(info['packages'])} | {sum(p['notice_review_required'] for p in info['packages'])} |")
        for package in info['packages']:
            key = identity(package)
            union.setdefault(key, (package, []))[1].append(variant)
    lines += ['', '## Exact package mapping', '', '| Package | Variants | Declared license | Notice bytes | Review |',
              '|---|---|---|---|---|']
    for key, (package, variants) in sorted(union.items(), key=lambda item: str(item[0])):
        links = ', '.join(f"[{n['sha256'][:12]}]({n['path']})" for n in package['notices'])
        lines.append(f"| {key[0]} {key[1]} | {', '.join(variants)} | {package['license_expression']} | {links} | {'OPEN' if package['notice_review_required'] else 'collected'} |")
    lines += ['', '## Unresolved publication decisions', '',
              'No unresolved issue is accepted by this assessment. No exception expiry is invented. '
              'Any future acceptance must name the maintainer, rationale, scope and expiry/recheck date. '
              'Dependency replacements/upgrades require fresh assessment and applicable tests.', '']
    for key, (package, variants) in sorted(union.items(), key=lambda item: str(item[0])):
        if package['notice_review_required']:
            notes = sorted(set(n.get('review_note') or 'Review required' for n in package['notices'] if n['review_required']))
            lines.append(f"- {key[0]} {key[1]} ({', '.join(variants)}): {' '.join(notes)}")
    lines += ['', '## Recheck policy', '',
              'Rerun before publication, after any lock/feature/target/notice change, and at least monthly '
              'while preparing a release. Acquire a fresh RustSec snapshot explicitly, record its retrieval '
              'date and content hash, then assess offline. No scheduler or CI gate is installed. '
              'Yanked-package registry queries are disabled for offline operation; RustSec maintenance '
              'advisories do not establish maintenance of every other package. Vulnerability results '
              'and proposed actions are in dependency-review.json; no findings are ignored.', '',
              '## Source provenance observations', '',
              'Registry archive hashes identify the exact reviewed distributions. A recorded VCS '
              'revision does not establish equality to an upstream source tree. The following '
              'archives record dirty=true; no upstream-tree equality or publication approval is inferred:', '']
    for key, (package, variants) in sorted(union.items(), key=lambda item: str(item[0])):
        if (package.get('vcs') or {}).get('git', {}).get('dirty'):
            lines.append(f"- {key[0]} {key[1]} ({', '.join(variants)}): archive checksum verified; dirty=true is preserved. Source-comparison disposition is recorded separately in the task evidence.")
    return '\n'.join(lines)


def assess(args):
    root = args.source_root.resolve()
    preparation = args.preparation.resolve()
    output = args.out_dir.resolve(); output.mkdir(parents=True, exist_ok=False)
    (output/'raw').mkdir(); (output/'notices').mkdir()
    spec = importlib.util.spec_from_file_location('package_release', root/'scripts/package-release.py')
    packaging = importlib.util.module_from_spec(spec); spec.loader.exec_module(packaging)
    if VARIANTS != packaging.VARIANT_FEATURES:
        raise ValueError('assessment and packaging distribution variants differ')
    lock_bytes = (root/'Cargo.lock').read_bytes()
    lock = tomllib.loads(lock_bytes.decode())
    database = database_binding(preparation)
    tool = json.loads((preparation/'tool.json').read_text())
    binary = preparation/'cargo-audit'
    if sha(binary.read_bytes()) != tool['binary_sha256']:
        raise ValueError('audit binary hash mismatch')
    commands = []

    def run(command, name, cwd=root, accepted=(0,)):
        completed = subprocess.run(command, cwd=cwd, capture_output=True, timeout=120)
        (output/'raw'/f'{name}.stdout').write_bytes(completed.stdout)
        (output/'raw'/f'{name}.stderr').write_bytes(completed.stderr)
        commands.append({'command': list(map(str, command)), 'cwd': str(cwd), 'exit': completed.returncode,
                         'stdout_sha256': sha(completed.stdout), 'stderr_sha256': sha(completed.stderr)})
        (output/'commands.json').write_bytes(encoded(commands))
        if completed.returncode not in accepted:
            raise ValueError('tool execution failed: '+name)
        return completed

    version = run([str(binary), '--version'], 'audit-version', cwd=output).stdout.decode().strip()
    if version != 'cargo-audit '+tool['version']:
        raise ValueError('audit tool version mismatch')
    for name, command in [('cargo-version', ['cargo', '--version']), ('rustc-version', ['rustc', '--version']),
                          ('python-version', ['python3', '--version'])]:
        run(command, name, cwd=output)
    raw = run([str(binary), 'audit', '--db', str(preparation/'advisory-db'), '--no-fetch',
               '--no-yanked', '--stale', '--file', str(root/'Cargo.lock'), '--json'],
              'audit', cwd=output, accepted=(0, 1))
    audit = json.loads(raw.stdout)
    findings = audit_findings(audit, raw.returncode)
    if audit['lockfile']['dependency-count'] != len(lock['package']):
        raise ValueError('audit lockfile population differs')
    variants = {}
    for variant, features in VARIANTS.items():
        command = ['cargo', 'metadata', '--locked', '--offline', '--format-version', '1', '--filter-platform', TARGET]
        if features:
            command += ['--features', ','.join('pqc_scan_cli/'+f for f in features)]
        metadata = json.loads(run(command, variant+'-metadata').stdout)
        packages = [package_record(root, p, n, lock, packaging, output) for p, n in closure(metadata)]
        keys = {identity(p) for p in packages}
        variants[variant] = {'features': features, 'target': TARGET, 'packages': packages,
                             'finding_ids': sorted({f['advisory']['id'] for f in findings if identity(f['package']) in keys})}
    if (root/'Cargo.lock').read_bytes() != lock_bytes:
        raise ValueError('lockfile changed during review')
    for finding in findings:
        finding['variants'] = [v for v, info in variants.items()
                               if identity(finding['package']) in {identity(p) for p in info['packages']}]
    source_files = ['Cargo.lock', 'Cargo.toml', 'rust-toolchain.toml', 'LICENSE',
                    'scripts/package-release.py', 'scripts/review-dependencies.py',
                    '.github/workflows/release.yml', 'third_party/notices/manifest.json']
    source_files += [p.relative_to(root).as_posix() for p in sorted((root/'crates').glob('*/Cargo.toml'))]
    review = {'source_head': args.source_head, 'source_inputs': {p: sha((root/p).read_bytes()) for p in source_files},
              'evaluated_at': datetime.now(timezone.utc).isoformat(), 'tool': tool, 'database': database,
              'lockfile_packages': len(lock['package']), 'variants': variants, 'findings': findings,
              'assessment': 'findings-require-action' if findings else 'no-rustsec-findings',
              'publication_approved': False,
              'limitations': ['RustSec snapshot coverage only; no advisory is not proof of safety or maintenance.',
                              'No registry yanked checks, binary linkage proof, cross-build or new platform certification.',
                              'No target OS/architecture advisory filtering: retain conservative lockfile findings.',
                              '--stale permits reproducible historical snapshots; acquisition and commit dates are explicit.'],
              'recheck': 'Before publication, after dependency/feature/target/notice changes, and monthly during release preparation.'}
    (output/'dependency-review.json').write_bytes(encoded(review))
    (output/'license-review.md').write_text(license_markdown(review))
    print(json.dumps({'assessment': review['assessment'], 'findings': len(findings),
                      'packages': {v: len(i['packages']) for v, i in variants.items()}}))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source-root', type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument('--source-head', required=True)
    parser.add_argument('--preparation', type=Path, required=True, help='Pinned cargo-audit binary, tool.json, advisory-db and database.json')
    parser.add_argument('--out-dir', type=Path, required=True, help='New output directory; existing evidence is never overwritten')
    args = parser.parse_args()
    if not re.fullmatch('[0-9a-f]{40}', args.source_head):
        parser.error('full source commit required')
    assess(args)


if __name__ == '__main__':
    main()
