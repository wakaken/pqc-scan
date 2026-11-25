import pathlib,json,re,statistics,hashlib
b=pathlib.Path('tasks/083/evidence');phases={p:json.loads((b/('measure-'+p)/'OBSERVATIONS.json').read_text())for p in ['baseline-comparable','final']}
a,z=phases.values()
for key in ['host','machine','rust','profile','features','CARGO_INCREMENTAL','inputs','protocol_sha256']:assert a[key]==z[key],key
assert len(a['runs'])==len(z['runs'])==16
rows=[]
for phase,data in phases.items():
 for group,denominator in data['inputs'].items():
  observations=[]
  for r in data['runs']:
   assert r['exit']==0
   if r['group']!=group or r['warmup']:continue
   raw=pathlib.Path(r['resource_log']).read_text();m=re.search(r'^\s*(\d+)\s+maximum resident set size$',raw,re.M);assert m
   observations.append({'sample':r['sample'],'wall_seconds':r['wall_seconds'],'peak_rss_bytes':int(m[1]),'output_bytes':sum(x['bytes']for x in r['outputs'].values()),'output_file_count':len(r['outputs'])})
  assert len(observations)==3
  rows.append({'phase':phase,'group':group,'input_files':denominator['files'],'input_bytes':denominator['bytes'],'observations':observations,'summaries':{k:{'median':statistics.median(o[k]for o in observations),'min':min(o[k]for o in observations),'max':max(o[k]for o in observations)}for k in ['wall_seconds','peak_rss_bytes','output_bytes']}})
result={'protocol_sha256':a['protocol_sha256'],'host':a['host'],'machine':a['machine'],'toolchain':a['rust'],'CARGO_INCREMENTAL':'0','binary_sha256':{p:x['binary_sha256']for p,x in phases.items()},'comparability':'Onewarmup+3sequential invocations/group/phase. Samepaths/scope/workers/limits/profile/defaultfeatures/host/toolchain/observer, separatebaseline/finalrules copiedto samepath. Compilationexcluded. Rules/binary/source and phaseoutputpaths differintentionally. Original076incrementalstratum preserved, not pooled.','rows':rows,'interpretation':'Descriptive syntheticcorpus timing/RSS/size only; no posthocnumericgate, independentpopulationaccuracy, realapplicationexecution orhumanusefulness claim. PeakRSS perprocess, not summed.'}
(b/'MEASUREMENT-SUMMARY.json').write_text(json.dumps(result,indent=2)+'\n')
for row in rows: print(row['phase'],row['group'],row['input_files'],row['input_bytes'],{k:round(v['median'],5)for k,v in row['summaries'].items()})
