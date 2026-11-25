import pathlib,subprocess,json,hashlib,tarfile,datetime
b=pathlib.Path('tasks/080/evidence');root=pathlib.Path('target/v05-c-family-cli-v2');root.mkdir(parents=True,exist_ok=True);inputs=root/'input';rules=root/'rules';inputs.mkdir(exist_ok=True);rules.mkdir(exist_ok=True)
specs={'a.phtml':'日本語<?php $CONTROL=1; $CONTROL=2; ?>','a.cs':'namespace App; class C { int CONTROL; }','a.c':'int CONTROL;','upper.C':'namespace App { int CONTROL; }','a.h':'int CONTROL;','b.blade.php':'@if($ok) <?php $CONTROL=1; ?> @endif'}
for n,t in specs.items():(inputs/n).write_text(t)
text='<?php /* 日本語 */ $CONTROL=1;';(inputs/'encoded.php').write_bytes(b'\xff\xfe'+text.encode('utf-16le'))
(rules/'r.yml').write_text(json.dumps([{'id':'FOUNDATION_CONTROL','kind':'tree_sitter','scope':'php,csharp,c,cpp','pattern':'CONTROL','category':'test','severity':'info','risk':'quantum-uncertain','confidence':0.5,'migration_hint':'Review'}]))
checks=[]
def run(label,args,expected=0):
 with (b/(label+'.log')).open('w') as f:p=subprocess.run(args,stdout=f,stderr=subprocess.STDOUT)
 checks.append({'label':label,'command':args,'exit':p.returncode,'expected_exit':expected,'log':str(b/(label+'.log'))});assert p.returncode==expected,label
args=['target/debug/pqc-scan','scan',str(inputs),'--threads','1','--rules-dir',str(rules),'--format','all','--diagnostics','--inventory-json','--repository-scope','v05/foundation','--out-dir',str(root/'output')]
run('cli-scan',args);run('cli-strict',args[:-1]+[str(root/'strict-output'),'--require-complete'],3)
inv=json.loads((root/'output/scan.inventory.v1.json').read_text());legacy=json.loads((root/'output/report.json').read_text());assert len(inv['observations'])==8,len(inv['observations']);assert len(legacy['findings'])==8
for obs in inv['observations']:
 t=obs['target'];name=pathlib.Path(t['path']).name;assert obs['assurance']=='syntactic' and obs['usage']=='referenced';raw=(inputs/name).read_bytes();a,z=t['byte_start'],t['byte_end'];assert raw[a:z]==('CONTROL'.encode('utf-16le') if name=='encoded.php' else b'CONTROL'),(name,a,z);assert t['column_kind']=='unicode_code_points'
assert len({o['id'] for o in inv['observations']})==8
run('package-prepare',['python3','scripts/package-release.py','--binary','target/debug/pqc-scan','--variant','static','--out-dir','target/v05-c-family-package'])
archive=next(pathlib.Path('target/v05-c-family-package').glob('*.tar.gz'));run('package-verify',['python3','scripts/package-release.py','--verify',str(archive)]);run('package-install',['python3','tests/distribution/check-install.py',str(archive),'--out-dir','target/v05-c-family-install'])
with tarfile.open(archive) as t:
 invmember=next(x for x in t.getmembers() if x.name.endswith('/DEPENDENCY-LICENSES.json'));licenses=json.load(t.extractfile(invmember));rows=[x for x in licenses['packages'] if x['name'] in ['tree-sitter-php','tree-sitter-c-sharp','tree-sitter-c','tree-sitter-cpp']];assert len(rows)==4;assert all(x['notices'] for x in rows)
result={'source_manifest_sha256':hashlib.sha256((b/'IMPLEMENTATION-SOURCE.json').read_bytes()).hexdigest(),'binary_sha256':hashlib.sha256(pathlib.Path('target/debug/pqc-scan').read_bytes()).hexdigest(),'profile':'dev/default features, local preparation only','checks':checks,'assertions':['7 files / 8 observations and legacy findings','real original-byte UTF16 and UTF8 spans','one header occurrence / distinct same-line IDs','strict exit3 for unrendered template','all four grammar notices present in verified package'],'archive_sha256':hashlib.sha256(archive.read_bytes()).hexdigest(),'archive':str(archive)};(b/'CLI-PACKAGE-CHECKS.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'checks':len(checks),'all_expected_exits':True,'archive':str(archive)}))

php = root/'php-input'; php.mkdir(exist_ok=True)
secret='PRIVATE078_KEY_CANARY_abcdefghijklmnopqrstuv0123456789'
source="<?php $key=\\PQCrypto\\MLDSA44\\SigningKey::fromBytes('"+secret+"'); $key->sign('message'); openssl_sign('message',$sig,$imported);"
(php/'api.php').write_text(source)
run('cli-php-default',['target/debug/pqc-scan','scan',str(php),'--threads','1','--rules-dir','rules','--format','all','--diagnostics','--inventory-json','--repository-scope','v05/php','--out-dir',str(root/'php-output')])
for file in (root/'php-output').iterdir():
 if file.is_file(): assert secret.encode() not in file.read_bytes(),str(file)
actual=json.loads((root/'php-output/scan.inventory.v1.json').read_text())
obs=[o for o in actual['observations'] if o['rule_id'].startswith('PHP_')]
assert len(obs)==3,obs
assert all(o['assurance']=='syntactic' and o['usage']=='referenced' for o in obs)
unknown=next(o for o in obs if o['rule_id']=='PHP_CRYPTO_API')
assert not any(a['kind']=='algorithm' and a['id'] in unknown['asset_ids'] for a in actual['assets'])
result['checks']=checks; result['assertions'] += ['Real PHP default-rule native and all report/export files omit private canary','Three PHP observations retain syntactic referenced assurance; imported OpenSSL key type stays unknown']
(b/'CLI-PACKAGE-CHECKS.json').write_text(json.dumps(result,indent=2)+'\n')

cs=root/'csharp-input';cs.mkdir(exist_ok=True)
secret='PRIVATE079_KEY_CANARY_abcdefghijklmnopqrstuvwxyz0123456789'
source='using dotnetOQS; using OpenForge.Cryptography.LibOqs.KEM; using Org.BouncyCastle.Tls; class C{void M(){var k=System.Security.Cryptography.MLKem.ImportPrivateSeed(System.Security.Cryptography.MLKemAlgorithm.MLKem768,System.Text.Encoding.UTF8.GetBytes("'+secret+'"));k.Encapsulate(data);System.Security.Cryptography.Hpke.GenerateKey(suite);var rsa=System.Security.Cryptography.RSA.Create();rsa.SignData(data);}}'
(cs/'api.cs').write_text(source)
run('cli-csharp-default',['target/debug/pqc-scan','scan',str(cs),'--threads','1','--rules-dir','rules','--format','all','--cyclonedx-17','--diagnostics','--inventory-json','--repository-scope','v05/csharp','--out-dir',str(root/'csharp-output')])
for file in (root/'csharp-output').iterdir():
 if file.is_file():assert secret.encode() not in file.read_bytes(),str(file)
actual=json.loads((root/'csharp-output/scan.inventory.v1.json').read_text());obs=[o for o in actual['observations'] if o['rule_id'].startswith('CSHARP_')]
assert len(obs)==8,len(obs)
assert all(o['assurance']=='syntactic' and o['usage']=='referenced' for o in obs)
assert {a['attributes'].get('identifier') for a in actual['assets'] if a['kind']=='algorithm'}=={'ML-KEM-768','RSA','HPKE'}
partial=root/'csharp-partial';partial.mkdir(exist_ok=True);(partial/'unknown.cs').write_text('class C{void M(){System.Security.Cryptography.MLKem.GenerateKey(dynamicAlgorithm);}}')
run('cli-csharp-strict',['target/debug/pqc-scan','scan',str(partial),'--rules-dir','rules','--require-complete','--out-dir',str(root/'csharp-partial-output')],3)
result['checks']=checks;result['environment']={'CARGO_INCREMENTAL':'0'};result['assertions']+=['Real C# default scan: eight observations; exact ML-KEM-768/RSA and unresolved HPKE; no canary in all reports or CycloneDX1.7 exports','C# unknown dynamic selector strict exit3']
(b/'CLI-PACKAGE-CHECKS.json').write_text(json.dumps(result,indent=2)+'\n')

legacy=json.loads((root/'csharp-output/report.json').read_text())
for name,label,marker in [
 ('dotnetOQS','C# obsolete official dotnetOQS wrapper reference','library_ref_csharp_dotnetoqs_legacy'),
 ('OpenForge.Cryptography.LibOqs.KEM','C# OpenForge community OQS wrapper reference','library_ref_csharp_openforge_community'),
 ('Org.BouncyCastle.Tls','C# Bouncy Castle TLS library reference','library_ref_csharp_bouncycastle_tls')]:
 start=source.index(name)
 item=next(o for o in obs if o['target']['byte_start']==start)
 assert item['evidence']['marker_kind']==marker
 finding=next(f for f in legacy['findings'] if f['location']['column']==start+1)
 assert finding['evidence']['match']==label
 link=next(a for a in actual['legacy_associations'] if a['legacy_id']==finding['finding_id'])
 assert link['observation_ids']==[item['id']]
 assert any(r['algorithm']==label for r in legacy['cbom'])
result['assertions']+=['Each opaque library occurrence retains its own finite public label, native marker and exact legacy link after final export; source preview remains masked']
(b/'CLI-PACKAGE-CHECKS.json').write_text(json.dumps(result,indent=2)+'\n')

cf=root/'c-family-input';cf.mkdir(exist_ok=True)
secret='PRIVATE080_CLI_KEY_CANARY_abcdefghijklmnopqrstuvwxyz0123456789'
(cf/'api.c').write_text('#include <openssl/evp.h>\n#include <oqs/oqs.h>\nvoid f(){EVP_PKEY_CTX *ctx=EVP_PKEY_CTX_new_from_name(NULL,"ML-KEM-768",NULL);EVP_PKEY *key=NULL;EVP_PKEY_keygen(ctx,&key);EVP_PKEY_encapsulate(ctx,"'+secret+'",more);OSSL_PROVIDER_load(NULL,"oqsprovider");EVP_set_default_properties(NULL,"provider=default");SSL_CTX_set1_groups_list(ssl,"X25519MLKEM768:P-256");EVP_DigestSignInit(md,NULL,NULL,NULL,imported);RSA_new();}')
(cf/'api.cpp').write_text('#include "oqs_cpp.hpp"\nvoid f(){oqs::Signature s{"ML-DSA-65"};s.sign("'+secret+'");oqs::KeyEncapsulation k("ML-KEM-512");k.encap_secret(data);}')
header='/* 日本語💻 */ void f(){OQS_KEM_new("ML-KEM-1024"); OQS_KEM_new("ML-KEM-1024");}'
(cf/'common.h').write_bytes(b'\xff\xfe'+header.encode('utf-16le'))
run('cli-c-family-default',['target/debug/pqc-scan','scan',str(cf),'--threads','1','--rules-dir','rules','--format','all','--cyclonedx-17','--diagnostics','--inventory-json','--repository-scope','v05/c-family','--out-dir',str(root/'c-family-output')])
actual=json.loads((root/'c-family-output/scan.inventory.v1.json').read_text());legacy=json.loads((root/'c-family-output/report.json').read_text())
observations=[o for o in actual['observations'] if o['rule_id'].startswith('C_FAMILY_')]
assert len(observations)==17,len(observations)
assert all(o['assurance']=='syntactic' and o['usage']=='referenced' for o in observations)
assert {a['attributes'].get('identifier') for a in actual['assets'] if a['kind']=='algorithm'}=={'ML-KEM-512','ML-KEM-768','ML-KEM-1024','ML-DSA-65','RSA'}
markers={o['evidence']['marker_kind'] for o in observations if o['evidence']['marker_kind']}
assert markers=={'library_ref_c_openssl_include','library_ref_c_liboqs_include','library_ref_cpp_liboqs_include','source_provider_ref:oqsprovider','source_property_ref:provider=default','source_tls_group_ref:X25519MLKEM768:P-256'},markers
for o in observations:
 t=o['target'];raw=(cf/t['path']).read_bytes();a,z=t['byte_start'],t['byte_end']
 expression=raw[a:z].decode('utf-16le' if t['path']=='common.h' else 'utf-8')
 assert expression and t['column_kind']=='unicode_code_points'
 if t['path']=='common.h':assert expression=='OQS_KEM_new("ML-KEM-1024")'
 if o['evidence']['marker_kind']:
  assert not any(v['kind']=='algorithm' and v['id'] in o['asset_ids'] for v in actual['assets'])
  link=next(v for v in actual['legacy_associations'] if o['id'] in v['observation_ids']);finding=legacy['findings'][link['legacy_index']]
  assert finding['evidence']['match'].startswith('C/C++ ')
  assert finding['evidence']['match']!='[masked-sensitive-content]'
  assert finding['evidence']['snippet_preview']=='[masked-sensitive-content]'
unknown=next(o for o in observations if o['rule_id']=='C_FAMILY_CRYPTO_API' and (cf/o['target']['path']).read_bytes()[o['target']['byte_start']:o['target']['byte_end']].startswith(b'EVP_DigestSignInit'))
assert not any(a['kind']=='algorithm' and a['id'] in unknown['asset_ids'] for a in actual['assets'])
assert any(f['rule_id']=='API_OPENSSL_RSA_NEW' for f in legacy['findings'])
for file in (root/'c-family-output').iterdir():
 if file.is_file():assert secret.encode() not in file.read_bytes(),str(file)
run('cli-c-family-assess',['target/debug/pqc-scan','assess',str(root/'c-family-output/scan.inventory.v1.json'),'--input-format','inventory-v1','--as-of',(datetime.datetime.fromisoformat(actual['run']['generated_at'])+datetime.timedelta(seconds=1)).strftime('%Y-%m-%dT%H:%M:%SZ'),'--out-dir',str(root/'c-family-assessment')])
assert (root/'c-family-assessment/migration.assessment.v1.json').exists()
partial=root/'c-family-partial';partial.mkdir(exist_ok=True);(partial/'unknown.c').write_text('void f(){EVP_PKEY_CTX_new_from_name(NULL,dynamicName,NULL);}')
run('cli-c-family-strict',['target/debug/pqc-scan','scan',str(partial),'--rules-dir','rules','--require-complete','--out-dir',str(root/'c-family-partial-output')],3)
result['checks']=checks;result['assertions']+=['C/C++ real default rules:17 typed source observations; exact5 primitives,3 include identities and3 provider/property/group references without algorithm assets; unknown digest key remains unknown','UTF16 shared header retains two original expression spans once each; legacy OpenSSL rule occurrences remain','All formats/CycloneDX1.7 omit key canary; saved native configuration/library markers ingest through existing inventory-v1 reader; dynamic selector strict3']
(b/'CLI-PACKAGE-CHECKS.json').write_text(json.dumps(result,indent=2)+'\n')
