"""Additional input-only campaign for nonempty inventory assets and references."""
import copy,hashlib,json,sys
from pathlib import Path
sys.path[:0]=['/source/tests/quality','/source/tests/quality/task047']
from study import run_target
root=Path('/scratch/evidence');fixture=Path('/source/crates/pqc_scan_cli/tests/fixtures/native-v1.golden.json')
raw=fixture.read_bytes();native=json.loads(raw)
# Golden output substitutes time/id; restore a valid timestamp without altering facts/refs.
native['run'].update(generated_at='2026-11-13T00:00:00Z',id='scan-rich047')
broken=copy.deepcopy(native);broken['unassociated_observation_ids']=['missing047']
future=copy.deepcopy(native);future['native_completeness']='future047'
seeds=[{'name':name,'hex':json.dumps(data).encode().hex(),'origin':'native-v1.golden.json: '+hashlib.sha256(raw).hexdigest()+'; run time/id normalized; '+name} for name,data in [('populated.json',native),('broken-reference.json',broken),('future-vocabulary.json',future)]]
binary=json.loads((root/'worker-binaries.json').read_text())['binaries']['pqc_scan_report']['path'];command=[binary,'--ignored','--exact','quality047_worker','--nocapture']
results=[]
for mode,seconds,cap in [('short',30,256),('long',120,2048)]:
 result=run_target('output-rich',command,seeds,root/'fuzz'/(mode+'-output-rich'),Path('/scratch/supplemental-work'),seconds=seconds,seed=4710000+(mode=='long'),max_cases=cap)
 results.append(dict(result,mode=mode));(root/'supplemental-results.json').write_text(json.dumps({'results':results},indent=2)+'\n')
 assert not result['failures'], 'failure evidence retained'
