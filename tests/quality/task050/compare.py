"""Compare measured runs, retaining drift and >2x investigation signals."""
import argparse
import json
from pathlib import Path
import sys
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from evaluate import bytes_digest
from run import write_json
from task050.study import comparison, FORMATS, validate_study


def compare_runs(paths, out, historical):
    out.mkdir(parents=True,exist_ok=False)
    runs=[(p,json.loads((p/'resources.json').read_text()),json.loads((p/'samples.json').read_text())) for p in paths]
    for _,_,rows in runs:validate_study(rows)
    comparisons=[];semantic=[];ordering=[]
    for i,(ap,a,ar) in enumerate(runs):
        for bp,b,br in runs[i+1:]:
            for ag,bg in zip(a['groups'],b['groups']):
                comparisons.append(dict(before=str(ap),after=str(bp),scale=ag['scale'],configuration=ag['configuration'],**comparison(ag,bg)))
            for scale in ['small','medium','large']:
                x=next(r for r in ar if r['scale']==scale and r['threads']==1 and r['index']==0)
                y=next(r for r in br if r['scale']==scale and r['threads']==1 and r['index']==0)
                semantic.append(dict(before=str(ap),after=str(bp),scale=scale,formats={n:x['outputs'][n]['semantic']==y['outputs'][n]['semantic'] for n in FORMATS},facts={key:x['facts'][key]==y['facts'][key] for key in x['facts']},note='Full ID-inclusive equality; run-scoped identities remain visible; false is measured drift, not evaluation failure'))
                ordering.append(dict(before=str(ap),after=str(bp),scale=scale,formats={n:x['outputs'][n]['order']==y['outputs'][n]['order'] for n in FORMATS}))
    past=json.loads(historical.read_text());oldenv=past['environment']
    old=dict(environment=oldenv,input=past['benchmarks']['small']['input'],configuration={'threads':1,'format':'all'},warmMax={key:past['benchmarks']['small']['warmDistribution'][key]['max'] for key in ['wallSeconds','peakRssBytes']})
    history=[]
    for p,r,rows in runs:
        history.append(dict(before=str(historical),beforeSha256=bytes_digest(historical),beforeDate=oldenv.get('clockUtc'),after=str(p),afterDate=r['environment']['clockUtc'],**comparison(old,r['groups'][0])))
    write_json(out/'environment-comparison.json',dict(runs=[dict(path=str(p),environmentSha256=bytes_digest(p/'environment.json'),environment=r['environment']) for p,r,_ in runs],comparisons=comparisons,historicalDayComparison=history,policy='Performance comparisons require matching host/OS/arch/toolchain/profile/features/source/constraints/input/config. Binary hashes recorded, environment changes never called product regressions. Static workload does not exercise TLS provider.'))
    write_json(out/'semantic-reproducibility.json',dict(crossRun=semantic,perRun=[dict(path=str(p),sha256=bytes_digest(p/'semantic-reproducibility.json')) for p,_,_ in runs],normalization='Per-run records define exact allowed volatile fields; never drop fact IDs, duplicates, status, rules, positions or versions. Run-scoped ID drift is preserved, not a stable identity contract.'))
    write_json(out/'ordering.json',dict(crossRun=ordering,perRun=[dict(path=str(p),sha256=bytes_digest(p/'ordering.json')) for p,_,_ in runs],note='Raw byte hashes, generated-time-normalized order and conservative semantic hashes are separate. HTML/Markdown no unordered equivalence claim.'))
    for name in ['resources','throughput','scaling','failure-resources']:
        write_json(out/(name+'.json'),dict(runs=[dict(path=str(p),sha256=bytes_digest(p/(name+'.json')),data=json.loads((p/(name+'.json')).read_text())) for p,_,_ in runs]))
    signals=[c for c in comparisons if c['investigate']]
    (out/'noise-study.md').write_text('# Noise study\n\n'+str(len(runs))+' studies, each 24 successful scans: 3 scales x 2 thread counts x (1 first-measured + 3 warm). Raw samples and warm min/median/max are retained. First-measured is not true cold: no cache flush, input materialization already touches data. No privileged cache mutation. Linux quota vs native macOS is an environment comparison, not product speed ranking.\n\nHistorical different-day samples are retained with dates, hashes and explicit comparison rejection reasons in environment-comparison.json. There are no controlled same-condition multi-day samples, so no isolated daily-noise estimate or cold-cache performance claim. Existing historical input/config/source/environment differences prevent a retrospective regression verdict.\n')
    (out/'regression-review.md').write_text('# Regression investigation\n\nExisting TASK-023-ACCEPTANCE.md conditions are unchanged: fixed eligible truth TP cannot decrease; FP/FN cannot increase. Same-condition warm maximum time/RSS >2x previous warm maximum is an investigation signal, not automatic CI failure. No threshold was revised. Any revision requires measured variation and maintainer approval.\n\nComparable >2x signals: '+json.dumps(signals,ensure_ascii=False,indent=2)+'\n\nIf any signal exists, retain it and examine raw sample distribution, host load, launcher bounds, source/binary/config and repeated controlled measurements before calling it product regression. Historical 2026-11-11 large-workload 2x finding and safety-I/O investigation remain in FINAL-GAPS-VALIDATION.md; they are not overwritten by this new matrix. Accuracy nonregression is checked using the unchanged 023-v2 truth in separate accuracy runs; the new product-source workload has no accuracy truth and is not used for TP/FP/FN.\n')
    return signals


def main():
    p=argparse.ArgumentParser();p.add_argument('--runs',nargs='+',type=Path,required=True);p.add_argument('--out',type=Path,required=True);p.add_argument('--historical',type=Path,default=Path('docs/quality/task023/BASELINE.json'));a=p.parse_args()
    compare_runs(a.runs,a.out,a.historical)


if __name__=='__main__':main()
