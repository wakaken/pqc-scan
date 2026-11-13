"""Offline TASK-047 campaign; shared verification first, then finite worker calls."""
import json
from pathlib import Path
import struct
import subprocess
import sys
sys.path[:0]=['/source/tests/protocol','/source/tests/quality','/source/tests/quality/task047']
from container_entry import EVIDENCE,SCRATCH,write
from inspection055_entry import main as verify
from study import run_target,digest

def seed(name,data,origin):return {'name':name,'hex':data.hex(),'origin':origin,'sha256':digest(data)}
def spki_from_public_certificate(data):
    def tlv(offset):
        start=offset;tag=data[offset];offset+=1;n=data[offset];offset+=1
        if n&128:
            k=n&127;assert 0<k<=4;n=int.from_bytes(data[offset:offset+k],'big');offset+=k
        end=offset+n;assert end<=len(data);return tag,start,offset,end
    _,_,body,_=tlv(0);_,_,body,_=tlv(body);offset=body
    tag,_,_,end=tlv(offset)
    if tag==0xa0:offset=end
    for _ in range(5):_,_,_,offset=tlv(offset)
    _,start,_,end=tlv(offset);return data[start:end]

def main():
    verify()
    recipes=EVIDENCE/'fuzz';recipes.mkdir()
    groups={k:[] for k in ['DER','PEM','PKCS12','SBOM','settings','source','output','SSH','TLS']}
    original=json.loads((EVIDENCE/'all-features/raw/formats055.json').read_text())['cases']
    for row in original:
        name=row['inputName'];family=row['format']
        category='PKCS12' if name.endswith(('.p12','.pfx')) else 'DER' if name.endswith('.der') else 'PEM' if name.endswith(('.pem','.cer','.crt')) else 'source' if family.startswith('source-') else 'settings' if family.startswith('configuration-') else 'SBOM'
        groups[category].append(seed(name,bytes.fromhex(row['inputHex']),f'formats055.json:{row["id"]}'))
    public=Path('/source/crates/pqc_scan_detectors/tests/fixtures/certificates/rsa2048.der').read_bytes()
    spki=spki_from_public_certificate(public);groups['DER'].append(seed('public-key.der',spki,'public rsa2048.der SPKI TLV, no private material'))
    unknown=spki.replace(bytes.fromhex('06092a864886f70d010101'),bytes.fromhex('06092a864886f70d01017f'),1)
    assert unknown!=spki
    groups['DER'].append(seed('unknown-key.der',unknown,'synthetic unknown OID; key size must remain unknown'))
    legacy={'generated_at':'2026-11-13T00:00:00Z','findings':[],'cbom':[],'dependency_sbom':[],'summary':{'total_findings':0,'by_severity':{},'by_risk':{},'scanned_files':0,'skipped_files':0}}
    populated=dict(legacy,findings=[{'finding_id':'public047','rule_id':'SYNTHETIC','category':'TLS','risk':'quantum-vulnerable','severity':'high','confidence':.8,'description':'public RS256','migration_hint':'public guidance','location':{'file':'public.py','line':1,'column':1},'evidence':{'type':'regex_match','match':'RS256','snippet_preview':'public RS256','metadata':{}},'recommended_actions':[]}])
    native={'schema_version':1,'identity_scheme':'pqc-scan-length-prefixed-sha256-v1','scanner_version':'0.2.0','catalog_version':'native-exact-identifiers-1','profile':'static-native-v1','rules_digest':'synthetic','run':{'id':'scan047','attempt_id':'static1','generated_at':'2026-11-13T00:00:00Z'},'repository_scope':'tests/project','scope':{},'coverage':{},'diagnostics':[],'native_completeness':'not_exhaustive','assets':[],'observations':[],'legacy_associations':[],'unassociated_observation_ids':[],'legacy_loss_records':[],'assessment_status':'not_evaluated','assessments':[],'export_loss_notes':[]}
    for name,value in [('empty.json',legacy),('findings.json',populated),('inventory.json',native),('future.json',dict(native,native_completeness='future047')),('broken-refs.json',dict(native,unassociated_observation_ids=['missing']))]:groups['output'].append(seed(name,json.dumps(value).encode(),'synthetic output contract'))
    groups['output'].append(seed('broken.json',b'{','synthetic malformed output'))
    # KEXINIT uses ten ordered name lists, no private key or key exchange execution.
    names=[b'curve25519-sha256',b'ssh-ed25519',b'aes128-ctr',b'aes128-ctr',b'hmac-sha2-256',b'hmac-sha2-256',b'none',b'none',b'',b'']
    kex=bytes([20])+bytes(16)+b''.join(struct.pack('>I',len(n))+n for n in names)+bytes(5)
    padding=4+(-(5+len(kex)+4))%8;packet=struct.pack('>I',1+len(kex)+padding)+bytes([padding])+kex+bytes(padding)
    for name,data in [('identification',b'SSH-2.0-public\r\n'),('kex',kex),('packet',packet),('oversize-header',b'\xff'*4),('broken',b'bad')]:groups['SSH'].append(seed(name,data,'synthetic RFC-shaped SSH byte sequence'))
    for name,data in [('empty',b''),('alert',bytes.fromhex('15030300020228')),('handshake-header',bytes.fromhex('160303000402000000')),('oversize-header',bytes.fromhex('160303ffff')),('broken',b'not TLS')]:groups['TLS'].append(seed(name,data,'synthetic TLS record, ordinary certificate verification remains enabled'))
    command=['cargo','test','--workspace','--all-features','--no-run','--message-format=json','--locked','--offline']
    with (EVIDENCE/'worker-build.jsonl').open('w') as stdout,(EVIDENCE/'worker-build.stderr.log').open('w') as stderr:
        result=subprocess.run(command,stdout=stdout,stderr=stderr,timeout=300)
    assert result.returncode==0
    artifacts=[json.loads(line) for line in (EVIDENCE/'worker-build.jsonl').read_text().splitlines() if line.startswith('{')]
    binaries={}
    for row in artifacts:
        if row.get('reason')=='compiler-artifact' and row.get('executable'):
            for crate in ['pqc_scan_detectors','pqc_scan_report','pqc_scan_probe_ssh','pqc_scan_probe_tls']:
                if crate in row['package_id'] and ((row['target']['name']=='quality047') if crate!='pqc_scan_probe_tls' else (row['target']['kind']==['lib'])):binaries[crate]=row['executable']
    assert len(binaries)==4,binaries
    write('worker-binaries.json',{'buildCommand':command,'exit':0,'binaries':{k:{'path':v,'sha256':digest(Path(v).read_bytes())} for k,v in binaries.items()}})
    results=[]
    for mode,seconds,cap in [('short',30,256),('long',120,2048)]:
        for i,(category,seeds) in enumerate(groups.items()):
            crate='pqc_scan_report' if category=='output' else 'pqc_scan_probe_ssh' if category=='SSH' else 'pqc_scan_probe_tls' if category=='TLS' else 'pqc_scan_detectors'
            exact='runtime::quality047::quality047_worker' if category=='TLS' else 'quality047_worker'
            command=[binaries[crate],'--ignored','--exact',exact,'--nocapture']
            observed=run_target(category,command,seeds,recipes/(mode+'-'+category),SCRATCH/'fuzz-work',seconds=seconds,seed=470000+(1000 if mode=='long' else 0)+i,max_cases=cap)
            results.append(dict(observed,mode=mode))
            write('fuzz-results.json',{'results':results,'scope':'public/synthetic fixed seeds, finite deterministic byte mutations; not coverage-guided and not proof of all inputs','productFailures':sum(len(r['failures']) for r in results)})
            if observed['failures']:raise RuntimeError('reproducible fuzz failure; original and minimized evidence retained')
    # The same real supervisor/minimizer control is run offline and its artifacts retained.
    control="import os;from pathlib import Path;d=Path(os.environ['PQC_FUZZ_INPUT']).read_bytes();print('PQC047:{}');raise SystemExit(42 if b'CRASH' in d else 0)"
    control_result=run_target('synthetic-panic-control',[sys.executable,'-c',control],[seed('control',b'paddingCRASHtail','deliberate harness fault, not product bug')],EVIDENCE/'reducer-control',SCRATCH/'fuzz-work',seconds=5,seed=47,max_cases=1)
    assert (EVIDENCE/'reducer-control/failure/minimal.bin').read_bytes()==b'CRASH'
    write('reducer-control.json',control_result)

if __name__=='__main__':main()
