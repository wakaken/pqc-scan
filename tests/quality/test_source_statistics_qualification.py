import importlib.util,pathlib,unittest
p=pathlib.Path(__file__).resolve().parents[2]/'benchmark/accuracy_assessment.py'
s=importlib.util.spec_from_file_location('candidate_assessment',p);a=importlib.util.module_from_spec(s);s.loader.exec_module(a)
class Qualification(unittest.TestCase):
    def stats(self,status,kind='positive_fact',scope=True,detected=False):
        fact={'factId':'f','kind':kind,'inScope':scope,'occurrenceId':'o'}
        census=[{'id':'s','complete':True,'facts':[fact]}]
        maps=[{'factId':'f','mapping':{'status':status,'predicate':'any','unitIds':['u'] if status=='mapped' else []}}]
        score={'accuracy':{'details':[{'unit':'u','disposition':'tp'}] if detected else []}}
        return a.source_statistics(census,maps,score)
    def group(self,stats):
        return a.aggregate_source_scores([{'kind':'source-assessment-score','repositoryId':'r','population':'comparison','sourceStatistics':stats}])['comparison']
    def test_all_unmapped_states_withhold_capability_point(self):
        for state in sorted(a.MAPPING_STATES-{'mapped'}):
            with self.subTest(state=state):
                stats=self.stats(state)
                self.assertIsNone(stats['finiteKnownSourceCapabilityCoverage'])
                self.assertEqual(stats['unresolvedPositiveMappings'],1)
                self.assertEqual(stats['mappedMissFactIds'],[])
                group=self.group(stats)
                self.assertIsNone(group['finiteKnownSourceCapabilityCoverage'])
                self.assertEqual(group['unresolvedPositiveMappings'],1)
    def test_exact_mapped_positive_preserves_bounded_rate_and_miss(self):
        for detected in (False,True):
            with self.subTest(detected=detected):
                stats=self.stats('mapped',detected=detected)
                self.assertEqual(stats['finiteKnownSourceCapabilityCoverage'],float(detected))
                self.assertEqual(stats['mappedMissFactIds'],[] if detected else ['f'])
                self.assertEqual(self.group(stats)['finiteKnownSourceCapabilityCoverage'],float(detected))
    def test_unknown_outside_is_not_positive_outside(self):
        stats=self.stats('unknown',kind='unknown',scope=False)
        self.assertEqual(stats['outsideMigrationScopeFactIds'],[])
        self.assertEqual(stats['unknownFactIds'],['f'])
        self.assertEqual(self.group(stats)['outsideMigrationScopeFacts'],0)
        self.assertEqual(self.group(stats)['unknownFacts'],1)
    def test_known_positive_outside_stays_separate(self):
        stats=self.stats('canonical_unrepresentable',scope=False)
        self.assertEqual(stats['outsideMigrationScopeFactIds'],['f'])
        self.assertEqual(stats['unknownFactIds'],[])
        self.assertEqual(self.group(stats)['outsideMigrationScopeFacts'],1)
if __name__=='__main__':unittest.main()
