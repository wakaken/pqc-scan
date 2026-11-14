import collections,hashlib,json,pathlib
r=pathlib.Path.cwd();base=r/'target/quality-task-042-reopen-20261112';out=r/'docs/quality/task042-reopen-20261112';out.mkdir(exist_ok=True)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
def save(n,v):(out/n).write_text(json.dumps(v,indent=2,ensure_ascii=False)+'\n')
datasets={};flat=[]
app=json.load(open(r/'tests/quality/applicability-042-v2/assessment.json'));excluded={d['id'] for d in app['dispositions']}
for name,q in [('legacy','tests/quality'),('v1','tests/quality/expanded'),('v2','tests/quality/expanded-v2'),('lock-controls','tests/quality/expanded-v2-lock-controls')]:
 c=json.load(open(r/q/'corpus.json'));t=json.load(open(r/q/'truth.json'));a=json.load(open(base/(name+'-report-complete')/'quality.json'));s=json.load(open(base/name/'summary.json'));datasets[name]=dict(corpus=c,truth=t,assessment=a,summary=s)
 if name in ['v2','lock-controls']:
  labels={l['id']:l for l in t['labels']};files={f['path']:f for f in c['files']};obs=collections.defaultdict(list)
  for x in s['accuracy']['details']:
   if x['unit'] is not None:obs[x['unit']].append(x['disposition'])
  for u in c['units']:flat.append(dict({**u,**labels[u['id']]},dataset=name,inputFormat=files[u['path']]['inputFormat'],source=files[u['path']]['source'],split=files[u['path']]['split'],observedDispositions=obs[u['id']],supportedScopeLabel=('unsupported' if name=='v2' and u['id'] in excluded else labels[u['id']]['label'])))
save('UNIT-LEDGER.json',flat)
formats=[]
for fmt in sorted({u['inputFormat'] for u in flat}):
 us=[u for u in flat if u['inputFormat']==fmt]
 row=dict(inputFormat=fmt,positive=[u['dataset']+'/'+u['id'] for u in us if u['label']=='positive'],negative=[u['dataset']+'/'+u['id'] for u in us if u['label']=='negative'],malformed=[u['dataset']+'/'+u['id'] for u in us if u['caseKind']=='malformed'],ambiguous=[u['dataset']+'/'+u['id'] for u in us if u['label']=='ambiguous'],unsupported=[u['dataset']+'/'+u['id'] for u in us if u['label']=='unsupported'],existingTests=[])
 if fmt=='Ruby':row['existingTests']=[dict(file='crates/pqc_scan_detectors/tests/pqc_readiness.rs',test='inventories_pqc_in_all_seven_languages_but_not_comments_or_similar_names',property='PQC_RUBY_API comment and similar-identifier negative, paired with explicit ML-DSA-65 literal positive',evidence='native-2.log')]
 if fmt=='package.json':row['existingTests']=[dict(file='crates/pqc_scan_detectors/tests/pqc_readiness.rs',test='inventories_dependencies_without_claiming_pqc_is_enabled',property='DEP_PQC_CAPABLE_LIBRARY @noble/post-quantum0.7.1 positive and wrong ecosystem negative',evidence='native-2.log')]
 outside=fmt in ['Swift (unsupported)','CBOM-1.7']
 row['status']='Not Applicable' if outside else 'Verified'
 row['reason']=('Swiftは対応grammarの対象外。適格分母から除外し、安全という負例にはしない。' if fmt.startswith('Swift') else '暗号資産だけのCBOMは、文書化されたソフトウェア依存入力の範囲外。安全という負例にはしない。' if fmt=='CBOM-1.7' else 'Positive, negative and malformed source cases are identified; existing matching default-rule assertions reused where listed. Verified means evaluated coverage, not all detector cases passed.')
 row['supportedScope']={label:[u['dataset']+'/'+u['id'] for u in us if u['supportedScopeLabel']==label] for label in ['positive','negative','ambiguous','unsupported']}
 if not outside:assert (row['positive'] or fmt=='package.json') and (row['negative'] or fmt=='Ruby') and row['malformed'],row
 formats.append(row)
save('FORMAT-MATRIX.json',formats)
rows=['# 形式・版別の受入証拠','', '対象は公開対応範囲の有限な代表例です。Verifiedは正負・破損例を評価した状態を表し、検出器が全件合格した意味ではありません。各IDの規則・原文・正解・測定結果はUNIT-LEDGER.jsonへ結び付きます。','', '| 形式・版 | 正例ID | 負例ID | 破損ID | 既存試験の再利用 / 対応外理由 |','|---|---|---|---|---|']
short=lambda xs:', '.join(xs) if len(xs)<7 else ', '.join(xs[:6])+f' ほか{len(xs)-6}（全IDはJSON）'
for f in formats: rows.append('| '+' | '.join([f['inputFormat'],(short(f['supportedScope']['positive']) or ('既存試験の@noble/post-quantum正例' if f['inputFormat']=='package.json' else '')), (short(f['supportedScope']['negative']) or ('既存試験のコメント・同名識別子負例' if f['inputFormat']=='Ruby' else '')),short(f['malformed']),'; '.join(x['test'] for x in f['existingTests']) or ('対応外: '+f['reason'] if f['status']=='Not Applicable' else '固定正解と実測を照合')])+' |')
rows+=['','暗号化された旧P12/PFXの6属性は、二者の適用範囲レビューに基づき上表の正負から除外しました。JSONの元positive/negativeとsupportedScopeを別に保持しています。原真値と測定は上書きしていません。','TLS/SSH textはJava property断片とSSH設定断片を含みます。7サーバー設定もdirective断片で、構成全体の有効性や実交渉は検証しません。PEM/CRT/CERの負例は同じRSA証明書にECDSA署名がないという独立属性です。PKCS#12の破損filename signalと証明書属性のunsupportedを分け、暗号化された旧容器と新しい公開証明書のみの平文容器を両方保持します。','依存の版は原文の宣言または入力ロックの記録です。インストール結果や機能有効化を証明しません。jose3例の能力はambiguousのまま保持し、認識済みライブラリの正例を別に補いました。','Ruby負例の旧u022はliteral定義と一致しないため、新しい負例の根拠に流用しません。上記の既存default規則試験のcomment/identifier負例を使います。']
(out/'FORMAT-MATRIX.md').write_text('\n'.join(rows)+'\n')
# All reviewed parameter memberships; rows overlap deliberately, never sum them as units.
alg=[]
for a in ['RSA','DSA','ECDSA','ECDH','DH','EdDSA','ML-KEM','ML-DSA','SLH-DSA','Hybrid','pre_standard','unknown','AES','HMAC']:
 us=[u for u in flat if a in u['algorithms']]
 params=sorted({p for u in us for p in u['parameters']})
 row=dict(algorithm=a,units=[u['dataset']+'/'+u['id'] for u in us],labels=dict(collections.Counter(u['label'] for u in us)),parameters=[])
 for p in params:
  pu=[u for u in us if p in u['parameters']];row['parameters'].append(dict(parameter=p,byLabel={l:[u['dataset']+'/'+u['id'] for u in pu if u['label']==l] for l in ['positive','negative','ambiguous','unsupported']},rules=sorted({u['ruleId'] for u in pu}),scope='Reviewed visible source/attribute value, not validated runtime parameter or classifier accuracy'))
 row['evaluationScope']=dict(ECDH='Javaの古典方式限定設定を正例、無効設定やhybridを負例に使用。一般ECDHのソースAPI専用規則はなく、APIの曲線対照をその検出精度とは扱わない。',EdDSA='一般EdDSAのソースAPI専用規則はない。Ed25519はRSA/PQC規則の負例であり、EdDSA利用の検出性能を証明しない。',Hybrid='最終hybridを網羅するソース専用規則はない。古典TLS限定・DH規則のhybrid対照を確認。文字列の存在は交渉結果ではなく、実診断はTASK-049で評価する。',unknown='不明・動的・不透明な値は不明または適格外に保持。負例は指定した既知の性質がないという意味で、安全判定ではない。',AES='RSA/PQC API参照の対照負例。AES全般の検出器を評価したとは扱わない。',HMAC='JWTのRS256に対するHS256負例。HMAC全般の検出器を評価したとは扱わない。').get(a,'有限な名目的参照・属性と対照を評価。暗号学的正しさや全変種対応を保証しない。')
 row['supportedScopeLabels']=dict(collections.Counter(u['supportedScopeLabel'] for u in us))
 row['existingTests'] = ([dict(file='crates/pqc_scan_detectors/tests/pqc_readiness.rs',test='identifies_draft_algorithms_separately_from_final_mlkem',property='Draft positive and final-hybrid/comment negative',evidence='native-2.log')] if a=='pre_standard' else [])
 if a in ['ML-KEM','ML-DSA','SLH-DSA']: row['existingTests'].append(dict(file='crates/pqc_scan_detectors/tests/pqc_readiness.rs',test='inventories_pqc_in_all_seven_languages_but_not_comments_or_similar_names',property='Ruby ML-DSA-65 positive paired with comment/similar-identifier negatives; per-rule assertions',evidence='native-2.log'))
 row['controlUnitsByRule']={rule:{l:[u['dataset']+'/'+u['id'] for u in flat if u['ruleId']==rule and u['label']==l] for l in ['positive','negative','unsupported','ambiguous']} for rule in sorted({u['ruleId'] for u in us})}
 alg.append(row)
save('ALGORITHM-MATRIX.json',alg)
rows=['# 方式・パラメータ別の受入証拠','','正解は出力前の二者ラベルから固定しました。以下は方式を含む参照の検出評価であり、暗号分類・実行成功・パラメータ抽出の正しさを評価した表ではありません。それらはTASK-056の独立した要求です。全パラメータと対応する規則・対照IDはALGORITHM-MATRIX.jsonに記録しています。','','| 方式 | positive | negative | ambiguous | unsupported | 評価範囲と対応外理由 |','|---|---:|---:|---:|---:|---|']
for a in alg:rows.append('| '+' | '.join([a['algorithm'],*[str(a['labels'].get(l,0)) for l in ['positive','negative','ambiguous','unsupported']],a['evaluationScope']])+' |')
rows+=['','ML-KEMの512/768/1024、ML-DSAの44/65/87、SLH-DSAのSHA2/SHAKE×128/192/256×s/fは、Node上流の明示的な有限配列とコメント負例を対応させています。同じループ由来の3単位が複数変種に属しても、独立標本として増やしません。元の評価結果の分母は246単位のままです。','RSAの2048/3072や曲線名、OAEP/AES-GCM、宣言版などは原文で確認した値です。API規則がkey sizeを抽出・判定する仕様はないため、その数値自体の検出精度には適用しません。証明書のRSA modulus属性は専用規則、署名は別規則で測定します。対象外を見逃しゼロや安全と読み替えません。','方式に対応する規則の正負はJSONのcontrolUnitsByRuleを参照してください。Java SLH-DSA、Rust import、Node finite-loopなどのFNはそのまま残しています。Rubyのcomment負例と旧称のdraft負例はpqc_readiness.rsの現行default規則試験でも検証しています。']
(out/'ALGORITHM-MATRIX.md').write_text('\n'.join(rows)+'\n')
# Complete catalog, separated by dataset: never silently pool historical semantics.
inv={}
for name,d in datasets.items():
 for rule,entry in d['assessment'].get('ruleCoverage',{}).items():inv.setdefault(rule,{})[name]=entry
save('RULE-INVENTORY.json',dict(totalRules=len(inv),rules=inv,note='Each data version is separate. Not Tested means no eligible nominated unit; it is not unsupported or a quality pass. One-sided Partial evidence remains explicit.'))
assert len(inv)==119
save('CASE-MATRIX.json',dict(v2=datasets['v2']['assessment']['byCaseKind'],supplement=datasets['lock-controls']['assessment']['byCaseKind'],unitLedger='UNIT-LEDGER.json',note='Each required category is inventoried, empty holdout cells remain empty. Dynamic source does not become safe negative; failed detections remain errorDetails.'))
tri=json.load(open(r/'docs/quality/task043/TRIAGE.json'));follow=[]
for g in tri['groups']:
 note={'C04':'Ruby literal property defined before v2 output; old API-labelled FP preserved as historical.', 'C06':'Python import and use are independent nominal references; extra source-audited facts added, missing detections retained.', 'C07':'Pinned recovery AST dump now succeeds; see PARSER-DIAGNOSTIC.md and parser-diagnostic.log. Required evaluation reproduced without changing detector.', 'C08':'Incomplete crate reference labelled explicitly under nominal-reference property, not execution.', 'C11':'Explicit restriction intent vs effective config clarified before measurement. Misses retained.', 'C16':'All nominated rules × all files reviewed by two sources before v2 measurement; omitted legitimate facts now units, no unattributed predictions.', 'C17':'Compatibility filename signal evaluated separately from valid public attributes; malformed and encrypted/unencrypted containers retained.'}.get(g['id'],'Historical observations, default-rule semantics and proposed changes preserved. Detector/regex/risk changes are not implemented or claimed by this evaluation.')
 follow.append(dict(id=g['id'],title=g['title'],observationIds=g['observationIds'],originalUnitIds=g['unitIds'],disposition=note,productChange=False))
save('043-FOLLOWUP.json',follow)
print('wrote',len(flat),'units',len(formats),'formats',len(inv),'rules')
