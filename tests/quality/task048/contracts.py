"""Source-bound offline export assessment; no product runtime dependencies.

Run after both Rust integration tests with PQC048_EVIDENCE=<absolute raw dir>.
Markdown rendering uses the already installed markdown-it-py 3.0.0, CommonMark
with raw HTML and tables enabled (no sanitizer hiding an escaping defect).
"""
import argparse
from collections import Counter
import hashlib
from html.parser import HTMLParser
import importlib.metadata
import json
from pathlib import Path
import shutil


def digest(data):
    return hashlib.sha256(data).hexdigest()


def verify_pins(root, pins):
    verified = []
    for row in pins['files']:
        if digest((root/row['path']).read_bytes()) != row['sha256']:
            raise ValueError('schema/license digest mismatch: '+row['path'])
        if not (root/row['license']).is_file():
            raise ValueError('missing license: '+row['license'])
        verified.append(row)
    return verified


def inspect_html(text, allowed_scripts):
    class Inspector(HTMLParser):
        def __init__(self):
            super().__init__(convert_charrefs=True)
            self.issues = []
            self.script = None
        def handle_starttag(self, tag, attrs):
            attrs = dict(attrs)
            if tag in {'iframe', 'object', 'embed', 'base', 'svg', 'img'}:
                self.issues.append('active element: '+tag)
            for key, value in attrs.items():
                if key.lower().startswith('on') or key == 'srcdoc':
                    self.issues.append('event/srcdoc: '+key)
                if key in {'href', 'src', 'action'}:
                    normalized = ''.join(c for c in (value or '') if ord(c) > 32).lower()
                    if normalized.startswith(('javascript:', 'data:', 'vbscript:')):
                        self.issues.append('active URI: '+normalized)
            if tag == 'script':
                if attrs:
                    self.issues.append('script attributes')
                self.script = ''
        def handle_data(self, data):
            if self.script is not None:
                self.script += data
        def handle_endtag(self, tag):
            if tag == 'script':
                if self.script not in allowed_scripts:
                    self.issues.append('untrusted script')
                self.script = None
    parser = Inspector()
    parser.feed(text)
    return parser.issues


def load(path):
    return json.loads(path.read_text())


def properties(value):
    rows = value.get('properties', [])
    result = {p['name']:p['value'] for p in rows}
    assert len(result) == len(rows), 'duplicate provenance properties'
    return result


def check_projection(native, document, expected_loss):
    """Compare emitted facts to source records, never to exporter helpers."""
    loss = json.loads(properties(document)['pqc-scan:export-loss'])
    assert loss.get('counts') == expected_loss, 'missing/incorrect export-loss counts'
    assert 'observations_are_properties_not_complete_native_records' in loss.get('notes', [])
    assets = {a['id']:a for a in native['assets']}
    compared = 0
    for component in document['components']:
        asset = assets[component['bom-ref']]
        props = properties(component)
        assert props['pqc-scan:identity-basis'] == asset['identity_basis']
        assert props.get('pqc-scan:uncertainty', '').split(',') == (asset['uncertainty'] or [''])
        expected = []
        for observation in native['observations']:
            if asset['id'] not in observation['asset_ids']:
                continue
            row = {key:observation[key] for key in ['id','method','assurance','usage']}
            row.update({key:observation['target'].get(key) for key in ['path','line','column','column_kind']})
            expected.append(row)
        actual = json.loads(props.get('pqc-scan:observations', '[]'))
        canonical = lambda rows: Counter(json.dumps(row,sort_keys=True) for row in rows)
        assert canonical(actual) == canonical(expected), 'missing/changed observation provenance'
        compared += len(actual)
        if asset.get('kind') == 'dependency':
            a = asset['attributes']
            for key in ['ecosystem','declared_constraint','reported_version','version_provenance']:
                assert props.get('pqc-scan:'+key) == a.get(key)
            if a['version_provenance'] == 'lock_resolved':
                assert component['version'] == a['resolved_version']
            else:
                assert 'version' not in component
    return compared


def semantics(raw):
    reports = []
    for language in ['en', 'ja']:
        directory = raw/('scan-'+language)
        legacy = load(directory/'report.json')
        native = load(directory/'scan.inventory.v1.json')
        sarif = load(directory/'report.sarif')
        cbom = load(directory/'cbom.json')
        assert isinstance(cbom, list) and cbom == legacy['cbom']
        assert load(directory/'dependency-sbom.json') == legacy['dependency_sbom']
        findings = legacy['findings']
        assert len(cbom) == len(findings) > 0
        want = Counter((f['rule_id'], f['location']['file'], f['location']['line']) for f in findings)
        actual = Counter((r['ruleId'], r['locations'][0]['physicalLocation']['artifactLocation']['uri'], r['locations'][0]['physicalLocation']['region']['startLine']) for r in sarif['runs'][0]['results'])
        assert actual == want
        assert Counter(c['location'] for c in cbom) == Counter(f"{f['location']['file']}:{f['location']['line']}" for f in findings)
        assets = native['assets']
        ids = {a['id'] for a in assets}
        assert len(ids) == len(assets)
        for a in assets:
            assert all(r['target'] in ids for r in a['relations'])
        crypto = load(directory/'cbom.cdx-1.7.json')
        software = load(directory/'dependency-sbom.cdx-1.7.json')
        for doc in [crypto, software]:
            assert (doc['bomFormat'], doc['specVersion']) == ('CycloneDX', '1.7')
            assert any(p['name'] == 'pqc-scan:export-loss' for p in doc['properties'])
        crypto_ids = {c['bom-ref'] for c in crypto['components']}
        assert len(crypto_ids) == len(crypto['components'])
        assert crypto_ids == {a['id'] for a in assets if a['kind'] in {'algorithm','certificate','public_key'}}
        assert {c['bom-ref'] for c in software['components']} == {a['id'] for a in assets if a['kind'] == 'dependency'}
        # Hand-accounted fixed input: one parsed certificate/key + three
        # algorithms, two dependency assertions, fifteen source usage anchors.
        crypto_provenance = check_projection(native,crypto,dict(certificate_tbs_fingerprint_unavailable=1,software_source_assertions_separate_document=2,unprojected_observations=9,usage_assets_in_observation_properties=15))
        software_provenance = check_projection(native,software,dict(crypto_assets_not_software=5,unprojected_observations=7,usage_assets_in_observation_properties=15))
        reports.append(dict(language=language, findings=len(findings), sarifResults=len(sarif['runs'][0]['results']), legacyCbom=len(cbom), assetKinds=dict(Counter(a['kind'] for a in assets)), cryptoComponents=len(crypto_ids), softwareComponents=len(software['components']), cryptoProvenanceRows=crypto_provenance,softwareProvenanceRows=software_provenance,legacyIsNotCycloneDX=True))
    return {'requirement':'Q3-12','status':'Passed','runs':reports,
            'assertionEvidence':'crates/pqc_scan_cli/tests/output_contracts048.rs; crates/pqc_scan_report/tests/cyclonedx_17.rs; sarif_contract.rs; inspection.rs; evidence_privacy.rs',
            'contracts': ['distinct algorithm/certificate/public key/usage', 'literal certificate DER digest and RSA2048', 'two RSA source occurrences at lines 2/3', 'unique finding/asset IDs and resolving observation associations', 'same finding IDs/assets after relocation and localization', 'manifest constraint is not lock resolution or installation', 'private marker remains lexical/unknown, no private key object', 'export losses and omitted usage objects remain explicit', 'SARIF/HTML/Markdown are projections; not full Inventory copies'],
            'limits':['duplicate weak anchors remain unresolved in diff','legacy CBOM carries source findings, not distinct parsed assets','HTML intentionally omits native asset/observation tables; Markdown retains details','not an assessment of all possible inputs or scanner accuracy']}


def render(raw, browser, compact_script):
    from markdown_it import MarkdownIt
    assert importlib.metadata.version('markdown-it-py') == '3.0.0'
    renderer = MarkdownIt('commonmark', {'html': True}).enable('table')
    browser.mkdir(parents=True, exist_ok=True)
    result = []
    names = [f'attack-{kind}-{lang}' for kind in ['static','tls','ssh-offers','ssh-kex'] for lang in ['en','ja']]
    for name in names:
        src = raw/name
        html = (src/'report.html').read_text()
        md = (src/'report.md').read_text()
        rendered = renderer.render(md)
        # Raw HTML enabled: a sanitizer cannot turn a broken writer into success.
        for text, allowed in [(html,{compact_script}), (rendered,set())]:
            issues = inspect_html(text,allowed)
            assert not issues, (name,issues)
            assert 'EXECUTED048' in text
            assert 'CANARY048_SECRET' not in text
        for p in src.iterdir():
            if p.suffix in {'.json','.sarif'}:
                load(p)
        out = browser/name
        out.mkdir(exist_ok=True)
        shutil.copyfile(src/'report.html', out/'report.html')
        (out/'markdown.html').write_text('<!doctype html><meta charset="utf-8"><title>Markdown048</title>'+rendered)
        result.append(dict(case=name,htmlSha256=digest(html.encode()),markdownSha256=digest(md.encode()),renderedSha256=digest(rendered.encode()),activeContentRejected=True))
    # Browser positive control proves that the title sentinel can execute.
    (browser/'control.html').write_text("<!doctype html><title>control</title><script>document.title='EXECUTED048'</script><p>synthetic positive control</p>")
    return result


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--raw',type=Path,required=True)
    parser.add_argument('--out',type=Path,required=True)
    args=parser.parse_args()
    root=Path(__file__).resolve().parents[3]
    pins=verify_pins(root,load(Path(__file__).with_name('schema-pins.json')))
    cases=load(args.raw/'schema-scan.json')+load(args.raw/'schema-probe.json')
    assert len(cases)==23 and all(c['positive'] and c['negativeRejected'] for c in cases)
    args.out.mkdir(parents=True,exist_ok=True)
    (args.out/'schema-results.json').write_text(json.dumps({'requirement':'Q3-11','status':'Passed','schemaPins':pins,'cases':cases,'offline':'jsonschema 0.29.1 default-features=false, --locked --offline; local resources only','legacyFormats':'report.json/cbom.json/dependency-sbom.json/diagnostics.json have no official standard schema; serialized compatibility and reconciliation tested, no invented standards claim'},indent=2)+'\n')
    (args.out/'output-semantics.json').write_text(json.dumps(semantics(args.raw),indent=2)+'\n')
    rendered=render(args.raw,args.out/'browser',(root/'crates/pqc_scan_report/src/compact.js').read_text())
    (args.out/'render-structural.json').write_text(json.dumps({'requirement':'Q4-05','renderer':'markdown-it-py 3.0.0 CommonMark raw HTML enabled + table','cases':rendered,'browserAcceptance':'separate actual browser observations required'},indent=2)+'\n')


if __name__ == '__main__': main()
