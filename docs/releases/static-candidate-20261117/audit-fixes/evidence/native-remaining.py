from pathlib import Path
import subprocess,json,time,hashlib,os
p=Path('/tmp/pqc-audit-fixes-20261117-9k0dmc45');env=dict(os.environ,CARGO_TARGET_DIR='/private/var/folders/vp/sb96fq311gl__cjkxvhswf040000gp/T/codex-security-artifacts-6d72bd922a6ce4329465e616926da9db3d3d28a5166ba1993548931f54ff24cf/native-target',PQC048_EVIDENCE=str(p/'exports-final'))
cmds=[['cargo','fmt','--all','--check'],['cargo','build','--workspace','--locked','--offline'],['cargo','test','--workspace','--locked','--offline'],['cargo','clippy','--workspace','--all-targets','--locked','--offline','--','-D','warnings'],['cargo','build','--workspace','--all-features','--locked','--offline'],['cargo','test','--workspace','--all-features','--locked','--offline'],['cargo','clippy','--workspace','--all-targets','--all-features','--locked','--offline','--','-D','warnings'],['cargo','build','--release','--locked','--offline','-p','pqc_scan_cli']]
records=json.loads((p/'NATIVE-COMMANDS.json').read_text())
for i,cmd in list(enumerate(cmds))[4:]:
 start=time.monotonic();log=p/('native-'+str(i)+'.log')
 with log.open('w') as f:r=subprocess.run(cmd,env=env,stdout=f,stderr=subprocess.STDOUT,timeout=1200)
 records.append(dict(argv=cmd,exit=r.returncode,seconds=time.monotonic()-start,log=str(log),sha256=hashlib.sha256(log.read_bytes()).hexdigest()));(p/'NATIVE-COMMANDS.json').write_text(json.dumps(records,indent=2)+'\n');print(i,r.returncode,flush=True)
 if r.returncode:raise SystemExit(r.returncode)
import shutil
shutil.copyfile(Path(env['CARGO_TARGET_DIR'])/'release/pqc-scan',p/'pqc-scan-static');(p/'pqc-scan-static').chmod(0o755)
