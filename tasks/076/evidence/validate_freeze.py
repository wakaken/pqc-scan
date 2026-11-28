"""Verify the adopted corpus/index; no subject matching or application execution."""
import hashlib, json, pathlib
B=pathlib.Path('tasks/076/evidence')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def main():
 f=json.loads((B/'FREEZE.json').read_text())
 for key in ['inputs','baseline_product_inputs']:
  for name,digest in f[key].items():assert sha(pathlib.Path(name))==digest,name
 sources={}
 for index in ['SOURCES.json','FORMAT-SOURCES.json']:
  for row in json.loads((B/index).read_text())['sources']:
   assert len(row['commit'])==40,row['id']; sources[row['id']]=row
   for entry in row['files']:
    if 'snapshot' in entry:assert sha(pathlib.Path(entry['snapshot']))==entry['sha256']
 for row in json.loads((B/'API-MATRIX.json').read_text())['rows']:
  assert all(s in sources for s in row['sources']),row['id']
 data=json.loads((B/'CORPUS-CASES.json').read_text())['cases'];ids=set()
 for row in data:
  assert row['id'] not in ids;ids.add(row['id']);p=pathlib.Path(row['path']);assert sha(p)==row['sha256'];n=len(p.read_bytes())
  for key in ['span','forbidden_span']:
   if row['expectation'].get(key):
    a,z=row['expectation'][key];assert 0<=a<z<=n,(row['id'],key)
 assert len(data)==f['denominators']['source_cases']
 assert len(list((B/'corpus').iterdir()))==len(data)
 deps=json.loads((B/'DEPENDENCY-MATRIX.json').read_text())['cases']
 for row in deps:
  assert sha(pathlib.Path(row['path']))==row['sha256']; assert all(s in sources for s in row['sources'])
 assert len(deps)==f['denominators']['dependency_cases']
 assert len(json.loads((B/'ROUTING-BOUNDARIES.json').read_text())['cases'])==f['denominators']['routing_cases']
 print(f"PASS: {len(data)} source, {len(deps)} dependency, 23 routing controls; source pins, hashes and original spans")
if __name__=='__main__':main()
