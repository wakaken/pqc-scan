import argparse
import json
import os
from pathlib import Path
import re
import shutil
import sys


def load_harness(directory):
    sys.path.insert(0, str(directory))
    import adapter
    import boundary as b
    import clocks
    import execution
    return adapter, b, clocks, execution


def admit(gate_ref, adapter, b, clocks):
    gate = b.document(gate_ref)
    subjects = gate.get('subjects', {})
    expected_names = {'policy', 'binaryAliases', 'recoveryRegistry', 'sourceRelation', 'witnessBuildReceipt',
                      'witnessBinary', 'witnessToolSource', 'witnessRules', 'overlayBuildReceipt', 'coordinator'}
    v2 = gate.get('kind') == 'temporal-witness-run-gate-v2'
    if v2:
        expected_names.add('boundaryPolicyRelation')
    b.require(gate.get('kind') in {'temporal-witness-run-gate-v1', 'temporal-witness-run-gate-v2'} and gate.get('verdict') == 'PASS' and
              gate.get('reviewerModel') == 'codex:gpt-6.1-sol@high' and
              isinstance(gate.get('reviewerId'), str) and gate['reviewerId'] and
              isinstance(gate.get('authorIds'), list) and set(gate['authorIds']) >= clocks.AUTHORS and
              gate['reviewerId'] not in set(gate['authorIds']) | clocks.AUTHORS and
              set(gate.get('scopes', [])) == clocks.REVIEW_SCOPES and set(subjects) == expected_names and
              subjects['coordinator'] == b.ref(Path(__file__).resolve()) and
              gate.get('harnessBindings') == b.harness_bindings(), 'E_PRODUCER_RUN_GATE')
    for reference in subjects.values():
        clocks._binding(reference)
        b.read(reference)
    source = gate['sourceBindings']
    b.verify_source(source)
    b.require(subjects['witnessToolSource'] == b.ref(b.CODE_DIR / 'temporal_witness.rs'), 'E_PRODUCER_TOOL_SOURCE')
    adapter.verify_build(b.document(subjects['overlayBuildReceipt']), source)
    build = b.document(subjects['witnessBuildReceipt'])
    b.require(build.get('kind') == 'temporal-witness-build-receipt-v1' and type(build.get('exitCode')) is int and
              build['exitCode'] == 0 and build.get('binary') == subjects['witnessBinary'] and
              build.get('toolSource') == subjects['witnessToolSource'] and build.get('sourceBindings') == source and
              build.get('rules') == subjects['witnessRules'], 'E_PRODUCER_BUILD_MANIFEST')
    actual = adapter.verify_witness_build(b.document(build['linkedSourceEvidence']), source)
    b.require(actual['binary'] == subjects['witnessBinary'], 'E_PRODUCER_BUILD_BINARY')
    plan = b.document(actual['plan']) if v2 else None
    if v2:
        b.require(build.get('witnessSchema') == adapter.WITNESS_SCHEMA_V2 and
                  build.get('schemaAuthority') == b.ref(b.CODE_DIR / 'adapter.py') and
                  plan['schema'] == adapter.WITNESS_SCHEMA_V2, 'E_PRODUCER_BUILD_SCHEMA')
    else:
        b.require('witnessSchema' not in build and 'schemaAuthority' not in build, 'E_PRODUCER_BUILD_SCHEMA')
    b.read(build['boundaryTests'])
    relation = b.document(subjects['sourceRelation'])
    b.require(relation.get('kind') == 'temporal-source-relation-v1' and relation.get('finalSourceBindings') == source and
              set(relation.get('clockClosure', {})) == clocks.CLOCK_CLOSURE and
              set(relation.get('historicalSourceBindings', {})) == {'primary-after', 'comparison-after'},
              'E_PRODUCER_SOURCE_RELATION')
    if v2:
        b.require(relation.get('boundaryPolicyRelation') == subjects['boundaryPolicyRelation'], 'E_BOUNDARY_POLICY_RELATION')
        policy = adapter.boundary_policy_relation(subjects['boundaryPolicyRelation'], relation)
        b.require(policy['linkedBuild'] == build['linkedSourceEvidence'], 'E_BOUNDARY_POLICY_BUILD')
    else:
        b.require('boundaryPolicyRelation' not in relation, 'E_BOUNDARY_POLICY_RELATION')
    for evidence in relation['clockClosure'].values():
        b.read(clocks._binding(evidence))
    aliases = b.document(subjects['binaryAliases'])['aliases']
    cohort = clocks.OriginalClockCohort.admit(subjects['policy'], aliases, relation)
    registry = b.document(subjects['recoveryRegistry'])
    rows = registry.get('repositories', [])
    b.require(isinstance(rows, list) and len(rows) == 98 and len({row['id'] for row in rows}) == 98,
              'E_PRODUCER_RECOVERY_MATRIX')
    return gate, relation, cohort, rows


def coverage_value(job, row, producer, witness, config, gate, relation, b, clocks):
    identity, phase, recovered, selected, original, started, generated = job
    subjects = gate['subjects']
    rules = b.document(subjects['witnessRules'])
    closure = subjects['sourceRelation']
    value = dict(kind='temporal-producer-coverage-v1', repositoryId=identity, phaseId=phase,
        bindings=config['bindings'], originalSourceBindings=relation['historicalSourceBindings'][phase],
        sourceRelation=closure, witness=row['witness'], originalStatus=original['analysisStatus'],
        originalInput=original['input'], originalSelection=original['selection'],
        window=dict(firstSecond=started.second, lastSecond=generated.second), clockClosure=sorted(clocks.CLOCK_CLOSURE),
        rulesDirectory=config['rules_dir'], witnessRuleId=config['witness_rule_id'],
        defaultValidityRules=rules['defaultValidityRuleIds'], files=witness['files'], producerCoverage=witness['coverage'],
        selectedFiles={file['path']: file['sourceSha256'] for file in witness['files']},
        timeIndependentStates={file['path']: dict(status=file['status'], sourceSha256=file['sourceSha256'],
            closure=closure, evidence=relation['clockClosure']['raw_detector_outcome'])
            for file in witness['files'] if file['status'] != 'complete'},
        timeIndependentDiagnostics={code: dict(closure=closure,
            evidence=relation['clockClosure']['walker_selection_limits' if code in {'excluded_path', 'file_too_large', 'read_failed'}
                                           else 'raw_detector_outcome']) for code in witness['diagnosticCodes']})
    if gate.get('kind') == 'temporal-witness-run-gate-v2':
        value['kind'] = 'temporal-producer-coverage-v2'
        value['boundaryInputs'] = {key: config[key] for key in
            ('manifest_projection', 'original_diagnostic_projection', 'boundary_policy_relation')}
    if str(config['root']) != original['input']['root']:
        value['inputRelocation'] = dict(kind='temporal-input-relocation-v1',
            originalRoot=original['input']['root'], currentRoot=config['root'], manifest=original['input']['manifest'],
            recoveryRegistry=subjects['recoveryRegistry'], registryEntry=recovered,
            selectionBefore=producer['selectionBefore'], selectionAfter=producer['selectionAfter'],
            sourceRelation=closure, evidence=relation['clockClosure']['walker_selection_limits'])
    return value


def run(gate_ref, destination, adapter, b, clocks, execution):
    b.require_external_storage()
    gate, relation, cohort, registry = admit(gate_ref, adapter, b, clocks)
    subjects = gate['subjects']
    temporal = dict(kind='temporal-replay-admission-v1', sourceBindings=gate['sourceBindings'], sourceRelation=subjects['sourceRelation'],
                    witnessBinary=subjects['witnessBinary'], witnessToolSource=subjects['witnessToolSource'],
                    witnessRules=subjects['witnessRules'])
    v2 = gate.get('kind') == 'temporal-witness-run-gate-v2'
    if v2:
        temporal.update(kind='temporal-replay-admission-v2', boundaryPolicyRelation=subjects['boundaryPolicyRelation'])
    jobs = cohort.interval_jobs(registry)
    if destination.exists():
        b.private_directory(destination)
        b.require(b.document(b.ref(destination / 'run-gate.json')) == dict(gate=gate_ref), 'E_PRODUCER_RESUME_GATE')
    else:
        b.mkdir(destination)
        b.publish(destination / 'run-gate.json', dict(gate=gate_ref))
    oss, external, _ = b.import_auditors()
    rows = []
    for job in jobs:
        identity, phase, recovered, selected, original, started, generated = job
        b.require(re.fullmatch(r'[A-Za-z0-9_.-]+', identity) is not None and identity not in {'.', '..'}, 'E_PRODUCER_ID')
        project = destination / identity
        if project.exists():
            b.private_directory(project)
            terminal = b.document(b.ref(project / 'terminal.json'))
            b.require(terminal.get('kind') == 'temporal-producer-terminal-v1' and terminal.get('gate') == gate_ref and
                      terminal.get('repositoryId') == identity and terminal.get('phaseId') == phase and
                      terminal.get('execution') == 'Succeeded', 'E_PRODUCER_RESUME_TERMINAL')
            row = terminal['row']
            clocks._coverage(row, temporal, relation, original, selected, started, generated)
            rows.append(row)
            continue
        b.mkdir(project)
        b.publish(project / 'started.json', dict(kind='temporal-producer-started-v1', gate=gate_ref,
                                                repositoryId=identity, phaseId=phase))
        b.require(shutil.disk_usage(project).free >= 4 * 1024**3, 'E_FREE_DISK_4GIB')
        b.verify_source(gate['sourceBindings'])
        b.require(b.harness_bindings() == gate['harnessBindings'], 'E_HARNESS_CHANGED')
        root, files, inputs = oss._acquisition(recovered, project)
        b.require(inputs['manifest']['sha256'] == original['input']['manifest']['sha256'], 'E_PRODUCER_INPUT')
        selection_before = external.selection_bindings(root, cwd=b.ROOT)
        execution.equivalent_selection(original['selection'], selection_before, original['input']['root'], root)
        if str(root) == original['input']['root']:
            b.require(selection_before == original['selection'], 'E_PRODUCER_SELECTION')
        expected = dict(source=b.digest(gate['sourceBindings']), rules=subjects['witnessRules']['sha256'],
            input=selected['inputManifestSha256'], selection=b.digest(original['selection']),
            tool=subjects['witnessToolSource']['sha256'])
        configure = adapter.witness_configuration_v2 if v2 else adapter.witness_configuration
        boundary_arguments = dict(original=original, boundary_policy=subjects['boundaryPolicyRelation']) if v2 else {}
        configuration = configure(project, **boundary_arguments, root=root,
            selection=dict(include_non_scannable=False, exclude_tests=False), detector_second=started.second,
            bindings=expected, final_source=gate['sourceBindings'])['configuration']
        config = b.document(configuration)
        b.require(config['rule_files'] == b.document(subjects['witnessRules'])['files'], 'E_PRODUCER_RULE_BUNDLE')
        b.read(subjects['witnessBinary'])
        b.read(original['input']['manifest'])
        source_before = b.source_bindings()
        b.require(source_before == gate['sourceBindings'], 'E_PRODUCER_SOURCE_BEFORE')
        b.read(subjects['coordinator'])
        command = [subjects['witnessBinary']['path'], configuration['path']]
        process = execution.spawn(command, b.ROOT, dict(os.environ), b.mkdir(project / 'logs'), timeout=600)
        b.publish(project / 'process.json', process)
        b.require(process['execution'] == 'Succeeded', 'E_PRODUCER_PROCESS')
        oss._validate_input(root, files)
        b.read(original['input']['manifest'])
        selection_after = external.selection_bindings(root, cwd=b.ROOT)
        b.require(selection_after == selection_before, 'E_PRODUCER_SELECTION_AFTER')
        execution.equivalent_selection(original['selection'], selection_after, original['input']['root'], root)
        source_after = b.source_bindings()
        b.require(source_before == source_after == gate['sourceBindings'] and
                  b.harness_bindings() == gate['harnessBindings'], 'E_PRODUCER_SOURCE_AFTER')
        b.read(subjects['witnessBinary'])
        b.read(subjects['coordinator'])
        b.read(configuration)
        witness_ref = b.ref(config['output'])
        witness = b.approved.decode(b.read(witness_ref))
        row = dict(repositoryId=identity, phaseId=phase, config=configuration, witness=witness_ref)
        producer = dict(kind='temporal-witness-process-v1', execution=process['execution'], exitCode=process['exitCode'],
            timedOut=process['timedOut'], performance=None, scope='static_time_condition_proof',
            binary=subjects['witnessBinary'], configuration=configuration, output=witness_ref,
            sourceBindings=gate['sourceBindings'], sourceBefore=source_before, sourceAfter=source_after,
            inputBefore=original['input']['manifest'], inputAfter=original['input']['manifest'],
            selectionBefore=selection_before, selectionAfter=selection_after,
            commandDigest=process['commandDigest'], cwdDigest=process['cwdDigest'],
            startedUtc=process['startedUtc'], endedUtc=process['endedUtc'])
        row['producerReceipt'] = b.publish(project / 'producer-receipt.json', producer)
        value = coverage_value(job, row, producer, witness, config, gate, relation, b, clocks)
        row['coverage'] = b.publish(project / 'coverage.json', value)
        clocks._coverage(row, temporal, relation, original, selected, started, generated)
        b.publish(project / 'terminal.json', dict(kind='temporal-producer-terminal-v1', gate=gate_ref,
            repositoryId=identity, phaseId=phase, execution='Succeeded', row=row, performance=None))
        rows.append(row)
    return cohort.publish_interval_index(rows, destination / 'coverage-index.json')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--harness', type=Path, required=True)
    parser.add_argument('--gate', required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    b = None
    try:
        adapter, b, clocks, execution = load_harness(args.harness.resolve(strict=True))
        gate = json.loads(args.gate)
        reference = run(gate, args.output, adapter, b, clocks, execution)
        print(json.dumps(dict(status='record_written', record=reference), sort_keys=True))
        return 0
    except (ImportError, OSError, ValueError, KeyError, TypeError, AssertionError) as error:
        code = str(error) if b is not None and isinstance(error, b.Rejected) and re.fullmatch(r'E_[A-Z0-9_]+', str(error)) else 'E_PRODUCER_REJECTED'
        print(json.dumps(dict(status='rejected', errorCode=code, payloadOmitted=True)), file=sys.stderr)
        return 2


if __name__ == '__main__':
    raise SystemExit(main())
