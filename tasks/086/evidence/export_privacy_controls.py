from pathlib import Path
import json,hashlib,subprocess
R=Path('/Users/waka/.codex/worktrees/pqc-language-research/pqc-scan');E=R/'tasks/086/evidence';C=Path('/Users/waka/.cache/pqc-scan-oss-validation-086');sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();read=lambda p:json.loads(p.read_text());M=E/'IMPLEMENTATION-SOURCE-05.json';m=read(M);receipt=read(E/'OSS-RESCAN-05.json');assert len(receipt['projects'])==4 and receipt['sourceManifestSha256']==sha(M);binary=C/'pqc-scan-source05';assert sha(binary)==receipt['binarySha256'];root=C/'export-control-input';out=C/'export-control-output';root.mkdir(exist_ok=False);sources={
 'case.php':"<?php openssl_sign($data,$sig,'EXPORT086_SECRET_PHP');",
 'case.c':'void f(){EVP_PKEY_new_raw_private_key(0,0,"EXPORT086_SECRET_C",32);}',
 'case.cpp':'void f(){OQS_KEM_decaps(k,"EXPORT086_SECRET_CPP",data,more);}',
 'case.cs':'using System.Security.Cryptography; class C{void M(){MLKem.ImportPrivateSeed(MLKemAlgorithm.MLKem768,"EXPORT086_SECRET_CS");}}',
 'Case.java':'import java.security.KeyPairGenerator; class C{void f()throws Exception{KeyPairGenerator.getInstance("RSA");}} // password="EXPORT086_SECRET_JAVA"',
 'case.go':'package main\nimport "crypto/rsa"\nfunc f(){rsa.GenerateKey(nil,2048)}\n// password="EXPORT086_SECRET_GO"',
}
for name,text in sources.items():
 data=text.encode()
 if name=='case.cpp':data=b'\xff\xfe'+text.encode('utf-16-le')
 (root/name).write_bytes(data)
cmd=[str(binary),'scan',str(root),'--format','all','--inventory-json','--repository-scope','synthetic/privacy086','--diagnostics','--threads','1','--rules-dir',str(R/'rules'),'--out-dir',str(out)];p=subprocess.run(cmd,cwd=R,capture_output=True,text=True);assert p.returncode==0,(p.returncode,p.stderr);outputs={}
for path in out.iterdir():
 if not path.is_file():continue
 data=path.read_bytes();assert b'EXPORT086_SECRET_'not in data,path.name;outputs[path.name]=sha(path)
assert {'report.html','report.md','report.json','report.sarif','cbom.json','dependency-sbom.json','scan.inventory.v1.json','diagnostics.json'}<=outputs.keys();d=read(out/'report.json');f=d['findings'];assert set(sources)=={x['location']['file']for x in f};assert all('arguments omitted'not in row['algorithm']for row in d['cbom']);assert b'arguments omitted'not in(out/'scan.inventory.v1.json').read_bytes();public={name:sum(x['location']['file']==name and x['evidence']['match'].endswith('(/* arguments omitted */)')for x in f)for name in sources};assert all(public[name]>0 for name in ['case.php','case.c','case.cpp','case.cs'])
for n,v in m['runtimeSourceBindings'].items():assert sha(R/n)==v,n
(E/'EXPORT-PRIVACY-CONTROLS-05.json').write_text(json.dumps(dict(sourceManifestSha256=sha(M),binarySha256=sha(binary),command=cmd,exitCode=p.returncode,operationSha256=sha(Path(__file__)),inputHashes={n:sha(root/n)for n in sources},outputs=outputs,publicExcerpts=public,allFormatsContainNoSyntheticSecrets=True,nativeAndCbomContainNoDisplayOnlyExcerpts=True,scope='Main-authored synthetic six-language consuming export smoke, including UTF16 C++. Not external accuracy or runtime crypto evidence.'),indent=2)+'\n');print('allformat privacy smoke passed',public)
