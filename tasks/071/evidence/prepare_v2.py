"""Main-authored pre-observation materialization, not an evaluator or oracle learner."""
import copy, datetime, hashlib, json, pathlib, struct
ROOT=pathlib.Path(__file__).resolve().parents[3]
BASE=ROOT/'tasks/071/evidence/study-v2'
RAW=ROOT/'tasks/071/evidence/author-bundle/output/cases.generated.json'
def sha(b): return hashlib.sha256(b).hexdigest()
def emit(path,value):
 path.parent.mkdir(parents=True,exist_ok=True);path.write_text(json.dumps(value,sort_keys=True,indent=2)+'\n')
def native_id(domain,parts):
 h=hashlib.sha256()
 for s in ['pqc-scan-native','1',domain]:b=s.encode();h.update(struct.pack('>Q',len(b)));h.update(b)
 h.update(struct.pack('>Q',len(parts)))
 for s in parts:b=s.encode();h.update(struct.pack('>Q',len(b)));h.update(b)
 return domain+'-v1-'+h.hexdigest()
def algorithm(case):
 c=case['capture'];name=c['algorithm'];params=copy.deepcopy(c.get('parameters',{}))
 if name=='AES':name='AES-'+params['key_bits']
 if params.get('curve')=='P-256':params={'named_curve_oid':'1.2.840.10045.3.1.7','key_bits':'256'}
 return name,params
def asset(id,kind,attrs,relations=[]):return dict(id=id,kind=kind,identity_basis='synthetic-declared-facts',attributes=attrs,relations=relations,uncertainty=['synthetic_capture_not_real_deployment'])
def native(case):
 c=case['capture'];id=case['id'];name,params=algorithm(case)
 # Reuse public-v1 envelope only, not old expected answers or old technical contents.
 v=json.loads((ROOT/'crates/pqc_scan_cli/tests/fixtures/native-v1.golden.json').read_text());a=native_id('asset',[id,'algorithm']);o=native_id('observation',[id]);role='algorithm';r=a
 assets=[asset(a,'algorithm',dict(identifier=name,namespace='named',purpose='reference',parameters=params))]
 if c['role'] in ['certificate_subject','certificate_signature']:
  r=native_id('asset',[id,'certificate']);k=native_id('asset',[id,'key']);unknown=native_id('asset',[id,'unresolved-other-role'])
  assets.append(asset(unknown,'algorithm',dict(identifier='unknown',namespace='named',purpose='reference',parameters={})))
  keyalg=a if c['role']=='certificate_subject' else unknown;sigalg=a if c['role']=='certificate_signature' else unknown
  assets += [asset(k,'public_key',dict(spki_sha256=sha((id+'spki').encode()),key_bits=None,named_curve_oid=None),[dict(relationship='algorithm',target=keyalg)]),asset(r,'certificate',dict(certificate_sha256=sha((id+'cert').encode()),not_before_unix=1767225600,not_after_unix=1924992000),[dict(relationship='subject_key',target=k),dict(relationship='signature_algorithm',target=sigalg)])]
  role='certificate.subject_key.algorithm' if c['role']=='certificate_subject' else 'certificate.signature_algorithm'
 obs=copy.deepcopy(v['observations'][0]);obs.update(id=o,asset_ids=[r],assurance=c['assurance'],method='certificate_parse' if c['assurance']=='parsed' else 'ast_pattern',identity_confidence='weak_structural_anchor',occurrence_key=native_id('occurrence',[id]),usage='referenced');obs['target']['path']=id+'/captured-record.txt'
 v.update(assets=assets,observations=[obs],assessments=[],assessment_status='not_evaluated',legacy_associations=[],legacy_loss_records=[],unassociated_observation_ids=[]);v['run'].update(id=native_id('scan',[id]),generated_at='2026-11-01T00:00:00Z');v['coverage']['status']='partial' if c['outcome']=='partial' else 'complete_within_scope'
 return v,'inventory-v1',o,r,role

def envelope(protocol,details,outcome):
 return dict(schema_version=1,scanner_version='0.3.0',generated_at='2026-11-01T00:00:00Z',protocol=protocol,attempt_id=1,target=dict(host='example.test',port=443,resolved_addresses=['192.0.2.1'],pinned_address='192.0.2.1:443',connected_peer='192.0.2.1:443'),limits=dict(max_dns_answers=8,dns_timeout_ns=5000000000,tcp_timeout_ns=3000000000,protocol_timeout_ns=5000000000,overall_timeout_ns=15000000000,max_received_bytes=524288,max_sent_bytes=65536),traffic=dict(received_bytes=1200,sent_bytes=300),outcome=outcome,diagnostics=[] if outcome=='completed' else ['certificate_parse_incomplete'],redaction_applied=False,details=details)
def tls(case):
 c=case['capture'];verified=c['assurance']=='verified_negotiation';version=dict(id=772,name='TLSv1_3');suite=dict(id=4865,name='TLS13_AES_128_GCM_SHA256');group=dict(id=4588,name='X25519MLKEM768')
 d=dict(client_profile=dict(name='default_secure_fresh',provider='aws_lc_rs',provider_version='1.18.0',rustls_version='0.23.46',protocol_versions=[version],cipher_suites=[suite],key_exchange_groups=[group]),verification=dict(verification_name='example.test',sent_sni='example.test',trust_source='bundled_mozilla',root_set_sha256='a'*64,root_set_digest_basis='sorted_length_prefixed_trust_anchor_der_fields_sha256_v1',certificate_chain_and_name='verified' if verified else 'failed',revocation='not_checked',trust_bundle_version='1.0.11'),handshake_complete=verified,handshake_kind='full' if verified else None,negotiated=dict(version=version,cipher_suite=suite,key_exchange_group=group,group_classification='hybrid') if verified else None,peer_certificates=[dict(certificate_der_sha256='b'*64,spki_sha256='c'*64,signature_oid='1.2.840.10045.4.3.2',public_key_oid='1.2.840.10045.2.1',key_bits=256,named_curve_oid='1.2.840.10045.3.1.7',not_before_unix=1767225600,not_after_unix=1924992000,attribute_status='complete')],certificate_collection=dict(status='complete',presented_count=1,presented_bytes=900,collected_count=1))
 v=envelope('tls',d,'completed' if verified else 'failed');v['diagnostics']=[] if verified else ['certificate_validation_failed']
 return v,'tls-v1','attempt1','','/details/negotiated/key_exchange_group'
def offers(case):
 c=case['capture'];ls=dict(kex_algorithms=[c['ssh_method'],'ext-info-s'],server_host_key_algorithms=['ecdsa-sha2-nistp256'],encryption_algorithms_client_to_server=['aes256-ctr'],encryption_algorithms_server_to_client=['aes256-ctr'],mac_algorithms_client_to_server=['hmac-sha2-256'],mac_algorithms_server_to_client=['hmac-sha2-256'],compression_algorithms_client_to_server=['none'],compression_algorithms_server_to_client=['none'],languages_client_to_server=[],languages_server_to_client=[])
 d=dict(client_profile=dict(identification='SSH-2.0-pqc_scan_0.3.0',purpose='diagnostic_proposal_only',offers=ls,first_kex_packet_follows=False,reserved=0),assurance='unauthenticated_advertisement',server_identification='SSH-2.0-synthetic-071',offers=ls,first_kex_packet_follows=False,kex_candidates=[],host_key_candidates=[],extension_markers=['ext-info-s'],pre_identification_lines=0,early_messages=0)
 return envelope('ssh_offers',d,'partial' if c['outcome']=='partial' else 'completed'),'ssh-offers-v1','attempt1','','/details/offers/kex_algorithms/0'

raw=json.loads(RAW.read_text());manifest=[];oracle=[];adjudications=[];unconfirmed=[]
# Main adjudication from adopted P1-P8/readiness/action rules, before any subject run.
# These literal tuples are not derived from product observations.
policy={
 'S01A':('urgent','incomplete','blocked','unblock','known_classical_concern'),
 'S01B':('urgent','incomplete','blocked','unblock','critical_use_declared'),
 'S02A':('urgent','incomplete','unknown','investigate','long_protection_or_historical_ciphertext'),
 'S02B':('urgent','incomplete','unknown','investigate','long_protection_or_historical_ciphertext'),
 'S03A':('plan','incomplete','unknown','investigate','known_concern_plan'),
 'S03B':('plan','incomplete','unknown','investigate','known_concern_plan'),
 'S04A':('undetermined','incomplete','unknown','investigate','insufficient_information'),
 'S04B':('undetermined','incomplete','unknown','investigate','insufficient_information'),
 'S05A':('monitor','sufficient','not_applicable','monitor','bounded_role_no_known_concern'),
 'S05B':('monitor','sufficient','not_applicable','monitor','bounded_role_no_known_concern'),
 'S06A':('undetermined','incomplete','unknown','investigate','insufficient_information'),
 'S06B':('undetermined','conflicting','unknown','investigate','insufficient_information'),
 'S07A':('plan','incomplete','unknown','investigate','known_concern_plan'),
 'S07B':('plan','incomplete','unknown','investigate','known_concern_plan'),
 'S08A':('monitor','sufficient','not_applicable','monitor','bounded_role_no_known_concern'),
 'S08B':('monitor','sufficient','not_applicable','monitor','bounded_role_no_known_concern'),
 'S09A':('undetermined','incomplete','unknown','investigate','insufficient_information'),
 'S09B':('monitor','sufficient','not_applicable','monitor','bounded_role_no_known_concern'),
 'S10A':('monitor','sufficient','not_applicable','monitor','bounded_role_no_known_concern'),
 'S10B':('monitor','sufficient','not_applicable','monitor','bounded_role_no_known_concern'),
 'S11A':('undetermined','incomplete','unknown','investigate','insufficient_information'),
 'S11B':('monitor','sufficient','not_applicable','monitor','bounded_role_no_known_concern'),
 'S12A':('undetermined','incomplete','unknown','investigate','insufficient_information'),
 'S12B':('undetermined','incomplete','unknown','investigate','insufficient_information')}

def check(ptr,value,dimension='technical',operation='equals'):return dict(pointer=ptr,operation=operation,expected=value,dimension=dimension)
for f in raw['families']:
 for original in f['cases']:
  case=copy.deepcopy(original);id=case['id'];c=case['capture'];claims=case['claims']
  if f['id']=='F01':claims['support']={'peer':False}
  if c['kind']=='primitive':v,fmt,anchor,root,role=native(case)
  elif c['kind']=='tls':v,fmt,anchor,root,role=tls(case)
  else:v,fmt,anchor,root,role=offers(case)
  p=BASE/id;emit(p/'input.json',v);input_sha=sha((p/'input.json').read_bytes());target=native_id('migration_target',[fmt,input_sha,anchor,root,role]);wrapped={};prov=dict(source_kind='operator_claim',**case['provenance'])
  for field,value in claims.items():
   if field=='support':wrapped[field]={n:dict(state='unknown' if val=='?' else 'known',provenance=prov,**({} if val=='?' else {'value':val})) for n,val in value.items()}
   else:wrapped[field]=dict(state='unknown' if value=='?' else 'known',provenance=prov,**({} if value=='?' else {'value':value}))
  ctx=dict(format='pqc-scan.migration-context',version=1,input=dict(format=fmt,sha256=input_sha),organization_id='synthetic-071',systems=[{'id':f['id'].lower()}],bindings=[dict(target_id=target,system_id=f['id'].lower(),claims=wrapped)])
  # Purely synthetic reference metadata: not independently checked organization policy.
  if claims.get('deadline',{}).get('kind')=='at':ctx['references']=[dict(id=claims['deadline']['reference_id'],issuer_id='synthetic-owner',document_id='synthetic-restoration-policy',edition='1',clause='restoration-date',scope_id=f['id'].lower())]
  emit(p/'context.json',ctx)
  # Native target vector order is signature then subject; runtime primary role is0.
  ix=1 if c.get('role')=='certificate_subject' else 0
  tp=f'/targets/{ix}';dp=f'/decisions/{ix}';tech=[]
  name,params=algorithm(case)
  if c['kind']=='primitive':
   purpose={'ECDSA':'signature','RSA':'signature' if c['role']=='certificate_signature' else 'unknown','AES-128':'encryption','AES-256':'encryption','MD5':'hash','ML-KEM-768':'key_establishment','ML-DSA-65':'signature','SLH-DSA-SHA2-128s':'signature'}[name]
   quantum='vulnerable' if name in ['RSA','ECDSA'] else 'designed_resistant' if name.startswith(('ML-','SLH-')) else 'generic_search'
   known=not(c['role'] in ['certificate_subject','certificate_signature'])
   tech += [check(tp+'/technical/identifier',name),check(tp+'/technical/purpose',purpose),check(tp+'/technical/quantum',quantum),check(tp+'/technical/parameters_known',known),check(tp+'/observation/assurance',c['assurance']),check(tp+'/observation/usage','referenced'),check(tp+'/observation/collection_outcome',c['outcome']),check(tp+'/source_ref/role',role),check(tp+'/technical/encoding_assurance','unknown')]
   if id=='S01A':tech.append(check(tp+'/technical/classical_concerns','public_key_below_local_minimum',operation='contains'))
   if name=='MD5':tech.append(check(tp+'/technical/classical_concerns','weak_collision_resistance',operation='contains'))
  else:
   verified=id=='S11B';unknown=id=='S11A';identifier='unknown' if unknown else 'X25519MLKEM768' if c['kind']=='tls' else c['algorithm']
   expectedrole='negotiated_key_establishment' if c['kind']=='tls' else 'advertised_key_establishment'
   tech += [check(tp+'/technical/identifier',identifier),check(tp+'/technical/quantum','unknown'),check(tp+'/technical/purpose','unknown' if unknown else 'key_establishment'),check(tp+'/technical/parameters_known',not unknown),check(tp+'/source_ref/role',expectedrole),check(tp+'/observation/assurance','collector_reported_verified_session' if verified else 'unverified_or_incomplete' if unknown else 'unauthenticated_advertisement'),check(tp+'/observation/usage','negotiated' if verified else 'unknown')]
   if not unknown:tech += [check(tp+'/technical/components/0/assessment/identifier','X25519' if verified else 'ECDH'),check(tp+'/technical/components/1/assessment/identifier','ML-KEM-768'),check(tp+'/technical/combiner_assessment','standard_specified_implementation_unverified')]
   if verified:tech += [check(tp+'/source_ref/cipher_suite/id',4865),check(tp+'/source_ref/key_exchange_group/id',4588)]
  po=policy[id];pol=[check(dp+'/'+field,value,'conformance') for field,value in zip(['urgency','information','readiness','action/kind'],po[:4])]
  pol += [check(dp+'/reason_codes/0',po[4],'conformance'),check(dp+'/target_id',target,'conformance'),check(dp+'/system_id',f['id'].lower(),'conformance'),check('/input/sha256',input_sha,'conformance'),check('/context/sha256',sha((p/'context.json').read_bytes()),'conformance'),check('/as_of','2026-11-19T00:00:00Z','conformance')]
  for section,expected_id in [('catalog','migration-facts-v1'),('policy','migration-decision-v1'),('model','decision_table')]:
   pol += [check('/'+section+'/id',expected_id,'conformance'),check('/'+section+'/version','1.0.0','conformance')]
  if f['id']=='F01':pol.append(check(dp+'/action/blocking_prerequisites',['peer'],'conformance'))
  if f['id']=='F07':pol.append(check(dp+'/action/blocking_prerequisites',[],'conformance'))
  if id=='S06B':pol.append(check(dp+'/reason_codes','purpose_mismatch','conformance','contains'))
  if f['id']=='F04':pol.append(check(dp+'/reason_codes','stale_claim','conformance','contains'))
  if f['id']=='F12':pol.append(check(dp+'/missing_inputs','observation.selected_use','conformance','contains'))
  # Literal missing/influence boundaries adjudicated before subject observations.
  common=['support.parameters','support.peer','support.protocol','support.provider','support.runtime']
  missing={1:[x for x in common if x!='support.peer'],2:common,3:common+['support.pki','technical.encoding','technical.parameters','usage_state'],4:['applicability','support.parameters','support.provider','support.runtime','usage_state','use_case'],5:[],6:['support.parameters','support.provider','support.runtime','technical.collision_sensitive_use'],7:common,8:[],9:common if id.endswith('A') else [],10:[],11:['support.parameters','support.provider','support.runtime','technical.identity','technical.parameters'] if id.endswith('A') else [],12:common+['observation.selected_use','usage_state']}[int(f['id'][1:])]
  fields={1:['applicability','use_case','usage_state','signature_verification_until','support.peer']+(['business_critical'] if id.endswith('B') else []),2:['applicability','use_case','usage_state','confidential','historical_ciphertext'],3:['applicability','use_case','business_critical','deadline','signature_verification_until'],4:['applicability','use_case'],5:['applicability','use_case','usage_state'],6:['applicability','use_case','usage_state'],7:['applicability','use_case','usage_state','business_critical','deadline','signature_verification_until'],8:['applicability','use_case','usage_state'],9:['applicability','use_case','usage_state','signature_verification_until'],10:['applicability','use_case','usage_state','signature_verification_until'],11:['applicability','use_case','usage_state'],12:['applicability','use_case']}[int(f['id'][1:])]
  expected_claims=[];context_sha=sha((p/'context.json').read_bytes())
  for field in fields:
   pointer='/bindings/0/claims/'+field.replace('.','/');value=wrapped
   for part in field.split('.'):value=value[part]
   expected_claims.append(dict(id=native_id('migration_claim',[context_sha,pointer,field]),context_pointer=pointer,field=field,**value,freshness='unknown' if f['id']=='F04' else 'active'))
  expected_claims.sort(key=lambda cl:cl['id'])
  pol += [check(dp+'/missing_inputs',sorted(missing),'conformance'),check(dp+'/applied_claim_ids',[cl['id'] for cl in expected_claims],'conformance'),check('/claims',expected_claims,'conformance'),check('/_study_reference_integrity',True,'conformance')]
  extras=([0] if ix==1 else [1]) if f['id']=='F03' else [1,2] if f['id']=='F11' else [1] if f['id']=='F12' else []
  for extra in extras:pol += [check(f'/decisions/{extra}/system_id',None,'conformance'),check(f'/decisions/{extra}/applied_claim_ids',[],'conformance')]
  if f['id']=='F10':pol.append(check(dp+'/source_ids','context:policy-local-restoration','conformance','not_contains'))
  manifest.append(dict(id=id,pair=f['id'],group='RSA' if f['split']=='tuning' else f['technical_family'],split=f['split'],format=fmt,input=str((p/'input.json').relative_to(ROOT)),context=str((p/'context.json').relative_to(ROOT)),primary_target_index=ix,primary_target_id=target,algorithm=name,role=role if c['kind']=='primitive' else 'negotiated_key_establishment' if c['kind']=='tls' else 'advertised_key_establishment',assurance=c['assurance'] if c['kind']=='primitive' else 'collector_reported_verified_session' if id=='S11B' else 'unverified_or_incomplete' if id=='S11A' else 'unauthenticated_advertisement'))
  oracle.append(dict(id=id,kind=f['technical_family'],basis='Main source-confirmed and adopted-policy adjudication; AI proposals not oracle',requirements=['R5-01','R5-02','R5-03'],input=dict(capture=c,claims=claims,provenance=case['provenance']),checks=tech+pol,sources=original['technical_expectations']))
  unconfirmed.extend({'case':id,'statement':s,'status':'Not Tested','included_in_success':False} for s in original['unconfirmed'])
  adjudications.append(dict(id=id,technical_sources=original['technical_expectations'],source_disposition='Confirmed bounded purposes/parameters/roles against supplied primary clauses; no deployment/trust/compliance claim',policy_expected=dict(zip(['urgency','information','readiness','action','primary_reason'],po)),policy_basis='docs/architecture/MIGRATION-ASSESSMENT-V1.md P1-P8, freshness, missing prerequisites and readiness/action precedence; no new subject output observed',interventions=['namespace name -> adopted named vocabulary','declared usage roles remain context, not proof of execution']+(['constant synthetic peer=false added to both pair members to include urgent-but-unready coverage before freeze'] if f['id']=='F01' else [])+(['AES family+key_bits translated to exact AES-128/AES-256 names'] if f['id']=='F05' else [])+(['P-256 represented by exact curve OID and field size; verifier support irrelevant to generic signature per adopted policy, so expected invariant'] if f['id']=='F07' else [])+(['TLS assurance conceptual factor expanded to consistent reported verification/handshake/outcome flags; failed case cannot assert selected group; ECC certificate scaffold stays in final lineage'] if f['id']=='F11' else [])+(['certificate role uses explicit links, other role intentionally unknown; no DER/trust verification'] if f['id']=='F03' else [])))
emit(BASE/'manifest.json',dict(version=2,cli='target/debug/pqc-scan',as_of='2026-11-19T00:00:00Z',cases=manifest));emit(BASE/'oracle.json',dict(cases=oracle,unassessed=unconfirmed));emit(BASE/'ADJUDICATION.json',dict(before_subject_outputs=True,main_exposed_to_implementation=True,rows=adjudications,unconfirmed_count=len(unconfirmed),human_expert_review=False));print('Prepared24 inputs/contexts and literal expectations; no product invocation.')
