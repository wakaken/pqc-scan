#!/usr/bin/env python3
"""Actual product CLI, independently implemented OpenSSH peer, no external runtime network."""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess
import tempfile
import threading
import urllib.request
ROOT=Path(__file__).resolve().parents[2]
STUDY=ROOT/'tests/feasibility/ssh-kex'
URL='https://cdn.openbsd.org/pub/OpenBSD/OpenSSH/portable/openssh-10.2p1.tar.gz'
SHA='ccc42c0419937959263fa1dbd16dafc18c56b984c03562d2937ce56a60f798b2'
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def inside(out):
    assert platform.system()=='Linux' and len(Path('/proc/net/route').read_text().splitlines())==1
    spec=importlib.util.spec_from_file_location('fixture_helpers',STUDY/'run-isolated.py');h=importlib.util.module_from_spec(spec);spec.loader.exec_module(h)
    binary=ROOT/'target/debug/pqc-scan'; binary_sha=sha(binary);out.mkdir(parents=True,exist_ok=False);records=[]
    with tempfile.TemporaryDirectory(prefix='pqc-kex-product-') as tmp:
        tmp=Path(tmp)
        for key in ['host','wrong']:
            subprocess.run([h.KEYGEN,'-q','-t','ed25519','-N','','-f',str(tmp/key)],check=True,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,timeout=10)
        for name,lang,trust,kex,tamper in [('success-ja','ja','host','mlkem768x25519-sha256',False),('success-en','en','host','mlkem768x25519-sha256',False),('wrong-pin','ja','wrong','mlkem768x25519-sha256',False),('tampered-signature','en','host','mlkem768x25519-sha256',True),('no-overlap','ja','host','curve25519-sha256',False)]:
            port=h.free_port();cfg=tmp/'sshd_config';cfg.write_text(f'Port {port}\nListenAddress 127.0.0.1\nHostKey {tmp}/host\nPidFile {tmp}/pid\nUsePAM no\nPasswordAuthentication no\nPubkeyAuthentication no\nKbdInteractiveAuthentication no\nUseDNS no\nKexAlgorithms {kex}\nLogLevel DEBUG3\n')
            server=subprocess.Popen([str(h.SSHD),'-D','-e','-ddd','-f',str(cfg)],stdout=subprocess.DEVNULL,stderr=subprocess.PIPE,start_new_session=True)
            received=[];sent=[]
            def consume():
                for line in server.stderr:
                    for direction,target in [(b'receive',received),(b'send',sent)]:
                        m=re.search(direction+rb' packet: type (\d+)',line)
                        if m:target.append(int(m[1]))
            reader=threading.Thread(target=consume);reader.start();proxy=None;proxy_facts={'server_plaintext_types':[],'signature_bytes_flipped':0}
            try:
                h.ready(server,port)
                if tamper:
                    import socket
                    listener=socket.socket();listener.bind(('127.0.0.1',0));listener.listen(1);listener.settimeout(6)
                    proxy=threading.Thread(target=h.proxy,args=(listener,port,proxy_facts));proxy.start();port=listener.getsockname()[1]
                cmd=[str(binary),'probe','ssh-kex','127.0.0.1','--port',str(port),'--allow-cidr','127.0.0.1/32','--host-key-file',str(tmp/(trust+'.pub')),'--lang',lang,'--out-dir',str(out/name)]
                result=subprocess.run(cmd,capture_output=True,text=True,timeout=8)
                record=json.loads((out/name/'probe.ssh-kex.v1.json').read_text())
                assert not result.stderr,'Unexpected stderr, not retained'
            finally:
                h.stop(server);reader.join(timeout=2);server.stderr.close()
                if proxy:proxy.join(timeout=7);assert not proxy.is_alive()
            assert not reader.is_alive() and 20 in received
            assert not any(t==5 or t>=50 for t in received),(name,received)
            good=name.startswith('success')
            assert result.returncode==(0 if good else 3),(name,record)
            assert record['details']['host_identity_verified']==good
            assert record['details']['handshake_complete']==good
            assert record['details']['user_authentication_attempted'] is False
            if good:
                assert record['outcome']=='completed' and record['details']['negotiated_kex']=='mlkem768x25519-sha256'
                assert 21 in received and 21 in sent and 1 in received
            else:
                assert record['details']['negotiated_kex'] is None and record['details']['host_signature'] is None
                assert record['diagnostics']==[{'wrong-pin':'host_key_untrusted','tampered-signature':'host_signature_invalid','no-overlap':'no_common_algorithm'}[name]],record
            assert record['target']['connected_peer']==f'127.0.0.1:{port}'
            assert 0<record['traffic']['sent_bytes']<=65536 and 0<record['traffic']['received_bytes']<=524288
            if tamper:assert proxy_facts['signature_bytes_flipped']==1 and proxy_facts['upload_thread_stopped']
            records.append({'case':name,'exit':result.returncode,'server_received_types':received,'server_sent_types':sent,'proxy':proxy_facts,'files':{p.name:sha(p) for p in (out/name).iterdir()},'cleanup':True})
    assert not tmp.exists() and sha(binary)==binary_sha
    (out/'RESULTS.json').write_text(json.dumps({'binary_sha256':binary_sha,'cases':records,'fixture_keys_removed':True,'binary_before_after_match':True,'sshd_version':subprocess.run([str(h.SSHD),'-V'],capture_output=True,text=True).stderr.strip()},indent=2)+'\n')
    print('Five actual product CLI cases passed with no authentication/service/channel requests.')
def main():
    parser=argparse.ArgumentParser();parser.add_argument('--out-dir',type=Path,required=True);parser.add_argument('--inside',action='store_true');args=parser.parse_args();out=args.out_dir.resolve()
    if args.inside:inside(out);return
    out.mkdir(parents=True,exist_ok=False)
    selected={}
    names=['Cargo.toml','Cargo.lock','rust-toolchain.toml','tests/protocol/ssh-kex-product.py','tests/feasibility/ssh-kex/run-isolated.py','tests/fixtures/ssh-kex-host.pub']
    for folder in ['crates','rules','schemas']:
        names.extend(str(p.relative_to(ROOT)) for p in (ROOT/folder).rglob('*') if p.is_file() and '__pycache__' not in p.parts)
    with tempfile.TemporaryDirectory(prefix='pqc-kex-product-build-') as temp:
        context=Path(temp)
        for name in sorted(names):
            src=ROOT/name
            if src.is_symlink() or any(x in ['target','.git'] or x.startswith('.env') for x in src.relative_to(ROOT).parts):raise RuntimeError('unsafe build input')
            dst=context/name;dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(src,dst);selected[name]=sha(src)
        data=urllib.request.urlopen(URL,timeout=30).read(4*1024*1024+1);assert hashlib.sha256(data).hexdigest()==SHA
        (context/'openssh-10.2p1.tar.gz').write_bytes(data)
        prefix=(STUDY/'Dockerfile').read_text().split('WORKDIR /work')[0]
        (context/'Dockerfile').write_text(prefix+'WORKDIR /work\nCOPY . .\nRUN cargo build -p pqc_scan_cli --features probe-ssh-kex --locked > /tmp/product-build.log 2>&1\nCMD ["python3","tests/protocol/ssh-kex-product.py","--inside","--out-dir","target/product-results"]\n')
        with (out/'build.log').open('w') as log:subprocess.run(['docker','build','--iidfile',str(out/'image-id'),str(context)],stdout=log,stderr=subprocess.STDOUT,check=True,timeout=900)
    image=(out/'image-id').read_text().strip();container=subprocess.check_output(['docker','create','--network','none','--security-opt','no-new-privileges','--cap-drop','NET_RAW','--memory','1g','--cpus','2','--pids-limit','128',image],text=True).strip()
    meta={'source_bindings':selected,'image':image,'openssh_sha256':SHA}
    try:
        inspect=json.loads(subprocess.check_output(['docker','inspect',container]))[0];host=inspect['HostConfig'];assert host['NetworkMode']=='none' and not host['Privileged'] and not host['Binds'];meta['isolation']={k:host[k] for k in ['NetworkMode','Privileged','Binds','SecurityOpt','CapDrop','Memory','NanoCpus','PidsLimit']}
        with (out/'run.log').open('w') as log:r=subprocess.run(['docker','start','-a',container],stdout=log,stderr=subprocess.STDOUT,timeout=90)
        meta['exit']=r.returncode
        subprocess.run(['docker','cp',container+':/work/target/product-results',str(out/'reports')],check=True)
        assert all(sha(ROOT/p)==v for p,v in selected.items()),'source changed during run'
        assert r.returncode==0,'product interop failed; inspect run.log'
    finally:
        subprocess.run(['docker','rm','-f',container],check=True,stdout=subprocess.DEVNULL);meta['container_removed']=True;(out/'DOCKER-RESULTS.json').write_text(json.dumps(meta,indent=2)+'\n')
    print((out/'run.log').read_text().strip())
if __name__=='__main__':main()
