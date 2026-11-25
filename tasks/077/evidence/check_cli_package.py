import pathlib,subprocess,json,hashlib,tarfile
b=pathlib.Path('tasks/077/evidence');root=pathlib.Path('target/v05-foundation-cli-v2');root.mkdir(parents=True,exist_ok=True);inputs=root/'input';rules=root/'rules';inputs.mkdir(exist_ok=True);rules.mkdir(exist_ok=True)
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
run('package-prepare',['python3','scripts/package-release.py','--binary','target/debug/pqc-scan','--variant','static','--out-dir','target/v05-foundation-package'])
archive=next(pathlib.Path('target/v05-foundation-package').glob('*.tar.gz'));run('package-verify',['python3','scripts/package-release.py','--verify',str(archive)]);run('package-install',['python3','tests/distribution/check-install.py',str(archive),'--out-dir','target/v05-foundation-install'])
with tarfile.open(archive) as t:
 invmember=next(x for x in t.getmembers() if x.name.endswith('/DEPENDENCY-LICENSES.json'));licenses=json.load(t.extractfile(invmember));rows=[x for x in licenses['packages'] if x['name'] in ['tree-sitter-php','tree-sitter-c-sharp','tree-sitter-c','tree-sitter-cpp']];assert len(rows)==4;assert all(x['notices'] for x in rows)
result={'source_manifest_sha256':hashlib.sha256((b/'IMPLEMENTATION-SOURCE.json').read_bytes()).hexdigest(),'binary_sha256':hashlib.sha256(pathlib.Path('target/debug/pqc-scan').read_bytes()).hexdigest(),'profile':'dev/default features, local preparation only','checks':checks,'assertions':['7 files / 8 observations and legacy findings','real original-byte UTF16 and UTF8 spans','one header occurrence / distinct same-line IDs','strict exit3 for unrendered template','all four grammar notices present in verified package'],'archive_sha256':hashlib.sha256(archive.read_bytes()).hexdigest(),'archive':str(archive)};(b/'CLI-PACKAGE-CHECKS.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'checks':len(checks),'all_expected_exits':True,'archive':str(archive)}))
