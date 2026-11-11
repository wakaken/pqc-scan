#!/usr/bin/env python3
"""Prepare local archives, without publication or signing. Standard library only."""
import argparse
import gzip
import hashlib
import io
import json
import os
from pathlib import Path, PurePosixPath
import re
import subprocess
import tarfile
import tomllib
from urllib.parse import urlsplit, unquote


def digest(data):
    return hashlib.sha256(data).hexdigest()


def safe_name(name):
    path = PurePosixPath(name)
    if not name or path.as_posix() != name or name == '.' or path.is_absolute() or '..' in path.parts or any(ord(c) < 32 for c in name) or '\\' in name:
        raise ValueError('unsafe archive path')
    return name


def write_archive(destination, prefix, entries, manifest=True):
    if prefix:
        safe_name(prefix)
    records = {safe_name('/'.join(filter(None, (prefix, safe_name(name))))): value for name, value in entries.items()}
    if manifest:
        sums = ''.join(f'{digest(value[0])}  {name}\n' for name, value in sorted(records.items()))
        records[safe_name(prefix + '/SHA256SUMS')] = (sums.encode(), 0o644)
    with open(destination, 'wb') as output:
        with gzip.GzipFile(filename='', fileobj=output, mode='wb', mtime=0) as compressed:
            with tarfile.open(fileobj=compressed, mode='w', format=tarfile.USTAR_FORMAT) as archive:
                for name, (data, mode) in sorted(records.items()):
                    info = tarfile.TarInfo(name)
                    info.size, info.mode, info.mtime = len(data), mode, 0
                    info.uid = info.gid = 0
                    info.uname = info.gname = ''
                    archive.addfile(info, io.BytesIO(data))


MAX_DECODED_BYTES = 512 * 1024 * 1024
MAX_MEMBERS = 10000


def verify_archive(path):
    # Accept only the regular-file USTAR subset generated above. Parsing fixed
    # headers ourselves prevents tarfile's transparent PAX/GNU metadata expansion.
    if Path(path).stat().st_size > MAX_DECODED_BYTES:
        raise ValueError('compressed archive resource limit exceeded')
    contents, manifests = {}, {}
    decoded = 0
    with gzip.open(path, 'rb') as stream:
        def read_exact(size):
            nonlocal decoded
            if decoded + size > MAX_DECODED_BYTES:
                raise ValueError('decoded archive resource limit exceeded')
            data = stream.read(size)
            decoded += len(data)
            if len(data) != size:
                raise ValueError('truncated archive')
            return data

        while True:
            block = read_exact(512)
            if block == bytes(512):
                if read_exact(512) != bytes(512):
                    raise ValueError('invalid tar terminator')
                # Generated USTAR archives pad to record size. Bound and validate
                # trailing decoded bytes, including concatenated gzip members.
                for _ in range(20):
                    tail = stream.read(512)
                    decoded += len(tail)
                    if decoded > MAX_DECODED_BYTES:
                        raise ValueError('decoded archive resource limit exceeded')
                    if not tail:
                        break
                    if tail != bytes(len(tail)):
                        raise ValueError('unexpected trailing archive data')
                else:
                    raise ValueError('excessive archive padding')
                break
            if len(contents) >= MAX_MEMBERS:
                raise ValueError('archive member count limit exceeded')
            if block[257:265] != b'ustar\x0000' or block[156:157] not in (b'0', b'\x00'):
                raise ValueError('unsupported archive format or member type')
            member = tarfile.TarInfo.frombuf(block, 'utf-8', 'strict')
            name = safe_name(member.name)
            if name in contents or member.size < 0:
                raise ValueError('duplicate or invalid archive member')
            if decoded + member.size > MAX_DECODED_BYTES:
                raise ValueError('decoded archive resource limit exceeded')
            manifest = name.endswith('/SHA256SUMS')
            if manifest and (member.size > 2 * 1024 * 1024 or manifests):
                raise ValueError('invalid checksum manifest size/count')
            checksum, remaining, chunks = hashlib.sha256(), member.size, []
            while remaining:
                data = read_exact(min(65536, remaining))
                remaining -= len(data)
                checksum.update(data)
                if manifest:
                    chunks.append(data)
            contents[name] = checksum.hexdigest()
            if manifest:
                manifests[name] = b''.join(chunks)
            padding = (-member.size) % 512
            if padding and read_exact(padding) != bytes(padding):
                raise ValueError('invalid member padding')
    if len(manifests) != 1:
        raise ValueError('expected one checksum manifest')
    manifest_name, data = next(iter(manifests.items()))
    contents.pop(manifest_name)
    expected = {}
    for line in data.decode().splitlines():
        checksum, name = line.split('  ', 1)
        safe_name(name)
        if name in expected or not re.fullmatch('[0-9a-f]{64}', checksum):
            raise ValueError('invalid checksum manifest')
        expected[name] = checksum
    if contents != expected:
        raise ValueError('checksum mismatch')


def license_files(root):
    root = root.resolve()
    found = []
    for directory, dirs, files in os.walk(root, followlinks=False):
        dirs[:] = sorted(d for d in dirs if d not in ('.git', 'target', '.cargo') and not (Path(directory)/d).is_symlink())
        for name in sorted(files):
            if not re.fullmatch(r'(LICENSE|LICENCE|NOTICE|COPYING|COPYRIGHT)([._-].*)?', name, re.IGNORECASE):
                continue
            path = Path(directory)/name
            if path.is_symlink() or not path.is_file() or not path.resolve().is_relative_to(root):
                raise ValueError('unsafe license file')
            found.append(path)
    return found


def local_bytes(base, name, limit):
    safe_name(name)
    base = base.resolve()
    path = base/name
    current = base
    for part in Path(name).parts:
        current = current/part
        if current.is_symlink():
            raise ValueError('symlink in reviewed notice input')
    if not path.is_file() or not path.resolve().is_relative_to(base):
        raise ValueError('unsafe reviewed notice input')
    with path.open('rb') as stream:
        data = stream.read(limit+1)
    if len(data) > limit:
        raise ValueError('reviewed notice input limit exceeded')
    return data


def reviewed_notice_files(root, package):
    """Offline fallback bound to an exact Cargo package and its upstream revision."""
    directory = root/'third_party/notices'
    if directory.is_symlink() or not directory.resolve().is_relative_to(root.resolve()):
        raise ValueError('unsafe notice directory')
    manifest = json.loads(local_bytes(directory, 'manifest.json', 512*1024))
    if manifest.get('version') != 1 or len(manifest.get('packages', [])) > 64:
        raise ValueError('unsupported notice manifest')
    matches = [entry for entry in manifest['packages'] if all(entry.get(k) == package[k] for k in ('name','version','source'))]
    if len(matches) != 1:
        raise ValueError('missing or duplicate exact-version notice registration')
    entry, = matches
    lock = tomllib.loads(local_bytes(root, 'Cargo.lock', 2*1024*1024).decode())
    locked = [p for p in lock['package'] if all(p.get(k) == package[k] for k in ('name','version','source'))]
    if (len(locked) != 1 or not entry.get('registry_checksum') or
            locked[0].get('checksum') != entry['registry_checksum'] or
            entry.get('license_expression') != package['license'] or
            entry.get('upstream_repository') != package.get('repository')):
        raise ValueError('reviewed notice package binding mismatch')
    crate_root = Path(package['manifest_path']).parent
    vcs = json.loads(local_bytes(crate_root, '.cargo_vcs_info.json', 4096))
    if (vcs.get('git', {}).get('sha1') != entry.get('upstream_revision') or
            vcs.get('path_in_vcs') != entry.get('path_in_vcs') or
            not re.fullmatch('[0-9a-f]{40}', entry.get('upstream_revision', ''))):
        raise ValueError('reviewed notice revision mismatch')
    if type(entry.get('review_required')) is not bool or not 1 <= len(entry.get('files', [])) <= 16:
        raise ValueError('invalid reviewed notice declaration')
    records, seen = [], set()
    for item in entry['files']:
        name = safe_name(item['path'])
        if (not name.startswith('texts/') or name in seen or
                not re.fullmatch(r'(LICENSE|LICENCE|NOTICE|COPYING|COPYRIGHT)([._-].*)?', Path(name).name, re.IGNORECASE)):
            raise ValueError('unsafe or duplicate reviewed notice name')
        seen.add(name)
        data = local_bytes(directory, name, 256*1024)
        if digest(data) != item.get('sha256'):
            raise ValueError('reviewed notice checksum mismatch')
        if item.get('kind') not in ('license_text', 'upstream_license_statement'):
            raise ValueError('unknown reviewed notice kind')
        if item['kind'] == 'upstream_license_statement' and not entry['review_required']:
            raise ValueError('license statement requires explicit review')
        repo = entry['upstream_repository'].removesuffix('.git').removeprefix('https://github.com/')
        prefix = 'https://raw.githubusercontent.com/'+repo+'/'+entry['upstream_revision']+'/'
        url = urlsplit(item.get('upstream_url', ''))
        if (not item.get('upstream_url', '').startswith(prefix) or url.scheme != 'https' or
                url.netloc != 'raw.githubusercontent.com' or url.query or url.fragment):
            raise ValueError('notice URL does not identify the bound upstream revision')
        relative = url.path.removeprefix('/'+repo+'/'+entry['upstream_revision']+'/')
        if unquote(relative) != relative:
            raise ValueError('encoded reviewed notice URL is unsupported')
        safe_name(relative)
        records.append({'data': data, 'source_relative_path': 'third_party/notices/'+name,
                        'upstream_url': item['upstream_url'], 'upstream_revision': entry['upstream_revision'],
                        'kind': item['kind'], 'review_required': entry['review_required'],
                        'review_note': entry.get('review_note')})
    return records


def run(root, *args):
    return subprocess.check_output(args, cwd=root, text=True).strip()


def json_bytes(value):
    return (json.dumps(value, sort_keys=True, indent=2, ensure_ascii=False)+'\n').encode()


def prepare(args):
    root = args.source_root.resolve()
    binary = args.binary.resolve(strict=True)
    target = args.target
    features = [] if args.variant == 'static' else ['probe-tls', 'probe-ssh']
    metadata_command = ['cargo', 'metadata', '--locked', '--offline', '--format-version', '1', '--filter-platform', target]
    if features:
        metadata_command += ['--features', 'pqc_scan_cli/probe-tls,pqc_scan_cli/probe-ssh']
    metadata = json.loads(run(root, *metadata_command))
    packages = {p['id']: p for p in metadata['packages']}
    nodes = {n['id']: n for n in metadata['resolve']['nodes']}
    cli = next(p for p in packages.values() if p['name'] == 'pqc_scan_cli')
    selected, todo = set(), [cli['id']]
    while todo:
        ident = todo.pop()
        if ident in selected:
            continue
        selected.add(ident)
        todo += [d['pkg'] for d in nodes[ident]['deps'] if any(k['kind'] != 'dev' for k in d['dep_kinds'])]
    binary_data = binary.read_bytes()
    if binary_data[:8] != bytes.fromhex('cffaedfe0c000001'):
        raise ValueError('expected thin Mach-O aarch64 binary for declared target')
    entries = {'bin/pqc-scan': (binary_data, 0o755)}
    explicit = ['LICENSE', 'SECURITY.md', 'docs/user/DISTRIBUTION.md', 'docs/user/DISTRIBUTION.ja.md', 'docs/user/PROBING.md', 'docs/user/PROBING.ja.md', 'Cargo.lock', 'rust-toolchain.toml', 'third_party/notices/manifest.json', 'third_party/notices/README.md']
    paths = [root/p for p in explicit] + sorted((root/'rules/default').glob('*.yml'))
    source_hashes = {}
    for path in paths:
        if path.is_symlink() or not path.resolve().is_relative_to(root):
            raise ValueError('unsafe source asset')
        name = path.relative_to(root).as_posix()
        data = local_bytes(root, name, 512*1024) if name == 'third_party/notices/manifest.json' else path.read_bytes()
        entries[name] = (data, 0o644)
        source_hashes[name] = digest(data)
    inventory = []
    for ident in sorted(selected):
        package = packages[ident]
        package_root = Path(package['manifest_path']).parent.resolve()
        notices = []
        label = safe_name(package['name'] + '-' + package['version'] + '-' + digest((package['source'] or 'workspace').encode())[:8])
        notice_files = license_files(package_root)
        if package['source'] is None and package_root.is_relative_to(root):
            notice_files = sorted(set(notice_files + [root/'LICENSE']))
        sources = [{'data': path.read_bytes(), 'source_relative_path': path.relative_to(package_root if path.is_relative_to(package_root) else root).as_posix(), 'kind': 'package_license_or_notice', 'review_required': False} for path in notice_files]
        if not sources:
            sources = reviewed_notice_files(root, package)
        for index, notice in enumerate(sources):
            data = notice['data']
            # Flatten nested names to keep ustar path lengths bounded and portable.
            name = f'licenses/{label}/{index:04d}.txt'
            entries[name] = (data, 0o644)
            notices.append({k:v for k,v in notice.items() if k != 'data'} | {'path': name, 'sha256': digest(data)})
            if notice['source_relative_path'].startswith('third_party/notices/'):
                source_hashes[notice['source_relative_path']] = digest(data)
        inventory.append({'name': package['name'], 'version': package['version'], 'source': package['source'], 'license_expression': package['license'], 'notices': notices, 'notice_review_required': any(n['review_required'] for n in notices)})
    entries['DEPENDENCY-LICENSES.json'] = (json_bytes({'scope': 'Resolved CLI normal/build dependencies for declared features and target; source filename notices plus exact-version pinned upstream fallbacks. Review-required statements are not full license texts. Not a legal certification or binary-linkage proof.', 'packages': inventory}), 0o644)
    entries['THIRD-PARTY-NOTICES.txt'] = (b'License texts are under licenses/. See DEPENDENCY-LICENSES.json for package versions, declared SPDX expressions, notice paths and hashes. Exact-version upstream fallbacks are bound in third_party/notices/manifest.json. Upstream license statements flagged review_required are not full license texts or publication clearance.\n', 0o644)
    status = run(root, 'git', 'status', '--porcelain', '--untracked-files=normal')
    # Bind the development snapshot without storing diffs or absolute local paths.
    diff = subprocess.check_output(['git', 'diff', 'HEAD', '--binary'], cwd=root)
    source_files = subprocess.check_output(['git', 'ls-files', '--cached', '--others', '--exclude-standard', 'crates', 'rules', 'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', 'scripts/package-release.py'], cwd=root).decode().splitlines()
    for name in sorted(set(source_files)):
        path = root/name
        if path.suffix not in ('.rs', '.toml', '.lock', '.yml', '.py') or not path.is_file():
            continue
        if path.is_symlink() or not path.resolve().is_relative_to(root):
            raise ValueError('unsafe source input')
        source_hashes[name] = digest(path.read_bytes())
    provenance = {'preparation_only': True, 'source_head': run(root, 'git', 'rev-parse', 'HEAD'), 'source_dirty': bool(status), 'tracked_diff_sha256': digest(diff), 'source_inputs_sha256': digest(json_bytes(source_hashes)), 'source_inputs': source_hashes, 'binary_sha256': digest(entries['bin/pqc-scan'][0]), 'binary_origin': 'Caller supplied; must bind to separate build evidence. Not independently rebuilt by packaging.', 'target': target, 'variant': args.variant, 'features': features, 'rustc': run(root, 'rustc', '--version'), 'cargo': run(root, 'cargo', '--version'), 'version': cli['version'], 'repeatability_scope': 'Identical binary, source snapshot and tool metadata only; not cross-host reproducible builds.'}
    entries['PROVENANCE.json'] = (json_bytes(provenance), 0o644)
    prefix = f'pqc-scan-{cli["version"]}-{target}-{args.variant}'
    args.out_dir.mkdir(parents=True, exist_ok=True)
    destination = args.out_dir/(prefix+'.tar.gz')
    write_archive(destination, prefix, entries)
    verify_archive(destination)
    checksum = digest(destination.read_bytes())
    destination.with_suffix(destination.suffix+'.sha256').write_text(f'{checksum}  {destination.name}\n')
    print(json.dumps({'archive': str(destination), 'sha256': checksum, 'packages': len(inventory), 'missing_notice_packages': [p['name'] for p in inventory if not p['notices']], 'notice_review_required': [p['name'] for p in inventory if p['notice_review_required']]}))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--verify', type=Path)
    parser.add_argument('--binary', type=Path)
    parser.add_argument('--variant', choices=['static', 'combined-probes'])
    parser.add_argument('--target', default='aarch64-apple-darwin', choices=['aarch64-apple-darwin'])
    parser.add_argument('--source-root', type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument('--out-dir', type=Path)
    args = parser.parse_args()
    if args.verify:
        verify_archive(args.verify)
        print('Archive member checksums verified; authenticity is not established.')
    elif args.binary and args.variant and args.out_dir:
        prepare(args)
    else:
        parser.error('--binary, --variant and --out-dir are required for preparation')

if __name__ == '__main__':
    main()
