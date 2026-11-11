#!/usr/bin/env python3
"""Produce synthetic TLS/SSH success and denial reports for real-browser acceptance.
Invoke through run-isolated.py --previews inside the bounded offline container.
Private test keys exist only in an OS temporary directory, never in the outputs.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import socket
import struct
import subprocess
import sys
import tempfile
import threading
import time


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def exact(stream, count):
    data = bytearray()
    while len(data) < count:
        chunk = stream.recv(count-len(data))
        if not chunk:
            raise RuntimeError('fixture peer closed before expected frame')
        data.extend(chunk)
    return bytes(data)


def ssh_peer(listener, record):
    try:
        listener.settimeout(12)
        connection, _ = listener.accept()
        with connection:
            connection.settimeout(8)
            connection.sendall(b'SSH-2.0-pqc_preview synthetic fixture only\r\n')
            banner = bytearray()
            while not banner.endswith(b'\n') and len(banner) < 256:
                banner.extend(exact(connection,1))
            if not banner.startswith(b'SSH-2.0-') or not banner.endswith(b'\r\n'):
                raise RuntimeError('unexpected diagnostic identification')
            length, = struct.unpack('>I', exact(connection,4))
            if not 12 <= length <= 256*1024:
                raise RuntimeError('unexpected client frame size')
            packet = exact(connection,length)
            padding = packet[0]
            if padding < 4 or padding+2 > length or packet[1] != 20:
                raise RuntimeError('expected one client KEXINIT')
            record['client_packet_types'] = [packet[1]]
            lists = [
                'sntrup761x25519-sha512@openssh.com,curve25519-sha256,kex-strict-s-v00@openssh.com,mlkem768x25519-sha256,unknown-preview-kex',
                'ssh-ed25519,rsa-sha2-512', 'aes128-ctr', 'aes128-ctr',
                'hmac-sha2-256', 'hmac-sha2-256', 'none', 'none', '', '',
            ]
            payload = b'\x14'+bytes(16)+b''.join(struct.pack('>I',len(x))+x.encode() for x in lists)+b'\0'+bytes(4)
            pad = 8-((len(payload)+5)%8)
            if pad < 4:
                pad += 8
            connection.sendall(struct.pack('>IB',len(payload)+pad+1,pad)+payload+bytes(pad))
            if connection.recv(1):
                raise RuntimeError('unexpected message after remote advertisements')
            record['no_post_kexinit_bytes'] = True
    except Exception as error:
        record['error'] = type(error).__name__
    finally:
        listener.close()


def openssl(*args):
    result = subprocess.run(['openssl',*map(str,args)],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,timeout=20)
    if result.returncode:
        raise RuntimeError('OpenSSL fixture setup failed; no key/error contents retained')


def stop(process):
    if process.poll() is None:
        os.killpg(process.pid,signal.SIGTERM)
        try:
            process.wait(timeout=3)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid,signal.SIGKILL)
            process.wait(timeout=3)


def run_cli(binary, out, args, expected):
    command = [str(binary),'probe',*args,'--out-dir',str(out)]
    result = subprocess.run(command,capture_output=True,text=True,timeout=20)
    if result.returncode != expected:
        raise RuntimeError(f'probe fixture expected exit {expected}, received {result.returncode}')
    return {'command':command,'exit':result.returncode,'files':{p.name:sha(p) for p in out.iterdir()}}


def linux_listener_ready(pid, port, proc=Path('/proc')):
    try:
        descriptors = list((proc/str(pid)/'fd').iterdir())
    except FileNotFoundError:
        return False
    owned = set()
    for descriptor in descriptors:
        try:
            owned.add(os.readlink(descriptor))
        except FileNotFoundError:
            # OpenSSL may close a setup/config descriptor between listing and
            # readlink. Only a currently owned listening socket establishes readiness.
            continue
    rows = (proc/'net/tcp').read_text().splitlines()[1:]
    return any(int(row.split()[1].split(':')[1],16)==port and row.split()[3]=='0A'
               and f'socket:[{row.split()[9]}]' in owned for row in rows)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary',type=Path,required=True)
    parser.add_argument('--out-dir',type=Path,required=True)
    args = parser.parse_args()
    if sys.platform not in {'darwin','linux'}:
        raise RuntimeError('unsupported fixture platform')
    binary = args.binary.resolve(strict=True)
    args.out_dir.mkdir(parents=True,exist_ok=False)
    result = {'scope':'Synthetic local protocol observations for browser acceptance, not public endpoints',
              'isolation':'Use the accompanying HOST-RESULTS.json for enforced container limits; this fixture does not establish its own sandbox',
              'binary_sha256':sha(binary),'openssl':subprocess.check_output(['openssl','version'],text=True).strip(),'runs':[]}
    with tempfile.TemporaryDirectory(prefix='pqc-preview-pki-') as temporary:
        tmp = Path(temporary)
        config = tmp/'cert.cnf'
        config.write_text('[req]\ndistinguished_name=dn\n[dn]\n[root]\nbasicConstraints=critical,CA:TRUE\nkeyUsage=critical,keyCertSign,cRLSign\n[server]\nbasicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature,keyEncipherment\nextendedKeyUsage=serverAuth\nsubjectAltName=DNS:localhost,IP:127.0.0.1\n')
        openssl('req','-x509','-newkey','rsa:2048','-nodes','-days','1','-subj','/CN=PQC Preview Root','-config',config,'-extensions','root','-keyout',tmp/'root.key','-out',tmp/'ca.pem')
        openssl('req','-new','-newkey','rsa:2048','-nodes','-subj','/CN=localhost','-config',config,'-keyout',tmp/'server.key','-out',tmp/'server.csr')
        openssl('x509','-req','-in',tmp/'server.csr','-CA',tmp/'ca.pem','-CAkey',tmp/'root.key','-CAcreateserial','-days','1','-extfile',config,'-extensions','server','-out',tmp/'server.pem')
        os.chmod(tmp/'root.key',0o600);os.chmod(tmp/'server.key',0o600)
        result['public_ca_sha256'] = sha(tmp/'ca.pem')
        for lang in ['ja','en']:
            with socket.socket() as reserve:
                reserve.bind(('127.0.0.1',0));port=reserve.getsockname()[1]
            server = subprocess.Popen(['openssl','s_server','-accept',f'127.0.0.1:{port}','-cert',str(tmp/'server.pem'),'-key',str(tmp/'server.key'),'-quiet','-naccept','1','-tls1_3'],stdin=subprocess.DEVNULL,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL,start_new_session=True)
            try:
                deadline=time.monotonic()+8
                while True:
                    if server.poll() is not None:
                        raise RuntimeError('TLS fixture server exited before listening')
                    if sys.platform == 'linux':
                        ready = linux_listener_ready(server.pid,port)
                    else:
                        ready=subprocess.run(['lsof','-n','-a','-p',str(server.pid),f'-iTCP:{port}','-sTCP:LISTEN'],capture_output=True).returncode==0
                    if ready:
                        break
                    if time.monotonic()>deadline:
                        raise RuntimeError('TLS fixture listener deadline exceeded')
                    time.sleep(.03)
                out=args.out_dir/('tls-'+lang)
                item=run_cli(binary,out,['tls','127.0.0.1','--port',str(port),'--allow-cidr','127.0.0.1/32','--server-name','localhost','--ca-file',str(tmp/'ca.pem'),'--lang',lang],0)
                report=json.loads((out/'probe.tls.v1.json').read_text())
                if (report['outcome'] != 'completed' or report['diagnostics'] or not report['details']['handshake_complete'] or report['details']['verification']['certificate_chain_and_name']!='verified' or report['details']['negotiated']['version']['id']!=772 or not report['details']['peer_certificates']):
                    raise RuntimeError('TLS preview lacks required verified fresh facts')
                # Retain no private key or content in the result artifacts.
                item['command']=[part.replace(temporary,'<ephemeral-public-ca-dir>') for part in item['command']]
                result['runs'].append(item)
            finally:
                stop(server)
            listener=socket.socket();listener.bind(('127.0.0.1',0));listener.listen(1)
            port=listener.getsockname()[1];record={};thread=threading.Thread(target=ssh_peer,args=(listener,record));thread.start()
            try:
                out=args.out_dir/('ssh-'+lang)
                item=run_cli(binary,out,['ssh','127.0.0.1','--port',str(port),'--allow-cidr','127.0.0.1/32','--lang',lang],0)
            finally:
                thread.join(timeout=14)
                if thread.is_alive():
                    listener.close()
                    raise RuntimeError('SSH fixture cleanup deadline exceeded')
            if record.get('error') or record.get('client_packet_types') != [20] or not record.get('no_post_kexinit_bytes'):
                raise RuntimeError('SSH fixture did not prove discovery-only completion')
            report=json.loads((out/'probe.ssh-offers.v1.json').read_text())
            if report['outcome']!='completed' or report['diagnostics'] or report['details']['assurance']!='unauthenticated_advertisement' or len(report['details']['offers'])!=10:
                raise RuntimeError('SSH preview lost unauthenticated offer scope')
            item['peer_record']=record;result['runs'].append(item)
            for protocol in ['tls','ssh']:
                out=args.out_dir/(protocol+'-denied-'+lang)
                item=run_cli(binary,out,[protocol,'127.0.0.1','--port','443','--lang',lang],3)
                filename='probe.tls.v1.json' if protocol=='tls' else 'probe.ssh-offers.v1.json'
                report=json.loads((out/filename).read_text())
                if report['diagnostics']!=['address_denied'] or report['traffic']['sent_bytes'] or report['traffic']['received_bytes'] or report['target']['connected_peer'] is not None:
                    raise RuntimeError('denied preview did not fail before connection/traffic')
                result['runs'].append(item)
    if sha(binary)!=result['binary_sha256']:
        raise RuntimeError('probe executable changed during fixtures')
    result['fixture_keys_removed']=True
    (args.out_dir/'RESULTS.json').write_text(json.dumps(result,indent=2)+'\n')
    print('Eight local protocol/report fixtures passed; private fixture keys removed.')

if __name__ == '__main__':
    main()
