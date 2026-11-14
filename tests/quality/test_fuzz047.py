import json
from pathlib import Path
import shutil
import sys
import tempfile
import unittest
from task047.study import generate,minimize,trial,run_target,MAX_INPUT
class Fuzz047Tests(unittest.TestCase):
 def test_generation_is_bounded_and_replayable_with_all_seeds_first(self):
  seeds=[{'name':'one','hex':b'abc'.hex()},{'name':'two','hex':b'1234'.hex()}]
  self.assertEqual(generate(seeds,47,1),('two',b'1234',1))
  seen=[]
  for i in range(100):
   x=generate(seeds,47,i);self.assertEqual(x,generate(seeds,47,i));self.assertLessEqual(len(x[1]),MAX_INPUT);seen.append(x[1])
  self.assertIn(b'',seen);self.assertIn(b'\0'*MAX_INPUT,seen)
 def test_real_process_panic_control_is_preserved_minimized_and_replayed(self):
  code="import os;from pathlib import Path;d=Path(os.environ['PQC_FUZZ_INPUT']).read_bytes();print('PQC047:{}');raise SystemExit(42 if b'CRASH' in d else 0)"
  with tempfile.TemporaryDirectory() as tmp:
   root=Path(tmp);result=run_target('synthetic-control',[sys.executable,'-c',code],[{'name':'control','hex':b'paddingCRASHtail'.hex()}],root/'out',root/'work',seconds=5,seed=47,max_cases=1)
   self.assertEqual(result['statuses'],{'panic':1});self.assertEqual((root/'out/failure/minimal.bin').read_bytes(),b'CRASH')
   self.assertTrue((root/'out/failure/original-run/logs/stdout.log').exists());self.assertTrue((root/'out/failure/minimal-run/logs/stderr.log').exists())
   minimal=(root/'out/failure/minimal.bin').read_bytes()
   self.assertTrue(all(b'CRASH' not in minimal[:i]+minimal[i+1:] for i in range(len(minimal))))
 def test_success_keeps_recipe_and_feedback_but_not_unbounded_worker_files(self):
  with tempfile.TemporaryDirectory() as tmp:
   root=Path(tmp);result=run_target('control',[sys.executable,'-c',"print('PQC047:{\"state\":\"accepted\"}')"],[{'name':'empty','hex':''}],root/'out',root/'work',seconds=5,seed=47,max_cases=2)
   self.assertEqual(result['statuses'],{'completed':2});self.assertFalse((root/'work').exists());self.assertEqual(len((root/'out/executions.jsonl').read_text().splitlines()),2)

 def test_shrinking_budget_is_finite_and_never_claims_minimality(self):
  with self.assertRaisesRegex(RuntimeError,'budget exhausted'):
   minimize(b'abcdef',lambda value:False,max_calls=1)

 def test_unbounded_campaign_is_rejected_before_running_worker(self):
  with tempfile.TemporaryDirectory() as tmp:
   root=Path(tmp)
   with self.assertRaises(ValueError):run_target('invalid',['not-a-worker'],[{'name':'x','hex':''}],root/'out',root/'work',seconds=float('inf'),seed=1)
   self.assertFalse((root/'out').exists())
