"""Offline reconciliation of real peer/report evidence, with destructive controls."""
import argparse
import hashlib
import json
from pathlib import Path

TLS_GOOD={'tls12-x25519':(771,29,'classical'),'tls13-x25519':(772,29,'classical'),'tls13-p256':(772,23,'classical'),'tls13-p384':(772,24,'classical'),'tls13-hybrid-hrr':(772,4588,'hybrid')}
TLS_BAD={'untrusted':'certificate_validation_failed','wrong-name':'certificate_validation_failed','expired':'certificate_validation_failed','not-yet-valid':'certificate_validation_failed','chain-limit':'certificate_collection_limit','message-limit':'local_resource_limit','mtls':'unsupported_local','refused':'transport_failed','timeout':'deadline_exceeded','cancelled':'cancelled','oversized-record':'local_resource_limit','truncated-record':'peer_disconnected'}
SSH_CASES=['classical','hybrid','unknown','fragmented','oversized','truncated','disconnect','unexpected-auth','timeout','cancelled','receive-limit','send-limit','refused']


def check_ssh(row):
    report=row['report']; peer=row['peer']; details=report['details']; mode=row['mode']
    assert peer['joined'] is True and peer['connections']<=1
    assert all(n in ({20} if mode=='offers' else {1,20,30}) for n in peer['receivedMessageTypes'])
    assert report['traffic']['sent_bytes']==peer['receivedBytes']
    for direction in ['sent','received']:
        assert 0<=report['traffic'][direction+'_bytes']<=report['limits']['max_'+direction+'_bytes']
    if mode=='offers':
        assert not any(key in details for key in ['negotiated','negotiated_kex','host_identity_verified','handshake_complete'])
        if row['case'] in ['classical','hybrid','unknown','fragmented']:
            assert details['assurance']=='unauthenticated_advertisement'
            assert report['outcome']=='completed'
            candidate=details['kex_candidates'][0]
            assert candidate['construction']=={'classical':'classical','hybrid':'hybrid','unknown':'unknown','fragmented':'hybrid'}[row['case']]
        else:
            assert report['outcome']!='completed'
            expected='not_observed' if row['case'] in ['refused','send-limit','receive-limit'] else 'peer_claim_only'
            assert details['assurance']==expected
            assert details['offers'] is None and details['kex_candidates']==[]
    else:
        assert report['outcome']!='completed' and not details['host_identity_verified']
        assert not details['handshake_complete'] and details['negotiated_kex'] is None and details['host_signature'] is None
        assert details['user_authentication_attempted'] is False


def check_tls(row):
    case=row['case'];r=row['report'];d=r['details'];assert row['peer']['joined'] is True
    if case in TLS_GOOD:
        version,group,kind=TLS_GOOD[case]
        assert r['outcome']=='completed' and not r['diagnostics'] and d['handshake_complete']
        assert d['verification']['certificate_chain_and_name']=='verified'
        n=d['negotiated'];assert n['version']['id']==version
        assert n['key_exchange_group'] is not None and n['key_exchange_group']['id']==group
        assert n['group_classification']==kind
        assert (row['peer']['tlsVersion'],row['peer']['group'],row['peer']['expectedClassification'])==(version,group,kind)
        if case=='tls13-hybrid-hrr':assert d['handshake_kind']=='full_with_hello_retry_request'
    else:
        assert r['outcome']!='completed'
        if case in TLS_BAD:assert TLS_BAD[case] in r['diagnostics']
        # Collection limits can follow a verified handshake; never erase this
        # distinction by demanding all incomplete reports contain no valid facts.
        if case not in ['chain-limit','message-limit']:
            assert not d['handshake_complete'] and d['negotiated'] is None
        if case=='mtls':assert 22 in row['peer']['receivedRecordTypes'] and all(t in [20,22] for t in row['peer']['receivedRecordTypes'])


def load(path):return json.loads(path.read_text())
def write(path,value):path.write_text(json.dumps(value,indent=2,ensure_ascii=False)+'\n')


def collect(raw,out):
    out.mkdir(parents=True,exist_ok=True)
    tls=[]
    for case in [*TLS_GOOD,*TLS_BAD,'profile-no-overlap']:
        row=load(raw/'tls'/f'{case}.json');assert row['case']==case;check_tls(row);tls.append(row)
    for case in ['absent-group','unknown-group']:
        row=load(raw/'tls'/f'{case}.json');n=row['details']['negotiated'];assert n['group_classification']=='unknown'
        assert n['key_exchange_group'] is None if case=='absent-group' else n['key_exchange_group']['id']==0x7a7a
        tls.append(row)
    fresh=load(raw/'tls/freshness.json');assert len(set(fresh['publicKeyShareSha256']))==2 and fresh['offeredVersions']==[772,771] and not fresh['pskOffered'] and not fresh['earlyDataOffered'] and fresh['joined']
    tls.append(fresh)
    assert next(r for r in tls if r['case']=='tls13-x25519')['report']['details']['negotiated']['cipher_suite']==next(r for r in tls if r['case']=='tls13-p256')['report']['details']['negotiated']['cipher_suite'],'same suite must exercise distinct groups'
    ssh=[]
    for mode in ['offers','kex']:
        for case in SSH_CASES:
            row=load(raw/'ssh'/f'{mode}-{case}.json');assert (row['mode'],row['case'])==(mode,case);check_ssh(row);ssh.append(row)
    openssh=[]
    for case in ['classical','hybrid']:
        row=load(raw/'openssh-offers'/f'{case}.json');assert row['receivedMessageTypes']==[20] and row['report']['details']['assurance']=='unauthenticated_advertisement'
        assert row['report']['outcome']=='completed';openssh.append(row)
    product=load(raw.parent/'ssh-kex/RESULTS.json');assert product['fixture_keys_removed'] and product['binary_before_after_match']
    assert {r['case'] for r in product['cases']}=={'success-ja','success-en','wrong-pin','tampered-signature','no-overlap'}
    for row in product['cases']:
        assert row['cleanup'] and row['server_received_types'] and all(t in [1,20,21,30] for t in row['server_received_types'])
        report=load(raw.parent/'ssh-kex'/row['case']/'probe.ssh-kex.v1.json');d=report['details'];good=row['case'].startswith('success')
        assert row['exit']==(0 if good else 3) and d['host_identity_verified']==good and d['handshake_complete']==good and not d['user_authentication_attempted']
        if good:assert report['outcome']=='completed' and d['negotiated_kex']=='mlkem768x25519-sha256'
        else:assert d['negotiated_kex'] is None and d['host_signature'] is None
        row['report']=report
    coverage={'classicalVsHybrid':'TLS1.2 X25519; TLS1.3 X25519/P256/MLKEM; SSH unauthenticated classical/hybrid/unknown and pinned MLKEM KEX',
              'profile':'existing fixed provider aws_lc_rs 1.18.0 / rustls 0.23.46; no provider swap',
              'unknownGroup':'unit-controlled post-verification accessor, not an unsupported real network handshake',
              'bounds':'parser exact/over-limit and malformed lists, real IO truncation/budgets/deadline/cancellation, single target/attempt; existing core transport tests bind DNS/pinning',
              'limits':['finite controlled local peers','no whole-protocol conformance or formal support expansion','Windows not executed','CI protocols remains paused']}
    write(out/'tls-matrix.json',{'requirement':'Q4-08','status':'Passed','cases':tls,'scope':coverage})
    write(out/'tls-failures.json',{'requirement':'Q4-09','status':'Passed','cases':[r for r in tls if r['case'] in TLS_BAD or r['case']=='profile-no-overlap']})
    write(out/'ssh-matrix.json',{'requirement':'Q4-10','status':'Passed','offers':openssh+[r for r in ssh if r['case'] in ['classical','hybrid','unknown','fragmented']],'verifiedKex':product,'scope':coverage})
    write(out/'ssh-failures.json',{'requirement':'Q4-12','status':'Passed','cases':[r for r in ssh if r['mode']=='kex' or r['case'] not in ['classical','hybrid','unknown','fragmented']],'boundaryTests':['tests/parser.rs: identification_and_frame_boundaries_are_checked_before_payload_access; exact_limits_and_ssh199_compatibility_are_supported','tests/runtime.rs: banner_aggregate_and_transport_budgets_are_enforced_at_actual_io','tests/negotiated.rs: oversized_ssh_frame_is_rejected_before_large_body_allocation; async_transport_enforces_send_and_receive_budgets','pqc_scan_core target_policy/transport tests: all answers, pinned peer, single attempt, resolver cancellation']})
    write(out/'ssh-no-auth.json',{'requirement':'Q4-11','status':'Passed','rawPeerCases':ssh,'opensshOffers':openssh,'opensshKex':product,'forbiddenMessageTypes':{'service':[5,6],'userauth':'50..79','global/channel':'80..255'},'scope':'received message types, not only a client boolean; encrypted KEX cases use OpenSSH server-side decoding; unknown raw peer cases never reach encryption'})
    write(out/'protocol-assurance.json',{'requirement':'Q3-06','status':'Passed','observations':{'sshOffers':'unauthenticated_advertisement, no negotiated or verified fields','sshKex':'only trusted pin + signature + completed initial KEX yields verified MLKEM+X25519; Ed25519 host authentication remains classical','tls':'verified version/suite/group distinct from incomplete collection; failed certificate verification never yields verified negotiation'},'scope':coverage,'inputHashes':{str(p.relative_to(raw.parent)):hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(raw.rglob('*.json'))}})


if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--raw',type=Path,required=True);parser.add_argument('--out',type=Path,required=True);args=parser.parse_args();collect(args.raw,args.out)
