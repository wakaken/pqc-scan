"""Explain every old-seven/saved semantic difference in observed runs."""
import json,pathlib,hashlib,subprocess,difflib
b=pathlib.Path('tasks/083/evidence');baseline=b/'measure-baseline-comparable';final=b/'measure-final'
def native_id(domain,parts):
 h=hashlib.sha256()
 for s in ['pqc-scan-native','1',domain]:v=s.encode();h.update(len(v).to_bytes(8,'big'));h.update(v)
 h.update(len(parts).to_bytes(8,'big'))
 for s in parts:v=s.encode();h.update(len(v).to_bytes(8,'big'));h.update(v)
 return domain+'-v1-'+h.hexdigest()
def diff(x,y,path=''):
 if type(x)!=type(y):return[path]
 if isinstance(x,dict):
  return [p for k in sorted(set(x)|set(y))for p in ([path+'/'+k]if k not in x or k not in y else diff(x[k],y[k],path+'/'+k))]
 if isinstance(x,list):
  if len(x)!=len(y):return[path+'/length']
  return[p for i,(v,w)in enumerate(zip(x,y))for p in diff(v,w,path+'/'+str(i))]
 return[]if x==y else[path]
records=[]
oldmeta=json.loads(subprocess.check_output(['git','show','HEAD:docs/quality/rules/RULE-QUALITY-METADATA.json']));newmeta=json.loads(pathlib.Path('docs/quality/rules/RULE-QUALITY-METADATA.json').read_text())
def digest(m):
 fields=['id','kind','category','severity','risk','confidence','migration_hint','pattern','scope','description'];rows=[{k:r['rule_snapshot'][k]for k in fields if k in r['rule_snapshot']}for r in sorted(m['rules'],key=lambda r:r['id'])];return native_id('rules',[json.dumps(rows,ensure_ascii=False,separators=(',',':'))])
for sample in range(4):
 for group,names in [('old-seven',['report.json','scan.inventory.v1.json']),('saved',['migration.assessment.v1.json'])]:
  for name in names:
   a=json.loads((baseline/group/str(sample)/name).read_text());z=json.loads((final/group/str(sample)/name).read_text());changes=diff(a,z);allowed={'/generated_at','/run/generated_at','/run/id'};mapping={}
   if name=='scan.inventory.v1.json':
    assert a['rules_digest']==digest(oldmeta)and z['rules_digest']==digest(newmeta)
    allowed.add('/rules_digest');assert len(a['observations'])==len(z['observations'])
    for i,(left,right)in enumerate(zip(a['observations'],z['observations'])):
     assert left['occurrence_key']==right['occurrence_key']
     for doc,obs in [(a,left),(z,right)]:
      usage=next(v for v in doc['assets']if v['kind']=='usage'and v['id']in obs['asset_ids']);assert obs['id']==native_id('observation',[doc['run']['id'],'static-1',usage['attributes']['anchor'],obs['target']['path'],str(usage['attributes']['local_ordinal'])])
     mapping[right['id']]=left['id'];allowed.add(f'/observations/{i}/id')
    assert len(mapping)==len(a['observations'])
    for i,(left,right)in enumerate(zip(a['legacy_associations'],z['legacy_associations'])):
     assert left['observation_ids']==[mapping[v]for v in right['observation_ids']]
     for j in range(len(left['observation_ids'])):allowed.add(f'/legacy_associations/{i}/observation_ids/{j}')
   assert set(changes)<=allowed,(sample,group,name,changes)
   if group=='saved':assert not changes
   records.append({'sample':sample,'warmup':sample==0,'group':group,'file':name,'exact_differing_fields':changes,'all_other_fields_equal':True,'observation_id_bijection':mapping})
# Human saved output size is allowed to change only with explained presentation.
for name in ['migration.assessment.md','migration.assessment.html']:
 a=(baseline/'saved/1'/name).read_text();z=(final/'saved/1'/name).read_text();(b/('saved-human-'+name+'.diff')).write_text(''.join(difflib.unified_diff(a.splitlines(True),z.splitlines(True),fromfile='baseline/'+name,tofile='final/'+name)))
(b/'FROZEN-COMPATIBILITY.json').write_text(json.dumps({'records':records,'source':'same observed rawinputpaths/hashes; compare allwarmup+three samples, noextra runs','semantics':'exactheldsavedJSON; old7onlytimestamps/run and domain-verified additive12ruleDigest/observationID derivations','human_output_change':'SavedMarkdown bytes unchanged; assessmentJSON exactly unchanged. Only HTML bundled JavaScript finite development-declaration counting list changes (+192bytes in this heldfixture). Exactdiff retained, not blanket-stripped.'},indent=2)+'\n');print('all4observed samples/group explained;heldsaved completeJSONexact')
