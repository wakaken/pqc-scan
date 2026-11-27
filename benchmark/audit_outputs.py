#!/usr/bin/env python3
"""Audit generated static scan exports using pinned schemas and native references."""
import argparse
from collections import Counter, defaultdict
import importlib.metadata
import json
from pathlib import Path
import sys
from urllib.parse import quote

if not __debug__:
    raise RuntimeError('audit_requires_python_assertions')

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT/'tests/quality'))
from evaluate import bytes_digest
from task048.contracts import check_projection, properties

LIMIT = 128 * 1024**2


def load(path):
    if path.is_symlink() or not path.is_file() or path.stat().st_size > LIMIT:
        raise ValueError('unavailable_or_unbounded_artifact')
    with path.open('rb') as stream:
        raw = stream.read(LIMIT + 1)
    if len(raw) > LIMIT:
        raise ValueError('artifact_grew_beyond_limit')
    return json.loads(raw)


def sarif_locations(findings, sarif):
    def uri(path):
        if sys.platform == 'win32':
            path = path.replace('\\', '/')
        encoded = quote(path, safe='-._~/')
        if sys.platform == 'win32' and path.startswith('//'):
            return 'file:' + encoded
        if sys.platform == 'win32' and len(path) > 2 and path[1:3] == ':/':
            return 'file:///' + encoded[0] + ':' + encoded[4:]
        return 'file://' + encoded if path.startswith('/') else encoded
    wanted, available, observed, columns = Counter(), Counter(), Counter(), Counter()
    for finding in findings:
        loc = finding['location']
        key = (finding['rule_id'], uri(loc['file']), loc['line'] if loc['file'] else 0)
        wanted[key] += 1
        available[key + (loc['column'],)] += 1
    omitted = 0
    for run in sarif['runs']:
        for result in run['results']:
            locations = result.get('locations', [])
            assert len(locations) <= 1, 'sarif_location_count'
            physical = locations[0]['physicalLocation'] if locations else {}
            region = physical.get('region', {})
            key = (result['ruleId'], physical.get('artifactLocation', {}).get('uri', ''), region.get('startLine', 0))
            observed[key] += 1
            if 'startColumn' in region:
                columns[key + (region['startColumn'],)] += 1
            else:
                omitted += 1
    assert wanted == observed, 'sarif_findings'
    assert all(count <= available[key] for key, count in columns.items()), 'sarif_present_columns'
    return omitted


def legacy_cbom_locations(findings, cbom):
    groups, projected_groups = defaultdict(list), defaultdict(list)
    available = Counter()
    for finding in findings:
        location = f"{finding['location']['file']}:{finding['location']['line']}"
        groups[location].append(finding)
        available[(location, finding['category'], finding['risk'], finding['migration_hint'])] += 1
    projected = Counter()
    for row in cbom:
        projected_groups[row['location']].append(row)
        projected[(row['location'], row['usage_type'], row['quantum_risk'], row['migration_hint'])] += 1
    assert all(count <= available[key] for key, count in projected.items()), 'legacy_cbom_location_projection'
    counts = dict(legacyCbomCoalesced=0, legacyCbomUncertainGroups=0, legacyCbomUnresolvedFindingRows=0)
    for location, group in groups.items():
        rows = projected_groups[location]
        known, unknown, expected, seen = defaultdict(set), Counter(), Counter(), set()
        for finding in group:
            evidence = finding['evidence']
            match = evidence['match']
            metadata = evidence.get('metadata', {})
            # Export masking and source display restoration lose the builder's original match.
            uncertain = ('[masked-' in match or metadata.get('node_kind') == 'bounded_source_fact'
                         or any('[masked-' in value for value in metadata.values() if isinstance(value, str)))
            rule = finding['rule_id']
            if uncertain:
                unknown[rule] += 1
                continue
            known[rule].add(match)
            identity = (rule, match)
            if identity not in seen:
                seen.add(identity)
                expected[(finding['category'], finding['risk'], finding['migration_hint'], match)] += 1
        if unknown:
            rules = known.keys() | unknown.keys()
            minimum = sum(max(1, len(known[rule])) for rule in rules)
            maximum = sum(len(known[rule]) + unknown[rule] for rule in rules)
            assert minimum <= len(rows) <= maximum, 'legacy_cbom_coverage_bounds'
            counts['legacyCbomUncertainGroups'] += 1
            counts['legacyCbomUnresolvedFindingRows'] += len(group) - len(rows)
        else:
            actual = Counter((row['usage_type'], row['quantum_risk'], row['migration_hint'], row['algorithm']) for row in rows)
            assert expected == actual, 'legacy_cbom_identity_projection'
            counts['legacyCbomCoalesced'] += len(group) - len(rows)
    return counts


def projection_checks(report, native, sarif, cbom, dependencies, crypto, software):
    findings = report['findings']
    assert report['summary']['total_findings'] == len(findings), 'summary_count'
    assert dict(Counter(f['severity'] for f in findings)) == report['summary']['by_severity'], 'severity_counts'
    assert dict(Counter(f['risk'] for f in findings)) == report['summary']['by_risk'], 'risk_counts'
    assert cbom == report['cbom'] and dependencies == report['dependency_sbom'], 'legacy_exports'
    cbom_counts = legacy_cbom_locations(findings, cbom)
    ids = [f['finding_id'] for f in findings]
    assert len(ids) == len(set(ids)), 'duplicate_finding_ids'
    omitted_columns = sarif_locations(findings, sarif)
    assets = {a['id']: a for a in native['assets']}
    observations = {o['id']: o for o in native['observations']}
    assert len(assets) == len(native['assets']), 'duplicate_asset_ids'
    assert len(observations) == len(native['observations']), 'duplicate_observation_ids'
    assert all(r['target'] in assets for a in assets.values() for r in a['relations']), 'asset_relations'
    assert all(i in assets for o in observations.values() for i in o['asset_ids']), 'observation_assets'
    for association in native['legacy_associations']:
        assert ids[association['legacy_index']] == association['legacy_id'], 'legacy_index_id'
        assert all(i in observations for i in association['observation_ids']), 'legacy_observations'
    counts = []
    for document, allowed in [(crypto, {'algorithm', 'certificate', 'public_key'}), (software, {'dependency'})]:
        components = document['components']
        projected = {c['bom-ref'] for c in components}
        assert len(projected) == len(components), 'duplicate_cyclonedx_refs'
        assert projected <= {a['id'] for a in assets.values() if a['kind'] in allowed}, 'cyclonedx_component_kinds'
        loss = json.loads(properties(document)['pqc-scan:export-loss'])['counts']
        omitted = sum(not projected.intersection(o['asset_ids']) for o in observations.values())
        assert loss.get('unprojected_observations', 0) == omitted, 'unprojected_observation_count'
        assert loss.get('usage_assets_in_observation_properties', 0) == sum(a['kind'] == 'usage' for a in assets.values()), 'usage_loss_count'
        check_projection(native, document, loss)
        counts.append(len(components))
    return dict(findings=len(findings), assets=len(assets), observations=len(observations), sarifOmittedColumns=omitted_columns, **cbom_counts,
                cryptoComponents=counts[0], softwareComponents=counts[1])


def validators():
    import jsonschema
    from referencing import Registry, Resource
    from referencing.jsonschema import DRAFT7
    from referencing.exceptions import NoSuchResource
    def unavailable(uri):
        raise NoSuchResource(ref=uri)
    registry = Registry(retrieve=unavailable)
    schema_paths = [ROOT/'schemas/native-scan-v1.schema.json', ROOT/'crates/pqc_scan_report/tests/schema/sarif-schema-2.1.0.json', *sorted((ROOT/'schemas/cyclonedx/1.7').glob('*.schema.json'))]
    documents = {}
    for path in schema_paths:
        document = load(path)
        documents[path] = document
        resource = Resource.from_contents(document, default_specification=DRAFT7)
        registry = registry.with_resource(path.as_uri(), resource)
        if document.get('$id', document.get('id')):
            registry = registry.with_resource(document.get('$id', document.get('id')), resource)
    mappings = {'scan.inventory.v1.json': schema_paths[0], 'report.sarif': schema_paths[1],
                'cbom.cdx-1.7.json': ROOT/'schemas/cyclonedx/1.7/bom-1.7.schema.json',
                'dependency-sbom.cdx-1.7.json': ROOT/'schemas/cyclonedx/1.7/bom-1.7.schema.json'}
    result = {}
    for name, path in mappings.items():
        schema = documents[path]
        cls = jsonschema.validators.validator_for(schema)
        cls.check_schema(schema)
        result[name] = cls(schema, registry=registry, format_checker=jsonschema.FormatChecker())
    return result, {str(p.relative_to(ROOT)): bytes_digest(p) for p in schema_paths}


def audit_directory(directory, validation):
    names = ['report.json', 'scan.inventory.v1.json', 'report.sarif', 'cbom.json', 'dependency-sbom.json', 'cbom.cdx-1.7.json', 'dependency-sbom.cdx-1.7.json']
    data = {name: load(directory/name) for name in names}
    errors = []
    for name, validator in validation.items():
        for error in validator.iter_errors(data[name]):
            errors.append(dict(file=name, path=list(error.absolute_path), validator=error.validator))
            if len(errors) >= 32:
                break
    counts = projection_checks(*(data[name] for name in names))
    return dict(directory=str(directory), schemaErrors=errors, counts=counts,
                artifacts={name: bytes_digest(directory/name) for name in names}, passed=not errors)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--directories', type=Path, required=True, help='JSON list of generated output directories')
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    validation, schemas = validators()
    rows = []
    for name in load(args.directories):
        directory = Path(name)
        try:
            rows.append(audit_directory(directory, validation))
        except (AssertionError, OSError, ValueError, KeyError, TypeError, IndexError) as error:
            rows.append(dict(directory=str(directory), passed=False, errorKind=type(error).__name__, failureCode=str(error) if isinstance(error, AssertionError) else None))
    result = dict(validator='jsonschema '+importlib.metadata.version('jsonschema'), offlineReferencesOnly=True,
                  schemaBindings=schemas, sourceBindings={str(p.relative_to(ROOT)): bytes_digest(p) for p in [Path(__file__), ROOT/'tests/quality/evaluate.py', ROOT/'tests/quality/task048/contracts.py']}, rows=rows,
                  limitations=['Legacy formats have no official standard schema.', 'Legacy CBOM groups with masked evidence or source display restoration have coverage bounds only; their unresolved finding/row differences are not proven coalescing.', 'Schema validity does not establish detection accuracy.', 'Rendered HTML and Markdown acceptance remains separate.', 'Other format-specific omission counts remain preserved, not independently adjudicated.'])
    with args.output.open('x') as stream:
        json.dump(result, stream, indent=2)
        stream.write('\n')
    return 0 if rows and all(row['passed'] for row in rows) else 1


if __name__ == '__main__':
    raise SystemExit(main())
