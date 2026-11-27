"""All-phase assessment admission precedes any preserved report parser."""
import os
import pathlib
import unittest
from unittest.mock import patch
import test_accuracy_assessment as f

a = f.a


class FirstVerified(Exception):
    pass


class AssessmentFourPhaseTests(unittest.TestCase):
    def setUp(self):
        self.fixture = f.AdmissionTests()
        self.fixture.setUp()
        self.addCleanup(self.fixture.tearDown)
        self.packet = self.fixture.packet('source')
        self.oss, self.refs = a.load_historical(f.QUALITY)

    def run_admission(self):
        bindings = dict(protocol=a.reference(self.fixture.protocol), packet=a.reference(self.fixture.packet_path))
        with patch.object(a, 'load_historical', return_value=(self.oss, self.refs)):
            return a.admit_historical_inputs(self.fixture.scoring_policy, a.load(self.fixture.protocol), self.packet, bindings, f.QUALITY)

    def test_all_four_terminal_admissions_precede_first_preserved_verifier(self):
        terminals = set()
        original_open = os.open
        def tracked(path, flags, *args, **kwargs):
            if path == 'terminal.json':
                stat = os.fstat(kwargs['dir_fd'])
                terminals.add((stat.st_dev, stat.st_ino))
            return original_open(path, flags, *args, **kwargs)
        def verified(*args):
            self.assertEqual(len(terminals), 4)
            raise FirstVerified()
        with patch.object(os, 'open', side_effect=tracked), patch.object(self.oss, '_verified_invocation', side_effect=verified):
            with self.assertRaises(FirstVerified):
                self.run_admission()

    def test_fourth_phase_artifact_drift_never_parses_old_reports(self):
        self.fixture.historical_reports['comparison-after'].write_bytes(b'SYNTHETIC_DRIFT')
        with patch.object(self.oss, '_verified_invocation', wraps=self.oss._verified_invocation) as verified:
            with self.assertRaises(ValueError):
                self.run_admission()
        verified.assert_not_called()

    def test_fourth_phase_metadata_rebound_scope_never_parses_old_reports(self):
        wrapper_path = self.fixture.invocation_paths['comparison-after']
        wrapper = a.load(wrapper_path)
        actual_path = pathlib.Path(wrapper['receiptPath'])
        actual = a.load(actual_path)
        actual['phase'] = 'baseline'
        f.save(actual_path, actual)
        f.save(wrapper_path, dict(actual, receiptPath=str(actual_path)))
        policy = a.load(self.fixture.scoring_policy)
        policy['invocations'][-1]['binding'] = a.reference(wrapper_path)
        f.save(self.fixture.scoring_policy, policy)
        with patch.object(self.oss, '_verified_invocation', wraps=self.oss._verified_invocation) as verified:
            with self.assertRaises(ValueError):
                self.run_admission()
        verified.assert_not_called()

    def test_valid_four_phases_reach_all_preserved_verifiers(self):
        with patch.object(self.oss, '_verified_invocation', wraps=self.oss._verified_invocation) as verified:
            result = self.run_admission()
        self.assertEqual(verified.call_count, 4)
        self.assertEqual(set(result['invocations']), {'primary-baseline','primary-after','comparison-baseline','comparison-after'})


if __name__ == '__main__':
    unittest.main()
