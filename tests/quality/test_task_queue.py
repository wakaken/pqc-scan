"""Task identifiers must survive YAML 1.1 parsing as exact filename strings."""
from pathlib import Path
import unittest
try:
    import yaml
except ImportError:
    yaml = None

ROOT = Path(__file__).resolve().parents[2]

@unittest.skipIf(yaml is None, 'PyYAML is needed for the optional YAML 1.1 cross-check; Rust covers YAML 1.2')
class TaskQueueTests(unittest.TestCase):
    def test_string_ids_filenames_unique_dependencies_and_preserved_decisions(self):
        tasks = {}
        for path in sorted((ROOT / 'tasks').glob('*.yaml')):
            with self.subTest(path=path.name):
                task = yaml.safe_load(path.read_text())
                identifier = task['id']
                self.assertIsInstance(identifier, str)
                self.assertEqual(identifier, path.name.split('-')[0])
                self.assertNotIn(identifier, tasks)
                tasks[identifier] = task
        self.assertEqual(len(tasks), 40)
        for identifier, task in tasks.items():
            for dependency in task.get('depends_on', []):
                self.assertIsInstance(dependency, str)
                self.assertIn(dependency, tasks, identifier)
                self.assertNotEqual(identifier, dependency)
        self.assertEqual(tasks['014']['status'], 'cancelled')
        self.assertEqual(tasks['016']['status'], 'cancelled')
        self.assertEqual(tasks['023']['status'], 'done')
        self.assertEqual(tasks['001']['status'], 'blocked')
        self.assertEqual(tasks['037']['status'], 'blocked')

if __name__ == '__main__':
    unittest.main()
