"""Final CLI controls for ambiguous/malformed headers; no scanned code executes."""
import hashlib,json,pathlib,subprocess
b=pathlib.Path('tasks/080/evidence');root=pathlib.Path('target/v05-c-family-adversarial');root.mkdir(exist_ok=True);checks=[]
cases=[
 ('malformed','void f(){OQS_KEM_new("ML-KEM-768");}\nvoid broken( {','OQS_KEM_new("ML-KEM-768")',['ML-KEM-768']),
 ('cpp-only','void f(){oqs::Signature key("ML-DSA-65"); key.sign(data);}',None,[]),
 ('conditional','void f(){EVP_PKEY_CTX *ctx;\n#if FEATURE\nctx=EVP_PKEY_CTX_new_from_name(NULL,"ML-KEM-768",NULL);\n#else\nctx=EVP_PKEY_CTX_new_from_name(NULL,"ML-KEM-1024",NULL);\n#endif\nEVP_PKEY_encapsulate(ctx,data);}','EVP_PKEY_encapsulate(ctx,data)',[]),
]
for label,source,focal,expected in cases:
 inp=root/label;inp.mkdir(exist_ok=True);(inp/'case.h').write_text(source);out=root/(label+'-out')
 args=['target/debug/pqc-scan','scan',str(inp),'--rules-dir','rules','--threads','1','--format','all','--diagnostics','--inventory-json','--repository-scope','v05/header-control','--require-complete','--out-dir',str(out)]
 r=subprocess.run(args,capture_output=True,text=True);(b/('cli-header-'+label+'.log')).write_text(r.stdout+r.stderr);assert r.returncode==3,(label,r.returncode,r.stderr)
 native=json.loads((out/'scan.inventory.v1.json').read_text());observations=[o for o in native['observations'] if o['rule_id'].startswith('C_FAMILY_')]
 if focal:
  start=source.index(focal);obs=[o for o in observations if o['target']['byte_start']==start and o['target']['byte_end']==start+len(focal)];assert len(obs)==1,(label,observations)
  names=[a['attributes']['identifier'] for a in native['assets'] if a['kind']=='algorithm' and a['id'] in obs[0]['asset_ids']];assert names==expected,(label,names)
 else:assert not observations,observations
 checks.append({'command':args,'exit':r.returncode,'expected_exit':3,'source_sha256':hashlib.sha256((inp/'case.h').read_bytes()).hexdigest(),'focal':focal,'expected_algorithms':expected,'files':{str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in out.iterdir() if p.is_file()}})
(b/'HEADER-ADVERSARIAL-CHECKS.json').write_text(json.dumps({'source_manifest_sha256':hashlib.sha256((b/'IMPLEMENTATION-SOURCE.json').read_bytes()).hexdigest(),'binary_sha256':hashlib.sha256(pathlib.Path('target/debug/pqc-scan').read_bytes()).hexdigest(),'checks':checks},indent=2)+'\n');print('PASS malformed/ambiguous/conditional shared-header CLI controls')
