from pathlib import Path

import boundary as b
import clocks
import delta
import execution
import storage


def compare_all(index_binding, allowance_binding):
    index, repaired = execution.load_index(index_binding)
    admission = b.document(index['admission'])
    ledger = b.document(allowance_binding)
    b.require(ledger['kind'] == 'execution-only-oss98-compatibility-allowances-v1' and
              ledger['index'] == index_binding and ledger['sourceBindingsDigest'] == b.digest(admission['sourceBindings']) and
              set(ledger['projects']) == set(repaired), 'E_DELTA_LEDGER')
    review = b.document(ledger['independentReview'])
    b.require(review['verdict'] == 'PASS' and review['reviewerModel'] == 'codex:gpt-6.1-sol@high' and
              isinstance(review.get('reviewerId'), str) and review['reviewerId'] and
              review['reviewerId'] not in clocks.AUTHORS | set(admission.get('authorIds', [])) | {admission['authorId']} and
              review['allowancesDigest'] == b.digest({key: value for key, value in ledger.items() if key != 'independentReview'}),
              'E_DELTA_LEDGER_REVIEW')
    policy, history = clocks.admit_history(b.document(admission['clockMap'])['policy'], admission.get('historicalBinaryAliases', []))
    oss, _, audit = b.import_auditors()
    validation, schemas = audit.validators()
    rows = []
    for population in ('primary', 'comparison'):
        phase = history[population + '-after']
        _, verified = oss._verified_invocation(phase['path'])
        for identity in phase['invocation']['selectedIds']:
            original_row, terminal, source = verified[(identity, 0)]
            output = Path(original_row['receipt']['path']).parent / 'output'
            original_report = b.approved.decode(b.approved.checked_file(output / 'report.json', terminal['artifacts']['report.json'], limit=b.MAX, collect=True))
            original_native = b.approved.decode(b.approved.checked_file(output / 'scan.inventory.v1.json', terminal['artifacts']['scan.inventory.v1.json'], limit=b.MAX, collect=True))
            new = repaired[identity]
            new_report = storage.read_document(new['artifacts'], 'report.json')
            new_native = storage.read_document(new['artifacts'], 'scan.inventory.v1.json')
            manifest_ref = source['input']['manifest']
            b.read(manifest_ref, b.approved.MAX_METADATA)
            manifest = oss._json(Path(manifest_ref['path']))
            files = manifest.get('files', []) if isinstance(manifest, dict) else manifest
            if isinstance(files, dict):
                files = [dict(value, path=name) for name, value in files.items()]
            source_manifest = {item['path']: item for item in files}
            import accuracy_native
            for native in (original_native, new_native):
                accuracy_native.validate_native(native, source_manifest, validation['scan.inventory.v1.json'].schema)
            original_counts = len(original_native['assets']), len(original_native['observations'])
            allowed = ledger['projects'][identity]
            legacy = delta.compare_legacy(original_report, new_report, allowed['legacyRemovals'], allowed['legacyChanges'])
            original_native = delta.remove_native_false_positives(original_native, allowed['nativeRemovals'],
                allowed['legacyRemovals'], source_manifest, [row['finding_id'] for row in new_report['findings']])
            old_normal = delta.normalize_native(original_native, allowed['beforeOccurrenceMapping'], allowed['nativeRuntimePaths'])
            new_normal = delta.normalize_native(new_native, allowed['afterOccurrenceMapping'], allowed['nativeRuntimePaths'])
            native_delta = delta.compare_native(old_normal, new_normal, allowed['nativeChanges'])
            rows.append(dict(repositoryId=identity, population=population, split=new['split'], commit=new['commit'],
                        inputManifestSha256=new['inputManifestSha256'], legacy=legacy, native=native_delta,
                        originalAssetCount=original_counts[0], newAssetCount=len(new_native['assets']),
                        originalObservationCount=original_counts[1], newObservationCount=len(new_native['observations']),
                        sourceBoundNativeSchemasAndReferencesValid=True, passed=legacy['passed'] and native_delta['passed']))
    b.require(len(rows) == 98 and sum(row['population'] == 'primary' for row in rows) == 90 and
              sum(row['population'] == 'comparison' for row in rows) == 8, 'E_DELTA_COHORT_MATRIX')
    return dict(kind='execution-only-oss98-after-compatibility-v1', index=index_binding, allowances=allowance_binding,
                schemas=schemas, rows=rows, passed=all(row['passed'] for row in rows), accuracy=None, performance=None,
                qualification='Mechanical compatibility only. Native structural removals require the same approved C# false-positive proof, exact source spans, and complete remaining references. Existing qualified scorers and rendered acceptance are separate.')
