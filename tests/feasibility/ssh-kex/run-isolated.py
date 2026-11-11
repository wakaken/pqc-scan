#!/usr/bin/env python3
"""Standalone no-userauth feasibility. All peers are ephemeral loopback fixtures."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import signal
import socket
import subprocess
import tempfile
import threading
import time

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
SSHD = Path('/opt/openssh/sbin/sshd')
KEYGEN = '/opt/openssh/bin/ssh-keygen'


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def free_port():
    with socket.socket() as s:
        s.bind(('127.0.0.1', 0))
        return s.getsockname()[1]


def stop(process):
    if process.poll() is None:
        os.killpg(process.pid, signal.SIGTERM)
        try:
            process.wait(timeout=2)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            process.wait(timeout=2)


def ready(process, port):
    deadline = time.monotonic() + 3
    while time.monotonic() < deadline:
        if process.poll() is not None:
            raise RuntimeError('Local sshd exited before listening')
        # Inspect kernel listener state; never consume the one connection as a readiness probe.
        rows = Path('/proc/net/tcp').read_text().splitlines()[1:]
        if any(int(row.split()[1].split(':')[1], 16) == port and row.split()[3] == '0A' for row in rows):
            return
        time.sleep(.03)
    raise RuntimeError('Local sshd listen timeout')


def proxy(listener, server_port, facts):
    """Flip one signature byte in the unencrypted KEX reply, never retain payloads."""
    def exact(sock, count):
        data = b''
        while len(data) < count:
            block = sock.recv(count-len(data))
            if not block:
                raise EOFError
            data += block
        return data

    try:
        with listener, listener.accept()[0] as client, socket.create_connection(('127.0.0.1', server_port), timeout=5) as server:
            client.settimeout(6)
            server.settimeout(6)
            def upload():
                total = 0
                try:
                    while block := client.recv(4096):
                        total += len(block)
                        if total > 65536:
                            raise ValueError('client traffic cap')
                        server.sendall(block)
                except (OSError, ValueError):
                    pass
                finally:
                    try:
                        server.shutdown(socket.SHUT_WR)
                    except OSError:
                        pass
            worker = threading.Thread(target=upload)
            worker.start()
            try:
                banner = b''
                while not banner.endswith(b'\n'):
                    banner += exact(server, 1)
                    if len(banner) > 8192:
                        raise ValueError('banner cap')
                client.sendall(banner)
                total = len(banner)
                while True:
                    head = exact(server, 4)
                    length = int.from_bytes(head, 'big')
                    if not 5 <= length <= 262144 or total + length + 4 > 524288:
                        raise ValueError('packet cap')
                    packet = bytearray(head + exact(server, length))
                    total += len(packet)
                    kind = packet[5]
                    facts['server_plaintext_types'].append(kind)
                    if kind == 31:
                        # Last payload byte is in the Ed25519 KEX signature, not padding.
                        packet[4 + length - packet[4] - 1] ^= 1
                        facts['signature_bytes_flipped'] += 1
                    client.sendall(packet)
                    if kind == 21:
                        while block := server.recv(4096):
                            total += len(block)
                            if total > 524288:
                                raise ValueError('server traffic cap')
                            client.sendall(block)
                        break
            except (EOFError, OSError, ValueError):
                pass
            finally:
                for sock in [client, server]:
                    try:
                        sock.shutdown(socket.SHUT_RDWR)
                    except OSError:
                        pass
                worker.join(timeout=2)
                facts['upload_thread_stopped'] = not worker.is_alive()
    except OSError:
        facts['proxy_socket_error'] = True


def fixtures(out, build_dir):
    binaries = {name: build_dir/'debug'/name for name in ['russh-client', 'ssh2-client']}
    bindings = {str(p.relative_to(ROOT)):sha(p) for p in sorted(HERE.rglob('*')) if p.is_file() and '__pycache__' not in p.parts}
    binary_hashes = {name:sha(path) for name,path in binaries.items()}
    cases = []
    with tempfile.TemporaryDirectory(prefix='pqc039-fixture-') as temp:
        tmp = Path(temp)
        for key in ['host', 'wrong']:
            subprocess.run([KEYGEN, '-q', '-t', 'ed25519', '-N', '', '-f', str(tmp/key)], check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=10)
            assert (tmp/key).stat().st_mode & 0o077 == 0
        variants = [
            ('hybrid-trusted', 'hybrid', 'mlkem768x25519-sha256', 'host', 'russh-client'),
            ('hybrid-wrong-pin', 'hybrid', 'mlkem768x25519-sha256', 'wrong', 'russh-client'),
            ('hybrid-unknown-pin', 'hybrid', 'mlkem768x25519-sha256', 'unknown', 'russh-client'),
            ('hybrid-tampered-signature', 'hybrid', 'mlkem768x25519-sha256', 'host', 'russh-client'),
            ('classical-trusted', 'classical', 'curve25519-sha256', 'host', 'russh-client'),
            ('hybrid-no-overlap', 'hybrid', 'curve25519-sha256', 'host', 'russh-client'),
            ('ssh2-service-control', 'classical', 'curve25519-sha256', 'host', 'ssh2-client'),
        ]
        for name, profile, kex, pin, binary in variants:
            port = free_port()
            config = tmp/'sshd_config'
            config.write_text(f'Port {port}\nListenAddress 127.0.0.1\nHostKey {tmp}/host\nPidFile {tmp}/pid\nUsePAM no\nPasswordAuthentication no\nPubkeyAuthentication no\nHostbasedAuthentication no\nKbdInteractiveAuthentication no\nUseDNS no\nKexAlgorithms {kex}\nLogLevel DEBUG3\n')
            server = subprocess.Popen([str(SSHD), '-D', '-e', '-ddd', '-f', str(config)], stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, start_new_session=True)
            packets = []
            sends = []
            errors = []
            def consume():
                # Do not store raw logs, usernames, keys, packet bodies or shared secrets.
                for line in server.stderr:
                    if any(marker in line.lower() for marker in [b'fatal', b'operation not permitted', b'permission denied', b'sshd-session']):
                        errors.append(line.decode(errors='replace').strip().replace(str(tmp), '<fixture-dir>'))
                    for direction, target in [(b'receive', packets), (b'send', sends)]:
                        match = re.search(direction + rb' packet: type (\d+)', line)
                        if match:
                            target.append(int(match[1]))
            reader = threading.Thread(target=consume)
            reader.start()
            proxy_facts = {'server_plaintext_types':[], 'signature_bytes_flipped':0}
            proxy_thread = None
            try:
                ready(server, port)
                if 'tampered' in name:
                    listener = socket.socket()
                    listener.bind(('127.0.0.1', 0))
                    listener.listen(1)
                    listener.settimeout(6)
                    proxy_thread = threading.Thread(target=proxy, args=(listener, port, proxy_facts))
                    proxy_thread.start()
                    port = listener.getsockname()[1]
                trust = 'unknown' if pin == 'unknown' else str(tmp/(pin+'.pub'))
                command = [str(binaries[binary]), str(port), trust]
                command.append(profile)
                start = time.monotonic()
                result = subprocess.run(command, capture_output=True, text=True, timeout=8)
                data = json.loads(result.stdout)
                assert not result.stderr, 'Unexpected client stderr (not retained)'
            finally:
                stop(server)
                reader.join(timeout=2)
                server.stderr.close()
                if proxy_thread:
                    proxy_thread.join(timeout=7)
                    assert not proxy_thread.is_alive()
            assert not reader.is_alive()
            assert 20 in packets, ('No decoded server-side KEXINIT evidence', name, data, errors)
            forbidden = [t for t in packets if t == 5 or t >= 50]
            if binary == 'russh-client':
                assert not forbidden, (name, forbidden)
                good = name in ['hybrid-trusted', 'classical-trusted']
                assert data['completed'] == good and result.returncode == (0 if good else 2), (name, data)
                if good:
                    assert data['pin_checked'] and data['pin_matched'] and data['selected_kex'] == kex
                    assert 21 in packets and 21 in sends, 'Both directions must reach NEWKEYS'
                else:
                    assert data['selected_kex'] is None
                if 'wrong-pin' in name or 'unknown-pin' in name:
                    assert data['pin_checked'] and not data['pin_matched'] and data['error_class'] == 'untrusted_host'
                if 'tampered' in name:
                    assert data['error_class'] == 'invalid_signature' and not data['pin_checked']
                    assert proxy_facts['signature_bytes_flipped'] == 1 and proxy_facts['upload_thread_stopped']
                if 'no-overlap' in name:
                    assert data['error_class'] == 'no_overlap' and not data['pin_checked']
            else:
                assert data['handshake'] and data['pin_matched'] and not data['authenticated']
                assert 5 in packets and not any(t >= 50 for t in packets), 'Negative control must expose automatic service request only'
            cases.append({'name':name, 'client':binary, 'profile':profile, 'exit':result.returncode, 'seconds':time.monotonic()-start, 'facts':data, 'server_received_types':packets, 'server_sent_types':sends, 'forbidden_client_types':forbidden, 'proxy':proxy_facts, 'server_and_threads_stopped':True})
        # Stalled identification: outer client deadline wins without any completed facts.
        with socket.socket() as listener:
            listener.bind(('127.0.0.1', 0))
            listener.listen(1)
            listener.settimeout(7)
            def stall():
                with listener.accept()[0] as sock:
                    sock.settimeout(7)
                    while sock.recv(1024):
                        pass
            thread = threading.Thread(target=stall)
            thread.start()
            result = subprocess.run([str(binaries['russh-client']), str(listener.getsockname()[1]), str(tmp/'host.pub'), 'hybrid'], capture_output=True, text=True, timeout=7)
            data = json.loads(result.stdout)
            thread.join(timeout=2)
            assert not thread.is_alive() and result.returncode == 2 and not data['completed'] and data['error_class'] == 'timeout' and data['selected_kex'] is None
            cases.append({'name':'stalled-banner', 'exit':result.returncode, 'facts':data, 'fixture_thread_stopped':True})
    assert not Path(temp).exists()
    assert all(sha(ROOT/p) == h for p,h in bindings.items())
    assert all(sha(binaries[name]) == h for name,h in binary_hashes.items())
    record = {'platform':platform.platform(), 'rust':subprocess.check_output(['rustc','--version'],text=True).strip(), 'sshd_version':subprocess.run([str(SSHD),'-V'],capture_output=True,text=True).stderr.strip(), 'sshd_sha256':sha(str(SSHD)), 'source_bindings':bindings, 'binaries':binary_hashes, 'before_after_match':True, 'fixture_keys_removed':True, 'cases':cases}
    (out/'RESULTS.json').write_text(json.dumps(record,indent=2)+'\n')
    print('Eight isolated study cases passed; ssh2 automatic service request demonstrated.')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--out-dir', type=Path, required=True)
    parser.add_argument('--build-dir', type=Path, required=True)
    args = parser.parse_args()
    if platform.system() != 'Linux':
        raise SystemExit('Run only in the reviewed Docker --network none environment.')
    interfaces = [p for p in Path('/sys/class/net').iterdir() if p.is_dir()]
    if any(p.name != 'lo' and int((p/'flags').read_text(), 16) & 1 for p in interfaces) or len(Path('/proc/net/route').read_text().splitlines()) != 1:
        raise SystemExit('Unexpected active non-loopback interface or IPv4 route.')
    args.out_dir.mkdir(parents=True, exist_ok=False)
    fixtures(args.out_dir, args.build_dir.resolve())


if __name__ == '__main__':
    main()
