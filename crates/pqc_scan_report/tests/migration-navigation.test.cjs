const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');
const path = require('node:path');
// Minimal host DOM. Assertions exercise the shipped renderer and its event handlers,
// not a reimplementation of its grouping or policy behavior. Actual layout is browser-tested.
function fixture(count=45,dangerous=false,savedStandalone=false,setup=()=>null,presentation={}){
  function node(attrs={}){const handlers={},attributes=new Map(Object.entries(attrs));return {innerHTML:'',value:'',lang:'en',dataset:{},open:false,handlers,attributes,classList:{contains:name=>name==='migration'},children:[],hasAttribute:k=>attributes.has(k),setAttribute(k,v){attributes.set(k,v)},addEventListener(k,f,options){(handlers[k]||=[]).push(f);options?.signal?.addEventListener('abort',()=>{handlers[k]=handlers[k].filter(x=>x!==f);});},replaceChildren(){this.innerHTML='';this.children=[]},append(...v){this.children.push(...v)},focus(){},setSelectionRange(){},showModal(){this.open=true},close(){this.open=false},getBoundingClientRect(){return {left:100,right:900,top:50,bottom:650}},closest(){return this},click(){}};}
  const root=node();root.nodes=new Map();root.querySelector=selector=>{if(!root.nodes.has(selector)){const n=node();if(selector==='dialog'){const body=node();n.querySelector=()=>body;}root.nodes.set(selector,n);}return root.nodes.get(selector);};
  const targets=Array.from({length:count},(_,i)=>({id:'t'+i,technical:{identifier:i===count-1?'ML-DSA':'RSA',purpose:'signature',quantum:'vulnerable',encoding_assurance:'unknown',classical_concerns:['public_key_below_local_minimum'],components:[{role:'classical',assessment:{identifier:'X25519',quantum:'vulnerable'}},{role:'post_quantum',assessment:{identifier:'ML-KEM-768',quantum:'designed_resistant'}}],combiner_assessment:'unverified_combiner_no_resistance_inference'},source_ref:{role:'algorithm',path:dangerous?'<img src=x onerror=alert(1)>'+String.fromCharCode(0):`src/file-${i}.java`},observation:{assurance:'syntactic',collection_outcome:'partial'},limitations:[]}));
  const decisions=targets.map((target,i)=>({id:'d'+i,target_id:target.id,system_id:'system-'+i,urgency:i%2?'urgent':'plan',information:'incomplete',readiness:'unknown',reason_codes:['insufficient_information'],missing_inputs:[],applied_claim_ids:[],source_ids:[],action:{kind:'investigate',validation_steps:['check'],blocking_prerequisites:[],candidate_ids:[]}}));
  const source={as_of:'2026-11-19T00:00:00Z',targets,decisions,claims:Array.from({length:61},(_,i)=>({id:'c'+i,field:'use_case',value:'signature',freshness:'active',provenance:{actor_id:'owner',source_kind:'operator_claim',asserted_at:'2026-11-18T00:00:00Z'}})),sources:[],candidates:[],coverage:{collection_outcome:'partial'},limitations:[],input:{collection_time:'2026-11-18T00:00:00Z',sha256:'digest'},catalog:{id:'catalog'},policy:{id:'policy'},model:{id:'model'},large:18446744073709551615n};
  const grouping=setup(source);
  const nodes=[],memo=new Map();function intern(v){let n;if(v===null)n=['z'];else if(Array.isArray(v))n=['a',v.map(intern)];else if(typeof v==='object')n=['o',Object.entries(v).flatMap(([k,v])=>[intern(k),intern(v)])];else if(typeof v==='bigint'||typeof v==='number')n=['n',String(v)];else n=[typeof v==='string'?'s':'b',v];const key=JSON.stringify(n);if(memo.has(key))return memo.get(key);const i=nodes.length;nodes.push(n);memo.set(key,i);return i;}
  const words={signature:{en:'Signing',ja:'署名'},algorithm:{en:'Algorithm',ja:'暗号方式'},insufficient_information:{en:'Missing facts',ja:'不足情報'},check:{en:'Check use',ja:'利用を確認'}};
  const payload={archive:{root:intern(source),nodes},groups:grouping||decisions.map((_,i)=>({members:[i]})),words,states:{en:{'action:investigate':'Investigate','urgency:urgent':'Urgent','urgency:plan':'Plan','information:incomplete':'Missing information','readiness:unknown':'Unknown'},ja:{'action:investigate':'調査','urgency:urgent':'早急な対応','urgency:plan':'計画','information:incomplete':'情報不足','readiness:unknown':'不明'}}};
  Object.assign(payload,presentation);
  const events={},windowEvents={},location={hash:''};let saved;
  const context={AbortController,Blob,location,setTimeout:f=>{f();},URL:{createObjectURL:b=>{saved=b;return 'blob:fixture'},revokeObjectURL(){}},window:{addEventListener(k,f){windowEvents[k]=f}},document:{documentElement:{dataset:{savedStandalone:String(savedStandalone)}},getElementById:()=>null,createElement:()=>node(),addEventListener(k,f){events[k]=f}}};
  vm.runInNewContext(fs.readFileSync(path.join(__dirname,'../src/migration.js'),'utf8'),context);
  const instance=context.window.PqcMigrationReport.mount(payload,root,true);
  const event=(kind,target,extra={})=>(root.handlers[kind]||[]).forEach(f=>f({target,button:0,...extra}));
  function click(dataset={},attributes=[]){const control=node(Object.fromEntries(attributes.map(k=>[k,''])));control.dataset=dataset;event('click',control);}
  return {root,instance,payload,context,event,click,dialog:()=>root.querySelector('dialog'),body:()=>root.querySelector('dialog').querySelector('[data-dialog-body]'),search(value){const control=node({'data-work-search':''});control.value=value;event('input',control)},urgency(value){const control=node({'data-work-urgency':''});control.value=value;event('change',control)},jump(hash){location.hash=hash;windowEvents.hashchange()},saved:()=>saved};
}
test('work pages are bounded and language changes preserve the selected page',()=>{
  const f=fixture();assert.equal((f.root.innerHTML.match(/class="decision-card"/g)||[]).length,20);assert.match(f.root.innerHTML,/1-20 \/ 45/);assert.doesNotMatch(f.root.innerHTML,/id="target-/);
  f.click({page:'1'});assert.match(f.root.innerHTML,/21-40 \/ 45/);assert.equal((f.root.innerHTML.match(/class="decision-card"/g)||[]).length,20);
  f.instance.setLanguage('ja');assert.match(f.root.innerHTML,/次に確認すること/);assert.match(f.root.innerHTML,/21-40 \/ 45/);
});
test('algorithm and location search reach a last-page group; filters and empty results remain exact',()=>{
  const f=fixture();f.search('ML-DSA');assert.equal((f.root.innerHTML.match(/class="decision-card"/g)||[]).length,1);assert.match(f.root.innerHTML,/work-45/);f.urgency('urgent');assert.equal((f.root.innerHTML.match(/class="decision-card"/g)||[]).length,0);assert.match(f.root.innerHTML,/No matching work items/);f.urgency('');f.search('file-44.java');assert.match(f.root.innerHTML,/work-45/);
});
test('exact target and paged claim fragments remain accessible without loading every detail',()=>{
  const f=fixture();f.jump('#target-45');assert.match(f.body().innerHTML,/id="target-45"/);assert.equal(f.dialog().open,true);f.click({},['data-close']);assert.equal(f.dialog().open,false);
  f.jump('#claim-51');assert.match(f.body().innerHTML,/id="claim-51"/);assert.doesNotMatch(f.body().innerHTML,/id="claim-1"/);assert.equal((f.body().innerHTML.match(/<section id="claim-/g)||[]).length,20);
});
test('empty input has zero work items and no fabricated detail',()=>{
  const f=fixture(0);assert.equal((f.root.innerHTML.match(/class="decision-card"/g)||[]).length,0);assert.match(f.root.innerHTML,/0 \/ 0/);f.jump('#target-1');assert.equal(f.dialog().open,false);
});
test('the shared archive exports the original assessment with exact unsafe numeric lexemes',async()=>{
  const f=fixture();f.click({},['data-download']);const text=await f.saved().text();assert.match(text,/"large":18446744073709551615/);const decoded=JSON.parse(text);assert.equal(decoded.targets.length,45);assert.equal(decoded.decisions.length,45);assert.equal(decoded.claims.length,61);
});
test('destroying a mounted report removes its handlers before a new run',()=>{
  const f=fixture();f.instance.destroy();assert.equal(f.root.innerHTML,'');for(const callbacks of Object.values(f.root.handlers))assert.equal(callbacks.length,0);
});

test('untrusted record text is escaped rather than becoming markup',()=>{
  const f=fixture(1,true);f.jump('#target-1');assert.match(f.body().innerHTML,/&lt;img/);assert.match(f.body().innerHTML,/\\u\{0\}/);assert.doesNotMatch(f.body().innerHTML,/<img/);
});

test('selected evidence keeps exact classical concerns and hybrid components without upgrading combined resistance',()=>{
 const f=fixture();f.jump('#target-1');const html=f.body().innerHTML;assert.match(html,/Classical concern/);assert.match(html,/public_key_below_local_minimum/);assert.match(html,/X25519/);assert.match(html,/ML-KEM-768/);assert.match(html,/unverified_combiner_no_resistance_inference/);assert.equal((html.match(/id="target-/g)||[]).length,1);
});


test('standalone saved HTML retains embedded evidence and JSON export without a missing Markdown sidecar link',async()=>{
 const f=fixture(2,false,true);
 assert.doesNotMatch(f.root.innerHTML,/migration\.assessment\.md/);
 f.jump('#target-1');assert.match(f.body().innerHTML,/id="target-1"/);
 f.click({},['data-download']);assert.equal(JSON.parse(await f.saved().text()).decisions.length,2);
 assert.match(fixture(2).root.innerHTML,/migration\.assessment\.md/);
});

test('one or zero work items omit ineffective search and urgency controls',()=>{
 for(const count of [0,1]) {const f=fixture(count);assert.doesNotMatch(f.root.innerHTML,/data-work-search|data-work-urgency/);}
 const f=fixture(2,false,false,s=>{s.decisions.forEach(d=>d.urgency='undetermined');});
 assert.match(f.root.innerHTML,/data-work-search/);assert.doesNotMatch(f.root.innerHTML,/data-work-urgency/);assert.match(f.root.innerHTML,/All work items/);
});
test('urgency options include only actual distinct states and show work-item counts',()=>{
 const f=fixture(4);assert.match(f.root.innerHTML,/value="urgent"[^>]*>Urgent \(2 items\)/);assert.match(f.root.innerHTML,/value="plan"[^>]*>Plan \(2 items\)/);
 assert.doesNotMatch(f.root.innerHTML,/<option value="monitor"/);assert.doesNotMatch(f.root.innerHTML,/<option value="undetermined"/);
 assert.match(f.root.innerHTML,/placeholder="e.g. RSA, encryption, package.json"/);
});
test('evaluation time is explicitly labelled Japan time and exact UTC is retained',()=>{
 const f=fixture(1);assert.match(f.root.innerHTML,/Evaluation time: 19\/11\/2026, 09:00:00 \(Japan time, UTC\+09:00\)/);
 assert.match(f.root.innerHTML,/datetime="2026-11-19T00:00:00Z"/);
 f.instance.setLanguage('ja');assert.match(f.root.innerHTML,/評価日時: 2026\/11\/19 09:00:00 （日本時間）/);
});
test('mixed known declarations are summarized across every member without upgrading observations or using stale claims',async()=>{
 const f=fixture(2,false,false,s=>{
  s.targets.forEach(t=>{t.technical.identifier='unknown';t.technical.purpose='unknown';t.source_ref.role='reference-only';t.source_ref.path='package.json';});
  s.claims=[{id:'e0',field:'use_case',value:'encryption',state:'known',freshness:'active'}, {id:'e1',field:'use_case',value:'signature',state:'known',freshness:'active'}, {id:'old',field:'migration_target',value:{use_case:'signature',algorithm:'STALE-SECRET',parameters:'old'},state:'known',freshness:'stale'}];
  s.decisions.forEach((d,i)=>{d.urgency='undetermined';d.missing_inputs=['technical.identity','technical.parameters'];d.reason_codes=['insufficient_information','reference_only'];d.applied_claim_ids=['e'+i,'old'];});return [{members:[0,1]}];
 });
 assert.match(f.root.innerHTML,/Declared purpose/);assert.match(f.root.innerHTML,/encryption \(1 \/ 2 records\)/);assert.match(f.root.innerHTML,/Signing \(1 \/ 2 records\)/);
 assert.match(f.root.innerHTML,/Observed purpose: Unidentified/);assert.doesNotMatch(f.root.innerHTML,/STALE-SECRET/);assert.match(f.root.innerHTML,/Locate where the referenced dependency is used/);
 assert.match(f.root.innerHTML,/Facts still needed/);assert.match(f.root.innerHTML,/Full assessment reasoning/);
 assert.doesNotMatch(f.root.innerHTML,/system-0 ·/);
 f.click({},['data-download']);const exported=JSON.parse(await f.saved().text());assert.equal(exported.decisions[0].urgency,'undetermined');assert.equal(exported.claims[2].freshness,'stale');
});
test('search reaches declared purposes and configuration while observed identity remains separate',()=>{
 const f=fixture(3,false,false,s=>{
  s.claims=[{id:'a',field:'use_case',value:'encryption',state:'known',freshness:'active'},{id:'b',field:'migration_target',value:{use_case:'encryption',algorithm:'AES',parameters:'AES-256'},state:'known',freshness:'active'}];s.decisions[2].applied_claim_ids=['a','b'];
 });
 f.search('AES-256');assert.match(f.root.innerHTML,/id="work-3"/);assert.equal((f.root.innerHTML.match(/class="decision-card"/g)||[]).length,1);assert.match(f.root.innerHTML,/Proposed configuration \(declared\)/);
 f.search('encryption');assert.match(f.root.innerHTML,/id="work-3"/);
});

test('known purpose is not asked again when only support or policy facts are missing; urgency reason stays visible',()=>{
 const f=fixture(2,false,false,s=>{
  s.claims=[{id:'declared',field:'use_case',value:'signature',state:'known',freshness:'active'}];s.decisions.forEach(d=>{d.applied_claim_ids=['declared'];d.missing_inputs=['support.runtime'];d.action.validation_steps=['confirm_actual_use_and_required_purpose'];});
  s.decisions[0].urgency='urgent';s.decisions[0].reason_codes=['known_classical_concern'];
 });
 assert.match(f.root.innerHTML,/Investigate the missing facts above/);assert.match(f.root.innerHTML,/decision-basis">known_classical_concern/);assert.match(f.root.innerHTML,/Investigate the detected cryptographic concern/);
 const first=f.root.innerHTML.split('<section class="next-checks">')[1].split('</section>')[0];assert.doesNotMatch(first,/confirm_actual_use_and_required_purpose/);
 assert.match(f.root.innerHTML,/confirm_actual_use_and_required_purpose/,'original steps retained in detailed reasoning');
});

test('technical encoding and selected negotiation require evidence as well as any declarations',()=>{
 const f=fixture(1,false,false,s=>{s.decisions[0].missing_inputs=['technical.encoding','observation.selected_use'];});
 assert.match(f.root.innerHTML,/technical evidence or declarations/);
 assert.match(f.root.innerHTML,/technical.encoding/);assert.match(f.root.innerHTML,/observation.selected_use/);
});
test('declaration variants are bounded and escaped, with unknown claims excluded and all values retained in export',async()=>{
 const f=fixture(7,false,false,s=>{
  s.claims=s.decisions.map((d,i)=>({id:'config'+i,field:'migration_target',state:'known',freshness:'active',value:{use_case:'encryption',algorithm:'AES',parameters:i===0?'<img src=x onerror=alert(1)>':'config-'+i}}));
  s.claims.push({id:'unknown',field:'use_case',state:'unknown',freshness:'active',value:'SHOULD-NOT-BE-CURRENT'});
  s.decisions.forEach((d,i)=>{d.applied_claim_ids=['config'+i,'unknown'];d.urgency='undetermined';});return [{members:[0,1,2,3,4,5,6]}];
 });
 assert.match(f.root.innerHTML,/&lt;img src=x onerror=alert\(1\)&gt;/);assert.doesNotMatch(f.root.innerHTML,/<img/);
 assert.match(f.root.innerHTML,/2 more variants/);assert.doesNotMatch(f.root.innerHTML,/config-6/);assert.doesNotMatch(f.root.innerHTML,/SHOULD-NOT-BE-CURRENT/);
 f.click({},['data-download']);const d=JSON.parse(await f.saved().text());assert.equal(d.claims[6].value.parameters,'config-6');assert.equal(d.claims[7].state,'unknown');
});

test('development dependency declarations and exact lines are shown without inferring production exclusion',()=>{
 const f=fixture(1,false,false,s=>{s.targets[0].source_ref.dependency_provenance='manifest_dev_declared';s.targets[0].source_ref.path='package.json';s.targets[0].source_ref.line=16;});
 assert.match(f.root.innerHTML,/Declared development dependencies: 1 \/ 1 records/);assert.match(f.root.innerHTML,/package.json:16/);
 f.jump('#target-1');assert.match(f.body().innerHTML,/Declared development dependency/);
 assert.doesNotMatch(f.root.innerHTML,/not used in production/);
});

test('binding limitations are explicit and source references are not called dependencies',()=>{
 const f=fixture(2,false,false,s=>{s.targets.forEach(t=>{t.source_ref.role='reference-only';t.technical.identifier='unknown'});s.decisions.forEach(d=>{d.reason_codes=['reference_only'];d.missing_inputs=['technical.identity','use_case'];d.urgency='undetermined'});return [{members:[0,1]}]}, {unbindable:['t0','t1']});
 f.instance.setLanguage('ja');assert.match(f.root.innerHTML,/この画面で回答を追加しても解消できません/);assert.match(f.root.innerHTML,/対応付けできない記録: 2/);assert.doesNotMatch(f.root.innerHTML,/該当する依存関係の利用箇所/);
 f.click({work:'0'});f.jump('#target-1');assert.match(f.body().innerHTML,/この画面で回答を追加しても解消できません/);
 const ordinary=fixture(1,false,false,s=>{s.targets[0].source_ref.role='reference-only';s.decisions[0].reason_codes=['reference_only'];s.decisions[0].missing_inputs=['technical.identity']});ordinary.instance.setLanguage('ja');assert.match(ordinary.root.innerHTML,/参照しているソース/);assert.doesNotMatch(ordinary.root.innerHTML,/この画面で回答を追加しても解消できません/);
});
test('applied planning claims retain mixed deadlines and human duration without changing exports',async()=>{
 const f=fixture(2,false,false,s=>{s.claims=[];s.targets[1].source_ref.path=s.targets[0].source_ref.path;s.targets.forEach(t=>t.source_ref.line=96);s.decisions.forEach((d,i)=>{for(const [field,value] of [['deadline',{kind:'at',at:i?'2028-03-31T14:59:59Z':'2027-03-31T14:59:59Z',scope_reason:'organization_policy',reference_id:'policy'}],['migration_lead_seconds',i?7776000:15552000]]){const id=`${i}-${field}`;s.claims.push({id,field,value,state:'known',freshness:'active'});d.applied_claim_ids.push(id)}});return [{members:[0,1]}]});
 f.instance.setLanguage('ja');assert.match(f.root.innerHTML,/180日/);assert.match(f.root.innerHTML,/90日/);assert.match(f.root.innerHTML,/2027\/03\/31/);assert.match(f.root.innerHTML,/2028\/03\/31/);assert.match(f.root.innerHTML,/記録された場所: 1か所/);assert.match(f.root.innerHTML,/1 \/ 2 記録/);f.click({},['data-download']);assert.equal(JSON.parse(await f.saved().text()).claims[1].value,15552000);
});
test('scope labels are escaped, retained in the view and identify standalone report titles',()=>{
 const f=fixture(1,false,false,()=>null,{label:'Keycloak / JWT <img onerror=x>'});f.instance.setLanguage('ja');assert.match(f.root.innerHTML,/Keycloak \/ JWT &lt;img onerror=x&gt;/);assert.doesNotMatch(f.root.innerHTML,/<img/);
 f.jump('#target-1');assert.match(f.body().innerHTML,/検出方法/);
});

test('the shipped presentation controller keeps rendered scope in initial and language-changed titles',()=>{
 const f=fixture(1,false,false,()=>null,{label:'Keycloak / JWT'});
 const lookup=f.root.querySelector;
 f.root.querySelector=selector=>{if(selector==='h1')return {textContent:f.root.innerHTML.match(/<h1>(.*?)<\/h1>/)[1]};if(selector==='.report-scope')return {textContent:f.root.innerHTML.match(/class="report-scope">(.*?)<\/p>/)[1]};return lookup(selector)};
 f.root.querySelectorAll=()=>[];const language={value:'en',addEventListener(_,f){this.change=f}},theme={value:'light',addEventListener(){}};
 const settings={querySelector:s=>s==='[data-language]'?language:theme},alternate={lang:'ja'};
 f.context.document.querySelector=s=>s==='[data-settings]'?settings:s==='[data-report-translation]'?alternate:f.root;
 vm.runInNewContext(fs.readFileSync(path.join(__dirname,'../src/presentation.js'),'utf8'),f.context);
 assert.equal(f.context.document.title,'Keycloak / JWT | Migration response work items');
 f.instance.setLanguage('ja');language.value='ja';language.change();assert.equal(f.context.document.title,'Keycloak / JWT | 移行に向けた確認事項');
});

test('identical planning pairs aggregate even when applied claim order differs',()=>{
 const f=fixture(2,false,false,s=>{s.claims=[];s.decisions.forEach((d,i)=>{d.applied_claim_ids=[];const pairs=[['deadline',{kind:'at',at:'2027-03-31T14:59:59Z',scope_reason:'organization_policy',reference_id:'policy'}],['migration_lead_seconds',15552000]];if(i)pairs.reverse();for(const [field,value] of pairs){const id=`${i}-${field}`;s.claims.push({id,field,value,state:'known',freshness:'active'});d.applied_claim_ids.push(id)}});return [{members:[0,1]}]});f.instance.setLanguage('ja');assert.equal((f.root.innerHTML.match(/移行に必要な期間: 180日/g)||[]).length,1);assert.match(f.root.innerHTML,/2 \/ 2 記録/);
});

test('zero assessment targets are distinct from filtered results and do not imply safety',()=>{
 const empty=fixture(0);assert.match(empty.root.innerHTML,/No assessment targets were found/);assert.match(empty.root.innerHTML,/does not establish that cryptography is absent, safe, or needs no action/);assert.doesNotMatch(empty.root.innerHTML,/No matching work items/);empty.instance.setLanguage('ja');assert.match(empty.root.innerHTML,/この入力から評価対象を検出できませんでした/);assert.match(empty.root.innerHTML,/対応不要と確認できた結果ではありません/);
 const filtered=fixture(2);filtered.search('zero-match-2026');assert.match(filtered.root.innerHTML,/No matching work items/);assert.doesNotMatch(filtered.root.innerHTML,/No assessment targets were found/);filtered.instance.setLanguage('ja');assert.match(filtered.root.innerHTML,/一致する確認項目はありません/);assert.doesNotMatch(filtered.root.innerHTML,/この入力から評価対象を検出できませんでした/);
});
