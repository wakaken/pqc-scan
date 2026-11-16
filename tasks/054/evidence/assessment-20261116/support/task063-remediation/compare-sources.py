from pathlib import Path
import urllib.request,json,hashlib,tarfile,io,difflib,datetime
OUT=Path('target/quality-task-063/upstream');OUT.mkdir(exist_ok=True)
old=json.load(open('tasks/051/evidence/dependency-20261114/final-run/artifacts/review/dependency-review.json'))
meta=json.load(open('tasks/051/evidence/dependency-20261114/final-run/artifacts/review/raw/all-probes-metadata.stdout'))
receipts=[]
def get(url,name,limit=32*1024**2):
 p=OUT/name
 if not p.exists():
  with urllib.request.urlopen(urllib.request.Request(url,headers={'User-Agent':'pqc-scan-dependency-review'}),timeout=60) as r:b=r.read(limit+1)
  if len(b)>limit:raise ValueError('download limit')
  p.write_bytes(b)
 b=p.read_bytes();receipts.append({'url':url,'path':name,'sha256':hashlib.sha256(b).hexdigest(),'retrieved_at':datetime.datetime.now(datetime.timezone.utc).isoformat()});return b

def members(data):
 result={};total=0
 with tarfile.open(fileobj=io.BytesIO(data),mode='r:gz') as t:
  for i,m in enumerate(t):
   if i>20000:raise ValueError('member limit')
   if not m.isfile():continue
   total+=m.size
   if m.size>16*1024**2 or total>128*1024**2:raise ValueError('decoded limit')
   name='/'.join(Path(m.name).parts[1:]);assert name not in result
   result[name]=t.extractfile(m).read()
 return result
results=[]
for name in ['russh-util','idna','ring']:
 p=next(p for p in old['variants']['all-probes']['packages'] if p['name']==name);m=next(p for p in meta['packages'] if p['name']==name)
 version=p['version'];vcs=p['vcs'];rev=vcs['git']['sha1'];repo=m['repository'].removesuffix('/').removesuffix('.git').removeprefix('https://github.com/')
 archive=get(f'https://static.crates.io/crates/{name}/{name}-{version}.crate',f'{name}-{version}.crate');assert hashlib.sha256(archive).hexdigest()==p['registry_checksum']
 upstream=get(f'https://codeload.github.com/{repo}/tar.gz/{rev}',f'{name}-{rev}.tar.gz')
 a=members(archive);full=members(upstream);prefix=vcs['path_in_vcs'].strip('/');prefix=prefix+'/' if prefix else '';b={n.removeprefix(prefix):v for n,v in full.items() if n.startswith(prefix)}
 records=[]
 for path,data in sorted(a.items()):
  if path=='.cargo_vcs_info.json':kind='cargo-vcs-metadata';other=None
  elif path=='Cargo.toml':kind='cargo-normalized-manifest';other=None
  else:
   source='Cargo.toml' if path=='Cargo.toml.orig' else path;other=b.get(source);kind='identical' if other==data else ('archive-only' if other is None else 'different')
   if other is not None and other!=data:
    try:diff=''.join(difflib.unified_diff(other.decode().splitlines(True),data.decode().splitlines(True),fromfile='upstream/'+source,tofile='archive/'+path))
    except UnicodeDecodeError:diff='Binary content differs\n'
    (OUT/(name+'-'+path.replace('/','_')+'.diff')).write_text(diff)
  records.append({'path':path,'kind':kind,'archive_sha256':hashlib.sha256(data).hexdigest(),'upstream_sha256':hashlib.sha256(other).hexdigest() if other is not None else None})
 result={'name':name,'version':version,'repository':repo,'revision':rev,'path_in_vcs':vcs['path_in_vcs'],'records':records,'upstream_only_files':sorted(set(b)-set(a)-{'Cargo.toml'}),'counts':{k:sum(x['kind']==k for x in records) for k in sorted({x['kind'] for x in records})}}
 results.append(result);print(name,result['counts'],flush=True)
(OUT/'SOURCE-COMPARISON.json').write_text(json.dumps(results,indent=2)+'\n');(OUT/'DOWNLOADS.json').write_text(json.dumps(receipts,indent=2)+'\n')
