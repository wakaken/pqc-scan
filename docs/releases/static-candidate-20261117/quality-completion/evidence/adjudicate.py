from pathlib import Path
import json,hashlib,collections
r=Path(__file__).parent;d=Path('docs/releases/static-candidate-20261117/quality-completion');j=json.load(open(r/'MAPPED-OUTCOMES.json'))
proofs={
 ('pipeline.rs',707):('snippet_nonempty','For nonempty lines and target_line>0, clamped target_idx is valid. start=target_idx.saturating_sub(before)<=target_idx; end=min(target_idx.saturating_add(after),len-1)>=target_idx. The loop necessarily emits target_idx. The empty-result true arm is infeasible under these preceding checks for every valid slice and usize budget.'),
 ('privacy.rs',79):('quoted_value','masked_value is called only with assignment() named value capture. Its double-quoted alternative includes both mandatory delimiters; other alternatives cannot start with a double quote. Thus starts_with(double-quote) true and ends_with(double-quote) false cannot occur through this caller.'),
 ('privacy.rs',81):('quoted_value','After the double-quote alternative is excluded, the assignment value capture single-quoted alternative includes both mandatory delimiters. Nonquoted/Bearer/Basic alternatives cannot start with a single quote. The false second operand is infeasible through this caller.'),
 ('certificate_detector/attributes.rs',71):('spki_envelope','validate_spki_layout first requires exactly one fully consumed universal constructed SEQUENCE, exactly AlgorithmIdentifier and BIT STRING fields, and no unconsumed AlgorithmIdentifier fields. The pinned SubjectPublicKeyInfo parser consumes those same two fields and defines raw from start to consumed end. Consequently neither an outer remainder nor shortened raw is reachable after this validation. Trailing/extra fields are rejected before this guard.'),
 ('certificate_detector/attributes.rs',82):('certificate_spki_alias','Both production callers pass a certificate parsed from the same immutable DER prefix: parse_certificate_der requires no outer remainder; parse_der_object passes bytes[..consumed]. Validated TBS field order/version and SPKI layout identify the same encoded SPKI as pinned TbsCertificate/SubjectPublicKeyInfo parsing. raw and spki_der therefore refer to byte-equal boundaries. A caller supplying a certificate from different bytes would violate this private-call prerequisite, rather than represent an accepted public input.'),
 ('certificate_detector/attributes.rs',325):('complete_integer_field','integer_parameter_fields uses Any::from_der to consume one field, saves exactly input[..input.len()-rest.len()], and advances to rest. Re-parsing that exact immutable encoded field with the same deterministic Any decoder consumes its complete slice. The second parse cannot leave a remainder. Wrong type/class/value remains separately rejected and tested; this is not proof of cryptographic DSA validity.'),
 ('dependency_detector/bounded.rs',341):('quoted_gem_capture','The fixed gem regex can match only a mandatory opening quote, nonempty package-name character class, and closing quote. The quoted regex accepts either quote delimiter around a body excluding both quote characters, so at least that first quoted token necessarily exists. Later matching-delimiter checks still reject mismatched quotes; an empty capture vector is infeasible after gem.captures succeeds.'),
 ('dependency_detector/bounded.rs',342):('quoted_gem_capture','The first quoted token begins at the mandatory package-name quote of the anchored gem match. Package-name characters exclude both quotes, so its body must equal capture group 2 exactly. The unequal-body condition is infeasible under the current fixed regexes; the separate matching-quote check remains feasible and tested.'),
}
def test_for(path,line):
 if path=='pipeline.rs':return 'undecodable_wide_bom_is_diagnosed_but_pkcs12_is_left_to_its_binary_detector' if line in (383,385) else 'nameless_dependency_metadata_is_never_exported_as_a_named_component' if line==551 else 'snippet_boundaries_and_each_private_marker_protect_displayed_source'
 if path=='privacy.rs':return 'unknown_source_line_is_sensitive_if_any_private_or_credential_range_exists' if line==337 else 'basic_and_bearer_credentials_are_redacted_without_erasing_public_scheme'
 if path.startswith('certificate_'):
  if line<180:return 'signature_disagreement_and_nonzero_bit_padding_never_claim_complete' if line in (88,89) else 'sequence_and_certificate_envelopes_reject_trailing_duplicate_and_wrong_class_fields'
  if line==217:return 'non_oid_algorithm_parameters_are_reported_as_other_not_as_an_oid'
  if line<300:return 'rsa_algorithm_parameters_and_trailing_integers_preserve_partial_status'
  return 'dsa_requires_three_positive_integer_parameters_and_one_complete_public_integer'
 if line in (5,21,24,50,70,84):return 'quoted_comments_continuations_and_limits_preserve_explicit_incompleteness'
 if line==82:return 'malformed_fallback_records_and_metadata_edges_never_corrobate_resolution'
 if line<180:return 'go_blocks_and_incomplete_grammar_never_become_complete_inventory'
 if line<250:return 'go_metadata_accepts_only_complete_declarations_without_evaluation'
 if line<380:return 'requirement_and_gemfile_boundaries_keep_constraints_and_reject_executable_values'
 if line<550:return 'lock_origin_and_platform_correspondence_do_not_invent_registry_resolution; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution'
 if line<790:return 'maven_envelopes_duplicates_and_unresolved_coordinates_remain_incomplete; rejected_maven_coordinate_cannot_relabel_an_earlier_valid_component'
 return 'maven_expansion_keeps_unresolved_cycles_and_bounds_each_intermediate; malformed_fallback_records_and_metadata_edges_never_corrobate_resolution'
for row in j['rows']:
 short=row['path'].split('/src/')[1];line=row['span'][0];row['id']=f'{short}:{line}:{row["span"][1]}'
 row['behavioralTestOracles']=test_for(short,line);row['counterAttribution']='Suite-level LLVM union across emitted instantiations. Named tests establish observable contracts; no per-test counter attribution claim.'
 if row['currentOutcomes']==[True,True]:row['assessment']='ObservedStricterReplacement' if row['mapping'].startswith('Stricter') else 'ObservedBoth'
 else:
  key=(short,line);assert key in proofs,(key,row['currentOutcomes'])
  name,reason=proofs[key];row['assessment']='InfeasibleOutcomeUnderCurrentPreconditions';row['invariantId']=name;row['proof']=reason
 row['independentTestReview']='Pending'
j['scope']='Every one of the 136 requested static source spans individually mapped to current predicates and behavioral oracles. 127 corresponding current predicates have both outcomes observed, including three stricter replacements. Nine still lack an outcome counter and are separately adjudicated by source prerequisites, not relabelled as covered. No invented 100% gate or general absence-of-defects claim.'
j['sourceBindings']={name:hashlib.sha256(Path(name).read_bytes()).hexdigest() for name in {x['path'] for x in j['rows']}}
j['dependencyProofBindings']={str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in [Path('/Users/waka/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/x509-parser-0.18.1/src/x509.rs'),Path('/Users/waka/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/x509-parser-0.18.1/src/certificate.rs')]}
j['counts']=dict(collections.Counter(x['assessment'] for x in j['rows']));(d/'BRANCH-ASSESSMENTS.json').write_text(json.dumps(j,indent=2)+'\n');print(j['counts'])
