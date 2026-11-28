import pathlib,subprocess,json,hashlib
b=pathlib.Path('tasks/077/evidence');out=pathlib.Path('target/v05-foundation-compat');out.mkdir(exist_ok=True);checks=[]
for group,args in [
 ('old-seven',['scan','tasks/076/evidence/old-seven','--threads','1','--format','all','--diagnostics','--inventory-json','--repository-scope','v05/frozen','--rules-dir','rules']),
 ('saved',['assess','tasks/076/evidence/saved.inventory.json','--input-format','inventory-v1','--as-of','2026-11-24T00:00:00Z'])]:
 argv=['target/debug/pqc-scan',*args,'--out-dir',str(out/group)];r=subprocess.run(argv,capture_output=True,text=True);(b/('compat-'+group+'.log')).write_text(r.stdout+r.stderr);assert r.returncode==0
 base=pathlib.Path('tasks/076/evidence/measure-baseline-review1')/group/'1';files=['report.json','scan.inventory.v1.json'] if group=='old-seven' else ['migration.assessment.v1.json'];rows=[]
 for name in files:
  a=json.loads((base/name).read_text());z=json.loads((out/group/name).read_text());diff=[]
  def walk(x,y,path=''):
   if type(x)!=type(y):diff.append(path);return
   if isinstance(x,dict):
    for k in set(x)|set(y):
     if k not in x or k not in y:diff.append(path+'/'+k)
     else:walk(x[k],y[k],path+'/'+k)
   elif isinstance(x,list):
    if len(x)!=len(y):diff.append(path+'/length')
    else:
     for i,(v,w) in enumerate(zip(x,y)):walk(v,w,path+'/'+str(i))
   elif x!=y:diff.append(path)
  walk(a,z)
  permitted={'/generated_at','/run/generated_at','/run/id'}
  id_map={}
  if name=='scan.inventory.v1.json':
   def native_id(domain,parts):
    h=hashlib.sha256()
    for value in ['pqc-scan-native','1',domain]:
     v=value.encode();h.update(len(v).to_bytes(8,'big'));h.update(v)
    h.update(len(parts).to_bytes(8,'big'))
    for value in parts:
     v=value.encode();h.update(len(v).to_bytes(8,'big'));h.update(v)
    return domain+'-v1-'+h.hexdigest()
   assert len(a['observations'])==len(z['observations'])
   for i,(left,right) in enumerate(zip(a['observations'],z['observations'])):
    assert left['occurrence_key']==right['occurrence_key']
    for doc,obs in [(a,left),(z,right)]:
     usage=next(v for v in doc['assets'] if v['kind']=='usage' and v['id'] in obs['asset_ids'])
     expected=native_id('observation',[doc['run']['id'],'static-1',usage['attributes']['anchor'],obs['target']['path'],str(usage['attributes']['local_ordinal'])])
     assert obs['id']==expected
    id_map[right['id']]=left['id'];permitted.add('/observations/'+str(i)+'/id')
   assert len(id_map)==len(a['observations'])
   for i,(left,right) in enumerate(zip(a['legacy_associations'],z['legacy_associations'])):
    assert left['observation_ids']==[id_map[v] for v in right['observation_ids']]
    for j in range(len(left['observation_ids'])):permitted.add('/legacy_associations/'+str(i)+'/observation_ids/'+str(j))
  assert set(diff)<=permitted,(group,name,diff)
  rows.append({'file':name,'exact_differing_fields':sorted(diff),'all_other_fields_identical':True,'observation_id_bijection':id_map,'baseline_sha256':hashlib.sha256((base/name).read_bytes()).hexdigest(),'current_sha256':hashlib.sha256((out/group/name).read_bytes()).hexdigest()})
 checks.append({'group':group,'command':argv,'exit':r.returncode,'comparisons':rows})
(b/'COMPATIBILITY.json').write_text(json.dumps({'source_manifest_sha256':hashlib.sha256((b/'IMPLEMENTATION-SOURCE.json').read_bytes()).hexdigest(),'baseline':'076 measure-baseline-review1 exact same input bytes/args/profile','checks':checks},indent=2)+'\n');print('PASS old-seven and held saved-input semantics; exact timestamp/run derivation changes only')
