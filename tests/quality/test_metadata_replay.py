"""Metadata corrections must retain independently frozen labels and exposures."""
import json
import pathlib
import shutil
import tempfile
import unittest
from evaluate import bytes_digest, load_truth

class ReplayTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = pathlib.Path(self.tmp.name).resolve()/'corpus'
        shutil.copytree(pathlib.Path(__file__).parent/'task043-corpus-v2', self.root)
    def save(self, name, value):
        (self.root/name).write_text(json.dumps(value))
    def mutate(self, kind):
        data=json.loads((self.root/'corpus.json').read_text())
        truth=json.loads((self.root/'truth.json').read_text())
        if kind=='input': data['files'][0]['sha256']='0'*64
        if kind=='question': data['units'][0]['line']+=1
        if kind=='label': truth['labels'][0]['label']='negative'
        if kind=='protocol': (self.root/'PROTOCOL.md').write_text('Changed question definitions')
        self.save('corpus.json',data);truth['corpusSha256']=bytes_digest(self.root/'corpus.json')
        truth['freeze']['protocolSha256']=bytes_digest(self.root/'PROTOCOL.md')
        for name in truth['labelSources']:
            record=json.loads((self.root/name).read_text());record['corpusSha256']=truth['corpusSha256']
            record['protocolSha256']=truth['freeze']['protocolSha256'];record['coverageAudit']['protocolSha256']=record['protocolSha256']
            if kind=='label': record['labels'][0]['label']='negative'
            self.save(name,record);truth['labelSources'][name]=bytes_digest(self.root/name)
        self.save('truth.json',truth)
    def test_honest_exposed_replay_loads_without_claiming_new_blind_labels(self):
        self.assertTrue(json.loads((self.root/'labels-b.json').read_text())['scannerResultsSeen'])
        data,labels=load_truth(self.root);self.assertEqual(len(labels),255)
        self.assertEqual(len(data['files']),206)
    def test_metadata_replay_cannot_launder_changed_data_questions_labels_or_definitions(self):
        for kind in ['input','question','label','protocol']:
            with self.subTest(kind=kind):
                original=pathlib.Path(__file__).parent/'task043-corpus-v2'
                shutil.copytree(original,self.root,dirs_exist_ok=True);self.mutate(kind)
                with self.assertRaises(ValueError):load_truth(self.root)
    def test_replay_rejects_individual_review_forgery_old_hash_and_chains(self):
        original=pathlib.Path(__file__).parent/'task043-corpus-v2'
        for kind in ['review-reason','old-hash','chain','exposed-original']:
            with self.subTest(kind=kind):
                shutil.copytree(original,self.root,dirs_exist_ok=True)
                truth=json.loads((self.root/'truth.json').read_text())
                if kind=='review-reason':
                    name=next(iter(truth['labelSources']))
                    r=json.loads((self.root/name).read_text());r['labels'][0]['reason']='Forged source justification'
                    self.save(name,r);truth['labelSources'][name]=bytes_digest(self.root/name)
                elif kind=='old-hash':truth['metadataReplay']['originalTruthSha256']='0'*64
                else:
                    prior=self.root/'inherited-v1';old=json.loads((prior/'truth.json').read_text())
                    if kind=='chain':old['metadataReplay']={}
                    else:
                        name=next(iter(old['labelSources']));r=json.loads((prior/name).read_text());r['scannerResultsSeen']=True
                        (prior/name).write_text(json.dumps(r));old['labelSources'][name]=bytes_digest(prior/name)
                    (prior/'truth.json').write_text(json.dumps(old));truth['metadataReplay']['originalTruthSha256']=bytes_digest(prior/'truth.json')
                self.save('truth.json',truth)
                with self.assertRaises(ValueError):load_truth(self.root)
if __name__=='__main__':unittest.main()
