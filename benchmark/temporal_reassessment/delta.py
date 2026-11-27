import copy
import re

import boundary as b

PRESENTATION = {('description',), ('migration_hint',), ('evidence', 'match'), ('evidence', 'snippet_preview')}
REASONS = {'description_wording', 'migration_hint_wording', 'source_display_privacy_mask', 'source_display_context'}


def _leaves(value, path=()):
    if isinstance(value, dict):
        if not value:
            return {path: value}
        return {leaf: cell for key, child in value.items() for leaf, cell in _leaves(child, path + (key,)).items()}
    if isinstance(value, list):
        if not value:
            return {path: value}
        return {leaf: cell for index, child in enumerate(value) for leaf, cell in _leaves(child, path + (index,)).items()}
    return {path: value}


def compare_values(before, after, changes):
    old, new = _leaves(before), _leaves(after)
    allowed = {}
    for change in changes:
        key = tuple(change['path'])
        b.require(key not in allowed and change['reason'] in REASONS and change.get('independentlyApproved') is True,
                  'E_DELTA_APPROVAL')
        b.read(change['evidence'])
        b.require(key in old and key in new and b.digest(old[key]) == change['beforeDigest'] and
                  b.digest(new[key]) == change['afterDigest'] and b.digest(old[key]) != b.digest(new[key]), 'E_DELTA_CHANGE_BINDING')
        allowed[key] = change
    changed = {path for path in old.keys() | new.keys() if path not in old or path not in new or b.digest(old[path]) != b.digest(new[path])}
    return dict(passed=changed == allowed.keys(), changedPaths=[list(path) for path in sorted(changed, key=repr)],
                unexpectedPaths=[list(path) for path in sorted(changed - allowed.keys(), key=repr)])


def compare_native(before, after, changes):
    b.require(changes == [], 'E_NATIVE_PRESENTATION_ALLOWANCE_UNSUPPORTED')
    return compare_values(before, after, [])


def compare_legacy(before, after, removals=(), changes=()):
    old = {finding['finding_id']: finding for finding in before['findings']}
    new = {finding['finding_id']: finding for finding in after['findings']}
    b.require(len(old) == len(before['findings']) and len(new) == len(after['findings']), 'E_DELTA_DUPLICATE')
    b.require(not new.keys() - old.keys(), 'E_DELTA_NEW_FINDING')
    approved = {}
    for removal in removals:
        identity = removal['findingId']
        b.require(identity in old and identity not in new and identity not in approved and
                  removal['reason'] == 'confirmed_csharp_predefined_type_false_positive' and
                  removal['independentlyApproved'] is True and removal['astNodeKind'] == 'predefined_type' and
                  re.fullmatch('[0-9a-f]{64}', removal['sourceSha256']) and
                  removal['ruleId'] == old[identity]['rule_id'] and removal['location'] == old[identity]['location'],
                  'E_DELTA_REMOVAL')
        b.read(removal['evidence'])
        approved[identity] = removal
    b.require(old.keys() - new.keys() == approved.keys(), 'E_DELTA_UNAPPROVED_REMOVAL')
    old_order = [finding['finding_id'] for finding in before['findings'] if finding['finding_id'] in new]
    b.require(old_order == [finding['finding_id'] for finding in after['findings']], 'E_DELTA_ORDER')
    grouped = {identity: [] for identity in new}
    for change in changes:
        identity, path = change['findingId'], tuple(change['path'])
        b.require(identity in new and (path in PRESENTATION or len(path) == 3 and path[:2] == ('evidence', 'metadata') or
                  len(path) == 4 and path[:2] == ('source_snippet', 'lines') and type(path[2]) is int and path[3] == 'text'),
                  'E_DELTA_SEMANTIC_FIELD')
        if path[:2] == ('evidence', 'metadata'):
            value = new[identity]['evidence']['metadata'][path[2]]
            b.require(len(path) == 3 and change['reason'] == 'source_display_privacy_mask' and
                      isinstance(value, str) and '[masked-' in value, 'E_DELTA_METADATA_SEMANTICS')
        grouped[identity].append(change)
    results = {identity: compare_values(old[identity], new[identity], grouped[identity]) for identity in new}
    return dict(kind='execution-only-legacy-compatibility-v1', passed=all(row['passed'] for row in results.values()),
                originalCount=len(old), newCount=len(new), removedCount=len(approved),
                remainingFindingIdsStable=True, remainingRelativeOrderStable=True, rows=results,
                accuracy=None, sourceTruthAssessment='External independently approved evidence only.')


def remove_native_false_positives(native, removals, legacy_removals, source_manifest, target_legacy_ids):
    result = copy.deepcopy(native)
    legacy = {row['findingId']: row for row in legacy_removals}
    deleted = {'assets': set(), 'observations': set()}
    indexed = {name: {row['id']: row for row in result[name]} for name in deleted}
    for removal in removals:
        collection, identity = removal['collection'], removal['id']
        finding = legacy.get(removal['findingId'])
        b.require(collection in indexed and identity in indexed[collection] and identity not in deleted[collection] and
                  finding is not None and removal['reason'] == 'confirmed_csharp_predefined_type_false_positive' and
                  removal['independentlyApproved'] is True and removal['astNodeKind'] == 'predefined_type', 'E_NATIVE_REMOVAL')
        row = indexed[collection][identity]
        b.require(b.digest(row) == removal['originalEntryDigest'] and removal['evidence'] == finding['evidence'] and
                  removal['sourcePath'] in source_manifest and
                  source_manifest[removal['sourcePath']]['sha256'] == finding['sourceSha256'], 'E_NATIVE_REMOVAL_SOURCE')
        b.read(removal['evidence'])
        if collection == 'observations':
            target = row['target']
            b.require(row['producer'] == 'tree_sitter_detector' and row['method'] == 'ast_pattern' and
                      target['path'] == finding['location']['file'] == removal['sourcePath'] and
                      target['line'] == finding['location']['line'] and
                      target['byte_start'] == removal['byteStart'] and target['byte_end'] == removal['byteEnd'] and
                      type(removal['byteStart']) is int and type(removal['byteEnd']) is int and
                      0 <= removal['byteStart'] < removal['byteEnd'] <= source_manifest[removal['sourcePath']]['bytes'],
                      'E_NATIVE_REMOVAL_AST_POSITION')
        deleted[collection].add(identity)
    removed_observations = [indexed['observations'][identity] for identity in deleted['observations']]
    linked_assets = {identity for row in removed_observations for identity in row['asset_ids']}
    b.require(deleted['assets'] <= linked_assets, 'E_NATIVE_REMOVAL_ASSET_LINK')
    for name, identities in deleted.items():
        result[name] = [row for row in result[name] if row['id'] not in identities]
    result['legacy_associations'] = [row for row in result['legacy_associations'] if row['legacy_id'] not in legacy]
    b.require([row['legacy_id'] for row in result['legacy_associations']] == target_legacy_ids, 'E_NATIVE_REMAINING_LEGACY_IDS')
    for index, row in enumerate(result['legacy_associations']):
        row['legacy_index'] = index
    result['unassociated_observation_ids'] = [identity for identity in result['unassociated_observation_ids'] if identity not in deleted['observations']]
    result['legacy_loss_records'] = [row for row in result['legacy_loss_records'] if row['observation_id'] not in deleted['observations']]
    remaining_assets = {row['id'] for row in result['assets']}
    remaining_observations = {row['id'] for row in result['observations']}
    b.require(all(link['target'] in remaining_assets for row in result['assets'] for link in row['relations']) and
              all(identity in remaining_assets for row in result['observations'] for identity in row['asset_ids']) and
              all(identity in remaining_observations for row in result['legacy_associations'] for identity in row['observation_ids']),
              'E_NATIVE_REMOVAL_DANGLING_REFERENCE')
    return result


def normalize_native(native, occurrence_mapping, runtime_paths):
    result = copy.deepcopy(native)
    mapping = {}
    for row in occurrence_mapping:
        b.require(set(row) == {'id', 'occurrenceKey'} and row['id'] not in mapping and
                  isinstance(row['occurrenceKey'], str), 'E_NATIVE_OCCURRENCE_MAPPING')
        mapping[row['id']] = row['occurrenceKey']
    b.require(len(set(mapping.values())) == len(mapping), 'E_NATIVE_OCCURRENCE_MAPPING')
    ids = {row['id'] for name in ('assets', 'observations') for row in result[name]}
    b.require(set(mapping) == ids, 'E_NATIVE_OCCURRENCE_COVERAGE')
    for name in ('assets', 'observations'):
        for row in result[name]:
            row['id'] = mapping[row['id']]
    for row in result['assets']:
        for relation in row['relations']:
            relation['target'] = mapping[relation['target']]
    for row in result['observations']:
        row['asset_ids'] = [mapping[value] for value in row['asset_ids']]
    for row in result['legacy_associations']:
        row['observation_ids'] = [mapping[value] for value in row['observation_ids']]
    result['unassociated_observation_ids'] = [mapping[value] for value in result['unassociated_observation_ids']]
    for row in result['legacy_loss_records']:
        row['observation_id'] = mapping[row['observation_id']]
    permitted = {('run', 'id'), ('run', 'generated_at'), ('rules_digest',)}
    b.require(set(map(tuple, runtime_paths)) <= permitted, 'E_NATIVE_RUNTIME_NORMALIZATION')
    for path in runtime_paths:
        target = result
        for key in path[:-1]:
            target = target[key]
        b.require(path[-1] in target, 'E_NATIVE_RUNTIME_NORMALIZATION')
        target[path[-1]] = '<explicit-runtime-normalization>'
    return result
