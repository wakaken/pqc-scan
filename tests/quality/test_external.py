"""External OSS measurement boundaries; hand-derived counts, no upstream execution."""
import copy
import hashlib
import gzip
import importlib.util
import pathlib
import tempfile
import unittest

class ExternalStudyTests(unittest.TestCase):
    def test_language_does_not_pool_uppercase_c_or_ambiguous_headers(self):
        classify = getattr(self.external(), 'source_language', None)
        self.assertTrue(callable(classify), 'new languages require source-based grouping')
        for name, expected in [('a.c', 'C'), ('a.C', 'C++'), ('a.cpp', 'C++'),
                               ('a.h', 'Ambiguous C/C++ header'), ('a.hpp', 'C++'),
                               ('a.cs', 'C#'), ('a.php', 'PHP'), ('a.phtml', 'PHP'),
                               ('a.PHP', 'PHP'), ('a.H', 'Ambiguous C/C++ header'),
                               ('a.inc', 'Unresolved INC'), ('a.tsx', 'TSX'),
                               ('a.swift', 'Unsupported Swift'), ('LICENSE', 'Not Applicable')]:
            with self.subTest(name=name):
                self.assertEqual(classify(name), expected)

    def external(self):
        spec = importlib.util.find_spec('external')
        self.assertIsNotNone(spec, 'external source validation/score adapter is missing')
        import external
        return external

    def test_partial_annotation_preserves_unattributed_and_duplicate_predictions(self):
        external = self.external()
        data = dict(files=[dict(path='a.php', split='tuning')], families=dict(api=['R']),
                    units=[dict(id='p', path='a.php', ruleId='R', line=2, startColumn=1, endColumn=5),
                           dict(id='n', path='a.php', ruleId='R', line=3, startColumn=1, endColumn=5),
                           dict(id='miss', path='a.php', ruleId='R', line=4, startColumn=1, endColumn=5)])
        labels = dict(p='positive', n='negative', miss='positive')
        def prediction(line, column=1, rule='R'):
            return dict(rule_id=rule, location=dict(file='a.php', line=line, column=column))
        predictions = [prediction(2), prediction(2), prediction(3), prediction(4, 5),
                       prediction(99), prediction(2, rule='OTHER')]
        scored = external.score_annotated(data, labels, predictions)
        group = scored['accuracy']['groups']['tuning/api']
        self.assertEqual({k: group[k] for k in ('tp', 'fp', 'fn', 'tn')},
                         dict(tp=1, fp=2, fn=1, tn=0))
        self.assertEqual(len(scored['unattributed']), 2)
        self.assertEqual(len(scored['other_rules']), 1)
        self.assertEqual(len(predictions), 6)
        # Reject overlapping truth, including overlaps with the same label.
        invalid = copy.deepcopy(data)
        invalid['units'].append(dict(id='overlap', path='a.php', ruleId='R', line=2, startColumn=2, endColumn=4))
        with self.assertRaises(ValueError):
            external.score_annotated(invalid, dict(labels, overlap='positive'), predictions)

    def test_dependency_unknown_position_matches_only_exact_name(self):
        external = self.external()
        data = dict(files=[dict(path='composer.json', split='tuning')],
                    families=dict(dependency=['D']), units=[dict(id='d', path='composer.json',
                    ruleId='D', line=4, startColumn=2, endColumn=20, dependencyName='ext-openssl')])
        finding = dict(rule_id='D', location=dict(file='composer.json', line=0, column=0),
                       evidence=dict(metadata=dict(dep_name='ext-openssl')))
        good = external.score_annotated(data, {'d': 'positive'}, [finding])
        self.assertEqual(good['accuracy']['groups']['tuning/dependency']['tp'], 1)
        finding['evidence']['metadata']['dep_name'] = 'ext-sodium'
        unknown = external.score_annotated(data, {'d': 'positive'}, [finding])
        self.assertEqual(unknown['accuracy']['groups']['tuning/dependency']['fn'], 1)
        self.assertEqual(len(unknown['unattributed']), 1)

    def test_bound_source_rejects_changes_traversal_and_symlink(self):
        external = self.external()
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            source = root/'a.php'
            source.write_bytes(b'<?php echo 1;\n')
            expected = hashlib.sha256(b'<?php echo 1;\n').hexdigest()
            self.assertEqual(external.bound_source(root, 'a.php', expected), source.resolve())
            source.write_bytes(b'changed')
            with self.assertRaises(ValueError):
                external.bound_source(root, 'a.php', expected)
            with self.assertRaises(ValueError):
                external.bound_source(root, '../a.php', expected)
            (root/'link').symlink_to(source)
            with self.assertRaises(ValueError):
                external.bound_source(root, 'link', hashlib.sha256(b'changed').hexdigest())
            (root/'dirlink').symlink_to(root, target_is_directory=True)
            with self.assertRaises(ValueError):
                external.bound_source(root, 'dirlink/a.php', hashlib.sha256(b'changed').hexdigest())

    def test_tree_binding_rejects_added_files_changed_links_and_changed_bytes(self):
        external = self.external()
        validate = getattr(external, 'validate_tree', None)
        self.assertTrue(callable(validate), 'full input population must be bound')
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            (root/'a.c').write_bytes(b'abc')
            (root/'link').symlink_to('a.c')
            (root/'.git').mkdir()
            (root/'.git/index').write_bytes(b'non-input Git metadata')
            files = [dict(path='a.c', mode='100644', bytes=3,
                          sha256='ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad'),
                     dict(path='link', mode='120000', symlink='a.c')]
            validate(root, files)
            (root/'added.c').write_bytes(b'not in pinned population')
            with self.assertRaises(ValueError):
                validate(root, files)
            (root/'added.c').unlink()
            (root/'link').unlink()
            (root/'link').symlink_to('elsewhere')
            with self.assertRaises(ValueError):
                validate(root, files)
            (root/'link').unlink()
            (root/'link').symlink_to('a.c')
            (root/'a.c').write_bytes(b'abd')
            with self.assertRaises(ValueError):
                validate(root, files)
            (root/'a.c').write_bytes(b'abc')
            (root/'nested/.git/info').mkdir(parents=True)
            with self.assertRaises(ValueError):
                validate(root, files)

    def test_scoring_input_rejects_modified_truth_and_compressed_or_plain_report(self):
        external = self.external()
        load = getattr(external, 'load_bound_report', None)
        self.assertTrue(callable(load), 'scoring must verify actual frozen truth and output bytes')
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            truth = root/'truth.json'
            truth.write_bytes(b'{}\n')
            output = root/'report.json.gz'
            raw = b'{"findings": []}\n'
            compressed = gzip.compress(raw)
            output.write_bytes(compressed)
            receipt = dict(truthBindings={'truth.json': hashlib.sha256(b'{}\n').hexdigest()},
                           artifacts={'report.json': dict(compressedPath=str(output),
                           compressedSha256=hashlib.sha256(compressed).hexdigest(),
                           sha256=hashlib.sha256(raw).hexdigest(), bytes=len(raw))})
            self.assertEqual(load(receipt, {'truth.json': truth}), {'findings': []})
            truth.write_bytes(b'changed')
            with self.assertRaises(ValueError):
                load(receipt, {'truth.json': truth})
            truth.write_bytes(b'{}\n')
            output.write_bytes(gzip.compress(b'{"findings": [1]}\n'))
            with self.assertRaises(ValueError):
                load(receipt, {'truth.json': truth})
            changed = copy.deepcopy(receipt)
            changed['artifacts']['report.json']['compressedSha256'] = hashlib.sha256(output.read_bytes()).hexdigest()
            with self.assertRaises(ValueError):
                load(changed, {'truth.json': truth})
            with self.assertRaises(ValueError):
                load(receipt, {})

    def test_selection_binding_tracks_effective_global_parent_and_local_ignores(self):
        external = self.external()
        bind = getattr(external, 'selection_bindings', None)
        self.assertTrue(callable(bind), 'ignore sources must be frozen without rewriting HOME')
        with tempfile.TemporaryDirectory() as tmp:
            base = pathlib.Path(tmp).resolve()
            home, xdg, root = base/'home', base/'xdg', base/'repos/project'
            home.mkdir()
            (xdg/'git').mkdir(parents=True)
            (root/'.git/info').mkdir(parents=True)
            (home/'.gitconfig').write_text('[core]\n excludesFile = ~/excluded\n')
            (home/'excluded').write_text('*.pem\n')
            (xdg/'git/config').write_text('[core]\nexcludesFile = ~/other\n')
            first = bind(root, home=home, xdg=xdg, cwd=base)
            self.assertEqual(first['effectiveGlobalIgnore'], str(home/'excluded'))
            self.assertIn(str(root.parent/'.ignore'), first['files'])
            (root/'.git/info/exclude').write_text('*.c\n')
            self.assertNotEqual(first, bind(root, home=home, xdg=xdg, cwd=base))
            (root/'.git/info/exclude').unlink()
            (home/'excluded').write_text('*.cs\n')
            self.assertNotEqual(first, bind(root, home=home, xdg=xdg, cwd=base))
            (home/'.gitconfig').unlink()
            self.assertEqual(bind(root, home=home, xdg=xdg, cwd=base)['effectiveGlobalIgnore'], str(home/'other'))
            (xdg/'git/config').unlink()
            default = bind(root, home=home, xdg=xdg, cwd=base)
            self.assertEqual(default['effectiveGlobalIgnore'], str(xdg/'git/ignore'))
            (root.parent/'.ignore').write_text('project/a.php\n')
            self.assertNotEqual(default, bind(root, home=home, xdg=xdg, cwd=base))
            (root.parent/'.ignore').unlink()
            (root.parent/'.git/info').mkdir(parents=True)
            (root.parent/'.git/info/exclude').write_text('*.c\n')
            self.assertNotEqual(default, bind(root, home=home, xdg=xdg, cwd=base))

    def test_relative_global_selection_paths_use_child_working_directory(self):
        external = self.external()
        with tempfile.TemporaryDirectory() as tmp:
            base = pathlib.Path(tmp).resolve()
            root = base/'repo'
            root.mkdir()
            (base/'cfg/git').mkdir(parents=True)
            (base/'home').mkdir()
            (base/'cfg/git/config').write_text('[core]\nexcludesFile = relative-ignore\n')
            (base/'relative-ignore').write_text('*.php\n')
            bound = external.selection_bindings(root, home=pathlib.Path('home'), xdg=pathlib.Path('cfg'), cwd=base)
            self.assertEqual(bound['effectiveGlobalIgnore'], str(base/'relative-ignore'))
            self.assertTrue(bound['files'][str(base/'relative-ignore')]['exists'])


if __name__ == '__main__':
    unittest.main()
