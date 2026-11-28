from pathlib import Path
import json,hashlib,subprocess
R=Path('/Users/waka/.codex/worktrees/pqc-language-research/pqc-scan');E=R/'tasks/087/evidence';C=Path('/Users/waka/.cache/pqc-scan-oss-validation-087');sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();read=lambda p:json.loads(p.read_text());M=E/'IMPLEMENTATION-SOURCE-09.json';m=read(M);receipt=read(E/'OSS-RESCAN-09.json');assert len(receipt['projects'])==4 and receipt['sourceManifestSha256']==sha(M);binary=C/'pqc-scan-source09';assert sha(binary)==receipt['binarySha256'];root=C/'export-control-input';out=C/'export-control-output';root.mkdir(exist_ok=False)
sources={
 'case.php':"<?php\nuse PQCrypto\\MLDSA44\\SigningKey as SK;\nuse PQCrypto\\MLDSA44\\VerifyingKey as VK;\nuse function openssl_sign as signIt;\n$k=SK::fromBytes('SYNTH087_ALIAS_IMPORT');\n$pub=VK::fromBytes('PUBLIC087_ALIAS_KEY');\nsignIt('PUBLIC087_MESSAGE',$sig,'SYNTH087_ALIAS_SIGN');\n$options=['password'=>'SYNTH087_ARRAY_PASSWORD','public_label'=>'PUBLIC087_OPTION'];\n$label='PUBLIC087_MESSAGE';\nsodium_crypto_sign_seed_keypair(seed:'SYNTH087_SEED');\nsodium_crypto_sign_publickey(key_pair:'SYNTH087_PAIR');\nopenssl_sign('PUBLIC087_MESSAGE',$sig,'SYNTH087_PHP_KEY');\nopenssl_pkey_get_private('file://keys/private.pem','file://SYNTH087_PASSPHRASE');\n\\ParagonIE\\PQCrypto\\Compat::mldsa44_sign('SYNTH087_COMPAT','PUBLIC087_MESSAGE');",
 'case.c':'void f(){\n const unsigned char material[32]="SYNTH087_C_KEY";\n EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,(unsigned char*)material,32);\n RSA_private_encrypt(size,from,to,"SYNTH087_RSA",padding);\n wc_RsaSSL_Sign(in,size,out,cap,"SYNTH087_WC",rng);\n RSA_new();\n}',
 'case.cpp':'void f(){\n const char *label="PUBLIC087_MESSAGE";\n oqs::Signature signer("ML-DSA-65",std::string("SYNTH087_CPP_KEY"));\n signer.sign(data);\n EVP_PKEY_new_raw_private_key(EVP_PKEY_ED25519,NULL,std::string("SYNTH087_CONVERSION"),32);\n RSA_new();\n}',
 'case.cs':'using System.Security.Cryptography; class C{void M(){\n var label="PUBLIC087_MESSAGE";\n byte[] privateKey={0x13,0x37,0x42};\n var material="SYNTH087_CS_SEED";\n MLKem.ImportPrivateSeed(MLKemAlgorithm.MLKem768,material);\n RSA.CreateFromEncryptedPem("SYNTH087_PEM","SYNTH087_PASSWORD");\n RSA.Create(2048);\n}}',
 'Case.java':'import java.security.KeyPairGenerator; class C{void f()throws Exception{\n// password="SYNTH087_JAVA"\nKeyPairGenerator.getInstance("RSA");\n}}',
 'case.go':'package main\nimport "crypto/rsa"\n// password="SYNTH087_GO"\nfunc f(){rsa.GenerateKey(nil,2048)}',
}
for name,text in sources.items():
 data=text.encode()
 if name=='case.cpp':data=b'\xff\xfe'+text.encode('utf-16-le')
 (root/name).write_bytes(data)
cmd=[str(binary),'scan',str(root),'--format','all','--inventory-json','--repository-scope','synthetic/privacy087','--diagnostics','--threads','1','--rules-dir',str(R/'rules'),'--out-dir',str(out)];p=subprocess.run(cmd,cwd=R,capture_output=True,text=True)
(E/'export-control-09.stdout.log').write_text(p.stdout);(E/'export-control-09.stderr.log').write_text(p.stderr);assert p.returncode==0,(p.returncode,p.stderr);outputs={}
for path in out.iterdir():
 if not path.is_file():continue
 data=path.read_bytes();assert b'SYNTH087_'not in data,path.name;assert b'0x13,0x37,0x42' not in data,path.name;outputs[path.name]=sha(path)
assert {'report.html','report.md','report.json','report.sarif','cbom.json','dependency-sbom.json','scan.inventory.v1.json','diagnostics.json'}<=outputs.keys()
d=read(out/'report.json');f=d['findings'];assert set(sources)=={x['location']['file']for x in f}
assert b'arguments omitted'not in(out/'scan.inventory.v1.json').read_bytes()
public={name:sum(x['location']['file']==name and any(line['highlighted'] and not line['text'].lstrip().startswith('[masked-') for line in (x.get('source_snippet')or{}).get('lines',[]))for x in f)for name in sources};assert all(public.values())
for name in ['case.php','case.cpp','case.cs']:
 assert any(x['location']['file']==name and 'PUBLIC087_MESSAGE' in json.dumps(x.get('source_snippet'))for x in f),name
assert any('PUBLIC087_OPTION' in json.dumps(x.get('source_snippet')) for x in f)
for n,v in m['runtimeSourceBindings'].items():assert sha(R/n)==v,n
(E/'EXPORT-PRIVACY-CONTROLS-09.json').write_text(json.dumps(dict(sourceManifestSha256=sha(M),binarySha256=sha(binary),command=cmd,exitCode=p.returncode,operationSha256=sha(Path(__file__)),inputHashes={n:sha(root/n)for n in sources},outputs=outputs,readableSourceLocations=public,allFormatsContainNoSyntheticSecrets=True,scope='Main-authored synthetic six-language all-format consumer smoke, including UTF16 C++, named Sodium seed/keypair, private slots, casts, conversion/OQS constructor keys and passphrase URI-like payloads. Not external accuracy or runtime crypto evidence.'),indent=2)+'\n');print('allformat privacy smoke passed',public)
