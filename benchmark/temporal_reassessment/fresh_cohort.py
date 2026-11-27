"""Finite fresh-source OSS98 attempts, with immutable evidence and complete-only reads."""
import os
import hashlib
from pathlib import Path
import re
import shutil
import stat
import subprocess
import uuid

import adapter
import boundary as b
import clocks
import execution as e
import storage

AUTHOR = 'fresh_cohort_implementation'
AUTHORS = frozenset({AUTHOR, '/root/fresh98_runner_implementation', '/root'})
FIELDS = ('LANG', 'LC_ALL', 'LC_CTYPE', 'TZ', adapter.DETECTOR_ENV, adapter.GENERATED_ENV)
KINDS = {name: 'fresh-cohort-' + name + '-v1' for name in
         ('policy', 'plan', 'plan-review', 'started', 'bindings', 'process', 'terminal', 'attempt', 'index')}


def resources(timeout):
    b.require(type(timeout) is int and timeout in (600, 3600), 'E_COHORT_TIMEOUT')
    return dict(e.RESOURCE, timeoutSeconds=timeout)


def output_owner(output):
    b.require_external_storage()
    output = Path(output)
    b.require(output.is_absolute() and output.parent == b.OWN and
              re.fullmatch(r'run-[A-Za-z0-9_.-]+', output.name), 'E_OUTPUT_OWNERSHIP')
    if output.exists():
        b.private_directory(output)
    return output


def _registry(rows):
    ids = [row['id'] for row in rows]
    b.require(len(ids) == len(set(ids)) == 98 and
              sum(row['population'] == 'primary' for row in rows) == 90 and
              sum(row['population'] == 'comparison' for row in rows) == 8 and
              all(re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_.-]*', identity) and identity not in ('.', '..')
                  for identity in ids), 'E_COHORT_REGISTRY')
    return ids


def _policy(binding, admission_binding, admission, rows):
    policy = b.document(binding)
    ids = _registry(rows)
    allowed = policy.get('allowedFallbackIds')
    b.require(policy.get('kind') == KINDS['policy'] and policy.get('admission') == admission_binding and
              isinstance(policy.get('sourceEpoch'), str) and re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_.-]{0,127}', policy['sourceEpoch']) and
              admission.get('sourceEpoch') == policy['sourceEpoch'] and admission.get('freshCohortEligible') is True,
              'E_COHORT_EPOCH')
    b.require(isinstance(allowed, list) and len(allowed) == len(set(allowed)) and set(allowed) <= set(ids) and
              allowed == [identity for identity in ids if identity in allowed], 'E_COHORT_FALLBACK_IDS')
    numeric_fields = ('initialTimeoutSeconds', 'fallbackTimeoutSeconds', 'maxAttemptsPerProject', 'threads', 'repeats',
                      'minFreeGiB', 'maxFileSizeMiB', 'maxArtifactBytes', 'maxAttemptArtifactBytes')
    b.require(all(type(policy.get(name)) is int for name in numeric_fields) and
              policy.get('initialTimeoutSeconds') == 600 and policy.get('fallbackTimeoutSeconds') == 3600 and
              policy.get('maxAttemptsPerProject') == 2 and policy.get('threads') == 1 and policy.get('repeats') == 1 and
              policy.get('minFreeGiB') == 4 and policy.get('maxFileSizeMiB') == 2 and
              policy.get('maxArtifactBytes') == b.MAX and policy.get('maxAttemptArtifactBytes') == b.TOTAL and
              policy.get('requiredArtifacts') == list(b.FORMATS) and policy.get('performance') is None and
              policy.get('performanceObservations') is False and
              isinstance(policy.get('authorIds'), list) and bool(policy['authorIds']) and
              all(isinstance(value, str) and value for value in policy['authorIds']), 'E_COHORT_POLICY')
    return policy


def _attempt_shape(bindings, command, output, identity, policy, attempt):
    project = output / identity / ('attempt-' + str(attempt))
    command = list(command)
    command[-1] = str(project / 'output')
    bindings = dict(bindings, kind=KINDS['bindings'], sourceEpoch=policy['sourceEpoch'], sample=0,
                    attempt=attempt, configuration=resources(600 if attempt == 0 else 3600))
    return bindings, command


def _project(admission, admission_binding, row, clock_rows, history, output, schemas, policy, attempt):
    root, files, bindings, command = e.expected_project(admission, admission_binding, row, clock_rows, history, output, schemas)
    bindings, command = _attempt_shape(bindings, command, output, row['id'], policy, attempt)
    return root, files, bindings, command


def _expectation(bindings, command):
    return dict(bindingsDigest=b.digest(bindings), commandDigest=b.digest(command))


def _shared(admission, plan_binding=None, review_binding=None, protected=(), active=None):
    if plan_binding is not None:
        plan = b.document(plan_binding)
        b.read(plan['policy'])
        b.read(plan['admission'])
        b.read(review_binding)
    b.verify_source(admission['sourceBindings'])
    b.require(b.harness_bindings() == admission['harnessBindings'], 'E_HARNESS_CHANGED')
    b.require(b.validation_environment() == admission['validationEnvironment'], 'E_VALIDATION_ENVIRONMENT')
    b.read(admission['adapterBinary'])
    b.read(admission['shippingBinary'])
    projects = {}
    for reference, tree in protected:
        parent = Path(reference['path']).parent
        projects.setdefault(parent, set()).update({'attempt-' + str(tree['attempt']), 'attempt-' + str(tree['attempt']) + '.json'})
        b.read(reference)
        b.require(snapshot(Path(reference['path']).parent / ('attempt-' + str(tree['attempt']))) == tree['snapshot'],
                  'E_COHORT_PROTECTED_DRIFT')
    if active is not None:
        projects.setdefault(active.parent, set()).add(active.name)
    for parent, names in projects.items():
        b.private_directory(parent)
        b.require(set(os.listdir(parent)) == names, 'E_COHORT_PROTECTED_PARENT')


def _input(root, files, bindings):
    oss, external, _ = b.import_auditors()
    oss._validate_input(root, files)
    b.read(bindings['input']['manifest'])
    b.require(external.selection_bindings(root, cwd=b.ROOT) == bindings['selection'], 'E_SELECTION_DRIFT')


def prepare_plan(admission_binding, policy_binding, output_plan_path, run_output_root=None):
    admission, rows, clock_rows, history = e.admit(admission_binding)
    policy = _policy(policy_binding, admission_binding, admission, rows)
    output = output_owner(run_output_root or b.OWN / ('run-fresh-' + uuid.uuid4().hex))
    b.require(not output.exists(), 'E_COHORT_OUTPUT_EXISTS')
    _, _, auditor = b.import_auditors()
    _, schemas = auditor.validators()
    tasks = []
    for row in rows:
        attempts = []
        for attempt in range(2 if row['id'] in policy['allowedFallbackIds'] else 1):
            root, files, bindings, command = _project(admission, admission_binding, row, clock_rows, history, output, schemas, policy, attempt)
            _input(root, files, bindings)
            attempts.append(_expectation(bindings, command))
        tasks.append(dict(registryEntry=row, sample=0, attempts=attempts))
    _shared(admission)
    b.read(admission_binding)
    b.read(policy_binding)
    plan = dict(kind=KINDS['plan'], admission=admission_binding, policy=policy_binding,
                sourceEpoch=policy['sourceEpoch'], sourceBindings=admission['sourceBindings'],
                harnessBindings=admission['harnessBindings'], validationEnvironment=admission['validationEnvironment'],
                authorIds=_authors(admission, policy),
                runOutputRoot=str(output), tasks=tasks, expectedProjects=98, primaryProjects=90, comparisonProjects=8,
                maxLaunches=98 + len(policy['allowedFallbackIds']),
                scannerBudgetSeconds=98 * 600 + len(policy['allowedFallbackIds']) * 3600,
                expectedSamples=1, performance=None, performanceObservations=False, evaluationComplete=False)
    return b.publish(Path(output_plan_path), plan)


def _author_aliases(identities):
    return {alias for identity in identities for alias in (identity, identity.rstrip('/').rsplit('/', 1)[-1])}


def _authors(admission, policy):
    return sorted(_author_aliases([*AUTHORS, admission['authorId'], *admission.get('authorIds', []), *policy['authorIds']]))


def _context(plan_binding, review_binding):
    plan = b.document(plan_binding)
    b.require(plan.get('kind') == KINDS['plan'], 'E_COHORT_PLAN')
    admission, rows, clock_rows, history = e.admit(plan['admission'])
    policy = _policy(plan['policy'], plan['admission'], admission, rows)
    ids = _registry(rows)
    b.require(plan.get('sourceEpoch') == policy['sourceEpoch'] and
              all(plan.get(name) == admission[name] for name in ('sourceBindings', 'harnessBindings', 'validationEnvironment')) and
              plan.get('tasks') is not None and [task['registryEntry'] for task in plan['tasks']] == rows and
              all(type(task['sample']) is int and task['sample'] == 0 and len(task['attempts']) == (2 if row['id'] in policy['allowedFallbackIds'] else 1)
                  for task, row in zip(plan['tasks'], rows)) and
              plan.get('expectedProjects') == 98 and plan.get('primaryProjects') == 90 and plan.get('comparisonProjects') == 8 and
              plan.get('expectedSamples') == 1 and plan.get('maxLaunches') == 98 + len(policy['allowedFallbackIds']) and
              plan.get('scannerBudgetSeconds') == 98 * 600 + len(policy['allowedFallbackIds']) * 3600 and
              plan.get('performance') is None and plan.get('performanceObservations') is False and
              plan.get('evaluationComplete') is False, 'E_COHORT_PLAN_BINDING')
    authors = _authors(admission, policy)
    b.require(plan.get('authorIds') == authors, 'E_COHORT_AUTHORS')
    review = b.document(review_binding)
    b.require(review.get('kind') == KINDS['plan-review'] and review.get('verdict') == 'PASS' and
              review.get('reviewerModel') == 'codex:gpt-6.1-sol@high' and
              isinstance(review.get('reviewerId'), str) and bool(review['reviewerId']) and
              not (_author_aliases([review['reviewerId']]) & _author_aliases([*authors, *clocks.AUTHORS])) and review.get('plan') == plan_binding and
              all(review.get(name) == plan[name] for name in ('sourceBindings', 'harnessBindings', 'validationEnvironment')) and
              all(review.get(name) == 'PASS' for name in ('implementationCorrectness', 'testAdequacy', 'evidenceAuthenticity', 'resourceDifferences')),
              'E_COHORT_REVIEW')
    output = output_owner(plan['runOutputRoot'])
    _, _, auditor = b.import_auditors()
    _, schemas = auditor.validators()
    for row, task in zip(rows, plan['tasks']):
        root, files, base, command = _project(admission, plan['admission'], row, clock_rows, history, output, schemas, policy, 0)
        _input(root, files, base)
        for attempt, expectation in enumerate(task['attempts']):
            bindings, attempt_command = _attempt_shape(base, command, output, row['id'], policy, attempt)
            b.require(_expectation(bindings, attempt_command) == expectation, 'E_COHORT_EXPECTATION_DRIFT')
    _shared(admission, plan_binding, review_binding)
    return plan, admission, rows, clock_rows, history, policy


def bounded_spawn(command, cwd, environment, logs, configuration):
    b.require(b.digest(configuration) == b.digest(resources(configuration['timeoutSeconds'])) and isinstance(command, list) and bool(command) and
              all(isinstance(arg, str) for arg in command), 'E_COHORT_COMMAND')
    b.private_directory(logs)
    started = e.now()
    try:
        process = subprocess.run(command, cwd=cwd, env=environment, stdin=subprocess.DEVNULL,
                                 stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
                                 timeout=configuration['timeoutSeconds'], shell=False, start_new_session=True, check=False)
        state, code = ('Succeeded' if process.returncode == 0 else 'Failed'), process.returncode
        error, kind = (None if code == 0 else 'E_SCANNER_EXIT_NONZERO'), None
    except subprocess.TimeoutExpired:
        state, code, error, kind = 'TimedOut', None, 'E_SCANNER_TIMEOUT', 'TimeoutExpired'
    except OSError as exception:
        state, code, error, kind = 'NotRun', None, 'E_SCANNER_LAUNCH', type(exception).__name__
    return dict(kind=KINDS['process'], startedUtc=started, endedUtc=e.now(), execution=state,
                commandDigest=b.digest(command), cwdDigest=b.digest(str(cwd)),
                environmentDigest=b.digest({name: environment.get(name) for name in FIELDS}),
                exitCode=code, timedOut=state == 'TimedOut', errorCode=error, errorKind=kind,
                measurement=None, resources=configuration, stdout=dict(state='NotCaptured', destination='DEVNULL'),
                stderr=dict(state='NotCaptured', destination='DEVNULL'))


def validate_process(process, command, environment, configuration):
    state = process.get('execution')
    code = process.get('exitCode')
    valid = ((state == 'Succeeded' and type(code) is int and code == 0 and process.get('errorCode') is None and process.get('errorKind') is None) or
             (state == 'Failed' and type(code) is int and code != 0 and process.get('errorCode') == 'E_SCANNER_EXIT_NONZERO' and process.get('errorKind') is None) or
             (state == 'TimedOut' and code is None and process.get('errorCode') == 'E_SCANNER_TIMEOUT' and process.get('errorKind') == 'TimeoutExpired') or
             (state == 'NotRun' and code is None and process.get('errorCode') == 'E_SCANNER_LAUNCH' and
              process.get('errorKind') in ('OSError', 'FileNotFoundError', 'PermissionError', 'NotADirectoryError', 'IsADirectoryError')))
    b.require(valid and process.get('kind') == KINDS['process'] and process.get('timedOut') is (state == 'TimedOut') and
              b.digest(process.get('resources')) == b.digest(configuration) and process.get('measurement') is None and
              process.get('commandDigest') == b.digest(command) and process.get('cwdDigest') == b.digest(str(b.ROOT)) and
              process.get('environmentDigest') == b.digest({name: environment.get(name) for name in FIELDS}) and
              all(process.get(name) == dict(state='NotCaptured', destination='DEVNULL') for name in ('stdout', 'stderr')), 'E_COHORT_PROCESS')
    started, ended = clocks.unix_time(process.get('startedUtc')), clocks.unix_time(process.get('endedUtc'))
    b.require(0 <= started <= ended <= started + configuration['timeoutSeconds'] + 1, 'E_COHORT_PROCESS_WINDOW')


def snapshot(path):
    result, total_bytes = {}, 0
    def walk(current, relative):
        nonlocal total_bytes
        fd = b.approved._open(current, directory=True)
        try:
            info = info_dir = os.fstat(fd)
            b.require(stat.S_IMODE(info.st_mode) == 0o700, 'E_PRIVATE_DIRECTORY')
            result[relative] = dict(kind='directory', device=info.st_dev, inode=info.st_ino, mode=info.st_mode,
                                    size=info.st_size, uid=info.st_uid, gid=info.st_gid, links=info.st_nlink,
                                    mtimeNs=info.st_mtime_ns, ctimeNs=info.st_ctime_ns)
            names = sorted(os.listdir(fd))
            b.require(len(names) <= 64 and len(result) <= 256, 'E_COHORT_TREE_BOUND')
            for name in names:
                child = current / name
                info = os.stat(name, dir_fd=fd, follow_symlinks=False)
                key = relative + '/' + name
                if stat.S_ISDIR(info.st_mode):
                    walk(child, key)
                else:
                    b.require(stat.S_ISREG(info.st_mode) and stat.S_IMODE(info.st_mode) == 0o600 and info.st_nlink == 1,
                              'E_COHORT_PROTECTED_FILE')
                    file_fd = b.approved._open(child)
                    try:
                        opened = os.fstat(file_fd)
                        b.require((opened.st_dev, opened.st_ino, opened.st_mode, opened.st_size, opened.st_mtime_ns, opened.st_ctime_ns) ==
                                  (info.st_dev, info.st_ino, info.st_mode, info.st_size, info.st_mtime_ns, info.st_ctime_ns) and
                                  info.st_size <= b.MAX, 'E_COHORT_PROTECTED_FILE')
                        digest, total = hashlib.sha256(), 0
                        while True:
                            chunk = os.read(file_fd, 1024 * 1024)
                            if not chunk:
                                break
                            total += len(chunk)
                            b.require(total <= b.MAX, 'E_COHORT_TREE_BOUND')
                            digest.update(chunk)
                        total_bytes += total
                        b.require(total_bytes <= b.TOTAL + 2 * b.approved.MAX_METADATA, 'E_COHORT_TREE_BOUND')
                        after = os.fstat(file_fd)
                        b.require((after.st_dev, after.st_ino, after.st_mode, after.st_size, after.st_mtime_ns, after.st_ctime_ns) ==
                                  (info.st_dev, info.st_ino, info.st_mode, info.st_size, info.st_mtime_ns, info.st_ctime_ns), 'E_COHORT_PROTECTED_DRIFT')
                        result[key] = dict(kind='file', binding=dict(path=str(child), bytes=total, sha256=digest.hexdigest()),
                                           device=info.st_dev, inode=info.st_ino, mode=info.st_mode, size=info.st_size,
                                           uid=info.st_uid, gid=info.st_gid, links=info.st_nlink, mtimeNs=info.st_mtime_ns, ctimeNs=info.st_ctime_ns)
                    finally:
                        os.close(file_fd)
            after = os.fstat(fd)
            b.require((after.st_dev, after.st_ino, after.st_mode, after.st_uid, after.st_gid, after.st_nlink, after.st_size, after.st_mtime_ns, after.st_ctime_ns) ==
                      (info_dir.st_dev, info_dir.st_ino, info_dir.st_mode, info_dir.st_uid, info_dir.st_gid, info_dir.st_nlink, info_dir.st_size, info_dir.st_mtime_ns, info_dir.st_ctime_ns), 'E_COHORT_PROTECTED_DRIFT')
        finally:
            os.close(fd)
    walk(Path(path), '.')
    return result


def _private_output(output):
    if not output.exists():
        return
    fd = b.approved._open(output, directory=True)
    try:
        names = os.listdir(fd)
        b.require(len(names) <= 32, 'E_COHORT_TREE_BOUND')
        os.fchmod(fd, 0o700)
        total = 0
        for name in names:
            reference = b.ref(output / name, min(b.MAX, b.TOTAL - total))
            total += reference['bytes']
            file_fd = b.approved._open(output / name)
            try:
                b.require(os.fstat(file_fd).st_nlink == 1, 'E_COHORT_PROTECTED_FILE')
                os.fchmod(file_fd, 0o600)
            finally:
                os.close(file_fd)
    finally:
        os.close(fd)


def _archive_owner(terminal, project):
    storage.validate_matrix(terminal['artifacts'])
    for name, item in terminal['artifacts'].items():
        b.require(Path(item['compressedPath']) == project / 'compressed' / (name + '.gz'), 'E_COHORT_ARCHIVE_OWNER')
        if 'path' in item:
            b.require(Path(item['path']) == project / 'output' / name, 'E_COHORT_ARCHIVE_ORIGIN')


def validate_attempt(marker_binding, project, bindings, command, environment, plan_binding, validation):
    marker = b.document(marker_binding)
    attempt = bindings['attempt']
    b.require(Path(marker_binding['path']) == project.parent / ('attempt-' + str(attempt) + '.json') and
              marker.get('kind') == KINDS['attempt'] and marker.get('attempt') == attempt and
              marker.get('plan') == plan_binding and marker.get('snapshot') == snapshot(project), 'E_COHORT_ATTEMPT')
    started = marker['started']
    terminal_binding = marker['terminal']
    b.require(Path(started['path']) == project / 'started.json' and Path(terminal_binding['path']) == project / 'terminal.json',
              'E_COHORT_ATTEMPT_OWNER')
    expected = dict(kind=KINDS['started'], plan=plan_binding, bindings=bindings, command=command, executionKey=b.digest(bindings))
    b.require(b.digest(b.document(started)) == b.digest(expected), 'E_COHORT_STARTED_BINDING')
    terminal = b.document(terminal_binding)
    b.require(terminal.get('kind') == KINDS['terminal'] and terminal.get('started') == started and
              b.digest(terminal.get('bindings')) == b.digest(bindings) and terminal.get('command') == command and
              terminal.get('executionKey') == b.digest(bindings) and terminal.get('evaluationComplete') is False and
              terminal.get('performance') is None and terminal.get('sharedInvariants') == 'Passed', 'E_COHORT_TERMINAL')
    validate_process(terminal['process'], command, environment, bindings['configuration'])
    state = terminal['execution']
    if state == 'Succeeded':
        b.require(terminal['process']['execution'] == 'Succeeded' and terminal.get('failure') is None and
                  isinstance(terminal.get('assessment'), dict), 'E_COHORT_SUCCESS')
        _archive_owner(terminal, project)
        e.verify_archived_clocks(terminal)
        e.unpack_assess(terminal, project.parent / ('verify-' + uuid.uuid4().hex), validation)
    else:
        b.require(state == terminal['process']['execution'] and state in ('TimedOut', 'Failed', 'NotRun') or
                  state == 'InvalidEvidence' and terminal['process']['execution'] == 'Succeeded', 'E_COHORT_FAILURE_STATE')
        b.require(terminal.get('artifacts') == {} and terminal.get('assessment') is None and
                  isinstance(terminal.get('failure'), dict), 'E_COHORT_FAILURE')
        if 'availableArtifacts' in terminal:
            b.require(terminal['availableArtifacts'] == e.available_artifacts(project / 'output'), 'E_COHORT_FAILED_OUTPUT')
    return marker, terminal


def _launch(project, bindings, command, environment, plan_binding, review_binding, admission, root, files, protected, validation):
    b.mkdir(project)
    b.mkdir(project / 'logs')
    b.mkdir(project / 'compressed')
    started = b.publish(project / 'started.json', dict(kind=KINDS['started'], plan=plan_binding, bindings=bindings,
                        command=command, executionKey=b.digest(bindings)))
    _shared(admission, plan_binding, review_binding, protected, active=project)
    _input(root, files, bindings)
    b.require(shutil.disk_usage(project).free >= 4 * 1024**3, 'E_FREE_DISK_4GIB')
    process = bounded_spawn(command, b.ROOT, environment, project / 'logs', bindings['configuration'])
    terminal = dict(kind=KINDS['terminal'], started=started, executionKey=b.digest(bindings), bindings=bindings,
                    command=command, execution=process['execution'], process=process, assessment=None, artifacts={},
                    performance=None, evaluationComplete=False, sharedInvariants='Passed', failure=None)
    output = project / 'output'
    expected = None
    if process['execution'] == 'Succeeded':
        try:
            _private_output(output)
            e.verify_output_clocks(output, bindings['clock'])
            expected, terminal['assessment'] = e.assess(output, bindings['input']['limitations'], validation,
                                                       {item['path']: item for item in files})
        except (OSError, ValueError, KeyError, TypeError, AssertionError) as error:
            terminal.update(execution='InvalidEvidence', assessment=None,
                            failure=dict(errorCode='E_COHORT_ARTIFACT_REJECTED', errorKind=type(error).__name__,
                                         validation=getattr(error, 'evidence', None), payloadOmitted=True))
    else:
        terminal['failure'] = dict(errorCode=process['errorCode'], errorKind=process['errorKind'], payloadOmitted=True)
    try:
        _shared(admission, plan_binding, review_binding, protected, active=project)
        _input(root, files, bindings)
        b.read(started)
        _private_output(output)
        validate_process(process, command, environment, bindings['configuration'])
        b.require(shutil.disk_usage(project).free >= 4 * 1024**3, 'E_FREE_DISK_4GIB')
    except (OSError, ValueError, KeyError, TypeError, AssertionError):
        terminal['sharedInvariants'] = 'Failed'
        terminal['availableArtifacts'] = e.available_artifacts(output)
        b.publish(project / 'terminal.json', terminal)
        raise b.Rejected('E_COHORT_GLOBAL_STOP_PRESERVED') from None
    if terminal['execution'] == 'Succeeded':
        terminal['artifacts'] = storage.archive_outputs(output, project / 'compressed', expected)
        _archive_owner(terminal, project)
    else:
        terminal['availableArtifacts'] = e.available_artifacts(output)
    terminal_binding = b.publish(project / 'terminal.json', terminal)
    marker = dict(kind=KINDS['attempt'], plan=plan_binding, attempt=bindings['attempt'], started=started,
                  terminal=terminal_binding, snapshot=snapshot(project))
    marker_binding = b.publish(project.parent / ('attempt-' + str(bindings['attempt']) + '.json'), marker)
    return marker_binding, marker, terminal


def _row(row, attempts, terminals):
    selected = [index for index, terminal in enumerate(terminals) if terminal['execution'] == 'Succeeded']
    b.require(len(selected) <= 1, 'E_COHORT_DOUBLE_SELECTION')
    terminal = terminals[selected[0]] if selected else None
    return dict(repositoryId=row['id'], population=row['population'], split=row['split'], sample=0,
                commit=row['commit'], inputManifestSha256=row['inputManifestSha256'], attempts=attempts,
                selectedAttempt=selected[0] if selected else None,
                terminal=attempts[selected[0]]['terminal'] if selected else None,
                execution='Succeeded' if selected else terminals[-1]['execution'] if terminals else 'NotRun',
                originalBudgetSatisfied=bool(terminals and terminals[0]['execution'] == 'Succeeded'),
                artifacts=terminal['artifacts'] if terminal else None, assessment=terminal['assessment'] if terminal else None)


def _index(plan_binding, review_binding, started, rows):
    complete = all(row['attempts'] for row in rows)
    return dict(kind=KINDS['index'], schemaVersion=1, plan=plan_binding, review=review_binding, started=started,
                rows=rows, expectedProjects=98, primaryProjects=90, comparisonProjects=8, repeats=1,
                executionComplete=complete, artifactCoverageComplete=all(row['selectedAttempt'] is not None for row in rows),
                evaluationComplete=False, performance=None, performanceObservations=False,
                uncompressedPathsAreLogicalArchiveOrigins=True)


def run(plan_binding, review_binding, output=None):
    context = _context(plan_binding, review_binding)
    plan, admission, rows, clock_rows, history, policy = context
    owner = output_owner(plan['runOutputRoot'])
    b.require(output is None or Path(output) == owner, 'E_COHORT_OUTPUT_BINDING')
    if (owner / 'index.json').exists():
        reference = b.ref(owner / 'index.json')
        existing = b.document(reference)
        b.require(existing.get('plan') == plan_binding and existing.get('review') == review_binding, 'E_COHORT_RESUME')
        index, _ = load_progress(reference)
        return reference, 0 if index['artifactCoverageComplete'] else 1
    if not owner.exists():
        b.mkdir(owner)
    initial = dict(kind=KINDS['started'], plan=plan_binding, review=review_binding, selectedIds=_registry(rows))
    started_path = owner / 'started.json'
    if started_path.exists():
        b.require(b.document(b.ref(started_path)) == initial, 'E_COHORT_RESUME')
        started = b.ref(started_path)
    else:
        b.require(not os.listdir(owner), 'E_COHORT_RESUME')
        started = b.publish(started_path, initial)
    ids = _registry(rows)
    b.require(set(os.listdir(owner)) <= {'started.json'} | set(ids), 'E_COHORT_OWNER_CONTENTS')
    existing = [identity for identity in ids if (owner / identity).exists()]
    b.require(existing == ids[:len(existing)], 'E_COHORT_RESUME_ORDER')
    _, _, auditor = b.import_auditors()
    validation, schemas = auditor.validators()
    protected, result = [], []
    for row, task in zip(rows, plan['tasks']):
        _shared(admission, plan_binding, review_binding, protected)
        b.read(started)
        project = owner / row['id']
        if not project.exists():
            b.mkdir(project)
        b.private_directory(project)
        allowed_names = {name for number in range(len(task['attempts'])) for name in ('attempt-' + str(number), 'attempt-' + str(number) + '.json')}
        b.require(set(os.listdir(project)) <= allowed_names, 'E_COHORT_PROJECT_CONTENTS')
        attempts, terminals = [], []
        for number, expectation in enumerate(task['attempts']):
            if number and (not terminals or terminals[0]['execution'] != 'TimedOut'):
                b.require(not (project / 'attempt-1').exists() and not (project / 'attempt-1.json').exists(), 'E_COHORT_UNAUTHORIZED_FALLBACK')
                break
            root, files, bindings, command = _project(admission, plan['admission'], row, clock_rows, history, owner, schemas, policy, number)
            b.require(_expectation(bindings, command) == expectation, 'E_COHORT_EXPECTATION_DRIFT')
            _input(root, files, bindings)
            environment = clock_rows.for_project(row['id'], row['population'] + '-after').bind_process(admission['scannerEnvironment'], os.environ)
            attempt_path = project / ('attempt-' + str(number))
            marker_path = project / ('attempt-' + str(number) + '.json')
            if marker_path.exists():
                reference = b.ref(marker_path)
                marker, terminal = validate_attempt(reference, attempt_path, bindings, command, environment, plan_binding, validation)
            else:
                b.require(not attempt_path.exists(), 'E_COHORT_STALE_STARTED_REQUIRES_INSPECTION')
                reference, marker, terminal = _launch(attempt_path, bindings, command, environment, plan_binding,
                                                      review_binding, admission, root, files, protected, validation)
            protected.append((reference, marker))
            attempts.append(dict(record=reference, terminal=marker['terminal']))
            terminals.append(terminal)
            if terminal['execution'] == 'Succeeded':
                break
        expected_names = {name for number in range(len(attempts)) for name in ('attempt-' + str(number), 'attempt-' + str(number) + '.json')}
        b.require(set(os.listdir(project)) == expected_names, 'E_COHORT_PROJECT_CONTENTS')
        result.append(_row(row, attempts, terminals))
    _shared(admission, plan_binding, review_binding, protected)
    b.read(started)
    reference = b.publish(owner / 'index.json', _index(plan_binding, review_binding, started, result))
    return reference, 0 if all(row['selectedAttempt'] is not None for row in result) else 1


def load_progress(binding):
    index = b.document(binding)
    b.require(index.get('kind') == KINDS['index'], 'E_COHORT_INDEX')
    plan, admission, rows, clock_rows, history, policy = _context(index['plan'], index['review'])
    owner = output_owner(plan['runOutputRoot'])
    b.require(Path(binding['path']) == owner / 'index.json', 'E_COHORT_INDEX_OWNER')
    b.require(b.document(index['started']) == dict(kind=KINDS['started'], plan=index['plan'], review=index['review'], selectedIds=_registry(rows)) and
              index['started'] == b.ref(owner / 'started.json'), 'E_COHORT_INDEX_STARTED')
    b.require(set(os.listdir(owner)) == {'started.json', 'index.json'} | set(_registry(rows)), 'E_COHORT_OWNER_CONTENTS')
    b.require(len(index['rows']) == 98 and [row['repositoryId'] for row in index['rows']] == _registry(rows), 'E_COHORT_INDEX_MATRIX')
    _, _, auditor = b.import_auditors()
    validation, schemas = auditor.validators()
    reconstructed, protected = [], []
    for row, task, published in zip(rows, plan['tasks'], index['rows']):
        attempts, terminals = [], []
        b.private_directory(owner / row['id'])
        expected_names = {name for number in range(len(published['attempts'])) for name in ('attempt-' + str(number), 'attempt-' + str(number) + '.json')}
        b.require(set(os.listdir(owner / row['id'])) == expected_names, 'E_COHORT_PROJECT_CONTENTS')
        b.require(isinstance(published['attempts'], list) and 1 <= len(published['attempts']) <= len(task['attempts']), 'E_COHORT_ATTEMPT_COUNT')
        for number, item in enumerate(published['attempts']):
            b.require(not number or terminals[0]['execution'] == 'TimedOut', 'E_COHORT_UNAUTHORIZED_FALLBACK')
            root, files, bindings, command = _project(admission, plan['admission'], row, clock_rows, history, owner, schemas, policy, number)
            b.require(_expectation(bindings, command) == task['attempts'][number], 'E_COHORT_EXPECTATION_DRIFT')
            _input(root, files, bindings)
            environment = clock_rows.for_project(row['id'], row['population'] + '-after').bind_process(admission['scannerEnvironment'], os.environ)
            marker, terminal = validate_attempt(item['record'], owner / row['id'] / ('attempt-' + str(number)), bindings,
                                                 command, environment, index['plan'], validation)
            b.require(item == dict(record=item['record'], terminal=marker['terminal']), 'E_COHORT_ATTEMPT_TRANSPORT')
            protected.append((item['record'], marker))
            attempts.append(item)
            terminals.append(terminal)
        b.require(not (terminals[-1]['execution'] == 'TimedOut' and len(attempts) < len(task['attempts'])), 'E_COHORT_UNFINISHED_FALLBACK')
        reconstructed.append(_row(row, attempts, terminals))
    b.require(b.digest(index) == b.digest(_index(index['plan'], index['review'], index['started'], reconstructed)), 'E_COHORT_INDEX_CONTENTS')
    _shared(admission, index['plan'], index['review'], protected)
    return index, {row['repositoryId']: row for row in reconstructed}


def load_index(binding):
    index, rows = load_progress(binding)
    b.require(index['executionComplete'] is True and index['artifactCoverageComplete'] is True, 'E_COHORT_PARTIAL_INDEX')
    return index, rows
