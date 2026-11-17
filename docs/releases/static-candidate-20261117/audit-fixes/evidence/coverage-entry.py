import os,sys,time,json,traceback,gzip,shutil
from pathlib import Path
sys.path.insert(0,'/source/tests/protocol');sys.path.insert(0,'/source/tests/quality')
import container_entry as c
from task044 import entry as e
S=Path('/scratch');E=S/'evidence';E.mkdir();(S/'home').mkdir();(S/'tmp').mkdir();(S/'cargo').mkdir();(S/'test-target').mkdir()
for name in ['registry','git']:
 if (Path('/usr/local/cargo')/name).exists():(S/'cargo'/name).symlink_to(Path('/usr/local/cargo')/name)
original=e.run
choices=[{'reason':'New individual static outcome study: collect ordinary export/profile and attribute any complete default diagnostic vector. The historical all-feature SIGSEGV is not retried or claimed fixed.'}]
def run(args,name,**kwargs):
 if False and name.endswith('/diagnostic.txt') and '-dump' in args:
  choices.append({'notRetried':'LLVMdebugdump historicalSIGSEGV','usedInstead':'ordinarysummaryexport','originalDebugAttemptExecuted':False});args=[a for a in args if a!='-dump']
 return original(args,name,**kwargs)
e.run=run
status={'exit':0}
try:
 c.write('checks.json',c.boundary());e.smoke();e.measure('static-branches',nightly=True)
except BaseException as error:
 status={'exit':1,'error':str(error)};(E/'ERROR.log').write_text(traceback.format_exc())
finally:
 c.write('diagnostic-choice.json',choices)
 for profile in S.glob('*.profdata'):
  if not (E/'static-branches/coverage.profdata.gz').exists():
   with profile.open('rb') as src,gzip.open(E/('partial-'+profile.name+'.gz'),'wb') as dst:shutil.copyfileobj(src,dst)
 for path in list(E.rglob('*')):
  if path.is_file() and path.stat().st_size>2*1024**2 and path.suffix!='.gz':
   with path.open('rb') as src,gzip.open(str(path)+'.gz','wb') as dst:shutil.copyfileobj(src,dst)
   path.unlink()
 (S/'status.json').write_text(json.dumps(status))
while True:time.sleep(30)
