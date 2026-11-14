"""Safety matrix observations on top of the shared verification entry."""
import hashlib
import json
from pathlib import Path
import signal
import socket
import subprocess
import sys
sys.path[:0]=['/source/tests/protocol','/source/tests/quality']
from container_entry import EVIDENCE,SCRATCH,write
from inspection055_entry import main as verify
from bounded_process import execute


def main():
    verify()
    source='''fn main(){let m=std::env::args().nth(1).unwrap();std::panic::set_hook(Box::new(|_|{}));match m.as_str(){"panic"=>{if std::panic::catch_unwind(||panic!("synthetic control")).is_err(){std::process::exit(42)}},"timeout"=>{println!("retained");std::thread::sleep(std::time::Duration::from_secs(60));},"error"=>std::process::exit(1),"signal"=>std::process::abort(),"cpu"=>loop{std::hint::black_box(1u64);},_=>println!("retained")}}'''
    path=EVIDENCE/'control057.rs';path.write_text(source)
    binary=SCRATCH/'control057'
    subprocess.run(['rustc',str(path),'-o',str(binary)],check=True,timeout=30)
    controls=[]
    for label,expected in [('success','completed'),('panic','panic'),('error','error'),('signal','signal'),('timeout','timeout'),('cpu','signal')]:
        result=execute([str(binary),label],EVIDENCE/('control-'+label),timeout=.2 if label=='timeout' else 4,cpu_seconds=1)
        assert result['status']==expected,(label,result)
        if label=='cpu':assert result['returncode'] in [-signal.SIGXCPU,-signal.SIGKILL],result
        if label=='signal':assert result['returncode']==-signal.SIGABRT,result
        if label=='timeout':assert 'retained' in (EVIDENCE/'control-timeout/stdout.log').read_text()
        controls.append(dict(result,label=label,expected=expected))
    write('failure-controls057.json',{'cases':controls,'binarySha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'sourceSha256':hashlib.sha256(path.read_bytes()).hexdigest(),'scope':'synthetic supervisor controls, not product failures'})
    root=SCRATCH/'static057';root.mkdir();out=SCRATCH/'static057-output'
    with socket.socket() as listener:
        listener.bind(('127.0.0.1',0));listener.listen();listener.settimeout(.2)
        (root/'package.json').write_text(json.dumps({'dependencies':{'openssl':f'http://127.0.0.1:{listener.getsockname()[1]}/must-not-fetch'}}))
        (root/'app.py').write_text("password='CANARY057_STATIC'\nx='RS256'\n")
        command=['/scratch/target/debug/pqc-scan','scan',str(root),'--rules-dir','/source/rules','--out-dir',str(out),'--format','all','--diagnostics','--inventory-json','--cyclonedx-17']
        observed=execute(command,EVIDENCE/'static-scan',timeout=30,cpu_seconds=20)
        assert observed['status']=='completed',observed
        try:listener.accept()
        except TimeoutError:connected=False
        else:connected=True
        assert not connected
    exported={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in out.iterdir() if p.is_file()}
    assert len(exported)>=8
    assert all(b'CANARY057_STATIC' not in p.read_bytes() for p in out.iterdir() if p.is_file())
    assert all(b'CANARY057_STATIC' not in (EVIDENCE/'static-scan'/name).read_bytes() for name in ['stdout.log','stderr.log'])
    write('static-network057.json',{'command':command,'result':observed,'configuredLoopbackReceivedConnections':connected,'externalNetwork':'none namespace, checked by boundary()','reportHashes':exported,'canaryAbsent':True})


if __name__=='__main__':main()
