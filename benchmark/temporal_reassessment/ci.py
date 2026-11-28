"""Compile only the first-party temporal witness regression tests, offline."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
REQUIRED = ('anyhow', 'pqc_scan_core', 'pqc_scan_rules', 'pqc_scan_detectors',
            'serde', 'serde_json', 'sha2', 'libc')


def invoke(argv, *, environment=None):
    result = subprocess.run(argv, cwd=ROOT, env=environment, stdout=subprocess.PIPE, check=True)
    return result.stdout


def runtime_features(metadata, dependencies):
    packages = {row['id']: row for row in metadata['packages']}
    names = {}
    for name, identity in dependencies.items():
        package = packages[identity]
        key = name, 'v' + package['version']
        if sum((row['name'], row['version']) == (name, package['version'])
               for row in packages.values()) != 1:
            raise ValueError('E_CI_FEATURE_PACKAGE_AMBIGUOUS')
        names[key] = name
    command = ['cargo', 'tree', '--locked', '--offline', '--edges', 'normal,no-proc-macro',
               '--prefix', 'none', '--format', '{p}|{f}']
    for name in ('pqc_scan_core', 'pqc_scan_rules', 'pqc_scan_detectors'):
        command.extend(['-p', dependencies[name]])
    features = {}
    for line in invoke(command).decode().splitlines():
        if not line:
            continue
        package, values = line.rsplit('|', 1)
        key = tuple(package.split()[:2])
        if key in names:
            name = names[key]
            value = frozenset(filter(None, values.removesuffix(' (*)').split(',')))
            if name in features and features[name] != value:
                raise ValueError('E_CI_RUNTIME_FEATURE_AMBIGUOUS:' + name)
            features[name] = value
    if set(features) != set(dependencies):
        raise ValueError('E_CI_RUNTIME_FEATURE_MISSING')
    return features


def select_artifacts(rows, dependencies, features):
    artifacts = {}
    for row in rows:
        if row.get('reason') != 'compiler-artifact':
            continue
        name = row['target']['name']
        if dependencies.get(name) != row['package_id'] or frozenset(row['features']) != features[name]:
            continue
        libraries = [path for path in row['filenames'] if path.endswith('.rlib')]
        if not libraries:
            continue
        if len(libraries) != 1 or name in artifacts:
            raise ValueError('E_CI_ARTIFACT_AMBIGUOUS')
        artifacts[name] = libraries[0]
    if set(artifacts) != set(dependencies):
        raise ValueError('E_CI_ARTIFACT_MISSING')
    return artifacts


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--target-dir', type=Path,
                        default=Path(os.environ.get('CARGO_TARGET_DIR', str(ROOT / 'target'))))
    args = parser.parse_args()
    target = args.target_dir.absolute()
    metadata = json.loads(invoke(['cargo', 'metadata', '--locked', '--offline', '--format-version=1']))
    nodes = {row['id']: row for row in metadata['resolve']['nodes']}
    own = {package['name']: package['id'] for package in metadata['packages']
           if package['name'] in ('pqc_scan_core', 'pqc_scan_rules', 'pqc_scan_detectors') and
           Path(package['manifest_path']).parent.parent == ROOT / 'crates'}
    if len(own) != 3:
        raise ValueError('E_CI_PRODUCT_PACKAGES')
    dependencies = {}
    for identity in own.values():
        for dependency in nodes[identity]['deps']:
            if dependency['name'] in REQUIRED:
                old = dependencies.setdefault(dependency['name'], dependency['pkg'])
                if old != dependency['pkg']:
                    raise ValueError('E_CI_DEPENDENCY_AMBIGUOUS')
    dependencies.update(own)
    if set(dependencies) != set(REQUIRED):
        raise ValueError('E_CI_DEPENDENCY_CLOSURE')
    output = invoke(['cargo', 'build', '--workspace', '--locked', '--offline',
                     '--message-format=json', '--target-dir', str(target)])
    rows = [json.loads(line) for line in output.splitlines()]
    artifacts = select_artifacts(rows, dependencies, runtime_features(metadata, dependencies))
    searches = {str(target / 'debug/deps')}
    for row in rows:
        if row.get('reason') == 'build-script-executed':
            searches.update(row.get('linked_paths', []))
    externs = []
    for name in REQUIRED:
        artifact = artifacts[name]
        externs.extend(['--extern', name + '=' + artifact])
        searches.add(str(Path(artifact).parent))
    with tempfile.TemporaryDirectory(prefix='pqc-temporal-ci-') as temporary:
        binary = Path(temporary) / 'temporal-witness-tests'
        compiler = Path(invoke(['rustc', '--print', 'sysroot']).decode().strip()) / 'bin/rustc'
        command = [str(compiler), '--edition=2021', '--test', '--crate-name',
                   'pqc_temporal_witness_tests', str(Path(__file__).with_name('temporal_witness.rs')),
                   '-C', 'debuginfo=0', '-D', 'warnings', '-o', str(binary), *externs]
        for search in sorted(searches):
            command.extend(['-L', search if '=' in search else 'dependency=' + search])
        environment = {**os.environ, 'PQC_TEMPORAL_PRODUCT_SOURCE': str(ROOT)}
        subprocess.run(command, cwd=ROOT, env=environment, check=True)
        digest = hashlib.sha256(binary.read_bytes()).hexdigest()
        subprocess.run([str(binary)], cwd=ROOT, env=environment, check=True)
        print(json.dumps({'kind': 'temporal-witness-ci-tests', 'binarySha256': digest,
                          'scope': 'Synthetic first-party regression tests only',
                          'corpusRuns': 0, 'performanceMeasurements': False}))


if __name__ == '__main__':
    main()
