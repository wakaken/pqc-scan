import pathlib,json,gzip,difflib,subprocess,collections
root=pathlib.Path.cwd();r=pathlib.Path(__file__).parent
p=root/'docs/releases/static-candidate-20261117/quality-completion/BRANCH-ASSESSMENTS.json';j=json.loads(p.read_text());import sys
c=r/(sys.argv[1] if len(sys.argv)>1 else 'coverage')/'static-branches/coverage.json.gz';data=json.loads(gzip.decompress(c.read_bytes()));coverage={}
for f in data['data'][0]['files']:
 if '/crates/' not in f['filename']:continue
 name='crates/'+f['filename'].split('/crates/',1)[1]
 for b in f['branches']:
  key=(name,tuple(b[:4]));v=coverage.setdefault(key,[False,False]);v[0]|=b[4]>0;v[1]|=b[5]>0
maps={}
for row in j['rows']:
 name=row['path']
 if name not in maps:
  a=subprocess.check_output(['git','show','ee6661a:'+name],text=True).splitlines();b=(root/name).read_text().splitlines();mapping={}
  for tag,a0,a1,b0,b1 in difflib.SequenceMatcher(None,a,b,autojunk=False).get_opcodes():
   if tag=='equal' or tag=='replace' and a1-a0==b1-b0:
    for x,y in zip(range(a0,a1),range(b0,b1)):mapping[x+1]=y+1
  maps[name]=(mapping,a,b)
 mapping,a,b=maps[name];span=row['span']
 overrides={326:'universal_shape(&v, Tag::Integer, false)',334:'universal_shape(&y, Tag::Integer, false)',386:'!universal_shape(p, Tag::Sequence, true)'} if name.endswith('/attributes.rs') else {}
 fragment=overrides.get(span[0])
 if fragment is None and span[0]==span[2]:fragment=a[span[0]-1][span[1]-1:span[3]-1]
 if span[0]==span[2]:
  expected=mapping.get(span[0])
  if expected is None:
   near=min(mapping,key=lambda n:abs(n-span[0]));expected=mapping[near]+span[0]-near
  candidates=[(abs(index+1-expected),index+1,line.index(fragment)+1) for index,line in enumerate(b) if fragment in line]
  assert candidates,(name,span,fragment)
  candidates.sort();assert len(candidates)==1 or candidates[0][0]!=candidates[1][0],(name,span,candidates[:2])
  _,line,col=candidates[0];new=[line,col,line,col+len(fragment)]
 else:
  new=[mapping[span[0]],span[1],mapping[span[2]],span[3]]
 row['mapping']='Stricter universal class/tag/shape predicate replacing old tag-only guard' if span[0] in overrides else 'Exact original predicate text retained at mapped source position'
 row['currentSpan']=new;row['currentSourceLine']=b[new[0]-1];row['currentOutcomes']=coverage.get((name,tuple(new)))
 row['assessment']='ObservedBoth; behavioral-test/oracle linkage pending' if row['currentOutcomes']==[True,True] else 'Individual missing-outcome adjudication pending; not Passed'
print(collections.Counter(x['assessment'] for x in j['rows']))
for x in j['rows']:
 if x['currentOutcomes']!=[True,True]: print(x['path'].split('/src/')[-1],x['span'],x['currentSpan'],x['currentOutcomes'],x['currentSourceLine'])
(r/'MAPPED-OUTCOMES.json').write_text(json.dumps(j,indent=2)+'\n')
