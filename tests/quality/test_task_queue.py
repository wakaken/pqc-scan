"""Task identifiers must survive YAML 1.1 parsing as exact task-directory strings."""
from pathlib import Path
import unittest
try:
    import yaml
except ImportError:
    yaml = None

ROOT = Path(__file__).resolve().parents[2]

@unittest.skipIf(yaml is None, 'PyYAML is needed for the optional YAML 1.1 cross-check; Rust covers YAML 1.2')
class TaskQueueTests(unittest.TestCase):
    def test_string_ids_directories_unique_dependencies_and_preserved_decisions(self):
        tasks = {}
        root = ROOT / 'tasks'
        self.assertFalse(list(root.glob('*.yaml')), 'task definitions must be nested')
        for directory in sorted(path for path in root.iterdir() if path.is_dir()):
            with self.subTest(directory=directory.name):
                self.assertRegex(directory.name, r'^[0-9]{3}$')
                path = directory / 'task.yaml'
                task = yaml.safe_load(path.read_text())
                identifier = task['id']
                self.assertIsInstance(identifier, str)
                self.assertEqual(identifier, directory.name)
                self.assertNotIn(identifier, tasks)
                tasks[identifier] = task
        self.assertTrue({f'{n:03}' for n in range(1, 63)} <= tasks.keys())
        for identifier, task in tasks.items():
            for dependency in task.get('depends_on', []):
                self.assertIsInstance(dependency, str)
                self.assertIn(dependency, tasks, identifier)
                self.assertNotEqual(identifier, dependency)
        self.assertEqual(tasks['014']['status'], 'cancelled')
        self.assertEqual(tasks['016']['status'], 'cancelled')
        self.assertEqual(tasks['023']['status'], 'done')
        for identifier in ['001', '037']:
            task = tasks[identifier]
            self.assertIn(task['status'], ['blocked', 'done'])
            if task['status'] == 'done':
                acceptance = task['final_acceptance']
                self.assertRegex(acceptance['source_head'], r'^[0-9a-f]{40}$')
                self.assertTrue(acceptance['ci'].startswith('https://github.com/wakaken/pqc-scan/actions/runs/'))
                self.assertEqual(acceptance['scope'], 'technical acceptance only; no publication or signing')
                self.assertEqual(len(set(acceptance['reviewers'])), 2)
                self.assertTrue(all(acceptance['reviewers']))

if __name__ == '__main__':
    unittest.main()
