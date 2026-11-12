import pathlib,subprocess,time,json,hashlib
r=pathlib.Path('target/quality-task-042-reopen-20261112');records=[]
def run(args,log):
 t=time.monotonic()
 with (r/log).open('x') as f:p=subprocess.run(args,stdout=f,stderr=subprocess.STDOUT,timeout=180)
 records.append(dict(command=args,exitCode=p.returncode,log=str(r/log),wallSeconds=time.monotonic()-t));(r/'CHECK-COMMANDS.json').write_text(json.dumps(records,indent=2)+'\n');assert p.returncode==0,(args,p.returncode);print(log,0,flush=True)
run(['target/release/pqc-scan','scan','.','--format','sarif','--rules-dir','./rules','--out-dir',str(r/'self-scan-final')],'self-scan-final.log')
# Rebind only the previously reviewed, identical report.py benchmark findings.
p=pathlib.Path('docs/quality/SELF-SCAN-EXPECTATIONS.json');s=p.read_text();policy=json.loads(s);entry=next(x for x in policy['files'] if x['path']=='tests/quality/report.py');old=entry['sha256'];new=hashlib.sha256(pathlib.Path('tests/quality/report.py').read_bytes()).hexdigest();assert s.count(old)==1;s=s.replace(old,new);p.write_text(s)
run(['node','shell/prepare-code-scanning.cjs','--input',str(r/'self-scan-final/report.sarif'),'--expectations',str(p),'--root','.','--output',str(r/'self-scan-final/code-scanning.sarif'),'--audit',str(r/'self-scan-final/code-scanning-audit.json')],'prepare-self-scan-final.log')
run(['python3','-m','unittest','discover','-s','tests/quality','-p','test_*.py'],'python-final.log')
run(['node','--test',*map(str,sorted(pathlib.Path('shell').glob('*.test.cjs')))],'node-verified.log')
run(['ruby','-r','yaml','-e','YAML.load_file(ARGV.fetch(0)); puts "Workflow YAML parsed"','.github/workflows/pqc-scan.yml'],'workflow-final.log')
run(['git','diff','--check'],'diff-check.log')
for name,q,ref in [('legacy','tests/quality','target/quality-task-042/legacy-regression-02/summary.json'),('v1','tests/quality/expanded','target/quality-task-042/expanded-accepted/summary.json'),('v2','tests/quality/expanded-v2','target/quality-task-042/expanded-accepted/summary.json'),('lock-controls','tests/quality/expanded-v2-lock-controls',None)]:
 args=['python3','tests/quality/report.py','--corpus-dir',q,'--baseline',str(r/name),'--out-dir',str(r/(name+'-report-complete'))]
 if ref:args+=['--reference',ref]
 if name=='v2':args+=['--applicability','tests/quality/applicability-042-v2/assessment.json']
 run(args,name+'-report-complete.log')
