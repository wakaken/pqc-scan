"""Trace frozen API descriptors to actually asserted focal corpus expressions.
Text indexing locates corpus cases only; detector/SavedEvidence tests establish behavior.
"""
import json,re,pathlib,hashlib
root=pathlib.Path('.');base=root/'tasks/076/evidence';matrix=json.loads((base/'API-MATRIX.json').read_text());cases=json.loads((base/'CORPUS-CASES.json').read_text())['cases']
records=[]
for c in cases:
 e=c['expectation']
 if not isinstance(e['span'],list):continue
 p=root/c['path'];raw=p.read_bytes();assert hashlib.sha256(raw).hexdigest()==c['sha256']
 records.append({**c,'text':raw.decode(),'focal':raw[e['span'][0]:e['span'][1]].decode()})
def token(s,t):return re.search(r'(?<![A-Za-z0-9_])'+re.escape(t)+r'(?![A-Za-z0-9_])',s,re.I) is not None
def cohort(row,c):
 if c['language'] not in row['language'].split(','):return False
 s=c['text'];r=row['id']
 return {'php-openssl':lambda:'openssl_' in s,'php-sodium':lambda:'sodium_crypto_' in s,'php-paragonie':lambda:'PQCrypto\\' in s or 'PQCrypto;' in s,'php-reference':lambda:'phpseclib3\\' in s or 'PQCrypto\\SLHDSA' in s,'csharp-standard':lambda:'System.Security.Cryptography.' in s,'csharp-dotnet-pqc':lambda:'System.Security.Cryptography.' in s,'csharp-bc':lambda:'Org.BouncyCastle.' in s,'csharp-construction':lambda:'System.Security.Cryptography.' in s,'c-openssl':lambda:any(x in s for x in ['EVP_','RSA_','DSA_','DH_','EC_KEY_','ECDSA_','ECDH_']),'c-provider-tls':lambda:any(x in s for x in ['OSSL_PROVIDER_','EVP_set_default_properties','set1_groups_list']),'c-oqs':lambda:'OQS_' in s,'cpp-oqs':lambda:'oqs::' in s,'third-party-wrapper-references':lambda:True}[r]()
def operations(op):
 if '/' not in op:return [op]
 owner,leaf=op.split('.',1);parts=leaf.split('/');return [owner+'.'+p for p in parts]
def has_operation(op,c):
 s,f=c['text'],c['focal']
 if c['language']=='php' and 'sodium_crypto_' in s:
  return token(f,'sodium_crypto_'+op)
 receiver_classes={'MLKemEncapsulator':['Encapsulate'],'MLKemDecapsulator':['Decapsulate'],'MLDsaSigner':['GenerateSignature','VerifySignature'],'SlhDsaSigner':['GenerateSignature','VerifySignature']}
 if op in receiver_classes:return token(s,op) and any(token(f,m)for m in receiver_classes[op])
 if '::' in op and '*' in op:
  owner,leaf=op.split('::',1);return token(s,owner) and bool(re.search(r'\b'+re.escape(leaf).replace(r'\*',r'[A-Za-z0-9_]*')+r'\s*\(',f))
 if op=='Import*':return bool(re.search(r'\bImport[A-Za-z0-9_]*\s*\(',f))
 if '.' in op:
  owner,leaf=op.rsplit('.',1)
  if token(s,owner) and token(f,leaf):return True
  return token(f,op)
 if '\\' in op:return op.lstrip('\\') in s
 return token(f,op) or (op=='OpenForge.Cryptography.LibOqs' and op in s)
def normalized(s):return re.sub('[^a-z0-9]','',s.lower()).replace('pure','')
rows=[];missing=[]
for row in matrix['rows']:
 cohort_cases=[c for c in records if cohort(row,c)]
 ops=[]
 for original in row['operations']:
  for op in operations(original):
   hits=[c['id']for c in cohort_cases if has_operation(op,c)]
   ops.append({'operation':op,'descriptor':original,'case_ids':hits})
   if not hits:missing.append((row['id'],'operation',op))
 parameters=[]
 for p in row['parameters']:
  term='HPKE' if p=='HPKE; suite unresolved' else p
  hits=[c['id']for c in cohort_cases if token(c['text'],term) or normalized(c['expectation']['canonical_algorithm'] or '')==normalized(term)]
  supplemental=[]
  if row['id']=='c-provider-tls' and p=='MLKEM768':
   path=root/'crates/pqc_scan_detectors/tests/c_family_source.rs';text=path.read_text();assert '"MLKEM768"' in text and 'fn c_family_group_lists_retain_standard_draft_and_alias_spelling_and_reject_opaque_data' in text
   supplemental=[{'path':str(path),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),'test':'c_family_group_lists_retain_standard_draft_and_alias_spelling_and_reject_opaque_data','scope':'MLKEM768 retained as requested configuration, with no algorithm asset; final component run required'}]
  parameters.append({'parameter':p,'case_ids':hits,'supplemental':supplemental})
  if not hits and not supplemental:missing.append((row['id'],'parameter',p))
 rows.append({'id':row['id'],'language':row['language'],'sources':row['sources'],'operations':ops,'parameters':parameters,'limits':row['limitation']})
result={'source_corpus_sha256':hashlib.sha256((base/'CORPUS-CASES.json').read_bytes()).hexdigest(),'api_matrix_sha256':hashlib.sha256((base/'API-MATRIX.json').read_bytes()).hexdigest(),'rows':rows,'missing':missing,'evidence_scope':'This text index is traceability, not a detection oracle. Each named focal case must pass the original assertion and saved-reader/evaluation integration in language_migration.rs on final source; no manager/compiler/runtime crypto executes. Alias/shadow/reassignment/help/comment and parameter/role cases remain in component suites. All rows remain incomplete until final-source test/review binding.'}
out=root/'tasks/082/evidence/API-COVERAGE.json';out.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'rows':len(rows),'missing':missing},indent=2))
