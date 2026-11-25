"""Current executable YAML population and honest companion disposition."""
import pathlib,json,hashlib,subprocess,yaml
root=pathlib.Path('.');b=root/'tasks/083/evidence';m=json.loads((root/'docs/quality/rules/RULE-QUALITY-METADATA.json').read_text());old=json.loads(subprocess.check_output(['git','show','HEAD:docs/quality/rules/RULE-QUALITY-METADATA.json']));sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
actual={}
for p in sorted((root/'rules/default').glob('*.yml')):
 data=yaml.safe_load(p.read_text());data=data if isinstance(data,list)else[data]
 for r in data:assert r['id']not in actual;actual[r['id']]=r
assert len(actual)==len(m['rules'])==137
assert {r['id']for r in m['rules']}==set(actual)
for source in m['source_files']:assert sha(root/source['path'])==source['sha256']
previous={r['id']:r for r in old['rules']};new=[r for r in m['rules']if r['id']not in previous];assert len(previous)==125 and len(new)==12
assert all(r==previous[r['id']]for r in m['rules']if r['id']in previous)
for row in m['rules']:
 assert row['rule_snapshot']==actual[row['id']],row['id']
 assert row['method_limits'] and row['reference_ids'] and row['purpose'] and row['quantum'] and row['classical']
 assert any(x['desired_positive']for x in row['fixtures'])and any(not x['desired_positive']for x in row['fixtures'])
 for rid in row['reference_ids']:assert m['references'][rid]['edition']and m['references'][rid]['scope_limit']
assert {r['id']:r['rule_snapshot']['risk']for r in new if r['rule_snapshot']['risk']!='quantum-uncertain'}=={'PHP_PQC_API':'quantum-safe'}
result={'source_metadata_sha256':sha(root/'docs/quality/rules/RULE-QUALITY-METADATA.json'),'current_rules':137,'historical_unchanged_rows':125,'source_files':m['source_files'],'pattern_controls':sum(len(r['fixtures'])for r in m['rules']),'new_rows':new,'all_rows_checked':['unique executable IDs andexact authoritative snapshots','all12currentYAMLsourceSHA','exact125historicalrows unchanged (including known limitations, not rewritten065)','references exacteditions/scope, purpose/quantum/classical separate axes','positive+negative patterncontrols withknown-gap disclosures; actual305regexexecutions in082workspacequality_metadata tests'],'custom_pack':'source_facts::bindings::tests::installed_rule_copies_bind_but_modified_semantics_do_not checks everyactivehandler id/kind/pattern/scope; generic routing/custom tests use realnewgrammarsemanticnodes','overlap':'c_family_overlap_preserves_original_lexical_rule_occurrences_and_ids compares regex-only vs defaultdetectors, requires nonzerooriginal andnewCfamily findings; unchanged IDs/locations/risk and documented intentionalcoexistence. Existing DEP_OPENSSL/DEP_BOUNCYCASTLE tags may overlap new ecosystems and do not select actualalgorithms. No severity/riskreclassification or threshold masking.','behavior_evidence':['tasks/082/evidence/workspace-final.log','tasks/082/evidence/all-features-final.log','tasks/082/evidence/CLI-PACKAGE-CHECKS.json','docs/rules-reference.md','docs/user/COVERAGE.md','docs/user/COVERAGE.ja.md'],'risk_disposition':'11new rules quantum-uncertain; previouslyaccepted078PHP_PQC_API retains quantum-safe/info reference classification. It does not upgrade unknown constructions, assurance or readiness. Do not silently reclassify an accepted rule. Pairedcoverageguide explains this label. Initial audit helper incorrectly assumed all12uncertain; corrected assumption only, executableYAMLunchanged.', 'not_claimed':'305compiledpatterncontrols are not sourceaccuracy units or runtimecrypto validation.137rows are currentboundedreview, not unrestrictedquality.'}
(b/'RULE-ACCEPTANCE.json').write_text(json.dumps(result,indent=2,ensure_ascii=False)+'\n');print('137rules/125unchanged/12new/305controls matched')
