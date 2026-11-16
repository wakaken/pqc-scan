from pathlib import Path
import json,urllib.request,tarfile,io,hashlib,datetime,re
p=Path('target/quality-task-063/upstream/objc');p.mkdir()
m=json.load(open('third_party/notices/manifest.json'));records=[]
for x in m['packages']:
 if x['name'] not in ['block2','dispatch2','objc2','objc2-encode']:continue
 name=x['name'];rev=x['upstream_revision'];v=x['version'];urls=[f'https://static.crates.io/crates/{name}/{name}-{v}.crate',x['files'][0]['upstream_url']];downloads=[]
 for i,url in enumerate(urls):
  with urllib.request.urlopen(urllib.request.Request(url,headers={'User-Agent':'pqc-scan'}),timeout=60) as r:b=r.read(16*1024**2+1)
  assert len(b)<=16*1024**2
  sha=hashlib.sha256(b).hexdigest();assert sha==(x['registry_checksum'] if i==0 else x['files'][0]['sha256']);(p/(name+('-'+v+'.crate' if i==0 else '-LICENSE.md'))).write_bytes(b);downloads.append({'url':url,'sha256':sha,'retrieved_at':datetime.datetime.now(datetime.timezone.utc).isoformat()})
 archive=(p/(name+'-'+v+'.crate')).read_bytes();copyrights=[];licensefiles=[];total=0
 with tarfile.open(fileobj=io.BytesIO(archive),mode='r:gz') as t:
  for item in t:
   if not item.isfile():continue
   total+=item.size;assert total<64*1024**2 and item.size<8*1024**2
   data=t.extractfile(item).read();base=Path(item.name).name
   if re.fullmatch(r'(LICENSE|LICENCE|NOTICE|COPYING|COPYRIGHT)([._-].*)?',base,re.I):licensefiles.append(item.name)
   try:text=data.decode()
   except UnicodeDecodeError:continue
   for i,line in enumerate(text.splitlines(),1):
    if re.search('copyright',line,re.I):copyrights.append({'path':item.name,'line':i,'text':line})
 records.append({'name':name,'version':v,'revision':rev,'downloads':downloads,'archive_notice_files':licensefiles,'copyright_occurrences':copyrights,'review_required':True,'reason':'Exact upstream statement does not include complete package-specific permission/attribution texts and explicitly leaves Apple SDK-derived-work questions unresolved. No clearance inferred.'});print(name,'notice files',len(licensefiles),'copyright occurrences',len(copyrights),flush=True)
(p/'NOTICE-INVESTIGATION.json').write_text(json.dumps(records,indent=2)+'\n')
