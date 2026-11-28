"""Small immutable receipts exercise the descriptive summarizer without scanning code."""
import contextlib
import copy
import csv
import importlib.util
import io
import json
import pathlib
import tempfile
import unittest
from unittest import mock

ROOT = pathlib.Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('oss_summary', ROOT / 'benchmark/summarize.py')
summary = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(summary)
oss = summary.oss
CANARY = 'PRIVATE_BODY_CANARY_NEVER_SUMMARIZED'


def write(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data), encoding='utf-8')


class SummaryTest(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix='pqc-summary-fixture-')
        self.addCleanup(self.temporary.cleanup)
        self.root = pathlib.Path(self.temporary.name)
        groups = ['Java', 'Go', 'Python', 'Rust', 'Ruby', 'JavaScript / TypeScript',
                  'PHP', 'C / C++', 'C#']
        self.rows = []
        for index in range(98):
            self.rows.append(dict(id='fixture__' + str(index),
                population='primary' if index < 90 else 'comparison',
                group=groups[index % 9] if index < 90 else 'JS/TS',
                languages=[summary.LANGUAGES[index % 11]],
                scope='synthetic fixture only', commit='a' * 40, split='fixture', priority=False))
        self.registry = self.root / 'registry.json'
        write(self.registry, dict(repositories=self.rows))
        self.receipts = {(population, phase): self.invocation(population, phase)
                         for population in ('primary', 'comparison') for phase in ('before', 'after')}

    def invocation(self, population, phase):
        selected = [r for r in self.rows if r['population'] == population]
        directory = self.root / population / phase
        config = dict(repeats=1, requiredArtifacts=['report.json', 'diagnostics.json', 'scan.inventory.v1.json'])
        environment = dict.fromkeys(('python', 'platform', 'machine', 'cpuCount', 'hardware', 'rust', 'cargo'), None)
        request = dict(phase=phase, selectedIds=[r['id'] for r in selected],
            selectedEntries={r['id']: r for r in selected}, expectedSamples=1,
            configuration=config, scanner=dict(binary=dict(path='fixture-scanner', sha256='b' * 64),
                rulesSha256='c' * 64), protocol=dict(path='fixture-protocol', sha256='d' * 64),
            harness={}, coordinatorEnvironment=environment,
            registry=dict(path='historical-registry.json', sha256='e' * 64))
        write(directory / 'started.json', request)
        result_rows = []
        for index, entry in enumerate(selected):
            row = dict(repositoryId=entry['id'], sample=0, population=population, split=entry['split'],
                       execution='NotRun', assessment=None)
            if index < 2:
                attempt = directory / ('attempt-' + str(index))
                output = attempt / 'output'
                output.mkdir(parents=True)
                bindings = dict(registryEntry=entry, phase=phase, sample=0, input={},
                    configuration=config, scanner=request['scanner'], protocol=request['protocol'],
                    harness={}, environment=environment)
                started = dict(repositoryId=entry['id'], phase=phase, bindings=bindings)
                write(attempt / 'started.json', started)
                if index == 0:
                    findings = [dict(rule_id='PUBLIC_RULE', category='CryptoAPI', risk='quantum-uncertain',
                        severity='info', location=dict(file=name, line=1, column=1),
                        description=CANARY, evidence=dict(type='source_reference', match=CANARY,
                            snippet_preview=CANARY, metadata=dict(purpose=CANARY)),
                        source_snippet=dict(lines=[dict(text=CANARY)]))
                        for name in ('source.rs', 'include.h', 'Cargo.toml', 'app.ts', 'app.js')]
                    if population == 'primary' and phase == 'after':
                        findings = []
                    report = dict(generated_at='fixture', findings=findings, cbom=[], dependency_sbom=[],
                        summary=dict(total_findings=len(findings),
                                     by_risk={'quantum-uncertain': 5} if findings else {},
                                     by_severity={'info': 5} if findings else {},
                                     scanned_files=5, skipped_files=0))
                    coverage = {key: 0 for key in summary.COVERAGE_COUNTERS}
                    coverage.update(selected_files=5, read_ok_files=5, analyzed_files=4, partial_files=1,
                                    not_enumerated_population=True, status='partial')
                    if population == 'comparison' and phase == 'after':
                        coverage.update(analyzed_files=5, partial_files=0,
                                        not_enumerated_population=False, status='complete')
                    write(output / 'report.json', report)
                    write(output / 'diagnostics.json', dict(coverage=coverage))
                    write(output / 'scan.inventory.v1.json', {})
                    artifacts, assessment = oss._artifacts(output, config['requiredArtifacts'])
                    execution = 'Succeeded'
                else:
                    artifacts, assessment, execution = {}, None, 'TimedOut'
                terminal = dict(execution=execution, startedDigest=oss.digest(started),
                    executionKey=oss.digest(bindings), artifacts=artifacts, assessment=assessment,
                    measurement=dict(wallSeconds=1.5 + index, cpuSeconds=None, peakRssBytes=100 + index,
                                     command=[CANARY]))
                write(attempt / 'terminal.json', terminal)
                row.update(execution=execution, assessment=assessment,
                           receipt=oss._reference(attempt / 'terminal.json'))
            result_rows.append(row)
        invocation = dict(request, rows=result_rows)
        path = directory / 'invocation.json'
        write(path, invocation)
        return path

    def test_descriptive_rectangles_privacy_and_separate_populations(self):
        result = summary.summarize(self.registry, self.receipts)
        self.assertEqual(len(result['repositories']), 98)
        self.assertEqual(len({r['repositoryId'] for r in result['repositories']}), 98)
        self.assertIsNone(result['accuracy'])
        self.assertFalse(result['evaluationComplete'])
        self.assertIsNone(result['cryptographicFamilyMapping'])
        self.assertNotIn(CANARY, json.dumps(result))
        self.assertNotIn('evidence', result['repositories'][0]['before']['samples'][0])
        for population, count in (('primary', 90), ('comparison', 8)):
            aggregate = result['populations'][population]['phases']['before']
            self.assertEqual(result['populations'][population]['repositoryCount'], count)
            self.assertEqual(aggregate['observedSamples'], count)
            self.assertEqual(aggregate['findingCount'], 5)
            self.assertEqual(aggregate['inspectionCounts'], {'Partial': 1, 'Unavailable': count - 1})
            self.assertEqual(aggregate['executionCounts']['TimedOut'], 1)
            self.assertEqual(aggregate['locationLanguageCounts']['Rust'], 1)
            self.assertEqual(aggregate['locationLanguageCounts']['C/C++ header unresolved'], 1)
            self.assertEqual(aggregate['locationLanguageCounts']['Manifest/configuration'], 1)
            self.assertEqual(sum(aggregate['locationLanguageCounts'].values()), 5)
            self.assertEqual(aggregate['coverage']['counterTotals']['partial_files'], 1)
            self.assertIsNone(aggregate['performanceByExecution']['Succeeded']['distributions']['cpuSeconds'])
        failed = result['repositories'][1]['before']['aggregate']
        self.assertIsNone(failed['findingCount'])
        self.assertIsNone(failed['nativeFields']['category'])
        self.assertIsNone(failed['coverage'])
        self.assertEqual(failed['performanceByExecution']['TimedOut']['distributions']['wallSeconds']['median'], 2.5)
        missing = result['repositories'][2]['before']['aggregate']
        self.assertIsNone(missing['findingCount'])
        self.assertIsNone(missing['performanceByExecution']['NotRun']['distributions'])
        self.assertEqual(result['populations']['comparison']['groups']['JavaScript / TypeScript']['repositoryCount'], 8)
        self.assertEqual(result['repositories'][-1]['registryGroup'], 'JS/TS')
        self.assertEqual(result['inputs']['primary']['before']['registry']['sha256'], 'e' * 64)
        self.assertEqual(result['populations']['comparison']['phases']['after']['inspectionCounts']['Complete'], 1)
        self.assertEqual(result['populations']['primary']['phases']['after']['findingCount'], 0)
        self.assertEqual(result['populations']['primary']['phases']['after']['inspectionCounts']['Partial'], 1)

    def test_tampered_report_is_rejected_by_existing_verifier(self):
        path = self.root / 'primary/before/attempt-0/output/report.json'
        report = json.loads(path.read_text())
        report['findings'].pop()
        write(path, report)
        with self.assertRaises(ValueError):
            summary.summarize(self.registry, self.receipts)

    def rebind_primary_before(self):
        attempt = self.root / 'primary/before/attempt-0'
        terminal_path = attempt / 'terminal.json'
        terminal = json.loads(terminal_path.read_text())
        artifacts, assessment = oss._artifacts(attempt / 'output',
            ['report.json', 'diagnostics.json', 'scan.inventory.v1.json'])
        terminal.update(artifacts=artifacts, assessment=assessment)
        write(terminal_path, terminal)
        invocation_path = self.receipts['primary', 'before']
        invocation = json.loads(invocation_path.read_text())
        invocation['rows'][0].update(receipt=oss._reference(terminal_path), assessment=assessment)
        write(invocation_path, invocation)

    def test_rebound_native_summary_must_match_actual_findings(self):
        path = self.root / 'primary/before/attempt-0/output/report.json'
        original = json.loads(path.read_text())
        cases = [('total_findings', 999), ('by_risk', {'quantum-uncertain': 4}),
                 ('by_severity', {'info': 4}), ('by_risk', {'quantum-safe': 5}),
                 ('by_severity', {'low': 5})]
        for key, value in cases:
            with self.subTest(key=key, value=value):
                report = copy.deepcopy(original)
                report['summary'][key] = value
                write(path, report)
                self.rebind_primary_before()
                with self.assertRaises(ValueError):
                    summary.summarize(self.registry, self.receipts)

    def test_zero_summary_buckets_are_consistent(self):
        path = self.root / 'primary/before/attempt-0/output/report.json'
        report = json.loads(path.read_text())
        report['summary']['by_risk']['quantum-safe'] = 0
        report['summary']['by_severity']['low'] = 0
        write(path, report)
        self.rebind_primary_before()
        result = summary.summarize(self.registry, self.receipts)
        self.assertEqual(result['populations']['primary']['phases']['before']['findingCount'], 5)

    def test_resource_metadata_accepts_only_known_methods_and_scopes(self):
        path = self.root / 'primary/before/attempt-0/terminal.json'
        original = json.loads(path.read_text())
        known_scope = summary.run.resource_scope(system='linux', available=True)
        cases = [dict(resourceScope=CANARY), dict(resourceMethod=CANARY),
                 dict(resourceScope=dict(known_scope, extra=dict(secret=CANARY))),
                 dict(resourceScope=dict(known_scope, wallScope=CANARY)),
                 dict(resourceScope=dict(known_scope, wallUnit=dict(secret=CANARY)))]
        for metadata in cases:
            with self.subTest(metadata=metadata):
                terminal = copy.deepcopy(original)
                terminal['measurement'].update(metadata)
                write(path, terminal)
                self.rebind_primary_before()
                with self.assertRaises(ValueError):
                    summary.summarize(self.registry, self.receipts)
        for system, available in [('linux', True), ('darwin', True), ('other', True), ('other', False)]:
            terminal = copy.deepcopy(original)
            scope = summary.run.resource_scope(system=system, available=available)
            terminal['measurement'].update(resourceScope=scope,
                resourceMethod='wait4 per child' if available else 'unavailable',
                unknown=dict(secret=CANARY))
            write(path, terminal)
            self.rebind_primary_before()
            result = summary.summarize(self.registry, self.receipts)
            self.assertNotIn(CANARY, json.dumps(result))
            measured = result['repositories'][0]['before']['samples'][0]['measurement']
            self.assertEqual(measured['resourceScope'], scope)
            self.assertEqual(measured['peakRssBytes'], 100)
            self.assertIn('attempt', result['repositories'][0]['before']['samples'][0])

    def test_second_publication_failure_rolls_back_and_can_retry(self):
        result = summary.summarize(self.registry, self.receipts)
        output, csv_output = self.root / 'pair.json', self.root / 'pair.csv'
        real_link = summary.os.link
        def fail_second(source, target):
            if pathlib.Path(target) == csv_output:
                raise PermissionError('synthetic second-publication failure')
            return real_link(source, target)
        with mock.patch.object(summary.os, 'link', side_effect=fail_second):
            with self.assertRaises(PermissionError):
                summary.publish_pair(output, csv_output, result)
        self.assertFalse(output.exists())
        self.assertFalse(csv_output.exists())
        self.assertEqual(list(self.root.glob('.pair*')), [])
        summary.publish_pair(output, csv_output, result)
        self.assertEqual(json.loads(output.read_text())['schemaVersion'], 1)
        self.assertEqual(len(list(csv.DictReader(io.StringIO(csv_output.read_text())))), 98)

    def test_publication_failure_preserves_concurrent_outputs(self):
        result = summary.summarize(self.registry, self.receipts)
        real_link = summary.os.link
        for replacement in (False, True):
            with self.subTest(replacement=replacement):
                output = self.root / ('concurrent-' + str(replacement) + '.json')
                csv_output = output.with_suffix('.csv')
                def race_second(source, target):
                    if pathlib.Path(target) == csv_output:
                        csv_output.write_text('other csv')
                        if replacement:
                            output.unlink()
                            output.write_text('other json')
                    return real_link(source, target)
                with mock.patch.object(summary.os, 'link', side_effect=race_second):
                    with self.assertRaises(FileExistsError):
                        summary.publish_pair(output, csv_output, result)
                self.assertEqual(csv_output.read_text(), 'other csv')
                if replacement:
                    self.assertEqual(output.read_text(), 'other json')
                else:
                    self.assertFalse(output.exists())
                self.assertEqual(list(self.root.glob('.concurrent-*')), [])

    def test_publication_preserves_each_preexisting_destination(self):
        result = summary.summarize(self.registry, self.receipts)
        for preexisting in ('json', 'csv'):
            with self.subTest(preexisting=preexisting):
                output = self.root / ('existing-' + preexisting + '.json')
                csv_output = output.with_suffix('.csv')
                existing = output if preexisting == 'json' else csv_output
                existing.write_text('prior evidence')
                with self.assertRaises(ValueError):
                    summary.publish_pair(output, csv_output, result)
                self.assertEqual(existing.read_text(), 'prior evidence')
                self.assertFalse((csv_output if preexisting == 'json' else output).exists())

    def test_control_metadata_is_rejected_before_either_output_is_published(self):
        result = summary.summarize(self.registry, self.receipts)
        output, csv_output = self.root / 'controls.json', self.root / 'controls.csv'
        controls = ['\x00', '\t', '\r', '\n', '\x1b', '\x7f', '\x85', '\u200b', '\ufeff', '\ud800']
        for control in controls:
            for value in (control + '=1+1', 'text' + control + 'tail'):
                with self.subTest(value=repr(value)):
                    result['repositories'][0]['split'] = value
                    with self.assertRaises(ValueError):
                        summary.publish_pair(output, csv_output, result)
                    self.assertFalse(output.exists())
                    self.assertFalse(csv_output.exists())
                    self.assertEqual(list(self.root.glob('.controls*')), [])

    def test_printable_unicode_labels_and_formula_escaping_remain_available(self):
        result = summary.summarize(self.registry, self.receipts)
        for value, expected in [('調整用 café', '調整用 café'), ('  =1+1', "'  =1+1")]:
            result['repositories'][0]['split'] = value
            rows = list(csv.DictReader(io.StringIO(summary.csv_text(result))))
            self.assertEqual(rows[0]['split'], expected)

    def test_duplicate_matrix_and_mismatched_selection_are_rejected(self):
        path = self.receipts['primary', 'before']
        invocation = json.loads(path.read_text())
        invocation['rows'].append(copy.deepcopy(invocation['rows'][0]))
        write(path, invocation)
        with self.assertRaises(ValueError):
            summary.summarize(self.registry, self.receipts)
        invocation['rows'].pop()
        write(path, invocation)
        registry = dict(repositories=copy.deepcopy(self.rows))
        registry['repositories'][0]['commit'] = 'f' * 40
        write(self.registry, registry)
        with self.assertRaises(ValueError):
            summary.summarize(self.registry, self.receipts)

    def test_wrapper_and_csv_cli_use_exclusive_outputs(self):
        path = self.receipts['primary', 'before']
        wrapper = dict(json.loads(path.read_text()), receiptPath=str(path))
        wrapper_path = self.root / 'wrapper.json'
        write(wrapper_path, wrapper)
        self.receipts['primary', 'before'] = wrapper_path
        output, csv_output = self.root / 'result.json', self.root / 'result.csv'
        args = ['--registry', str(self.registry), '--output', str(output), '--csv-output', str(csv_output)]
        for (population, phase), receipt in self.receipts.items():
            args.extend(['--' + population + '-' + phase, str(receipt)])
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(summary.main(args), 0)
        with csv_output.open(newline='') as stream:
            rows = list(csv.DictReader(stream))
        self.assertEqual(len(rows), 98)
        self.assertEqual(rows[0]['before_findingCount'], '5')
        self.assertEqual(rows[1]['before_findingCount'], 'null')
        self.assertTrue(all(row['accuracy'] == 'null' for row in rows))
        self.assertNotIn(CANARY, csv_output.read_text())
        original = output.read_bytes(), csv_output.read_bytes()
        with contextlib.redirect_stderr(io.StringIO()):
            self.assertEqual(summary.main(args), 2)
        self.assertEqual((output.read_bytes(), csv_output.read_bytes()), original)
        result = json.loads(output.read_text())
        result['repositories'][0]['repositoryId'] = '=DANGEROUS()'
        rendered = list(csv.DictReader(io.StringIO(summary.csv_text(result))))
        self.assertEqual(rendered[0]['repositoryId'], "'=DANGEROUS()")
        wrapper['rows'].pop()
        write(wrapper_path, wrapper)
        with self.assertRaises(ValueError):
            summary.summarize(self.registry, self.receipts)

    def test_location_attribution_does_not_use_declared_languages(self):
        cases = {'path/include.h': 'C/C++ header unresolved', 'file.C': 'C++', 'file.c': 'C',
                 'Gemfile': 'Manifest/configuration', 'src/App.tsx': 'TypeScript',
                 'src/app.cjs': 'JavaScript', 'unknown.inc': 'Other/unattributed'}
        for filename, language in cases.items():
            with self.subTest(filename=filename):
                self.assertEqual(summary.location_language(filename), language)

    def test_repeats_keep_observation_units_and_missing_optional_coverage(self):
        observations, _ = summary.invocation_observations(self.receipts['primary', 'before'],
                                                         'primary', self.rows)
        first = observations['fixture__0'][0]
        second = copy.deepcopy(first)
        second['sample'] = 1
        second['coverage'].pop('excluded_enumerated_files')
        result = summary.aggregate([first, second])
        self.assertEqual(result['observedSamples'], 2)
        self.assertEqual(result['findingCount'], 10)
        self.assertIn('observations', result['findingCountUnit'])
        self.assertEqual(result['coverage']['counterAvailableSamples']['excluded_enumerated_files'], 1)
        self.assertEqual(result['performanceByExecution']['Succeeded']['distributions']['peakRssBytes']['max'], 100)


if __name__ == '__main__':
    unittest.main()
