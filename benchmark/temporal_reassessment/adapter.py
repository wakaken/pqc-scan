import difflib
import hashlib
import json
import os
import re
import stat
import struct
import sys
from pathlib import Path

import boundary as b
import capture_rustc as capture

DETECTOR_ENV = 'PQC_SCAN_OSS_REPLAY_DETECTOR_SECOND'
GENERATED_ENV = 'PQC_SCAN_OSS_REPLAY_GENERATED_AT'
CHANGED_CRATES = ('pqc_scan_cli', 'pqc_scan_core', 'pqc_scan_detectors')
CRATE_NAMES = dict(pqc_scan_cli='pqc_scan', pqc_scan_core='pqc_scan_core', pqc_scan_detectors='pqc_scan_detectors')
DETECTOR_PATH = 'crates/pqc_scan_detectors/src/certificate_detector.rs'
PIPELINE_PATH = 'crates/pqc_scan_core/src/pipeline.rs'
DETECTOR_BLOCK = '''{
            let text = std::env::var("PQC_SCAN_OSS_REPLAY_DETECTOR_SECOND")
                .unwrap_or_else(|_| panic!("E_REPLAY_DETECTOR_DECODE"));
            let second: i64 = text.parse().unwrap_or_else(|_| panic!("E_REPLAY_DETECTOR_DECODE"));
            assert!(second >= 0 && second.to_string() == text, "E_REPLAY_DETECTOR_DECODE");
            Some(second)
        }'''
GENERATED_BLOCK = '''{
            let text = std::env::var("PQC_SCAN_OSS_REPLAY_GENERATED_AT")
                .unwrap_or_else(|_| panic!("E_REPLAY_GENERATED_DECODE"));
            chrono::DateTime::parse_from_rfc3339(&text)
                .unwrap_or_else(|_| panic!("E_REPLAY_GENERATED_DECODE"))
                .with_timezone(&Utc)
        }'''
REPLACEMENTS = {
    DETECTOR_PATH: (b'pub(crate) fn clock_snapshot() -> impl Detector {\n    ClockedCertificateDetector {\n        assessment_time: ', b'unix_seconds(SystemTime::now())', DETECTOR_BLOCK.encode()),
    PIPELINE_PATH: (b'        generated_at: ', b'Utc::now()', GENERATED_BLOCK.encode()),
}


def directory_binding(path):
    path = Path(path)
    descriptor = b.approved._open(path, directory=True)
    os.close(descriptor)
    files = {}
    for item in sorted(path.rglob('*')):
        if item.is_file() or item.is_symlink():
            files[item.relative_to(path).as_posix()] = b.ref(item)
        elif not item.is_dir():
            raise b.Rejected('E_RUSTC_SEARCH_DIRECTORY_TYPE')
    return dict(path=str(path), files=files)


def overlay_bytes(relative, raw):
    if relative not in REPLACEMENTS:
        return raw, None
    prefix, old, new = REPLACEMENTS[relative]
    needle = prefix + old
    b.require(raw.count(needle) == 1, 'E_OVERLAY_SOURCE_SPAN')
    start = raw.index(needle) + len(prefix)
    result = raw[:start] + new + raw[start + len(old):]
    b.require(result[:start] == raw[:start] and result[start + len(new):] == raw[start + len(old):], 'E_OVERLAY_BYTE_PROOF')
    return result, dict(path=relative, offset=start, removed=old.decode(), inserted=new.decode(),
                        originalSHA256=hashlib.sha256(raw).hexdigest(), overlaySHA256=hashlib.sha256(result).hexdigest())


def dep_info_inputs(binding, cwd):
    try:
        text = b.read(binding).decode('utf-8').replace('\\\n', '')
    except UnicodeError:
        raise b.Rejected('E_COMPILER_DEP_INFO') from None
    result = set()
    for line in text.splitlines():
        if ': ' not in line or line.startswith('#'):
            continue
        rest = line.split(': ', 1)[1]
        words, word, escaped = [], '', False
        for char in rest:
            if escaped:
                word += char
                escaped = False
            elif char == '\\':
                escaped = True
            elif char.isspace():
                if word:
                    words.append(word)
                    word = ''
            else:
                word += char
        b.require(not escaped, 'E_COMPILER_DEP_INFO')
        if word:
            words.append(word)
        for name in words:
            path = Path(name)
            try:
                actual = (path if path.is_absolute() else Path(cwd) / path).resolve(strict=True)
            except (OSError, RuntimeError, ValueError):
                raise b.Rejected('E_COMPILER_DEP_INFO_PATH') from None
            result.add(str(actual))
    b.require(result, 'E_COMPILER_DEP_INFO')
    return result


def verify_compiled_inputs(row, source, overlay):
    inputs = row['sourceInputs']
    b.require(isinstance(inputs, list) and inputs and len({item['path'] for item in inputs}) == len(inputs) and
              {item['path'] for item in inputs} == dep_info_inputs(row['depInfo'], row['cwd']), 'E_COMPILER_DEP_INFO_CLOSURE')
    generated = {item['path']: item for item in row['generatedInputs']}
    for item in inputs:
        b.read(item)
        path = Path(item['path'])
        if path.is_relative_to(overlay):
            relative = path.relative_to(overlay).as_posix()
            b.require(source.get(relative) == item['sha256'], 'E_COMPILER_SOURCE_INPUT')
        else:
            b.require(generated.get(item['path']) == item, 'E_COMPILER_GENERATED_INPUT')


def validate_invocations(rows, compiler, source, *, overlay=None):
    b.require(isinstance(rows, list) and {row.get('crate') for row in rows} == set(CHANGED_CRATES) and
              len(rows) == len(CHANGED_CRATES), 'E_COMPILER_CRATE_COVERAGE')
    for row in rows:
        argv = row['argv']
        b.require(isinstance(argv, list) and all(isinstance(arg, str) and '\0' not in arg for arg in argv) and
                  argv and argv[0] == compiler['path'] and '--test' not in argv and
                  not any(arg.startswith('incremental=') for arg in argv), 'E_COMPILER_ARGV')
        cfgs = [argv[i + 1] for i, value in enumerate(argv[:-1]) if value == '--cfg'] + [value.split('=', 1)[1] for value in argv if value.startswith('--cfg=')]
        b.require(all(not value.startswith('feature=') or value == 'feature=\"default\"' for value in cfgs) and
                  not any(value in {'clippy', 'cargo-clippy'} for value in cfgs), 'E_COMPILER_FEATURES')
        b.require(row['sourceBindings'] == source and row['features'] == [] and
                  '--crate-name' in argv and argv[argv.index('--crate-name') + 1] == CRATE_NAMES[row['crate']] and
                  len([arg for arg in argv if arg.startswith('--emit=')]) == 1 and
                  next(arg for arg in argv if arg.startswith('--emit=')).split('=', 1)[1] in
                  {'link', 'dep-info,link', 'dep-info,metadata,link'}, 'E_COMPILER_SOURCE')
        environment = row['environment']
        b.require(isinstance(environment, dict) and all(isinstance(k, str) and isinstance(v, str) and '\0' not in k + v for k, v in environment.items()) and
                  all(k.startswith('CARGO_') or k in {'PATH', 'DYLD_FALLBACK_LIBRARY_PATH', 'LD_LIBRARY_PATH', 'RUSTUP_TOOLCHAIN', 'RUSTUP_HOME', 'RUSTC_BOOTSTRAP', 'OUT_DIR', 'RUSTC', 'RUSTDOC', 'HOST', 'TARGET', 'OPT_LEVEL', 'PROFILE', 'DEBUG', 'NUM_JOBS'} for k in environment) and
                  not any(k.startswith('CARGO_FEATURE_') or k in {DETECTOR_ENV, GENERATED_ENV} for k in environment), 'E_COMPILER_ENVIRONMENT')
        expected_manifest = (Path(overlay) if overlay else b.ROOT) / 'crates' / row['crate']
        b.require(environment.get('CARGO_MANIFEST_DIR') == str(expected_manifest), 'E_COMPILER_MANIFEST')
        expected_main = expected_manifest / 'src' / ('main.rs' if row['crate'] == 'pqc_scan_cli' else 'lib.rs')
        cwd = Path(row['cwd'])
        sources = [arg for arg in argv[1:] if arg.endswith('.rs')]
        b.require(len(sources) == 1 and (cwd / sources[0]).resolve() == expected_main.resolve(), 'E_COMPILER_ENTRY')
        artifacts = row['dependencyArtifacts']
        b.require(isinstance(artifacts, list) and artifacts, 'E_COMPILER_DEPENDENCIES')
        for binding in artifacts + row['generatedInputs']:
            b.read(binding)
        paths = {item['path'] for item in artifacts}
        externs = [argv[i + 1] for i, arg in enumerate(argv[:-1]) if arg == '--extern']
        b.require(externs and all('=' in value and value.split('=', 1)[1] in paths for value in externs), 'E_COMPILER_EXTERNS')
        searches = [argv[i + 1] for i, arg in enumerate(argv[:-1]) if arg == '-L']
        b.require(all('=' in value for value in searches) and {item['path'] for item in row['linkSearchDirectories']} ==
                  {value.split('=', 1)[1] for value in searches}, 'E_COMPILER_SEARCH_PATHS')
        for item in row['linkSearchDirectories']:
            b.require(directory_binding(item['path']) == item, 'E_COMPILER_SEARCH_CHANGED')
        if overlay:
            b.require(type(row['exitCode']) is int and row['exitCode'] == 0 and row['outputs'] and row['depInfo'], 'E_COMPILER_RESULT')
            for item in row['outputs'] + [row['depInfo'], row['stdout'], row['stderr']]:
                b.read(item)
            verify_compiled_inputs(row, source, Path(overlay))


def cargo_command(envelope):
    return [envelope['cargo']['path'], 'build', '--locked', '--offline', '--profile', 'dev', '-p', 'pqc_scan_cli', '--bin', 'pqc-scan', '--message-format=json-render-diagnostics', '--target-dir', envelope['exclusiveTarget']]


PROFILE_KEYS = ('CARGO_PROFILE_DEV_DEBUG', 'CARGO_PROFILE_TEST_DEBUG')


def cargo_environment(envelope):
    environment = envelope['environment']
    allowed = {'PATH', 'HOME', 'CARGO_HOME', 'RUSTUP_HOME', 'RUSTUP_TOOLCHAIN', 'TMPDIR', 'LANG', 'LC_ALL', 'TZ',
               'CARGO_INCREMENTAL', 'CARGO_TARGET_DIR', *PROFILE_KEYS}
    b.require(isinstance(environment, dict) and all(k in allowed and isinstance(v, str) and '\0' not in v for k, v in environment.items()) and
              environment.get('CARGO_INCREMENTAL') == '0' and environment.get('CARGO_TARGET_DIR') == envelope['exclusiveTarget'] and
              all(environment.get(key) == '0' for key in PROFILE_KEYS), 'E_CARGO_ENVIRONMENT')
    if 'actualShippingEnvironment' in envelope:
        b.require(all(envelope['actualShippingEnvironment'].get(key) == environment[key] for key in PROFILE_KEYS), 'E_CARGO_PROFILE_MISMATCH')
    return dict(environment)


def capture_configuration(envelope, destination, overlay, overlay_source, script):
    return dict(kind='temporal-rustc-capture-configuration-v1', script=script, compiler=envelope['compiler'],
                recordsDirectory=str(Path(destination) / 'compiler-records'), storageRoot=str(b.OWN), productRoot=str(b.ROOT),
                overlayRoot=str(overlay), sourceBindings=overlay_source,
                manifestDirectories=sorted(str(Path(overlay) / Path(name).parent) for name in overlay_source
                                           if name.startswith('crates/') and name.endswith('/Cargo.toml')),
                profileEnvironment={key: envelope['environment'][key] for key in PROFILE_KEYS})


def capture_script_bytes(source, interpreter):
    b.require(not any(char.isspace() for char in interpreter['path']), 'E_CAPTURE_INTERPRETER_PATH')
    return ('#!' + interpreter['path'] + ' -I\n').encode() + b.read(source)


def build_capture(envelope, destination, overlay, overlay_source):
    source = b.ref(b.CODE_DIR / 'capture_rustc.py')
    interpreter = b.ref(Path(sys.executable).resolve())
    script_path = destination / 'capture-rustc'
    with b.create(script_path) as stream:
        stream.write(capture_script_bytes(source, interpreter))
        os.fchmod(stream.fileno(), 0o500)
    script = b.ref(script_path)
    records = b.mkdir(destination / 'compiler-records')
    configuration = b.publish(destination / 'capture-config.json',
                              capture_configuration(envelope, destination, overlay, overlay_source, script))
    return dict(source=source, script=script, interpreter=interpreter, configuration=configuration, recordsDirectory=str(records))


def build_environment(envelope, capture_plan):
    return dict(cargo_environment(envelope), RUSTC=envelope['compiler']['path'], RUSTC_WRAPPER=capture_plan['script']['path'],
                **{capture.RECORDS_ENV: capture_plan['recordsDirectory']})


def verify_capture_plan(plan, envelope):
    capture_plan = plan['compilerCapture']
    script = Path(capture_plan['script']['path'])
    destination = script.parent
    b.require_external_storage()
    b.private_directory(destination)
    b.private_directory(capture_plan['recordsDirectory'])
    b.require(script.name == 'capture-rustc' and not destination.is_relative_to(b.ROOT) and
              Path(plan['cwd']) == destination / 'overlay' and
              capture_plan['source'] == b.ref(b.CODE_DIR / 'capture_rustc.py') and
              capture_plan['recordsDirectory'] == str(destination / 'compiler-records') and
              capture_plan['configuration']['path'] == str(destination / 'capture-config.json'), 'E_CAPTURE_PLAN')
    b.read(capture_plan['interpreter'])
    b.require(b.read(capture_plan['script']) == capture_script_bytes(capture_plan['source'], capture_plan['interpreter']) and
              b.document(capture_plan['configuration']) == capture_configuration(envelope, destination, plan['cwd'],
                  plan['overlaySourceBindings'], capture_plan['script']), 'E_CAPTURE_SOURCE')
    b.require(plan['baseEnvironment'] == cargo_environment(envelope) and plan['environment'] == build_environment(envelope, capture_plan) and
              plan['profileEnvironment'] == {key: envelope['environment'][key] for key in PROFILE_KEYS}, 'E_CAPTURE_ENVIRONMENT')
    return capture_plan


def verify_compiler_captures(receipt, plan, capture_plan):
    records = Path(capture_plan['recordsDirectory'])
    bindings = receipt['compilerCaptures']
    b.require(isinstance(bindings, list) and bindings and len({item['path'] for item in bindings}) == len(bindings) and
              {item['path'] for item in bindings} == {str(path) for path in records.iterdir()}, 'E_CAPTURE_RECORD_SET')
    config = b.document(capture_plan['configuration'])
    captured = {}
    for binding in bindings:
        path = Path(binding['path'])
        b.require(path.parent == records and path.name.startswith('invocation-') and path.suffix == '.json', 'E_CAPTURE_RECORD_PATH')
        descriptor = b.approved._open(path)
        try:
            b.require(stat.S_IMODE(os.fstat(descriptor).st_mode) == 0o600, 'E_CAPTURE_RECORD_MODE')
        finally:
            os.close(descriptor)
        raw = b.document(binding)
        b.require(raw['kind'] == 'temporal-rustc-invocation-capture-v1' and
                  raw['captureConfigurationSHA256'] == capture_plan['configuration']['sha256'] and
                  raw['captureSourceSHA256'] == capture_plan['script']['sha256'] and raw['argv'][0] == plan['shippingCompiler']['path'] and
                  set(raw['environment']) <= capture.ENVIRONMENT_KEYS and raw['manifestDirectory'] in config['manifestDirectories'] and
                  raw['environment'].get('CARGO_MANIFEST_DIR') == raw['manifestDirectory'] and
                  all(raw['environment'].get(key) == value for key, value in config['profileEnvironment'].items()), 'E_CAPTURE_RECORD_BINDING')
        try:
            expected = capture.invocation_record(config, capture_plan['configuration']['sha256'], raw['argv'], raw['environment'], raw['cwd'])
        except (capture.CaptureRejected, OSError, ValueError, KeyError, TypeError):
            raise b.Rejected('E_CAPTURE_RECORD_CONTENT') from None
        b.require(raw == expected, 'E_CAPTURE_RECORD_CONTENT')
        captured[binding['path']] = (binding, raw)
    for row in receipt['crateInvocations']:
        bound = row['rawCapture']
        b.require(bound['path'] in captured and captured[bound['path']][0] == bound, 'E_CAPTURE_INVOCATION_BINDING')
        raw = captured[bound['path']][1]
        b.require(all(row[key] == raw[key] for key in ('argv', 'cwd', 'environment')), 'E_CAPTURE_INVOCATION_BINDING')


def build_plan(envelope, destination, final_source):
    b.require_external_storage()
    destination = b.private_directory(destination)
    b.verify_source(final_source, root=b.ROOT)
    b.require(envelope['kind'] == 'shipping-overlay-cargo-envelope-v1' and envelope['features'] == [] and
              Path(envelope['cwd']) == b.ROOT and envelope['sourceBindings'] == final_source,
              'E_OVERLAY_ENVELOPE')
    for name in ('compiler', 'cargo', 'shippingBinary'):
        b.read(envelope[name])
    validate_invocations(envelope['crateInvocations'], envelope['compiler'], final_source)
    target = Path(envelope['exclusiveTarget'])
    b.require(target.is_absolute() and not target.is_relative_to(b.ROOT) and
              envelope['exclusiveTargetHandoff']['target'] == str(target) and
              envelope['exclusiveTargetHandoff']['finalRootValidationComplete'] is True,
              'E_EXCLUSIVE_TARGET_HANDOFF')
    b.read(envelope['exclusiveTargetHandoff']['evidence'])
    environment = cargo_environment(envelope)
    shipping_envelope = b.publish(destination / 'shipping-envelope.json', envelope)
    overlay = b.mkdir(destination / 'overlay')
    proof = []
    patch = []
    for relative, expected in sorted(final_source.items()):
        name = Path(relative)
        b.require(not name.is_absolute() and '..' not in name.parts, 'E_OVERLAY_PATH')
        raw = b.read(dict(b.ref(b.ROOT / name), sha256=expected))
        rewritten, span = overlay_bytes(relative, raw)
        path = overlay / name
        for parent in reversed(path.parents):
            if parent.is_relative_to(overlay) and parent != overlay and not parent.exists():
                b.mkdir(parent)
        with b.create(path) as stream:
            stream.write(rewritten)
        if span:
            proof.append(span)
            patch.extend(difflib.unified_diff(raw.decode().splitlines(keepends=True), rewritten.decode().splitlines(keepends=True), fromfile='a/' + relative, tofile='b/' + relative))
    b.require({row['path'] for row in proof} == set(REPLACEMENTS) and len(proof) == 2, 'E_OVERLAY_EXACT_TWO_SPANS')
    b.verify_source(final_source, root=b.ROOT)
    overlay_source = b.source_bindings(overlay)
    b.require(set(overlay_source) == set(final_source) and {name for name in final_source if final_source[name] != overlay_source[name]} == set(REPLACEMENTS), 'E_OVERLAY_FILE_SET')
    patch_path = destination / 'overlay.patch'
    with b.create(patch_path) as stream:
        stream.write(''.join(patch).encode())
    command = cargo_command(envelope)
    capture_plan = build_capture(envelope, destination, overlay, overlay_source)
    return dict(kind='execution-only-clock-overlay-build-plan-v1', shippingEnvelopeDigest=b.digest(envelope), shippingEnvelope=shipping_envelope,
                shippingCompiler=envelope['compiler'], cargo=envelope['cargo'], command=command,
                baseEnvironment=environment, environment=build_environment(envelope, capture_plan), compilerCapture=capture_plan,
                profileEnvironment={key: environment[key] for key in PROFILE_KEYS},
                cwd=str(overlay), sourceBindings=final_source, overlaySourceBindings=overlay_source,
                patch=b.ref(patch_path), sourceProof=proof, crateInvocations=envelope['crateInvocations'],
                shippingBinary=envelope['shippingBinary'], features=[], outputDirectory=str(target / 'debug'),
                clockEnvironment=[DETECTOR_ENV, GENERATED_ENV], sourceEdits=True, shippingBinaryEquivalent=False,
                unexecuted=True, expectedDifference='Two bound temporal expressions in the private source overlay.')


def verify_build(receipt, source):
    b.require(receipt['kind'] == 'execution-only-clock-adapter-build-receipt-v1' and type(receipt['exitCode']) is int and receipt['exitCode'] == 0 and
              receipt['sourceBindings'] == source and receipt['features'] == [] and receipt['shippingBinaryEquivalent'] is False and
              receipt['smokeTestsPassed'] is True, 'E_OVERLAY_BUILD_RECEIPT')
    plan = b.document(receipt['plan'])
    b.require(plan['kind'] == 'execution-only-clock-overlay-build-plan-v1' and plan['sourceBindings'] == source and
              plan['shippingBinary'] == receipt['shippingBinary'] and plan['clockEnvironment'] == [DETECTOR_ENV, GENERATED_ENV] and
              receipt['command'] == plan['command'] and receipt['environment'] == plan['environment'] and
              receipt['cwd'] == plan['cwd'], 'E_OVERLAY_BUILD_PLAN')
    envelope = b.document(plan['shippingEnvelope'])
    b.require(b.digest(envelope) == plan['shippingEnvelopeDigest'] and envelope['sourceBindings'] == source and
              envelope['shippingBinary'] == plan['shippingBinary'] and envelope['compiler'] == plan['shippingCompiler'] and
              envelope['cargo'] == plan['cargo'] and envelope['environment'] == plan['baseEnvironment'] and
              plan['command'] == cargo_command(envelope) and plan['features'] == [] and
              plan['shippingBinaryEquivalent'] is False and plan['unexecuted'] is True and plan['sourceEdits'] is True and
              Path(plan['outputDirectory']) == Path(envelope['exclusiveTarget']) / 'debug', 'E_SHIPPING_ENVELOPE_BINDING')
    capture_plan = verify_capture_plan(plan, envelope)
    verify_compiler_captures(receipt, plan, capture_plan)
    b.verify_source(source, root=b.ROOT)
    b.require(b.source_bindings(Path(plan['cwd'])) == plan['overlaySourceBindings'], 'E_OVERLAY_SOURCE_CHANGED')
    for relative, expected in source.items():
        rewritten, span = overlay_bytes(relative, b.read(dict(b.ref(b.ROOT / relative), sha256=expected)))
        actual = b.read(b.ref(Path(plan['cwd']) / relative))
        b.require(actual == rewritten, 'E_OVERLAY_BYTE_PROOF')
        if span:
            b.require(span in plan['sourceProof'], 'E_OVERLAY_SPAN_BINDING')
    b.require(len(plan['sourceProof']) == 2, 'E_OVERLAY_EXACT_TWO_SPANS')
    b.read(plan['patch'])
    b.read(plan['shippingCompiler'])
    for name in ('adapterBinary', 'shippingBinary', 'stdout', 'stderr'):
        b.read(receipt[name])
    b.require(receipt['adapterBinary']['sha256'] != receipt['shippingBinary']['sha256'] and
              Path(receipt['adapterBinary']['path']).parent == Path(plan['outputDirectory']), 'E_OVERLAY_BINARY_IDENTITY')
    validate_invocations(receipt['crateInvocations'], plan['shippingCompiler'], plan['overlaySourceBindings'], overlay=plan['cwd'])
    outputs = {item['path']: item for row in receipt['crateInvocations'] for item in row['outputs']}
    b.require(outputs.get(receipt['adapterBinary']['path']) == receipt['adapterBinary'], 'E_OVERLAY_CLI_ARTIFACT')
    for crate in ('pqc_scan_core', 'pqc_scan_detectors'):
        produced = {item['path'] for row in receipt['crateInvocations'] if row['crate'] == crate for item in row['outputs']}
        consumer = next(row for row in receipt['crateInvocations'] if row['crate'] == 'pqc_scan_cli')
        b.require(any(value.startswith(crate + '=') and value.split('=', 1)[1] in produced
                      for i, arg in enumerate(consumer['argv'][:-1]) if arg == '--extern' for value in [consumer['argv'][i + 1]]), 'E_OVERLAY_LINKED_CHANGED_CRATE')
    smoke = b.document(receipt['smokeEvidence'])
    b.require(smoke['adapterBinary'] == receipt['adapterBinary'] and smoke['sourceBindings'] == source and
              smoke['kind'] == 'temporal-overlay-synthetic-cli-proof-v1' and smoke['passed'] is True and
              set(smoke['checks']) >= {'two_independent_clocks', 'nanosecond_serialization', 'invalid_pair_before_spawn', 'fixed_runtime_errors', 'native_assessment', 'references', 'required_formats'}, 'E_OVERLAY_SMOKE_EVIDENCE')
    return receipt


WITNESS_ID = 'TEMPORAL_PRIVATE_VALIDITY_WITNESS'
WITNESS_RULE = b'''- id: TEMPORAL_PRIVATE_VALIDITY_WITNESS
  kind: certificate
  category: temporal-proof-only
  severity: info
  risk: non-quantum-risk
  confidence: 1.0
  migration_hint: Private condition proof only.
  pattern: '^(?:valid|not_yet_valid|expired)$'
  scope: certificate_validity
'''
WITNESS_SCHEMA = dict(
    kind='temporal-producer-witness-v1',
    topKeys=['kind', 'configurationSha256', 'binarySha256', 'detectorSecond', 'ruleCount', 'status', 'bindings', 'objects', 'files', 'diagnosticCodes', 'coverage'],
    objectKeys=['path', 'sourceSha256', 'certificateSha256', 'objectIndex', 'notBeforeUnix', 'notAfterUnix', 'validityState', 'matchingDefaultValidityRuleIds'],
    fileKeys=['path', 'sourceSha256', 'status', 'objects'],
    bindingKeys=['source', 'rules', 'input', 'selection', 'tool'],
    coverageKeys=['selectedFiles', 'readOkFiles', 'readFailedFiles', 'analyzedFiles', 'partialFiles', 'analysisFailedFiles', 'unsupportedFiles', 'notApplicableFiles', 'unknownFiles', 'excludedEnumeratedFiles', 'enumerationFailedEntries'],
    status='producer_recorded_requires_admission', configLimitBytes=65536,
    outputLimitBytes=134217728, maxFileSizeBytes=2097152, threads=1,
    originalRuleCount=139, proofRuleCount=140, witnessRuleId=WITNESS_ID,
    witnessPattern='^(?:valid|not_yet_valid|expired)$',
    sourceOnlyUntilIndependentReview=True,
)


WITNESS_SCHEMA_V2 = dict(WITNESS_SCHEMA, kind='temporal-producer-witness-v2',
    topKeys=WITNESS_SCHEMA['topKeys'] + ['boundaryFailures', 'inspectionStatus'],
    boundaryKeys=['path', 'reason', 'manifestEntrySha256', 'linkMetadataSha256'],
    configKind='temporal-witness-config-v2', boundaryReason='nofollow_symlink',
    projectionLimitBytes=67108864, projectionRowLimit=500000)
CONFIG_KEYS = {'root', 'max_file_size_bytes', 'threads', 'selection', 'detector_second', 'rules_dir',
               'witness_rule_id', 'rule_files', 'output', 'bindings'}
BOUNDARY_CONFIG_KEYS = {'kind', 'manifest_projection', 'original_diagnostic_projection', 'boundary_policy_relation'}
ALLOWED_DIAGNOSTICS = {'detector_unsupported', 'detector_partial', 'excluded_path', 'file_too_large'}


def relative_path(value):
    b.require(isinstance(value, str) and value and len(value.encode('utf-8')) <= 4096 and
              not any(ord(c) < 32 or 127 <= ord(c) <= 159 for c in value) and
              not Path(value).is_absolute() and Path(value).as_posix() == value and
              all(part not in {'.', '..', ''} for part in value.split('/')), 'E_BOUNDARY_PATH')
    return value


def link_metadata_hash(path, link):
    relative_path(path)
    b.require(isinstance(link, str) and link and '\0' not in link and len(link.encode('utf-8')) <= 4096,
              'E_BOUNDARY_LINK')
    fields = [b'pqc-temporal-link-v2', path.encode('utf-8'), b'120000', link.encode('utf-8')]
    return hashlib.sha256(b''.join(struct.pack('>Q', len(value)) + value for value in fields)).hexdigest()


def _projection_bytes(reference, *, limit=b.MAX):
    b.require(isinstance(reference, dict) and set(reference) in ({'path', 'sha256'}, {'path', 'sha256', 'bytes'}) and
              isinstance(reference.get('path'), str) and isinstance(reference.get('sha256'), str) and
              re.fullmatch('[0-9a-f]{64}', reference['sha256']) is not None and
              ('bytes' not in reference or type(reference['bytes']) is int and reference['bytes'] >= 0),
              'E_BOUNDARY_ARTIFACT_BINDING')
    raw = b.read(reference, limit=limit)
    return raw, dict(path=reference['path'], sha256=reference['sha256'], bytes=len(raw))


def manifest_projection(original):
    raw, manifest_binding = _projection_bytes(original['input']['manifest'])
    try:
        manifest = json.loads(raw, object_pairs_hook=b.approved._unique,
                              parse_constant=lambda _: b.require(False, 'E_BOUNDARY_MANIFEST'))
    except (ValueError, UnicodeError, RecursionError):
        raise b.Rejected('E_BOUNDARY_MANIFEST') from None
    files = manifest.get('files', []) if isinstance(manifest, dict) else manifest
    if isinstance(files, dict):
        files = [dict(item, path=path) for path, item in files.items()]
    b.require(isinstance(files, list), 'E_BOUNDARY_MANIFEST')
    seen, entries = set(), []
    for entry in files:
        b.require(isinstance(entry, dict), 'E_BOUNDARY_MANIFEST')
        path = relative_path(entry.get('path'))
        b.require(path not in seen, 'E_BOUNDARY_MANIFEST')
        seen.add(path)
        if entry.get('mode') == '120000':
            b.require(set(entry) in ({'mode', 'path', 'symlink'}, {'blob', 'mode', 'path', 'symlink'}),
                      'E_BOUNDARY_MANIFEST')
            entries.append(dict(path=path, manifestEntrySha256=b.digest(entry),
                                linkMetadataSha256=link_metadata_hash(path, entry.get('symlink'))))
    b.require(len(entries) <= WITNESS_SCHEMA_V2['projectionRowLimit'], 'E_BOUNDARY_QUOTA')
    return dict(kind='temporal-manifest-link-projection-v2', manifest=manifest_binding,
                entries=sorted(entries, key=lambda row: row['path']))


def original_diagnostic_projection(original):
    reference, terminal_ref = original['originalDiagnostics'], original['originalTerminal']
    terminal_raw, terminal_binding = _projection_bytes(terminal_ref, limit=b.approved.MAX_METADATA)
    terminal = b.approved.decode(terminal_raw)
    expected = terminal.get('artifacts', {}).get('diagnostics.json')
    b.require(terminal.get('assessment') == original['analysisStatus'] and
              isinstance(expected, dict) and reference == dict(path=str(Path(terminal_ref['path']).parent /
              'output/diagnostics.json'), **expected), 'E_BOUNDARY_ORIGINAL_DIAGNOSTICS')
    diagnostic_raw, diagnostic_binding = _projection_bytes(reference)
    document = b.approved.decode(diagnostic_raw)
    b.require(isinstance(document, dict) and isinstance(document.get('diagnostics'), list) and
              document.get('coverage') == original['analysisStatus']['coverage'], 'E_BOUNDARY_ORIGINAL_DIAGNOSTICS')
    identities = {}
    for row in document['diagnostics']:
        b.require(isinstance(row, dict) and isinstance(row.get('code'), str) and
                  row['code'] in ALLOWED_DIAGNOSTICS | {'read_failed', 'metadata_failed', 'enumeration_failed',
                  'detector_failed', 'detector_truncated', 'detector_not_reported', 'no_detectors'}, 'E_BOUNDARY_ORIGINAL_DIAGNOSTICS')
        path, detector = row.get('file'), row.get('detector')
        if path is not None:
            relative_path(path)
        b.require(detector is None or isinstance(detector, str) and 0 < len(detector) <= 256 and
                  all(32 <= ord(c) < 127 for c in detector), 'E_BOUNDARY_ORIGINAL_DIAGNOSTICS')
        key = row['code'], path, detector
        identities[key] = identities.get(key, 0) + 1
        if row['code'] == 'read_failed':
            b.require(path is not None and detector is None and identities[key] == 1,
                      'E_BOUNDARY_ORIGINAL_DIAGNOSTICS')
    entries = [dict(code=code, path=path, detector=detector, occurrences=count)
               for (code, path, detector), count in identities.items()]
    entries.sort(key=lambda row: (row['code'], row['path'] or '', row['detector'] or ''))
    b.require(len(entries) <= WITNESS_SCHEMA_V2['projectionRowLimit'], 'E_BOUNDARY_QUOTA')
    return dict(kind='temporal-original-diagnostic-projection-v2', phaseId=original['originalPhaseId'],
                originalTerminal=terminal_binding, diagnostics=diagnostic_binding, entries=entries)


def original_nonboundary_diagnostic(item):
    if item['code'] in ALLOWED_DIAGNOSTICS:
        return True
    return (item['code'] in {'detector_failed', 'detector_truncated', 'detector_not_reported'} and
            item['detector'] is not None and item['detector'] != 'certificate_detector')


def boundary_policy_relation(reference, relation):
    value = b.document(reference)
    b.require(set(value) == {'kind', 'historicalSourceBindings', 'finalSourceBindings', 'policy',
              'phaseEvidence', 'finalEvidence', 'linkedBuild'} and
              value['kind'] == 'temporal-nofollow-policy-relation-v1' and
              value['policy'] == 'static_nofollow_symlink_read_failed_enumeration_v1' and
              value['historicalSourceBindings'] == relation['historicalSourceBindings'] and
              value['finalSourceBindings'] == relation['finalSourceBindings'] and
              set(value['phaseEvidence']) == {'primary-after', 'comparison-after'}, 'E_BOUNDARY_POLICY_RELATION')
    for binding in [*value['phaseEvidence'].values(), value['finalEvidence'], value['linkedBuild']]:
        b.read(binding)
    return value


def witness_configuration_v2(destination, *, original, boundary_policy, **arguments):
    context = dict(manifest_projection=manifest_projection(original),
                   original_diagnostic_projection=original_diagnostic_projection(original),
                   boundary_policy_relation=boundary_policy)
    return _witness_configuration(destination, boundary_context=context, **arguments)


def witness_configuration(destination, **arguments):
    return _witness_configuration(destination, boundary_context=None, **arguments)


def _witness_configuration(destination, *, boundary_context, root, selection, detector_second, bindings, final_source, output_name='witness.json'):
    destination = b.private_directory(destination)
    b.verify_source(final_source, root=b.ROOT)
    b.require(type(detector_second) is int and detector_second >= 0 and detector_second <= 9223372036854775807 and
              isinstance(selection, dict) and set(selection) == {'include_non_scannable', 'exclude_tests'} and
              all(type(v) is bool for v in selection.values()) and
              set(bindings) == set(WITNESS_SCHEMA['bindingKeys']) and all(isinstance(v, str) and len(v) == 64 and
                  all(c in '0123456789abcdef' for c in v) for v in bindings.values()), 'E_WITNESS_CONFIG')
    b.require(Path(output_name).name == output_name and output_name not in {'', '.', '..', 'witness-config.json'} and
              all(ord(c) >= 32 for c in output_name), 'E_WITNESS_OUTPUT_PATH')
    root = Path(root)
    b.require(root.is_absolute(), 'E_WITNESS_INPUT_PATH')
    rules = b.mkdir(destination / 'proof-rules')
    copied = []
    for relative in sorted(final_source):
        if relative.startswith('rules/default/'):
            path = Path(relative)
            b.require(path.parent.as_posix() == 'rules/default', 'E_WITNESS_RULE_PATH')
            raw = b.read(dict(b.ref(b.ROOT / relative), sha256=final_source[relative]), limit=65536)
            target = rules / path.name
            with b.create(target) as stream:
                stream.write(raw)
            bound = b.ref(target)
            copied.append(dict(path=target.name, sha256=bound['sha256'], bytes=bound['bytes']))
    b.require(copied, 'E_WITNESS_RULES')
    extra = rules / 'temporal-private-witness.yml'
    with b.create(extra) as stream:
        stream.write(WITNESS_RULE)
    bound = b.ref(extra)
    copied.append(dict(path=extra.name, sha256=bound['sha256'], bytes=bound['bytes']))
    config = dict(root=str(root), max_file_size_bytes=2097152, threads=1, selection=selection,
                  detector_second=detector_second, rules_dir=str(rules), witness_rule_id=WITNESS_ID,
                  rule_files=sorted(copied, key=lambda x: x['path']), output=str(destination / output_name), bindings=bindings)
    schema = WITNESS_SCHEMA
    if boundary_context is not None:
        schema = WITNESS_SCHEMA_V2
        config['kind'] = schema['configKind']
        for key in ('manifest_projection', 'original_diagnostic_projection'):
            binding = b.publish(destination / (key + '.json'), boundary_context[key])
            b.require(binding['bytes'] <= schema['projectionLimitBytes'], 'E_BOUNDARY_QUOTA')
            config[key] = binding
        b.read(boundary_context['boundary_policy_relation'])
        config['boundary_policy_relation'] = boundary_context['boundary_policy_relation']
    config_ref = b.publish(destination / 'witness-config.json', config)
    b.require(config_ref['bytes'] <= 65536, 'E_WITNESS_CONFIG_LIMIT')
    return dict(configuration=config_ref, source=b.ref(Path(__file__).parent / 'temporal_witness.rs'),
                schema=schema, rules=copied, actualRunAdmitted=False, compileSmoke='PENDING')


WITNESS_EXTERNS = ('anyhow', 'pqc_scan_core', 'pqc_scan_rules', 'pqc_scan_detectors', 'serde', 'serde_json', 'sha2', 'libc')


def witness_compile_command(compiler, source, output, externs, search_flags):
    command = [compiler['path'], '--crate-name', 'temporal_witness', '--edition=2021', source['path'], '--crate-type', 'bin', '--emit=dep-info,link', '--out-dir', str(output)]
    for name in WITNESS_EXTERNS:
        command.extend(['--extern', name + '=' + externs[name]])
    for value in sorted(search_flags):
        command.extend(['-L', value])
    return command


def rlib_metadata(raw):
    b.require(raw.startswith(b'!<arch>\n'), 'E_WITNESS_RLIB_METADATA')
    position, members = 8, []
    while position < len(raw):
        header = raw[position:position + 60]
        b.require(len(header) == 60 and header[58:] == b'`\n' and header[48:58].strip().isdigit(), 'E_WITNESS_RLIB_METADATA')
        size = int(header[48:58])
        end = position + 60 + size
        b.require(end <= len(raw), 'E_WITNESS_RLIB_METADATA')
        name = header[:16].rstrip()
        body = raw[position + 60:end]
        if name.startswith(b'#1/'):
            b.require(name[3:].isdigit() and int(name[3:]) <= len(body), 'E_WITNESS_RLIB_METADATA')
            length = int(name[3:])
            name, body = body[:length].rstrip(b'\0'), body[length:]
        else:
            name = name.rstrip(b'/')
        if name == b'lib.rmeta':
            members.append(body)
        position = end + size % 2
    b.require(position == len(raw) and len(members) == 1, 'E_WITNESS_RLIB_METADATA')
    metadata = members[0]
    if metadata.startswith(b'rust'):
        return metadata
    b.require(metadata[:4] == b'\xcf\xfa\xed\xfe' and len(metadata) >= 32, 'E_WITNESS_RLIB_METADATA')
    count, commands_size = struct.unpack_from('<II', metadata, 16)
    position, sections = 32, []
    b.require(commands_size <= len(metadata) - 32 and count <= commands_size // 8, 'E_WITNESS_RLIB_METADATA')
    for _ in range(count):
        b.require(position + 8 <= 32 + commands_size, 'E_WITNESS_RLIB_METADATA')
        command, size = struct.unpack_from('<II', metadata, position)
        b.require(size >= 8 and position + size <= 32 + commands_size, 'E_WITNESS_RLIB_METADATA')
        if command == 25:
            b.require(size >= 72, 'E_WITNESS_RLIB_METADATA')
            section_count = struct.unpack_from('<I', metadata, position + 64)[0]
            b.require(size == 72 + section_count * 80, 'E_WITNESS_RLIB_METADATA')
            for index in range(section_count):
                section = position + 72 + index * 80
                if metadata[section:section + 16].rstrip(b'\0') == b'.rmeta' and metadata[section + 16:section + 32].rstrip(b'\0') == b'__DWARF':
                    length, offset = struct.unpack_from('<QI', metadata, section + 40)
                    b.require(offset >= 32 + commands_size and offset + length <= len(metadata), 'E_WITNESS_RLIB_METADATA')
                    sections.append(metadata[offset:offset + length])
        position += size
    b.require(position == 32 + commands_size and len(sections) == 1 and sections[0].startswith(b'rust'), 'E_WITNESS_RLIB_METADATA')
    return sections[0]


def witness_link_inputs(envelope):
    dependencies, searches = {}, {}
    observed = {name: set() for name in WITNESS_EXTERNS}
    for row in envelope['crateInvocations']:
        for item in row['dependencyArtifacts']:
            b.require(item['path'] not in dependencies or dependencies[item['path']] == item, 'E_WITNESS_COMPILER_AMBIGUITY')
            dependencies[item['path']] = item
        for index, arg in enumerate(row['argv'][:-1]):
            if arg == '--extern':
                name, path = row['argv'][index + 1].split('=', 1)
                if name in observed:
                    observed[name].add(path)
        for item in row['linkSearchDirectories']:
            b.require(item['path'] not in searches or searches[item['path']] == item, 'E_WITNESS_COMPILER_AMBIGUITY')
            searches[item['path']] = item
            for filename, binding in item['files'].items():
                b.require(binding['path'] == str(Path(item['path']) / filename), 'E_WITNESS_LINK_BINDING')
                b.require(binding['path'] not in dependencies or dependencies[binding['path']] == binding, 'E_WITNESS_COMPILER_AMBIGUITY')
                dependencies[binding['path']] = binding
    externs, pairs = {}, []
    for name, paths in observed.items():
        b.require(paths, 'E_WITNESS_COMPILER_EXTERNS')
        candidates = {str(Path(path).with_suffix('.rlib')) for path in paths}
        b.require(all(Path(path).suffix in {'.rlib', '.rmeta'} for path in paths) and len(candidates) == 1, 'E_WITNESS_COMPILER_AMBIGUITY')
        link = candidates.pop()
        b.require(link in dependencies, 'E_WITNESS_COMPILER_EXTERNS')
        link_raw = b.read(dependencies[link])
        for path in sorted(paths):
            if path != link:
                metadata = dependencies[path]
                raw = b.read(metadata)
                b.require(rlib_metadata(link_raw) == raw, 'E_WITNESS_METADATA_IDENTITY')
                pairs.append(dict(crate=name, linkArtifact=dependencies[link], metadataArtifact=metadata,
                                  metadataSHA256=hashlib.sha256(raw).hexdigest(), metadataBytes=len(raw)))
        externs[name] = link
    return dict(externs=externs, dependencyArtifacts=[dependencies[externs[name]] for name in WITNESS_EXTERNS],
                metadataPairs=pairs, linkSearchDirectories=list(searches.values()))


def witness_build_plan(envelope, destination, final_source):
    return _witness_build_plan(envelope, destination, final_source, WITNESS_SCHEMA)


def witness_build_plan_v2(envelope, destination, final_source):
    return _witness_build_plan(envelope, destination, final_source, WITNESS_SCHEMA_V2)


def _witness_build_plan(envelope, destination, final_source, schema):
    destination = b.private_directory(destination)
    b.verify_source(final_source, root=b.ROOT)
    b.require(envelope['kind'] == 'shipping-overlay-cargo-envelope-v1' and envelope['sourceBindings'] == final_source and
              envelope['features'] == [], 'E_WITNESS_COMPILER_ENVELOPE')
    b.read(envelope['compiler'])
    b.read(envelope['shippingBinary'])
    validate_invocations(envelope['crateInvocations'], envelope['compiler'], final_source)
    inputs = witness_link_inputs(envelope)
    source = b.ref(b.CODE_DIR / 'temporal_witness.rs')
    output = b.mkdir(destination / 'build')
    test_output = b.mkdir(destination / 'test-build')
    search_flags = {row['argv'][i + 1] for row in envelope['crateInvocations'] for i, arg in enumerate(row['argv'][:-1]) if arg == '-L'}
    command = witness_compile_command(envelope['compiler'], source, output, inputs['externs'], search_flags)
    test_command = list(command)
    test_command[test_command.index('--out-dir') + 1] = str(test_output)
    test_command.extend(['--test'])
    return dict(kind='temporal-producer-witness-build-plan-v1', source=source, sourceBindings=final_source,
                shippingEnvelope=b.publish(destination / 'shipping-envelope.json', envelope),
                shippingBinary=envelope['shippingBinary'], compiler=envelope['compiler'],
                command=command, testCommand=test_command, environment=dict(envelope['environment'], PQC_TEMPORAL_PRODUCT_SOURCE=str(b.ROOT)),
                cwd=str(b.ROOT), outputDirectory=str(output), testOutputDirectory=str(test_output),
                dependencyArtifacts=inputs['dependencyArtifacts'], metadataPairs=inputs['metadataPairs'],
                linkSearchDirectories=inputs['linkSearchDirectories'], schema=schema,
                shippingBinaryEquivalent=False, unexecuted=True, actualRunAdmitted=False)


def verify_witness_build(receipt, final_source):
    b.require(receipt['kind'] == 'temporal-producer-witness-build-receipt-v1' and type(receipt['exitCode']) is int and receipt['exitCode'] == 0 and
              receipt['sourceBindings'] == final_source, 'E_WITNESS_BUILD_RECEIPT')
    plan = b.document(receipt['plan'])
    b.require(plan['kind'] == 'temporal-producer-witness-build-plan-v1' and plan['sourceBindings'] == final_source and
              plan['source'] == b.ref(b.CODE_DIR / 'temporal_witness.rs') and plan['schema'] in (WITNESS_SCHEMA, WITNESS_SCHEMA_V2) and
              plan['unexecuted'] is True and plan['actualRunAdmitted'] is False, 'E_WITNESS_BUILD_SOURCE')
    envelope = b.document(plan['shippingEnvelope'])
    b.require(envelope['kind'] == 'shipping-overlay-cargo-envelope-v1' and envelope['sourceBindings'] == final_source and
              envelope['features'] == [] and plan['compiler'] == envelope['compiler'] and
              plan['shippingBinary'] == envelope['shippingBinary'], 'E_WITNESS_COMPILER_ENVELOPE')
    validate_invocations(envelope['crateInvocations'], envelope['compiler'], final_source)
    inputs = witness_link_inputs(envelope)
    b.require(all(plan[key] == inputs[key] for key in ('dependencyArtifacts', 'metadataPairs', 'linkSearchDirectories')), 'E_WITNESS_COMPILER_EXTERNS')
    search_flags = {row['argv'][i + 1] for row in envelope['crateInvocations'] for i, arg in enumerate(row['argv'][:-1]) if arg == '-L'}
    expected = witness_compile_command(plan['compiler'], plan['source'], plan['outputDirectory'], inputs['externs'], search_flags)
    test_expected = list(expected)
    test_expected[test_expected.index('--out-dir') + 1] = plan['testOutputDirectory']
    test_expected.append('--test')
    b.require(plan['command'] == expected and plan['testCommand'] == test_expected and plan['cwd'] == str(b.ROOT) and
              plan['environment'] == dict(envelope['environment'], PQC_TEMPORAL_PRODUCT_SOURCE=str(b.ROOT)), 'E_WITNESS_COMPILER_COMMAND')
    b.verify_source(final_source, root=b.ROOT)
    b.read(plan['compiler'])
    b.read(plan['shippingBinary'])
    for binding in plan['dependencyArtifacts']:
        b.read(binding)
    for item in plan['linkSearchDirectories']:
        b.require(directory_binding(item['path']) == item, 'E_WITNESS_LINK_INPUT_CHANGED')
    for key, command_key, output_key in (('compilerInvocation', 'command', 'outputDirectory'), ('testCompilerInvocation', 'testCommand', 'testOutputDirectory')):
        row = receipt[key]
        b.require(row['argv'] == plan[command_key] and row['environment'] == plan['environment'] and row['cwd'] == plan['cwd'] and
                  type(row['exitCode']) is int and row['exitCode'] == 0 and row['sourceInputs'] and {item['path'] for item in row['sourceInputs']} ==
                  dep_info_inputs(row['depInfo'], row['cwd']), 'E_WITNESS_COMPILER_RECEIPT')
        for binding in row['sourceInputs']:
            b.read(binding)
            if binding['path'] == plan['source']['path']:
                b.require(binding == plan['source'], 'E_WITNESS_SOURCE_CHANGED')
            else:
                path = Path(binding['path'])
                b.require(path.is_relative_to(b.ROOT) and final_source.get(path.relative_to(b.ROOT).as_posix()) == binding['sha256'], 'E_WITNESS_INCLUDE_CHANGED')
        b.require(plan['source'] in row['sourceInputs'] and row['outputs'] and
                  all(Path(item['path']).parent == Path(plan[output_key]) for item in row['outputs']), 'E_WITNESS_COMPILER_OUTPUT')
        for binding in row['outputs'] + [row['depInfo'], row['stdout'], row['stderr']]:
            b.read(binding)
    b.read(receipt['binary'])
    b.require(receipt['binary'] in receipt['compilerInvocation']['outputs'], 'E_WITNESS_BINARY_BINDING')
    tests = receipt['syntheticTests']
    b.require(type(tests['exitCode']) is int and tests['exitCode'] == 0 and tests['command'] and tests['environment'] == plan['environment'] and
              tests['command'] == [str(Path(plan['testOutputDirectory']) / 'temporal_witness'), '--test-threads=1'] and
              tests['command'][0] in {item['path'] for item in receipt['testCompilerInvocation']['outputs']}, 'E_WITNESS_SYNTHETIC_TESTS')
    for name in ('stdout', 'stderr'):
        b.read(tests[name])
    return receipt
