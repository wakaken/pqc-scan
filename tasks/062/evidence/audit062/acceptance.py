"""Check preserved audit artifacts; inputs are pinned by COMPARISON.json."""
import collections
import copy
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
BASE = Path('/tmp/pqc-scan-audit')
OUT = Path('/tmp/pqc-audit062-final')
EVIDENCE = ROOT / 'docs/quality/audit062'
rows = json.loads((EVIDENCE / 'COMPARISON.json').read_text())['targets']
assert len(rows) == 10
summary = []
clock_changes = {}
for row in rows:
    name = row['target']
    old = json.loads((BASE / 'reports' / name / 'report.json').read_text())['findings']
    new = json.loads((OUT / name / 'report.json').read_text())['findings']
    a = {f['finding_id']: f for f in old}
    b = {f['finding_id']: f for f in new}
    old_lines = {(f['rule_id'], f['location']['file'], f['location']['line']) for f in old}
    stats = collections.Counter()
    for key in b.keys() - a.keys():
        f = b[key]
        if f['rule_id'] != 'TS_RSA_KEY_SIZE_LITERAL':
            assert f['rule_id'] == 'PQC_STANDARD_REFERENCE'
            continue
        loc = f['location']
        line = (BASE / 'targets' / name / loc['file']).read_text().splitlines()[loc['line'] - 1]
        assert re.search('rsa', line, re.I), (name, loc)
        stats['same_rule_and_line_new_anchor' if (f['rule_id'], loc['file'], loc['line']) in old_lines else 'new_explicit_rsa_context'] += 1
    summary.append({'target': name, 'added_size_classification': dict(stats)})
    clocks = 0
    for key in a.keys() & b.keys():
        before, after = copy.deepcopy(a[key]), copy.deepcopy(b[key])
        for f in (before, after):
            for field in ('description', 'migration_hint', 'recommended_actions'):
                f.pop(field, None)
            # Certificate validity deliberately uses a fresh clock per scan.
            evidence = f['evidence']
            if evidence['type'] == 'certificate' and 'assessment_time_unix' in evidence['metadata']:
                if f is before and evidence['metadata']['assessment_time_unix'] != after['evidence']['metadata']['assessment_time_unix']:
                    clocks += 1
                evidence['metadata'].pop('assessment_time_unix')
                evidence['snippet_preview'] = re.sub(r'assessment_time_unix=\d+', 'assessment_time_unix=<scan-clock>', evidence['snippet_preview'])
        assert before == after, (name, key)
    clock_changes[name] = clocks

java = json.loads((OUT / 'java/report.json').read_text())['findings']
for suffix, lines in [('MLKEM.java', [28, 35, 41]), ('Falcon.java', [24, 29]), ('MLKEMTest.java', [321]), ('HSSTest.java', [131, 139])]:
    assert not [f for f in java if f['rule_id'] == 'TS_RSA_KEY_SIZE_LITERAL' and f['location']['file'].endswith('/' + suffix) and f['location']['line'] in lines], suffix
checks = {}
for name, paths in {
    'go': ['kem/mlkem/mlkem768/kyber.go', 'sign/mldsa/mldsa44/dilithium.go', 'sign/slhdsa/params.go', 'kem/schemes/schemes.go', 'sign/schemes/schemes.go'],
    'typescript': ['src/key/generate_key_pair.ts', 'src/lib/jws_algorithms.ts'],
    'rust': ['rustls/src/crypto/kx/mod.rs'],
}.items():
    findings = json.loads((OUT / name / 'report.json').read_text())['findings']
    checks[name] = {p: sum(f['rule_id'] == 'PQC_STANDARD_REFERENCE' and f['location']['file'] == p for f in findings) for p in paths}
    assert all(checks[name].values()), checks
cert = collections.Counter(f['rule_id'] for f in json.loads((OUT / 'certificates/report.json').read_text())['findings'])
assert cert == {'CERT_RSA_SIGNATURE': 79, 'CERT_ECDSA_SIGNATURE': 39, 'CERT_RSA_2048': 20, 'CERT_SHA1_SIGNATURE': 3}, cert
names = {x['name'] for x in json.loads((OUT / 'java/dependency-sbom.json').read_text())}
assert {'javax.mail:mail', 'junit:junit', 'org.openjdk.jmh:jmh-core', 'com.yubico.yubikit:openpgp', 'com.unboundid:unboundid-ldapsdk'} <= names
(EVIDENCE / 'ACCEPTANCE.json').write_text(json.dumps({
    'source': 'SOURCE-FINAL.json', 'exit': 0,
    'exact_java_false_positive_cases_absent': True, 'pqc_paths': checks,
    'gradle_named_dependencies_present': True, 'certifi_attributes_unchanged': dict(cert),
    'unchanged_findings_preserve_fields_except_guidance_and_scan_clock': True,
    'certificate_scan_clock_changes': clock_changes,
    'changes': summary,
    'classification': 'Added standard PQC references and changed RSA-size predicates/anchors. Removed unbound size contexts are not all independently adjudicated false positives.',
}, indent=2) + '\n')
print('10-target acceptance passed')
print(json.dumps(checks))
