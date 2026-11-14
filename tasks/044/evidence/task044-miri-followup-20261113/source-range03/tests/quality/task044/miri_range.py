"""Original-workspace Miri matrix with bounded tests and explicit non-results."""
import ctypes
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import time
import traceback

sys.path.insert(0, '/source/tests/protocol')
sys.path.insert(0, '/source/tests/quality')
from container_entry import boundary, EVIDENCE, SCRATCH, write
from task044.miri_support import NIGHTLY, miri_command, parse_test_events, classify_failure

COMMANDS = []


def execute(args, log, env, *, known=(), wall=600, idle=120, stdout_json=False, cwd='/source'):
    started = time.monotonic()
    path = EVIDENCE/log
    status = None
    reaped = 0
    with path.open('wb') as stream, (EVIDENCE/(log+'.stderr')).open('wb') as errors:
        process = subprocess.Popen(args, cwd=cwd, env=env, stdout=stream,
                                   stderr=errors if stdout_json else subprocess.STDOUT,
                                   start_new_session=True)
        previous = None
        progress = started
        try:
            while process.poll() is None:
                current = path.read_text(errors='replace') if known else ''
                events = parse_test_events(current, known) if known else {}
                marker = json.dumps(events, sort_keys=True)
                if marker != previous:
                    previous = marker
                    progress = time.monotonic()
                if time.monotonic()-started > wall or time.monotonic()-progress > idle:
                    status = 'timeout'
                    break
                time.sleep(.1)
            if status is None:
                status = process.returncode
        finally:
            try:
                os.killpg(process.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            process.wait()
            deadline = time.monotonic()+5
            while True:
                try:
                    pid, _ = os.waitpid(-process.pid, os.WNOHANG)
                except ChildProcessError:
                    break
                if pid:
                    reaped += 1
                elif time.monotonic() > deadline:
                    raise RuntimeError('test descendants did not exit')
                else:
                    time.sleep(.01)
    record = {'command':args,'cwd':cwd,'environment':env,'exit':status,'log':log,
              'wallSeconds':time.monotonic()-started,'wallLimit':wall,'idleLimit':idle,
              'descendantsReaped':reaped}
    COMMANDS.append(record)
    write('commands.json', COMMANDS)
    return status


def startup():
    write('checks.json', boundary())
    # Keep orphaned interpreter children reapable on both ordinary and timeout
    # exits, rather than exhausting the container's finite PID budget.
    if ctypes.CDLL(None).prctl(36, 1, 0, 0, 0) != 0:
        raise RuntimeError('Linux child subreaper unavailable')
    (SCRATCH/'test-target').mkdir()
    cargo = SCRATCH/'cargo'; cargo.mkdir()
    for name in ('registry','git'):
        source=Path('/usr/local/cargo')/name
        if source.exists(): (cargo/name).symlink_to(source,target_is_directory=True)
    return dict(os.environ, MIRI_SYSROOT='/opt/miri-sysroot',RUSTUP_TOOLCHAIN=NIGHTLY,
                CARGO_TARGET_DIR='/scratch/miri-workspace')


def controls(env):
    code='import subprocess,time; subprocess.Popen(["/bin/sleep","30"]); time.sleep(30)'
    result=execute([sys.executable,'-c',code],'timeout-control.log',env,wall=.3,idle=.3)
    assert result == 'timeout' and COMMANDS[-1]['descendantsReaped'] >= 1
    folder=SCRATCH/'build-regression'; (folder/'src').mkdir(parents=True)
    (folder/'Cargo.toml').write_text('[package]\nname="build-regression"\nversion="0.0.0"\nedition="2021"\n')
    (folder/'build.rs').write_text('fn main(){println!("cargo:rerun-if-changed=build.rs");}\n')
    positive='#[test] fn check(){assert_eq!(2+3,5);}\n'
    (folder/'src/lib.rs').write_text(positive)
    manifest=['--manifest-path',str(folder/'Cargo.toml'),'--offline']
    broken_env=dict(env,CARGO_TARGET_DIR='/scratch/control-broken')
    broken=execute(['cargo','+'+NIGHTLY,'miri','test',*manifest],'broken-control.log',broken_env)
    assert broken == 101 and 'invalid JSON' in (EVIDENCE/'broken-control.log').read_text()
    fixed_env=dict(env,CARGO_TARGET_DIR='/scratch/control-fixed')
    fixed=execute(miri_command('test',*manifest,'--','--format=json','-Zunstable-options'),
                  'fixed-control.log',fixed_env,known=['check'])
    events=parse_test_events((EVIDENCE/'fixed-control.log').read_text(),['check'])
    assert fixed==0 and events['passed']==['check']
    # phase_runner removes MIRI_BE_RUSTC before launching the interpreter,
    # then restores build-time env only when it differs from the caller.
    # Match Cargo's normal runner env; otherwise the stored host value gets
    # reintroduced and the interpreter acts as rustc. Validate both outcomes.
    direct_env=dict(fixed_env,MIRI_BE_RUSTC='host',MIRI_VERBOSE='1')
    sysroot=subprocess.check_output(['rustc','+'+NIGHTLY,'--print','sysroot'],env=env,text=True).strip()
    runner=str(Path(sysroot)/'bin/cargo-miri')
    descriptor_args=miri_command('test',*manifest,'--no-run','--message-format=json')
    code=execute(descriptor_args,'control-descriptor.jsonl',fixed_env,stdout_json=True)
    assert code==0
    descriptor=next(json.loads(line)['executable'] for line in (EVIDENCE/'control-descriptor.jsonl').read_text().splitlines()
                    if line.startswith('{') and json.loads(line).get('reason')=='compiler-artifact'
                    and json.loads(line).get('profile',{}).get('test') and json.loads(line).get('executable'))
    direct=execute([runner,'runner',descriptor,'--format=json','-Zunstable-options'],
                   'direct-positive.log',direct_env,known=['check'],cwd=str(folder))
    assert direct==0 and parse_test_events((EVIDENCE/'direct-positive.log').read_text(),['check'])['passed']==['check']
    assert re.search(r'MIRI_CWD=[\"\']?'+re.escape(str(folder)), (EVIDENCE/'direct-positive.log').read_text())
    negative='#[test] fn check(){unsafe{let _=std::ptr::read_volatile(std::ptr::null::<u8>());}}\n'
    (folder/'src/lib.rs').write_text(negative)
    bad=execute(miri_command('test',*manifest,'--','--format=json','-Zunstable-options'),
                'ub-control.log',fixed_env,known=['check'])
    assert bad!=0 and classify_failure(bad,(EVIDENCE/'ub-control.log').read_text())=='Failed: UB'
    direct_bad=execute([runner,'runner',descriptor,'--format=json','-Zunstable-options'],
                       'direct-ub.log',direct_env,known=['check'],cwd=str(folder))
    assert direct_bad!=0 and classify_failure(direct_bad,(EVIDENCE/'direct-ub.log').read_text())=='Failed: UB'

    write('controls.json',{'broken':broken,'fixed':fixed,'undefinedBehavior':bad,
                           'positiveSource':positive,'negativeSource':negative,'directPositive':direct,'directUB':direct_bad,
                           'timeoutDescendantsReaped':COMMANDS[0]['descendantsReaped']})


def main():
    env=startup()
    controls(env)
    command=miri_command('test','--workspace','--all-features','--no-run',
                         '--locked','--offline','--message-format=json')
    status=execute(command,'workspace-build.jsonl',env,wall=700,idle=700,stdout_json=True)
    if status!=0:
        raise RuntimeError('original workspace Miri build failed')
    artifacts=[]
    for line in (EVIDENCE/'workspace-build.jsonl').read_text().splitlines():
        if not line.startswith('{'): continue
        item=json.loads(line)
        if item.get('reason')=='compiler-artifact' and item.get('profile',{}).get('test') and item.get('executable'):
            path=Path(item['executable'])
            artifacts.append({'package':item['package_id'],'target':item['target'],
                              'workingDirectory':str(Path(item['manifest_path']).parent),
                              'executable':str(path),'sha256':hashlib.sha256(path.read_bytes()).hexdigest()})
    assert artifacts
    artifacts.sort(key=lambda a:(a['target']['kind']==['test'],a['package'],a['target']['name']))
    sysroot=subprocess.check_output(['rustc','+'+NIGHTLY,'--print','sysroot'],env=env,text=True).strip()
    runner=str(Path(sysroot)/'bin/cargo-miri')
    runner_env=dict(env,MIRI_BE_RUSTC='host')
    inventory=[]
    for i, artifact in enumerate(artifacts):
        result=execute([runner,'runner',artifact['executable'],'--list','--format=terse'],
                       f'list-{i:02}.log',runner_env,wall=90,idle=90,cwd=artifact['workingDirectory'])
        text=(EVIDENCE/f'list-{i:02}.log').read_text()
        names=re.findall(r'^(.+): test$',text,re.MULTILINE) if result==0 else []
        inventory.append({**artifact,'id':i,'listingExit':result,'tests':names})
    write('inventory.json',inventory)
    assert any(x['tests'] for x in inventory), 'no original test could be listed'
    results=[]
    deadline=time.monotonic()+1600
    for item in inventory:
        remaining=list(item['tests'])
        batch=0
        while remaining and time.monotonic()<deadline:
            log=f'target-{item["id"]:02}-batch-{batch:03}.log'
            args=[runner,'runner',item['executable'],'--exact',*remaining,
                  '--format=json','-Zunstable-options','--test-threads=1']
            result=execute(args,log,runner_env,known=remaining,cwd=item['workingDirectory'],wall=min(600,max(1,deadline-time.monotonic())),idle=60)
            text=(EVIDENCE/log).read_text(errors='replace')
            parsed=parse_test_events(text,remaining)
            removed=[]
            for key,status in [('passed','Passed'),('failed','Failed: assertion'),('ignored','Not Tested: ignored')]:
                for name in parsed[key]:
                    results.append({'targetId':item['id'],'test':name,'status':status,'log':log,'exit':result})
                    removed.append(name)
            if parsed['active']:
                for name in parsed['active']:
                    results.append({'targetId':item['id'],'test':name,'status':classify_failure(result,text),'log':log,'exit':result})
                    removed.append(name)
            if not removed:
                for name in remaining:
                    results.append({'targetId':item['id'],'test':name,'status':'Unknown: no attributable test event','log':log,'exit':result})
                remaining=[]
            else:
                remaining=[name for name in remaining if name not in removed]
            write('results.json',results)
            batch+=1
        for name in remaining:
            results.append({'targetId':item['id'],'test':name,'status':'Not Tested: study deadline','reason':'finite measured execution budget'})
    write('results.json',results)
    counts={status:sum(r['status']==status for r in results) for status in sorted({r['status'] for r in results})}
    assert sum(len(x['tests']) for x in inventory)==len(results)
    for item in inventory:
        assert hashlib.sha256(Path(item['executable']).read_bytes()).hexdigest()==item['sha256']
    write('summary.json',{'featureConfiguration':'workspace --all-features','counts':counts,
          'registeredTests':len(results),'targets':len(inventory),
          'listingFailures':[x['id'] for x in inventory if x['listingExit']!=0],
          'executableDescriptorsUnchanged':True,'doctests':'Not Tested: no-run inventory excludes workspace doctests'})


if __name__ == '__main__':
    try:
        main()
    except BaseException:
        traceback.print_exc()
        raise
