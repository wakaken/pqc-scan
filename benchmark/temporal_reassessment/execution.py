import datetime as dt
import os
from pathlib import Path
import re
import shutil
import subprocess
import uuid

import adapter
import boundary as b
import clocks
import storage


RESOURCE = dict(mode='execution_only_user_deferred_performance', timeoutSeconds=600, minFreeGiB=4,
                threads=1, maxFileSizeMiB=2, maxArtifactBytes=b.MAX, maxAttemptArtifactBytes=b.TOTAL,
                wallSeconds=None, cpuSeconds=None, peakRssBytes=None,
                measurementState='user_deferred', timingComparisonEligible=False,
                confinement='subprocess timeout only; no kernel CPU/RSS/network/filesystem confinement')


def now():
    return dt.datetime.now(dt.timezone.utc).isoformat()


def spawn(command, cwd, environment, log_directory, *, timeout=600):
    b.require(isinstance(command, list) and all(isinstance(arg, str) for arg in command) and
              0 < timeout <= 600, 'E_EXECUTION_COMMAND')
    b.private_directory(log_directory)
    started = now()
    try:
        process = subprocess.run(command, cwd=cwd, env=environment, stdin=subprocess.DEVNULL,
                                 stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=timeout, shell=False,
                                 start_new_session=True, check=False)
        execution = 'Succeeded' if process.returncode == 0 else 'Failed'
        code = process.returncode
        error_code = None if code == 0 else 'E_SCANNER_EXIT_NONZERO'
        error_kind = None
    except subprocess.TimeoutExpired:
        execution, code, error_code, error_kind = 'TimedOut', None, 'E_SCANNER_TIMEOUT', 'TimeoutExpired'
    except OSError as error:
        execution, code, error_code, error_kind = 'NotRun', None, 'E_SCANNER_LAUNCH', type(error).__name__
    return dict(kind='execution-only-process-v1', startedUtc=started, endedUtc=now(), execution=execution,
                commandDigest=b.digest(command), cwdDigest=b.digest(str(cwd)),
                environmentDigest=b.digest({name: environment.get(name) for name in
                    ('LANG', 'LC_ALL', 'LC_CTYPE', 'TZ', adapter.DETECTOR_ENV, adapter.GENERATED_ENV)}),
                exitCode=code, timedOut=execution == 'TimedOut', measurement=None,
                resources=RESOURCE, errorCode=error_code, errorKind=error_kind,
                stdout=dict(state='NotCaptured', destination='DEVNULL'), stderr=dict(state='NotCaptured', destination='DEVNULL'))


def equivalent_selection(original, current, original_root, current_root):
    def normalize(value, root):
        root = str(root)
        def path(value):
            return '@ROOT' + value[len(root):] if value == root or value.startswith(root + '/') else value
        return dict(effectiveGlobalIgnore=path(value['effectiveGlobalIgnore']), files={path(name):
                    dict(sha256=item['sha256'], resolved=path(item['resolved']))
                    for name, item in value['files'].items() if item['exists']})
    b.require(normalize(original, original_root) == normalize(current, current_root), 'E_SELECTION_RELOCATION')


class AssessmentRejected(b.Rejected):
    def __init__(self, evidence):
        super().__init__(evidence['errorCode'])
        self.evidence = evidence


def safe_schema_errors(errors, validation):
    allowed = set()
    def properties(value):
        if isinstance(value, dict):
            allowed.update(value.get('properties', {}))
            for child in value.values():
                properties(child)
        elif isinstance(value, list):
            for child in value:
                properties(child)
    for validator in validation.values():
        properties(validator.schema)
    return [dict(file=row['file'] if row['file'] in b.FORMATS else '<withheld-file>',
                 validator=row['validator'] if re.fullmatch(r'[A-Za-z$][A-Za-z0-9$]{0,40}', row['validator']) else '<withheld-validator>',
                 path=[part if type(part) is int and 0 <= part <= 1000000 or isinstance(part, str) and part in allowed
                       else '<withheld-field>' for part in row['path']]) for row in errors]


def schema_evidence(output, validation):
    errors = []
    complete = True
    for name, validator in validation.items():
        value = b.approved.decode(b.approved.checked_file(output / name, limit=b.MAX, collect=True))
        for error in validator.iter_errors(value):
            if len(errors) == 32:
                complete = False
                break
            errors.append(dict(file=name, path=list(error.absolute_path), validator=error.validator))
        if not complete:
            break
    safe = safe_schema_errors(errors, validation)
    return dict(schemaValidation='Failed' if safe else 'Passed', schemaErrors=safe,
                schemaErrorsReportedCount=len(safe), schemaErrorsComplete=complete,
                schemaErrorsTruncated=not complete)


def assess(output, limitations, validation, source_manifest=None):
    oss, _, audit = b.import_auditors()
    evidence = dict(errorCode='E_OUTPUT_MATRIX', stage='format_matrix', errorKind=None,
                    descriptiveAssessment='NotRun', schemaValidation='NotRun', exportProjectionReferences='NotRun',
                    nativeSourceReferences='NotRun', schemaErrorsReportedCount=None, schemaErrorsComplete=None,
                    schemaErrors=[], schemaErrorsTruncated=None, failedChecks=0, totalInvalidReferences=None)
    try:
        b.require(set(os.listdir(output)) == set(b.FORMATS), 'E_FORMAT_MATRIX')
        evidence.update(errorCode='E_DESCRIPTIVE_ASSESSMENT', stage='descriptive_assessment')
        artifacts, assessment = oss._artifacts(output, b.FORMATS, limitations)
        evidence['descriptiveAssessment'] = 'Passed'
        evidence.update(errorCode='E_OUTPUT_SCHEMA', stage='export_schema')
        evidence.update(schema_evidence(output, validation))
        if evidence['schemaValidation'] != 'Passed':
            evidence['exportProjectionReferences'] = 'BlockedBySchema'
            raise b.Rejected('E_OUTPUT_SCHEMA')
        evidence.update(errorCode='E_EXPORT_PROJECTION', stage='export_projection')
        audit_result = audit.audit_directory(output, validation)
        errors = safe_schema_errors(audit_result['schemaErrors'], validation)
        evidence.update(schemaValidation='Failed' if errors else 'Passed', schemaErrors=errors,
                        schemaErrorsReportedCount=len(errors), schemaErrorsComplete=len(errors) < 32,
                        exportProjectionReferences='Passed')
        b.require(audit_result['passed'] and not errors, 'E_OUTPUT_AUDIT')
        evidence.update(errorCode='E_NATIVE_SCHEMA_REFERENCE_OR_SPAN', stage='native_source_references')
        import accuracy_native
        native = b.approved.decode(b.approved.checked_file(output / 'scan.inventory.v1.json', limit=b.MAX, collect=True))
        accuracy_native.validate_native(native, source_manifest or {}, validation['scan.inventory.v1.json'].schema)
        evidence['nativeSourceReferences'] = 'Passed'
        audit_result.pop('directory')
        return artifacts, dict(descriptive=assessment, outputAudit=audit_result)
    except (OSError, ValueError, KeyError, TypeError, AssertionError, IndexError) as error:
        evidence.update(errorKind=type(error).__name__, failedChecks=1)
        if evidence['stage'] == 'descriptive_assessment':
            evidence['descriptiveAssessment'] = 'Failed'
        elif evidence['stage'] == 'export_projection' and evidence['exportProjectionReferences'] == 'NotRun':
            evidence['exportProjectionReferences'] = 'Failed'
        elif evidence['stage'] == 'export_schema' and evidence['schemaValidation'] == 'NotRun':
            evidence['schemaValidation'] = 'Failed'
        elif evidence['stage'] == 'native_source_references':
            evidence['nativeSourceReferences'] = 'Failed'
        raise AssessmentRejected(evidence) from None


def available_artifacts(output):
    result = dict(state='Unavailable', known={}, unexpectedEntryCount=None)
    try:
        descriptor = b.approved._open(output, directory=True)
        try:
            names = set(os.listdir(descriptor))
        finally:
            os.close(descriptor)
        result.update(state='Present', unexpectedEntryCount=len(names - set(b.FORMATS)))
        total = 0
        for name in b.FORMATS:
            if name not in names:
                result['known'][name] = dict(state='Missing')
                continue
            try:
                item = b.ref(output / name, min(b.MAX, b.TOTAL - total))
                total += item['bytes']
                b.require(total <= b.TOTAL, 'E_OUTPUT_TOTAL')
                result['known'][name] = dict(state='BoundNotAudited', binding=item)
            except (OSError, ValueError):
                result['known'][name] = dict(state='Rejected', errorCode='E_UNSAFE_OR_UNBOUNDED_ARTIFACT')
    except (OSError, ValueError):
        result['errorCode'] = 'E_OUTPUT_UNAVAILABLE_OR_UNSAFE'
    return result


def verify_output_clocks(output, row):
    report = b.approved.decode(b.read(b.ref(output / 'report.json')))
    native = b.approved.decode(b.read(b.ref(output / 'scan.inventory.v1.json')))
    clocks.ProjectReplay._from_row(row).verify_outputs(report, native)


def verify_archived_clocks(terminal):
    report = storage.read_document(terminal['artifacts'], 'report.json')
    native = storage.read_document(terminal['artifacts'], 'scan.inventory.v1.json')
    clocks.ProjectReplay._from_row(terminal['bindings']['clock']).verify_outputs(report, native)


def unpack_assess(terminal, workspace, validation):
    storage.validate_matrix(terminal['artifacts'])
    workspace = b.mkdir(workspace)
    try:
        for name, item in terminal['artifacts'].items():
            with b.create(workspace / name) as stream:
                stream.write(storage.compressed_read(item))
            b.approved.checked_file(workspace / name, {key: item[key] for key in ('bytes', 'sha256')}, limit=b.MAX)
        inputs = terminal['bindings']['input']
        manifest = inputs.get('manifest')
        files = []
        if manifest:
            b.read(manifest, b.approved.MAX_METADATA)
            oss, _, _ = b.import_auditors()
            value = oss._json(Path(manifest['path']))
            files = value.get('files', []) if isinstance(value, dict) else value
            if isinstance(files, dict):
                files = [dict(item, path=name) for name, item in files.items()]
        verify_output_clocks(workspace, terminal['bindings']['clock'])
        _, result = assess(workspace, inputs['limitations'], validation, {item['path']: item for item in files})
        b.require(result == terminal['assessment'], 'E_RESUME_ASSESSMENT')
    finally:
        for name in b.FORMATS:
            path = workspace / name
            if path.exists():
                b.approved.checked_file(path, {key: terminal['artifacts'][name][key] for key in ('bytes', 'sha256')}, limit=b.MAX)
                path.unlink()
        workspace.rmdir()


def admit(admission_binding):
    admission = b.document(admission_binding)
    b.require(admission['kind'] == 'execution-only-oss98-final-admission-v1' and
              admission['eligibleActualRuns'] is True and admission['features'] == [] and
              admission['performanceDeferredByUser'] is True, 'E_ACTUAL_RUN_GATE')
    b.require('temporalAdmission' in admission, 'E_TEMPORAL_REPLAY_DESIGN_UNIMPLEMENTED')
    for name in ('harnessReview', 'productReview', 'shippingMatrix'):
        record = b.document(admission[name])
        b.require(record['verdict'] == 'PASS' and record['sourceBindings'] == admission['sourceBindings'], 'E_INDEPENDENT_REVIEW')
        if name != 'shippingMatrix':
            b.require(record['reviewerModel'] == 'codex:gpt-6.1-sol@high' and
                      record['reviewerId'] not in clocks.AUTHORS | set(admission.get('authorIds', [])) | {admission['authorId']}, 'E_REVIEWER_INDEPENDENCE')
    review = b.document(admission['harnessReview'])
    b.require(review['harnessBindings'] == b.harness_bindings(), 'E_HARNESS_REVIEW_SOURCE')
    b.require(admission['harnessBindings'] == b.harness_bindings(), 'E_HARNESS_CHANGED')
    b.verify_source(admission['sourceBindings'])
    b.require(admission['validationEnvironment'] == b.validation_environment(), 'E_VALIDATION_ENVIRONMENT')
    b.read(admission['shippingBinary'])
    b.read(admission['adapterBinary'])
    b.require(admission['shippingBinary']['sha256'] != admission['adapterBinary']['sha256'], 'E_ADAPTER_IDENTITY')
    build = b.document(admission['adapterBuildReceipt'])
    adapter.verify_build(build, admission['sourceBindings'])
    b.require(build['adapterBinary'] == admission['adapterBinary'] and
              build['shippingBinary'] == admission['shippingBinary'], 'E_ADAPTER_BUILD')
    protocol = b.document(admission['protocol'])
    b.require(protocol['threads'] == 1 and protocol['repeats'] == 1 and protocol['timeoutSeconds'] == 600 and
              protocol['minFreeGiB'] == 4 and set(protocol['requiredArtifacts']) == set(b.FORMATS) and
              protocol['limits']['maxFileSizeMiB'] == 2 and protocol['limits']['maxArtifactBytes'] == b.MAX,
              'E_EXECUTION_PROTOCOL')
    clock_plan = clocks.ClockPlan.admit(admission['clockMap'], admission['temporalAdmission'],
        admission.get('historicalBinaryAliases', []), admission['sourceBindings'],
        [admission['authorId'], *admission.get('authorIds', [])], admission['adapterBuildReceipt'])
    history = clock_plan.history
    registry = b.document(admission['recoveryRegistry'])
    oss, _, _ = b.import_auditors()
    rows = oss.select_repositories(admission['recoveryRegistry']['path'], oss.Selector(all=True))
    selected = [history[population + '-after']['invocation']['selectedEntries'][identity]
                for population in ('primary', 'comparison')
                for identity in history[population + '-after']['invocation']['selectedIds']]
    b.require(len(rows) == 98 and [row['id'] for row in rows] == [row['id'] for row in selected], 'E_RECOVERY_MATRIX')
    for row, original in zip(rows, selected):
        clocks.admit_recovered_entry(row, original, 'E_RECOVERY_PIN_SCOPE')
    return admission, rows, clock_plan, history


def _historical_bindings(history, row):
    phase = row['population'] + '-after'
    admission = history[phase]
    matches = [bindings for old_row, _, bindings in admission['attempts'].values() if old_row['repositoryId'] == row['id']]
    b.require(len(matches) == 1, 'E_HISTORICAL_AFTER_ID')
    return matches[0]


def expected_project(admission, admission_binding, row, clock_rows, history, output, schemas):
    oss, external, _ = b.import_auditors()
    original = _historical_bindings(history, row)
    b.require(admission['scannerEnvironment'] == original['configuration']['environment'], 'E_ORIGINAL_ENVIRONMENT')
    root, files, inputs = oss._acquisition(row, output)
    selection = external.selection_bindings(root, cwd=b.ROOT)
    equivalent_selection(original['selection'], selection, original['input']['root'], root)
    clock = clock_rows[(row['id'], row['population'] + '-after')]
    bindings = dict(kind='execution-only-oss98-bindings-v1', registryEntry=row, input=inputs,
                    originalInput=original['input'], relocation=dict(originalRoot=original['input']['root'], currentRoot=str(root),
                    sameManifestAndFileHashes=True, physicalCopyOnWriteClaim=False), clock=clock,
                    selection=selection, configuration=RESOURCE, admission=admission_binding,
                    sourceBindings=admission['sourceBindings'], harnessBindings=admission['harnessBindings'],
                    environment=admission['scannerEnvironment'], validationEnvironment=b.validation_environment(), schemas=schemas)
    project = output / row['id']
    command = [admission['adapterBinary']['path'], 'scan', str(root), '--format', 'all', '--diagnostics',
               '--inventory-json', '--cyclonedx-17', '--threads', '1', '--max-file-size-mib', '2',
               '--lang', 'en', '--rules-dir', str(b.ROOT / 'rules/default'), '--out-dir', str(project / 'output')]
    return root, files, bindings, command


def validate_terminal(terminal, project, bindings, command):
    key = b.digest(bindings)
    b.require(b.document(b.ref(project / 'started.json')) == dict(bindings=bindings, executionKey=key),
              'E_PROJECT_STARTED_BINDING')
    b.require(terminal.get('kind') == 'execution-only-oss98-terminal-v1' and terminal.get('execution') == 'Succeeded' and
              terminal.get('executionKey') == key and b.digest(terminal.get('bindings')) == key and
              terminal.get('command') == command and terminal.get('evaluationComplete') is False and
              terminal.get('performance') is None and 'failure' not in terminal, 'E_PROJECT_TERMINAL_BINDING')
    process = terminal.get('process')
    b.require(isinstance(process, dict), 'E_PROJECT_PROCESS')
    replay = clocks.ProjectReplay._from_row(bindings['clock'])
    environment = {name: replay.bind_process(bindings['environment']).get(name) for name in
                   ('LANG', 'LC_ALL', 'LC_CTYPE', 'TZ', adapter.DETECTOR_ENV, adapter.GENERATED_ENV)}
    b.require(process.get('kind') == 'execution-only-process-v1' and process.get('execution') == 'Succeeded' and
              type(process.get('exitCode')) is int and process['exitCode'] == 0 and process.get('timedOut') is False and
              process.get('errorCode') is None and process.get('errorKind') is None and process.get('measurement') is None and
              process.get('resources') == RESOURCE and process.get('commandDigest') == b.digest(command) and
              process.get('cwdDigest') == b.digest(str(b.ROOT)) and process.get('environmentDigest') == b.digest(environment) and
              process.get('stdout') == dict(state='NotCaptured', destination='DEVNULL') and
              process.get('stderr') == dict(state='NotCaptured', destination='DEVNULL'), 'E_PROJECT_PROCESS')
    started, ended = clocks.unix_time(process.get('startedUtc')), clocks.unix_time(process.get('endedUtc'))
    b.require(0 <= started <= ended <= started + RESOURCE['timeoutSeconds'] + 1, 'E_PROJECT_PROCESS_WINDOW')


def run(admission_binding, output):
    admission, rows, clock_rows, history = admit(admission_binding)
    output = Path(output)
    b.require(output.is_absolute() and output.parent == b.OWN and re.fullmatch(r'run-[A-Za-z0-9_.-]+', output.name),
              'E_OUTPUT_OWNERSHIP')
    resume = output.exists()
    if not resume:
        b.mkdir(output)
    b.private_directory(output)
    b.require(set(os.listdir(output)) <= {'started.json'} | {row['id'] for row in rows}, 'E_RESUME_DIRECTORY_CONTENTS')
    initial = dict(kind='execution-only-oss98-started-v1', admission=admission_binding,
                   selectedIds=[row['id'] for row in rows], expectedSamples=1, configuration=RESOURCE)
    if resume:
        b.require(b.document(b.ref(output / 'started.json')) == initial, 'E_RESUME_REQUEST')
        b.require(not (output / 'index.json').exists(), 'E_ALREADY_COMPLETED')
    else:
        b.publish(output / 'started.json', initial)
    oss, external, auditor = b.import_auditors()
    validation, schemas = auditor.validators()
    result_rows = []
    for row in rows:
        b.verify_source(admission['sourceBindings'])
        b.require(b.harness_bindings() == admission['harnessBindings'], 'E_HARNESS_CHANGED')
        root, files, bindings, command = expected_project(admission, admission_binding, row, clock_rows, history, output, schemas)
        inputs, selection, clock = (bindings[name] for name in ('input', 'selection', 'clock'))
        key = b.digest(bindings)
        project = output / row['id']
        if project.exists():
            terminal = b.document(b.ref(project / 'terminal.json'))
            validate_terminal(terminal, project, bindings, command)
            b.require(shutil.disk_usage(project).free >= 4 * 1024**3, 'E_FREE_DISK_4GIB')
            oss._validate_input(root, files)
            unpack_assess(terminal, project / ('resume-check-' + uuid.uuid4().hex), validation)
        else:
            b.mkdir(project)
            b.mkdir(project / 'logs')
            b.mkdir(project / 'compressed')
            b.publish(project / 'started.json', dict(bindings=bindings, executionKey=key))
            if shutil.disk_usage(project).free < 4 * 1024**3:
                b.publish(project / 'terminal.json', dict(kind='execution-only-oss98-terminal-v1', executionKey=key,
                    bindings=bindings, execution='NotRun', process=None, assessment=None, artifacts={},
                    evaluationComplete=False, failure=dict(errorCode='E_FREE_DISK_4GIB', stage='before_launch'), performance=None))
                raise b.Rejected('E_FREE_DISK_4GIB')
            b.read(admission['adapterBinary'])
            b.read(admission['shippingBinary'])
            replay = clock_rows.for_project(row['id'], row['population'] + '-after')
            environment = replay.bind_process(admission['scannerEnvironment'], os.environ)
            out = project / 'output'
            process = spawn(command, b.ROOT, environment, project / 'logs')
            terminal = dict(kind='execution-only-oss98-terminal-v1', executionKey=key, bindings=bindings,
                            execution=process['execution'], process=process, command=command, assessment=None,
                            artifacts={}, evaluationComplete=False, performance=None)
            try:
                b.require(process['execution'] == 'Succeeded', 'E_SCANNER_NOT_SUCCEEDED')
                descriptor = b.approved._open(out, directory=True)
                try:
                    os.fchmod(descriptor, 0o700)
                finally:
                    os.close(descriptor)
                verify_output_clocks(out, bindings['clock'])
                files_out, terminal['assessment'] = assess(out, inputs['limitations'], validation, {item['path']: item for item in files})
                oss._validate_input(root, files)
                b.require(b.ref(inputs['manifest']['path'])['sha256'] == inputs['manifest']['sha256'] and
                          external.selection_bindings(root, cwd=b.ROOT) == selection, 'E_INPUT_AFTER')
                b.verify_source(admission['sourceBindings'])
                b.require(b.harness_bindings() == admission['harnessBindings'], 'E_HARNESS_CHANGED')
                b.read(admission['adapterBinary'])
                b.read(admission['shippingBinary'])
                terminal['artifacts'] = storage.archive_outputs(out, project / 'compressed', files_out)
            except (OSError, ValueError, KeyError, TypeError, AssertionError) as error:
                terminal['execution'] = 'InvalidEvidence' if process['execution'] == 'Succeeded' else process['execution']
                terminal['failure'] = dict(kind=type(error).__name__, errorCode=process['errorCode'] or 'E_EXECUTION_EVIDENCE',
                    reason='Execution or admission or artifact audit failed; payload omitted.', validation=getattr(error, 'evidence', None))
                terminal['availableArtifacts'] = available_artifacts(out)
                try:
                    oss._validate_input(root, files)
                    b.require(b.ref(inputs['manifest']['path'])['sha256'] == inputs['manifest']['sha256'], 'E_INPUT_MANIFEST_AFTER')
                    terminal['inputAfterFailure'] = 'Verified'
                except (OSError, ValueError, KeyError, TypeError):
                    terminal['inputAfterFailure'] = 'Failed'
                try:
                    b.verify_source(admission['sourceBindings'])
                    terminal['sourceAfterFailure'] = 'Verified'
                except (OSError, ValueError, KeyError, TypeError):
                    terminal['sourceAfterFailure'] = 'Failed'
                b.publish(project / 'terminal.json', terminal)
                raise b.Rejected('E_PROJECT_INCOMPLETE_PRESERVED') from None
            validate_terminal(terminal, project, bindings, command)
            b.publish(project / 'terminal.json', terminal)
        result_rows.append(dict(repositoryId=row['id'], population=row['population'], split=row['split'],
                               sample=0, terminal=b.ref(project / 'terminal.json'), execution='Succeeded',
                               inputManifestSha256=row['inputManifestSha256'], commit=row['commit'],
                               artifacts=terminal['artifacts'], assessment=terminal['assessment']))
    index = dict(kind='execution-only-after-repair-artifact-index-v1', schemaVersion=1,
                 admission=admission_binding, started=b.ref(output / 'started.json'), rows=result_rows,
                 expectedProjects=98, primaryProjects=90, comparisonProjects=8, repeats=1,
                 executionComplete=True, evaluationComplete=False, performance=None,
                 remainingRequirements=['Final legacy/native compatibility comparison', 'Qualified existing scorer evaluation',
                                        'Rendered report acceptance', 'Independent integrated assessment'],
                 uncompressedPathsAreLogicalArchiveOrigins=True)
    return b.publish(output / 'index.json', index)


def load_index(binding):
    index = b.document(binding)
    b.require(index['kind'] == 'execution-only-after-repair-artifact-index-v1' and len(index['rows']) == 98 and
              index['executionComplete'] is True and index['expectedProjects'] == 98 and index['repeats'] == 1,
              'E_AFTER_REPAIR_INDEX')
    admission, rows, clock_rows, history = admit(index['admission'])
    output = Path(binding['path']).parent
    b.require(output.parent == b.OWN and re.fullmatch(r'run-[A-Za-z0-9_.-]+', output.name) and
              Path(binding['path']) == output / 'index.json', 'E_INDEX_OWNERSHIP')
    b.private_directory(output)
    initial = dict(kind='execution-only-oss98-started-v1', admission=index['admission'],
                   selectedIds=[row['id'] for row in rows], expectedSamples=1, configuration=RESOURCE)
    b.require(index.get('started') == b.ref(output / 'started.json') and b.document(index['started']) == initial,
              'E_INDEX_STARTED_BINDING')
    _, _, auditor = b.import_auditors()
    _, schemas = auditor.validators()
    b.require([row['repositoryId'] for row in index['rows']] == [row['id'] for row in rows], 'E_AFTER_REPAIR_MATRIX')
    result = {}
    for row, registry_row in zip(index['rows'], rows):
        b.require(row['sample'] == 0 and all(row[name] == registry_row[name] for name in ('population', 'split', 'commit', 'inputManifestSha256')),
                  'E_AFTER_REPAIR_SOURCE')
        project = output / registry_row['id']
        b.require(row['terminal'] == b.ref(project / 'terminal.json'), 'E_INDEX_TERMINAL_PATH')
        _, _, bindings, command = expected_project(admission, index['admission'], registry_row, clock_rows, history, output, schemas)
        terminal = b.document(row['terminal'])
        validate_terminal(terminal, project, bindings, command)
        b.require(row.get('execution') == 'Succeeded' and terminal['execution'] == 'Succeeded' and terminal['artifacts'] == row['artifacts'] and
                  terminal['assessment'] == row['assessment'] and terminal['bindings']['registryEntry'] == registry_row and
                  terminal['bindings']['admission'] == index['admission'] and
                  terminal['executionKey'] == b.digest(terminal['bindings']), 'E_AFTER_REPAIR_TERMINAL')
        storage.validate_matrix(row['artifacts'])
        verify_archived_clocks(terminal)
        result[row['repositoryId']] = row
    return index, result


def admitted_artifact(index_binding, repository_id, name):
    _, rows = load_index(index_binding)
    b.require(repository_id in rows, 'E_AFTER_REPAIR_REPOSITORY')
    row = rows[repository_id]
    value = storage.read_document(row['artifacts'], name)
    item = row['artifacts'][name]
    return dict(repositoryId=repository_id, value=value,
                artifact={key: item[key] for key in ('path', 'sha256', 'bytes')} if 'path' in item else
                dict(path=str(Path(row['terminal']['path']).parent / 'output' / name), sha256=item['sha256'], bytes=item['bytes']),
                compression={key: item[key] for key in ('compressedPath', 'compressedSha256', 'compressedBytes')},
                logicalUncompressedPath=True, inspection=row['assessment']['descriptive']['inspection'])
