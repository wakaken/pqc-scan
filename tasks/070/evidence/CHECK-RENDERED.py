"""Check final preview data/escaping against its actual product JSON, not an oracle."""
import json,sys
from pathlib import Path
from html.parser import HTMLParser
class Read(HTMLParser):
 def __init__(self):
  super().__init__(convert_charrefs=True);self.pre=False;self.code=False;self.blocks=[];self.inline=[];self.parts=[];self.external=[];self.executable=[]
 def handle_starttag(self,tag,attrs):
  if tag=='pre':self.pre=True
  if tag=='code':self.code=True;self.parts=[]
  if tag in ['script','img','iframe']:self.executable.append(tag)
  if tag=='a' and 'href' in dict(attrs):
   href=dict(attrs).get('href','')
   if not href.startswith('#'):self.external.append(href)
 def handle_endtag(self,tag):
  if tag=='code':
   (self.blocks if self.pre else self.inline).append(''.join(self.parts));self.code=False
  if tag=='pre':self.pre=False
 def handle_data(self,text):
  if self.code:self.parts.append(text)
root=Path(sys.argv[1]);checks=[]
for html in sorted(root.glob('*/*/preview.html')):
 value=json.loads((html.parent/'migration.assessment.v1.json').read_text());p=Read();p.feed(html.read_text());assert not p.executable and not p.external,(html,p.executable,p.external)
 decoder=json.JSONDecoder();records=[]
 for block in p.blocks:
  while block.strip():
   block=block.lstrip();v,n=decoder.raw_decode(block);records.append(v);block=block[n:]
 for name in ['targets','claims','candidates','sources']:
  for record in value[name]:assert record in records,(html,name,record)
 for name in ['input','context','catalog','policy','model','coverage']:assert value[name] in records,(html,name)
 for target in value['targets']:
  path=target['source_ref'].get('path');line=target['source_ref'].get('line')
  if path:
   location=f'{path}:{line}' if line is not None else path
   assert location in p.inline,(html,location)
 assert 'SECRET_CANARY_070' not in html.read_text()
 checks.append({'preview':str(html.relative_to(root)),'records':len(records),'literal_locations':True,'scripts_images_external_links':0})
for folder in root.iterdir():
 if folder.is_dir():assert (folder/'en/migration.assessment.v1.json').read_bytes()==(folder/'ja/migration.assessment.v1.json').read_bytes()
print(json.dumps({'exit':0,'previews':len(checks),'checks':checks},indent=2))
