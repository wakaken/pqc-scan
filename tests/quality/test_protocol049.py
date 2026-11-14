"""Independent counterexamples must prevent a false protocol pass."""
import copy
import unittest
from task049.assess import check_ssh, check_tls


class Protocol049Tests(unittest.TestCase):
    def test_incomplete_offers_retain_lower_assurance_instead_of_claiming_advertisement(self):
        for case,assurance in [('timeout','peer_claim_only'),('refused','not_observed')]:
            row={'mode':'offers','case':case,'report':{'outcome':'partial','details':{'assurance':assurance,'offers':None,'kex_candidates':[]},'traffic':{'sent_bytes':0,'received_bytes':0},'limits':{'max_sent_bytes':65536,'max_received_bytes':524288}},'peer':{'receivedMessageTypes':[],'receivedBytes':0,'joined':True,'connections':0}}
            check_ssh(row)
            row['report']['details']['assurance']='unauthenticated_advertisement'
            with self.assertRaises(AssertionError):check_ssh(row)

    def test_unauthenticated_offer_or_failure_cannot_be_promoted(self):
        row={'mode':'offers','case':'hybrid','report':{'outcome':'completed','details':{'assurance':'unauthenticated_advertisement','kex_candidates':[{'name':'mlkem768x25519-sha256','construction':'hybrid'}]},'traffic':{'sent_bytes':24,'received_bytes':48},'limits':{'max_sent_bytes':65536,'max_received_bytes':524288}},'peer':{'receivedMessageTypes':[20],'receivedBytes':24,'joined':True,'connections':1}}
        check_ssh(row)
        for kind in ['auth','service','channel','negotiated','cleanup','traffic']:
            broken=copy.deepcopy(row)
            if kind in ['auth','service','channel']:broken['peer']['receivedMessageTypes'].append({'auth':50,'service':5,'channel':90}[kind])
            elif kind=='negotiated':broken['report']['details']['negotiated_kex']='mlkem768x25519-sha256'
            elif kind=='cleanup':broken['peer']['joined']=False
            else:broken['report']['traffic']['sent_bytes']=999999
            with self.subTest(kind=kind),self.assertRaises(AssertionError):check_ssh(broken)

    def test_cipher_name_cannot_supply_missing_group_or_failed_verification(self):
        row={'case':'tls13-hybrid-hrr','peer':{'group':4588,'tlsVersion':772,'expectedClassification':'hybrid','joined':True},'report':{'outcome':'completed','diagnostics':[],'details':{'handshake_complete':True,'handshake_kind':'full_with_hello_retry_request','verification':{'certificate_chain_and_name':'verified'},'negotiated':{'version':{'id':772},'cipher_suite':{'id':4866},'key_exchange_group':{'id':4588},'group_classification':'hybrid'}}}}
        check_tls(row)
        for kind in ['group','trust','handshake','unknown']:
            broken=copy.deepcopy(row);details=broken['report']['details']
            if kind=='group':details['negotiated']['key_exchange_group']=None
            elif kind=='trust':details['verification']['certificate_chain_and_name']='failed'
            elif kind=='handshake':details['handshake_complete']=False
            else:details['negotiated']['key_exchange_group']['id']=0x7a7a
            with self.subTest(kind=kind),self.assertRaises(AssertionError):check_tls(broken)


if __name__=='__main__':unittest.main()
