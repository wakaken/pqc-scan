const { test } = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');
const path = require('node:path');
// Minimal host DOM. Assertions exercise the shipped renderer and its event handlers,
// not a reimplementation of its grouping or policy behavior. Actual layout is browser-tested.
function fixture(count=45,dangerous=false,savedStandalone=false,setup=()=>null,presentation={}){
  function node(attrs={}){const handlers={},attributes=new Map(Object.entries(attrs));return {innerHTML:'',value:'',lang:'en',dataset:{},open:false,handlers,attributes,classList:{contains:name=>name==='migration'},children:[],hasAttribute:k=>attributes.has(k),setAttribute(k,v){attributes.set(k,v)},addEventListener(k,f,options){(handlers[k]||=[]).push(f);options?.signal?.addEventListener('abort',()=>{handlers[k]=handlers[k].filter(x=>x!==f);});},replaceChildren(){this.innerHTML='';this.children=[]},append(...v){this.children.push(...v)},focus(){this.focused=true;},setSelectionRange(){},showModal(){this.open=true},close(){this.open=false},getBoundingClientRect(){return {left:100,right:900,top:50,bottom:650}},closest(){return this},click(){}};}
  const root=node();root.nodes=new Map();root.querySelector=selector=>{if(!root.nodes.has(selector)){const n=node();if(selector==='dialog'){const body=node();n.querySelector=()=>body;}root.nodes.set(selector,n);}return root.nodes.get(selector);};
  const targets=Array.from({length:count},(_,i)=>({id:'t'+i,technical:{identifier:i===count-1?'ML-DSA':'RSA',purpose:'signature',quantum:'vulnerable',encoding_assurance:'unknown',classical_concerns:['public_key_below_local_minimum'],components:[{role:'classical',assessment:{identifier:'X25519',quantum:'vulnerable'}},{role:'post_quantum',assessment:{identifier:'ML-KEM-768',quantum:'designed_resistant'}}],combiner_assessment:'unverified_combiner_no_resistance_inference'},source_ref:{role:'algorithm',path:dangerous?'<img src=x onerror=alert(1)>'+String.fromCharCode(0):`src/file-${i}.java`},observation:{method:'ast_pattern',assurance:'syntactic',collection_outcome:'partial'},limitations:[]}));
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
  const instance=context.window.PqcMigrationReport.mount(payload,root,presentation.standaloneMount !== false,"",Boolean(presentation.hasScanReport));
  const event=(kind,target,extra={})=>(root.handlers[kind]||[]).forEach(f=>f({target,button:0,...extra}));
  function click(dataset={},attributes=[]){const control=node(Object.fromEntries(attributes.map(k=>[k,''])));control.dataset=dataset;event('click',control);}
  return {root,instance,payload,context,event,click,dialog:()=>root.querySelector('dialog'),body:()=>root.querySelector('dialog').querySelector('[data-dialog-body]'),jump(hash){location.hash=hash;windowEvents.hashchange()},saved:()=>saved};
}
test('work pages are bounded and language changes preserve the selected page',()=>{
  const f=fixture();assert.equal((f.root.innerHTML.match(/class="decision-card"/g)||[]).length,20);assert.match(f.root.innerHTML,/1-20 \/ 45/);assert.doesNotMatch(f.root.innerHTML,/id="target-/);
  f.click({page:'1'});assert.match(f.root.innerHTML,/21-40 \/ 45/);assert.equal((f.root.innerHTML.match(/class="decision-card"/g)||[]).length,20);
  f.instance.setLanguage('ja');assert.match(f.root.innerHTML,/次に進める条件/);assert.match(f.root.innerHTML,/21-40 \/ 45/);
});
test('work list omits search and urgency controls while preserving pages',()=>{
  const f=fixture();
  assert.doesNotMatch(f.root.innerHTML,/data-work-search|data-work-urgency/);
  assert.match(f.root.innerHTML,/1-20 \/ 45/);
  f.click({page:'1'});f.click({page:'1'});
  assert.match(f.root.innerHTML,/41-45 \/ 45/);
  const single=fixture(1);
  assert.doesNotMatch(single.root.innerHTML,/data-page=|data-work-search|data-work-urgency/);
});

test('exact target and paged claim fragments remain accessible without loading every detail',()=>{
  const f=fixture();f.jump('#target-45');assert.match(f.body().innerHTML,/id="target-45"/);assert.equal(f.dialog().open,true);f.click({},['data-close']);assert.equal(f.dialog().open,false);
  f.jump('#claim-51');assert.match(f.body().innerHTML,/id="claim-51"/);assert.doesNotMatch(f.body().innerHTML,/id="claim-1"/);assert.equal((f.body().innerHTML.match(/<section id="claim-/g)||[]).length,20);
});
test('detection locations open concise record details and keep archived source references',async()=>{
  const f=fixture(1,false,false,s=>{
    s.sources=[{id:'test-citation',uri:'https://example.invalid/citation',edition:'v1',clause:'example',scope_limit:'example'}];
    s.decisions[0].source_ids=['test-citation'];
  });
  for (const [lang, button, title, reasons, method, below] of [
    ['ja','検出箇所を見る','検出箇所','判定理由','検出方法',/出典|参照情報|資料を開く|根拠となる記録/],
    ['en','View detection locations','Detection locations','Decision reasons','Observation method',/Sources|Reference identities|Open reference/],
  ]) {
    f.instance.setLanguage(lang);
    assert.match(f.root.innerHTML,new RegExp('data-work="0">'+button+'<'));
    f.click({work:'0'});
    assert.match(f.body().innerHTML,new RegExp('<h2>'+title+'</h2>'));
    assert.equal(f.dialog().attributes.get('aria-label'),title);
    f.click({target:'0'});
    const detail=f.body().innerHTML;
    for (const text of ['id="target-1"', 'src/file-0.java', reasons, method]) assert.ok(detail.includes(text),text);
    assert.doesNotMatch(detail,below);
    assert.doesNotMatch(detail,/test-citation|id="evidence-1"/);
    f.click({},['data-list']);
    assert.match(f.body().innerHTML,new RegExp('<h2>'+title+'</h2>'));
    f.click({},['data-close']);
  }
  f.jump('#evidence-1');
  assert.match(f.body().innerHTML,/id="target-1"/);
  f.jump('#source-1');
  assert.match(f.body().innerHTML,/test-citation/);
  f.click({},['data-download']);
  const data=JSON.parse(await f.saved().text());
  assert.deepEqual(data.decisions[0].source_ids,['test-citation']);
  assert.equal(data.sources[0].id,'test-citation');
});
test('empty input has zero work items and no fabricated detail',()=>{
  const f=fixture(0);assert.equal((f.root.innerHTML.match(/class="decision-card"/g)||[]).length,0);assert.doesNotMatch(f.root.innerHTML,/data-page=/);f.jump('#target-1');assert.equal(f.dialog().open,false);
});

function workflowInput(source, use = 'jwt', action = 'unblock') {
  source.claims = [
    {id:'purpose',field:'use_case',value:use,state:'known',freshness:'active'},
    {id:'usage',field:'usage_state',value:'active',state:'known',freshness:'active'},
  ];
  source.decisions.forEach(d=>{
    Object.assign(d,{system_id:'confirmed-flow',urgency:action==='monitor'?'monitor':'urgent',applied_claim_ids:['purpose','usage']});
    d.action.kind = action;
  });
  if (use === 'encryption') source.targets.forEach(t=>t.technical.purpose='encryption');
}
test('a confirmed use shows migration decisions once and preserves exact evidence across recorded roles',()=>{
  const f=fixture(3,false,false,source=>{
    workflowInput(source);
    source.targets[1].source_ref.role='reference-only';
    source.decisions[2].applied_claim_ids=[];
    source.decisions[2].urgency='undetermined';
  });
  f.instance.setLanguage('ja');
  const html=f.root.innerHTML;
  assert.equal((html.match(/class="migration-workflow"/g)||[]).length,1);
  assert.equal((html.match(/data-migration-phase=/g)||[]).length,3);
  assert.match(html,/最初に着手[\s\S]*やること[\s\S]*次に進める条件/);
  assert.match(html,/候補方式のトークン[\s\S]*既存のトークン/);
  assert.match(html,/ほかの確認項目/);
  f.click({work:'0'});
  assert.match(f.body().innerHTML,/file-0\.java/);
  assert.match(f.body().innerHTML,/file-1\.java/);
  assert.doesNotMatch(f.body().innerHTML,/file-2\.java/);
  f.jump('#work-2');
  assert.equal(f.root.querySelector('#work-1').focused,true);
});
test('expired, independent and differently assessed declarations do not borrow the workflow',()=>{
  const f=fixture(5,false,false,source=>{
    workflowInput(source);
    source.claims.push({...source.claims[0],id:'stale-purpose',freshness:'stale'});
    source.decisions[1].system_id='different-flow';
    source.decisions[2].applied_claim_ids=['stale-purpose','usage'];
    source.decisions[2].urgency='undetermined';
    source.decisions[3].action.kind='investigate';
    source.decisions[4].urgency='plan';
  });
  assert.equal((f.root.innerHTML.match(/class="migration-workflow"/g)||[]).length,3);
  assert.match(f.root.innerHTML,/Other checks/);
  f.click({work:'0'});
  assert.doesNotMatch(f.body().innerHTML,/file-[1-4]\.java/);
});
test('encryption monitoring and declared exclusions show their own next condition',()=>{
  const monitor=fixture(2,false,false,s=>workflowInput(s,'encryption','monitor'));
  monitor.instance.setLanguage('ja');
  assert.match(monitor.root.innerHTML,/暗号化設定を継続確認する[\s\S]*次に進める条件[\s\S]*上書き/);
  assert.doesNotMatch(monitor.root.innerHTML,/候補方式のトークン/);
  const excluded=fixture(2,false,false,s=>{
    workflowInput(s,'jwt','none');
    s.claims=[{id:'scope',field:'applicability',value:'not_applicable',state:'known',freshness:'active'}];
    s.decisions.forEach(d=>Object.assign(d,{urgency:'not_applicable',applied_claim_ids:['scope']}));
    s.decisions[0].reason_codes.push('reference_only','unknown_primitive');
    s.targets[0].technical.purpose='unknown';
  });
  excluded.instance.setLanguage('ja');
  assert.match(excluded.root.innerHTML,/申告した対象外の範囲を確認する[\s\S]*次に進める条件[\s\S]*再評価/);
  assert.equal((excluded.root.innerHTML.match(/data-migration-phase=/g)||[]).length,1);
  assert.match(excluded.root.innerHTML,/unknown_primitive/);
  excluded.click({work:'0'});
  assert.match(excluded.body().innerHTML,/file-0\.java/);
  assert.match(excluded.body().innerHTML,/file-1\.java/);
});

test('pending candidate support is a rollout gate and urgent unbound checks are visible with urgent workflows',()=>{
  const f=fixture(4,false,false,s=>{
    workflowInput(s);
    s.decisions[0].action.blocking_prerequisites=['jose','verifiers'];
    s.decisions[1].system_id='monitored';
    s.decisions[1].action.kind='monitor';
    s.decisions[1].urgency='monitor';
    s.decisions[2].applied_claim_ids=[];
    s.decisions[2].action.kind='investigate';
    s.decisions[3].applied_claim_ids=[];
    s.decisions[3].urgency='undetermined';
  });
  f.instance.setLanguage('ja');
  const html=f.root.innerHTML;
  assert.match(html,/試験する候補方式と鍵形式[\s\S]*互換性試験/);
  assert.match(html,/JOSE|JWT実装/);
  assert.match(html,/未対応の条件を解消/);
  assert.match(html,/<a href="#work-3" data-check-work="2">[^<]*早急/);
  const disclosureStart=html.indexOf('<details data-other-work>');
  assert.ok(disclosureStart>0, 'the remaining checks start collapsed');
  const visible=html.slice(0,disclosureStart);
  assert.match(visible,/data-urgency-group="urgent"[^>]*>[\s\S]*? · 2 件<\/h3>/);
  assert.ok(visible.indexOf('id="work-1"')<visible.indexOf('id="work-3"'));
  assert.match(visible,/id="work-3"/);
  assert.doesNotMatch(visible,/id="work-4"/);
  assert.match(html.slice(disclosureStart),/ほかの確認項目 · 1 件[\s\S]*?id="work-4"/);
});

test('differences in blockers or candidate prerequisites keep their own judgments and evidence',()=>{
  for (const variant of ['blocker','candidate']) {
    const f=fixture(3,false,false,s=>{
      workflowInput(s);
      s.decisions.forEach((d,i)=>{
        const prerequisite=i===1?'verifiers':'jose';
        if (variant==='blocker') {
          d.action.blocking_prerequisites=[prerequisite];
        } else {
          const id='candidate-'+i;
          s.candidates.push({id,identifier:'ML-DSA',purpose:'signature',status:'consider',
            parameter_selection:'ML-DSA-65',prerequisites:[{name:prerequisite,claimed_support:null}],validation_steps:[]});
          d.action.candidate_ids=[id];
        }
      });
    });
    assert.equal((f.root.innerHTML.match(/class="migration-workflow"/g)||[]).length,2,variant);
    f.click({work:'0'});
    assert.match(f.body().innerHTML,/file-0\.java/,variant);
    assert.match(f.body().innerHTML,/file-2\.java/,variant);
    assert.doesNotMatch(f.body().innerHTML,/file-1\.java/,variant);
    f.click({},['data-close']);
    f.click({work:'1'});
    assert.match(f.body().innerHTML,/file-1\.java/,variant);
    assert.doesNotMatch(f.body().innerHTML,/file-[02]\.java/,variant);
  }
});

test('one original evidence group can contain different declared uses without mixing their workflows',()=>{
  const f=fixture(3,false,false,s=>{
    workflowInput(s);
    s.claims.push({id:'signing',field:'use_case',value:'signature',state:'known',freshness:'active'});
    s.decisions[1].applied_claim_ids=['signing','usage'];
    s.decisions[2].applied_claim_ids=[];
    return [{members:[0,1,2]}];
  });
  assert.equal((f.root.innerHTML.match(/class="migration-workflow"/g)||[]).length,2);
  f.click({work:'1'});
  assert.match(f.body().innerHTML,/file-1\.java/);
  assert.doesNotMatch(f.body().innerHTML,/file-[02]\.java/);
});

test('applied protection conditions separate judgments and preserve observed-purpose differences',()=>{
  const f=fixture(3,false,false,s=>{
    workflowInput(s);
    for (const [id, year] of [['verify-a',2027],['verify-b',2032]]) {
      s.claims.push({id,field:'signature_verification_until',value:year+'-01-01T00:00:00Z',state:'known',freshness:'active'});
    }
    s.targets[0].technical.purpose='unknown';
    s.decisions.forEach((d,i)=>d.applied_claim_ids.push(i<2?'verify-a':'verify-b'));
  });
  assert.equal((f.root.innerHTML.match(/class="migration-workflow"/g)||[]).length,2);
  assert.match(f.root.innerHTML,/Keep existing signatures verifiable until 01\/01\/2027/);
  assert.match(f.root.innerHTML,/Keep existing signatures verifiable until 01\/01\/2032/);
  assert.match(f.root.innerHTML,/Observed purpose: Unidentified \(1 record\) \/ Signing \(1 record\)/);
});

test('urgent unclassified evidence is visible before lower-urgency workflows across pages',()=>{
  const f=fixture(21,false,false,s=>{
    workflowInput(s,'encryption','monitor');
    s.decisions.slice(0,-1).forEach((d,i)=>d.system_id='monitored-'+i);
    Object.assign(s.decisions.at(-1),{urgency:'urgent',applied_claim_ids:[],system_id:null});
    s.decisions.at(-1).action.kind='investigate';
  });
  const work = f.root.innerHTML.indexOf('id="decisions"');
  const urgent = f.root.innerHTML.indexOf('id="work-21"');
  const monitor = f.root.innerHTML.indexOf('class="migration-workflow"');
  assert.ok(work < urgent && urgent < monitor);
  assert.match(f.root.innerHTML,/href="#work-21" data-check-work="20">Go to urgent investigations/);
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


test('migration HTML ends with only assessment export across rendering modes',async()=>{
 for(const [saved,standaloneMount] of [[true,true],[false,true],[false,false]]) {
   const f=fixture(2,false,saved,()=>null,{standaloneMount});
   for(const [locale,label] of [['en','Save assessment data'],['ja','評価データを保存']]) {
     f.instance.setLanguage(locale);
     const html=f.root.innerHTML;
     const footer=html.match(/<footer[^>]*>(.*?)<\/footer>/s)[1].trim();
     assert.equal(footer,'<button data-download>'+label+'</button>');
     assert.doesNotMatch(html,/assessment-scope|data-reference=|migration\.assessment\.md/);
   }
   f.click({},['data-download']);
   assert.equal(JSON.parse(await f.saved().text()).decisions.length,2);
   if(standaloneMount){f.jump('#target-1');assert.match(f.body().innerHTML,/id="target-1"/);}
 }
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
 assert.match(f.root.innerHTML,/Observed purpose: Unidentified/);assert.doesNotMatch(f.root.innerHTML,/STALE-SECRET/);assert.match(f.root.innerHTML,/Check the configured algorithm and parameters there/);
 assert.match(f.root.innerHTML,/Facts still needed/);assert.match(f.root.innerHTML,/Assessment details/);
 assert.doesNotMatch(f.root.innerHTML,/system-0 ·/);
 f.click({},['data-download']);const exported=JSON.parse(await f.saved().text());assert.equal(exported.decisions[0].urgency,'undetermined');assert.equal(exported.claims[2].freshness,'stale');
});
test('mixed investigation gives each evidence type its own action and keeps gaps in details',()=>{
 const f=fixture(3,false,false,s=>{
  s.targets.forEach(t=>{t.source_ref.role='reference-only';t.technical.identifier='unknown';t.technical.purpose='unknown'});
  s.targets[0].source_ref={role:'reference-only',path:'Cargo.lock',dependency_provenance:'lock_resolved'};
  s.targets[1].source_ref.path='src/tls/incoming.rs';
  s.targets[2].source_ref.path='src/tls/mod.rs';
  s.decisions.forEach(d=>{d.urgency='undetermined';d.reason_codes=['reference_only'];d.missing_inputs=['technical.identity','support.runtime']});
  return [{members:[0,1,2]}];
 });
 const card=f.root.innerHTML.split('<article class="decision-card"')[1].split('</article>')[0];
 const [overview,details]=card.split('<div class="decision-reasoning"');
 assert.match(overview,/What to do.*src\/tls\/incoming.rs/s);
 assert.match(details,/Check dependency use.*Cargo.lock/s);
 assert.doesNotMatch(overview,/support.runtime/);
 assert.doesNotMatch(overview,/Facts still needed|Observed purpose: Unidentified/);
 assert.match(details,/Facts still needed.*support.runtime/s);
 f.instance.setLanguage('ja');
 assert.match(f.root.innerHTML,/依存関係の利用箇所を確認する/);
});

test('recorded purpose and declared configuration remain distinct in card details',()=>{
  const f=fixture(1,false,false,s=>{
    s.claims=[{id:'a',field:'use_case',value:'encryption',state:'known',freshness:'active'}, {id:'b',field:'migration_target',value:{use_case:'encryption',algorithm:'AES',parameters:'AES-256'},state:'known',freshness:'active'}];
    s.decisions[0].applied_claim_ids=['a','b'];
  });
  const html=f.root.innerHTML;
  assert.match(html,/class="decision-reasoning"[^>]+hidden/);
  assert.match(html,/Observed purpose: Signing/);
  assert.match(html,/Proposed configuration \(declared\).*encryption \/ AES \/ AES-256/);
});

test('known purpose is not asked again when only support or policy facts are missing; urgency reason stays visible',()=>{
 const f=fixture(2,false,false,s=>{
  s.claims=[{id:'declared',field:'use_case',value:'signature',state:'known',freshness:'active'}];s.decisions.forEach(d=>{d.applied_claim_ids=['declared'];d.missing_inputs=['support.runtime'];d.action.validation_steps=['confirm_actual_use_and_required_purpose'];});
  s.decisions[0].urgency='urgent';s.decisions[0].reason_codes=['known_classical_concern'];
 });
 assert.match(f.root.innerHTML,/Check support for the proposed configuration/);assert.match(f.root.innerHTML,/decision-basis">known_classical_concern/);assert.match(f.root.innerHTML,/Investigate the detected cryptographic concern/);
 const first=f.root.innerHTML.split('<div class="next-checks">')[1].split('<div class="card-actions">')[0];assert.doesNotMatch(first,/confirm_actual_use_and_required_purpose/);
 assert.match(f.root.innerHTML,/Recorded locations.*src\/file-0.java/s);
 assert.match(f.root.innerHTML,/confirm_actual_use_and_required_purpose/,'original steps retained in detailed reasoning');
});

test('technical encoding and selected negotiation require evidence as well as any declarations',()=>{
 const f=fixture(1,false,false,s=>{s.decisions[0].missing_inputs=['technical.encoding','observation.selected_use'];});
 assert.match(f.root.innerHTML,/settings actually negotiated/);
 assert.match(f.root.innerHTML,/Inspect the original artifact/);
 assert.match(f.root.innerHTML,/technical.encoding/);assert.match(f.root.innerHTML,/observation.selected_use/);
});
test('saved connection evidence and collision-sensitive use receive evidence-specific checks',()=>{
 const f=fixture(2,false,false,s=>{
  s.targets[0].source_ref={role:'reference-only',detail_path:'/details/negotiated_kex'};
  s.targets[0].observation.method='ssh_saved_kex';
  s.targets[0].technical.identifier='unknown';
  s.decisions[0].missing_inputs=['technical.identity'];
  s.decisions[1].missing_inputs=['technical.collision_sensitive_use'];
 });
 assert.match(f.root.innerHTML,/Check the original evidence for the algorithm.*\/details\/negotiated_kex/s);
 assert.doesNotMatch(f.root.innerHTML,/Check the referenced code/);
 assert.match(f.root.innerHTML,/Check whether the hash is used where collisions matter/);
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
 const ordinary=fixture(1,false,false,s=>{s.targets[0].source_ref.role='reference-only';s.decisions[0].reason_codes=['reference_only'];s.decisions[0].missing_inputs=['technical.identity']});ordinary.instance.setLanguage('ja');assert.match(ordinary.root.innerHTML,/参照先から設定をたどる|実行時に選ばれる方式/);assert.doesNotMatch(ordinary.root.innerHTML,/この画面で回答を追加しても解消できません/);
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

test('zero assessment targets are stated without implying safety',()=>{
  const f=fixture(0);
  assert.match(f.root.innerHTML,/No assessment targets were found/);
  assert.doesNotMatch(f.root.innerHTML,/No matching work items/);
});

test('blocked JWT work distinguishes cross-file, token recipient and key-selection checks', () => {
  const f = fixture(5, false, false, s => {
    const assertion = {id:'jwt', field:'use_case', state:'known', freshness:'active', value:'jwt'};
    s.claims = [assertion];
    s.targets[2].source_ref.path = 'src/token.ts';
    s.targets[3].source_ref.path = s.targets[4].source_ref.path = 'src/tokens.py';
    s.targets[4].source_ref.observation_id = 'key-module';
    for (const d of s.decisions) {
      d.applied_claim_ids = ['jwt'];
      d.action.kind = 'unblock';
      d.action.blocking_prerequisites = ['jose','peer','verifiers'];
    }
    s.decisions[3].system_id = s.decisions[4].system_id;
    return [{members:[0,1]}, {members:[2]}, {members:[3]}, {members:[4]}];
  }, {scanReferences:{'key-module':{rule:'API_PY_RSA_MODULE'}}});
  assert.match(f.root.innerHTML, /Compare the signer&#39;s alg/);
  assert.match(f.root.innerHTML, /Trace where the token is sent/);
  assert.match(f.root.innerHTML, /Distinguish key generation/);
  assert.match(f.root.innerHTML, /Peer support/);
  assert.doesNotMatch(f.root.innerHTML, /Peer operator or service provider/);
  assert.match(f.root.innerHTML, /class="decision-reasoning"[\s\S]*Blocking requirements/);
});

test('start-by rendering uses only paired active claims and distinguishes equality from overdue', () => {
  const f = fixture(4, false, false, s => {
    s.as_of = '2026-11-19T00:00:00Z';
    s.claims = [
      {id:'at', field:'deadline', value:{kind:'at',at:'2026-11-20T00:00:00Z'}},
      {id:'late', field:'migration_lead_seconds', value:172800},
      {id:'equal', field:'migration_lead_seconds', value:86400},
      {id:'stale', field:'migration_lead_seconds', value:259200, freshness:'stale'},
    ].map(c => ({state:'known',freshness:'active',...c}));
    const pairs = [['at','late'],['at','equal'],['at','stale'],['equal']];
    s.decisions.forEach((d,i) => { d.applied_claim_ids = pairs[i]; });
    return [{members:[0,1,2,3]}];
  });
  assert.match(f.root.innerHTML, /datetime="2026-11-18T00:00:00.000Z".*Start-by date passed at assessment · 1 \/ 4 records/);
  assert.match(f.root.innerHTML, /datetime="2026-11-19T00:00:00.000Z".*Start-by date reached at assessment · 1 \/ 4 records/);
  assert.doesNotMatch(f.root.innerHTML, /datetime="2026-11-17T00:00:00.000Z"/);
});

test('equivalent observations open one location detail with every matched rule', () => {
  const id = 'finding-v1-' + 'a'.repeat(64);
  const otherId = 'finding-v1-' + 'b'.repeat(64);
  const setup = s => {
    s.targets.forEach((target,i) => {
      target.source_ref.path = 'src/token.go';
      target.source_ref.line = 18;
      target.source_ref.observation_id = 'o'+i;
      target.technical.identifier = 'RSA';
    });
    s.targets[1].observation.method = 'lexical_match';
    s.targets[1].observation.assurance = 'lexical';
    s.decisions[1].system_id = s.decisions[0].system_id;
    s.decisions[1].urgency = s.decisions[0].urgency;
    return [{members:[0,1]}];
  };
  const presentation = {hasScanReport:true,scanReferences:{o0:{rule:'RULE_1',findings:[id]},o1:{rule:'RULE_2',findings:[id,otherId]}}};
  const f = fixture(2,false,false,setup,presentation);
  f.instance.setLanguage('ja'); f.click({work:'0'});
  assert.equal((f.body().innerHTML.match(/<tbody><tr>/g)||[]).length,1);
  assert.match(f.body().innerHTML, /RULE_1 · RULE_2.*data-target="0">詳細/);
  assert.doesNotMatch(f.body().innerHTML, /件の記録|data-location=/);
  const search = f.context.document.createElement();
  search.setAttribute('data-detail-search', '');
  search.value = 'RULE_2';
  f.event('input', search);
  assert.match(f.body().innerHTML, /RULE_1 · RULE_2.*data-target="0">詳細/);
  f.click({target:'0'});
  assert.equal((f.body().innerHTML.match(/<section id="target-/g)||[]).length,1);
  assert.match(f.body().innerHTML, /スキャンのルールID.*RULE_1 · RULE_2/s);
  assert.match(f.body().innerHTML, /検出方法.*構文解析 · 文字列一致/s);
  assert.match(f.body().innerHTML, new RegExp('report.html\\?lang=ja#scan-match-'+id));
  assert.match(f.body().innerHTML, new RegExp('report.html\\?lang=ja#scan-match-'+otherId));
  assert.equal((f.body().innerHTML.match(/#scan-match-/g)||[]).length,2);
  f.click({},['data-list']);
  assert.match(f.body().innerHTML, /data-detail-search[^>]*value="RULE_2"/);
  assert.equal(f.root.querySelector('dialog [data-target="0"]').focused,true);
  const saved = fixture(2,false,true,setup,presentation);
  saved.jump('#target-1');
  assert.doesNotMatch(saved.body().innerHTML, /href="report\.html/);
  assert.match(saved.body().innerHTML, /Scan rule ID.*RULE_1/s);
  assert.doesNotMatch(saved.body().innerHTML, new RegExp(id));
});

test('one location detail shows materially distinct records without repeating the path', () => {
  const f=fixture(3,false,false,s=>{
    s.targets.forEach((t,i)=>{t.source_ref.path='src/token.go';t.source_ref.line=i===2?19:18;});
    s.targets[1].technical.identifier='RS256';
    s.decisions.slice(0,2).forEach(d=>{
      Object.assign(d,{system_id:'same-system',urgency:'plan',applied_claim_ids:['c0']});
    });
    return [{members:[0,1,2]}];
  });
  f.click({work:'0'});
  assert.equal((f.body().innerHTML.match(/data-target=/g)||[]).length,2);
  f.click({target:'0'});
  const detail=f.body().innerHTML;
  assert.equal((detail.match(/src\/token.go:18/g)||[]).length,1);
  assert.match(detail,/<h3>RSA<\/h3>/);
  assert.match(detail,/<h3>RS256<\/h3>/);
  assert.doesNotMatch(detail,/src\/token.go:19|ML-DSA/);
  const ids=[...detail.matchAll(/\bid="([^"]+)"/g)].map(match=>match[1]);
  assert.equal(ids.length,new Set(ids).size);
});

test('details at one line remain separate when the algorithm agrees but the judgment differs', () => {
  const f=fixture(22,false,false,s=>{
    s.targets.forEach(t=>{t.source_ref.path='src/token.go';t.source_ref.line=18;t.technical.identifier='RSA';});
    s.decisions.forEach(d=>{d.system_id=null;d.urgency='urgent';});
    for (const [i,d] of s.decisions.entries()) d.reason_codes=['reason-'+i];
    return [{members:s.decisions.map((_,i)=>i)}];
  });
  f.click({work:'0'});
  assert.equal((f.body().innerHTML.match(/data-target=/g)||[]).length,1);
  f.click({target:'0'});
  assert.equal((f.body().innerHTML.match(/<section id="target-/g)||[]).length,20);
  assert.match(f.body().innerHTML,/reason-0/);
  assert.doesNotMatch(f.body().innerHTML,/reason-20/);
  f.click({recordPage:'1'});
  assert.match(f.body().innerHTML,/reason-20/);
  assert.equal((f.body().innerHTML.match(/src\/token.go:18/g)||[]).length,1);
  f.click({},['data-list']);
  assert.match(f.body().innerHTML,/data-target="0">Details/);
});

test('monitor and scoped exclusions have different reassessment triggers and no empty detail headings', () => {
  const f = fixture(2, false, false, s => {
    s.claims = [{id:'e',field:'use_case',value:'encryption',freshness:'active',state:'known'}];
    s.decisions[0].action.kind = 'monitor';
    s.decisions[0].applied_claim_ids = ['e'];
    s.decisions[1].action.kind = 'none';
    s.decisions.forEach((d,i) => {
      d.action.validation_steps = [];
      d.reason_codes = ['unknown_parameter_encoding'];
      s.targets[i].limitations = ['unknown_parameter_encoding'];
    });
  });
  assert.match(f.root.innerHTML, /defaults and application overrides/);
  assert.match(f.root.innerHTML, /a caller starts using this reference/);
  f.jump('#target-1');
  assert.doesNotMatch(f.body().innerHTML, /<h4>Next checks<\/h4>/);
  assert.equal((f.body().innerHTML.match(/unknown_parameter_encoding/g)||[]).length,1);
});

test('a declared candidate configuration and the unverified integration state both remain visible', () => {
  const f = fixture(1, false, false, s => {
    s.claims = [{id:'selected',field:'migration_target',value:{use_case:'jwt',algorithm:'ML-DSA',parameters:'ML-DSA-65'},freshness:'active',state:'known'}];
    s.decisions[0].applied_claim_ids = ['selected'];
    s.decisions[0].action.candidate_ids = ['candidate'];
    s.candidates = [{id:'candidate',identifier:'ML-DSA',purpose:'signature',status:'candidate_primitive_integration_unverified',
      parameter_selection:'explicit_parameter_selection_required',prerequisites:[],validation_steps:[]}];
  });
  f.jump('#target-1');
  assert.match(f.body().innerHTML, /Declared configuration ML-DSA-65/);
  assert.match(f.body().innerHTML, /candidate_primitive_integration_unverified/);
  assert.doesNotMatch(f.body().innerHTML, /explicit_parameter_selection_required/);
});

test('a library-name hit on the same line does not hide the JWT check', () => {
  const f = fixture(3, false, false, s => {
    s.claims = [{id:'jwt',field:'use_case',value:'jwt',freshness:'active',state:'known'}];
    s.targets.forEach((target,i) => {
      target.source_ref.path = i < 2 ? 'mixed.go' : 'library.go';
      target.source_ref.line = 5;
      target.source_ref.observation_id = 'o'+i;
      s.decisions[i].action.kind = 'unblock';
      s.decisions[i].urgency = 'urgent';
      s.decisions[i].applied_claim_ids = ['jwt'];
    });
    return [{members:[2]}, {members:[0,1]}];
  },{scanReferences:{o0:{rule:'GO_TLS_RSA_IMPORT'},o1:{rule:'JWT_RS256'},o2:{rule:'GO_TLS_RSA_IMPORT'}}});
  const cards = f.root.innerHTML.split('<article class="decision-card"').slice(1);
  assert.match(cards[0], /id="work-2"/);
  assert.match(cards[0], /Trace where the token is sent[\s\S]*Follow the library reference/);
  assert.match(cards[1], /Follow the library reference/);
  assert.doesNotMatch(cards[1], /Trace where the token is sent/);
});

test('blocking conditions remain on their own card details without checklist sections',()=>{
  const f=fixture(4,false,false,s=>{
    s.decisions[1].action.blocking_prerequisites=['jose','runtime'];
    s.decisions[3].action.blocking_prerequisites=['jose','peer'];
    s.decisions[1].action.validation_steps=['resolve_declared_blocking_prerequisites'];
  },{
    words:{resolve_declared_blocking_prerequisites:{
      en:'Resolve each declared blocking prerequisite below.',
      ja:'下記の申告された阻害条件を解消してください。',
    }},
  });
  for(const [lang,label,jose,runtime,peer] of [
    ['en','Blocking requirements','JWT implementation','Runtime','Peer support'],
    ['ja','未対応の条件','JWT実装','実行環境','接続先'],
  ]){
    f.instance.setLanguage(lang);
    const html=f.root.innerHTML;
    const cards=html.split('<article class="decision-card"').slice(1).map(s=>s.split('</article>')[0]);
    const first=cards.find(card=>card.includes('id="work-2"'));
    const second=cards.find(card=>card.includes('id="work-4"'));
    assert.match(first,new RegExp('class="decision-reasoning"[^>]* hidden>[\\s\\S]*'+label+'[\\s\\S]*'+jose+'[\\s\\S]*'+runtime));
    assert.match(second,new RegExp('class="decision-reasoning"[^>]* hidden>[\\s\\S]*'+label+'[\\s\\S]*'+jose+'[\\s\\S]*'+peer));
    assert.match(first,lang==='en'?/Resolve the declared blocking requirements\./:/申告された未対応の条件を解消する。/);
    assert.doesNotMatch(first,/blocking prerequisite below|下記の申告された阻害条件/);
    assert.doesNotMatch(html,/work-checklist|migration-checklist|shared-checklist|確認リスト|Checklist/);
    assert.doesNotMatch(html,/data-owner-copy|担当者|依頼文をコピー|再評価の手順と前提|migration-usage|反映された値と対象は各カードで確認できます|View the applied values on each work item/);
  }
});

test('compact locations retain saved pointers and disambiguate files with the same name',()=>{
  const f=fixture(3,false,false,s=>{
    s.targets[0].source_ref={role:'algorithm',path:'backend/auth.go',line:42};
    s.targets[1].source_ref={role:'algorithm',path:'frontend/auth.go',line:42};
    s.targets[2].source_ref={role:'algorithm',detail_path:'/details/peer_certificates/0'};
    s.decisions[0].missing_inputs=['support.runtime'];
    return [{members:[0,1,2]}];
  });
  const visible=f.root.innerHTML.split('class="recorded-locations"')[1].split('</ul>')[0];
  assert.match(visible,/Recorded locations/);
  assert.match(visible,/>backend\/auth.go:42<\/code>/);
  assert.match(visible,/>frontend\/auth.go:42<\/code>/);
  assert.match(visible,/>\/details\/peer_certificates\/0<\/code>/);
});

test('the details control updates aria-expanded and preserves expansion across locale changes',()=>{
  const f=fixture(1);
  const panel=f.root.querySelector('#work-details-1');
  panel.hidden=true;
  const button={dataset:{workDetails:'0'},hasAttribute(){return false;},setAttribute(name,value){this[name]=value;},closest(selector){return selector==='button'?this:null;}};
  f.event('click',button);
  assert.equal(panel.hidden,false);
  assert.equal(button['aria-expanded'],'true');
  f.instance.setLanguage('ja');
  assert.match(f.root.innerHTML,/aria-controls="work-details-1" aria-expanded="true"/);
  f.event('click',button);
  assert.equal(panel.hidden,true);
  assert.equal(button['aria-expanded'],'false');
});

test('the conclusion can open the other urgent check on a later work page',()=>{
  const f=fixture(45,false,false,s=>{
    workflowInput(s);
    s.decisions.forEach((d,i)=>d.system_id='system-'+i);
    s.decisions[44].applied_claim_ids=[];
  },{standaloneMount:false});
  assert.match(f.root.innerHTML,/data-check-work="44"/);
  const link={dataset:{checkWork:'44'},closest(selector){return selector==='a[data-check-work]'?this:null;}};
  f.event('click',link,{preventDefault(){}});
  assert.match(f.root.innerHTML,/id="work-45"/);
  assert.match(f.root.innerHTML,/41-45 \/ 45/);
  assert.equal(f.root.querySelector('#work-45').focused,true);
});

test('declaration overview groups equal deadlines and counts each covered record once',async()=>{
  const f=fixture(3,false,false,s=>{
    s.claims=[
      {id:'da',field:'deadline',value:{kind:'at',at:'2027-03-31T14:59:59Z',scope_reason:'organization_policy',reference_id:'policy-a'}},
      {id:'db',field:'deadline',value:{kind:'at',at:'2027-03-31T23:59:59+09:00',scope_reason:'contractual_requirement',reference_id:'policy-b'}},
      {id:'sunset',field:'deadline',value:{kind:'at',at:'2028-03-31T14:59:59Z',scope_reason:'organization_policy',reference_id:'policy-c'}},
      {id:'use',field:'use_case',value:'jwt'},
    ].map(c=>({...c,state:'known',freshness:'active'}));
    [['da','db','use'],['db','use'],['sunset']].forEach((ids,i)=>{s.decisions[i].applied_claim_ids=ids;});
  });
  f.instance.setLanguage('ja');
  const summary=f.root.innerHTML.match(/<details class="summary-declarations"[\s\S]*?<\/details>/)[0];
  assert.match(summary,/評価対象.*3記録/);
  assert.match(summary,/<dt>移行期限<\/dt>/);
  assert.match(summary,/2027\/03\/31 23:59:59 JST<\/span>\s*<small>2記録<\/small>/);
  assert.match(summary,/2028\/03\/31 23:59:59 JST<\/span>\s*<small>1記録<\/small>/);
  assert.equal((summary.match(/2027\/03\/31/g)||[]).length,1);
  assert.doesNotMatch(summary,/policy-a|policy-b|\/ 3 記録/);
  assert.match(f.root.innerHTML,/policy-b/);
  f.click({},['data-download']);
  const exported=JSON.parse(await f.saved().text());
  assert.deepEqual(exported.claims.filter(c=>c.field==='deadline').map(c=>c.value.reference_id),['policy-a','policy-b','policy-c']);
});

test('timeline presents labelled values and places each status beside the date it describes',()=>{
  const f=fixture(1,false,false,s=>{
    s.claims=[
      {id:'d',field:'deadline',value:{kind:'at',at:'2026-11-18T00:00:00Z',reference_id:'policy',scope_reason:'organization_policy'},freshness:'active',state:'known'},
      {id:'l',field:'migration_lead_seconds',value:86400,freshness:'active',state:'known'},
    ];
    s.decisions[0].applied_claim_ids=['d','l'];
  });
  f.instance.setLanguage('ja');
  const timeline=f.root.innerHTML.match(/id="migration-timeline"[\s\S]*?<\/section>/)[0];
  assert.match(timeline,/<dt>着手目安<\/dt>\s*<dd>\s*<time[^>]*datetime="2026-11-17T00:00:00.000Z"[^>]*>.*?着手目安を超過<\/small><\/dd>/s);
  assert.match(timeline,/<dt>準備期間<\/dt>\s*<dd>1日<\/dd>/);
  assert.match(timeline,/<dt>移行期限<\/dt>\s*<dd>\s*<time[^>]*datetime="2026-11-18T00:00:00.000Z"[^>]*>.*?移行期限を超過<\/small><\/dd>/s);
  assert.match(f.root.innerHTML,/class="collection-status">/);
  const complete=fixture(1,false,false,s=>{s.coverage.collection_outcome='complete';});
  assert.doesNotMatch(complete.root.innerHTML,/class="collection-status">/);
  const unknown=fixture(1,false,false,s=>{s.coverage.collection_outcome='unknown';});
  assert.match(unknown.root.innerHTML,/Collection status is unknown/);
  unknown.instance.setLanguage('ja');
  assert.match(unknown.root.innerHTML,/収集状況は不明/);
});

test('timeline uses active deadline and lead pairs and distinguishes due from overdue', () => {
  const f = fixture(4, false, false, s => {
    s.claims = [];
    for (const [i, at, freshness] of [[0, '2026-11-19T00:00:01Z', 'active'], [1, '2026-11-19T00:00:00Z', 'active'], [2, '2026-11-13T00:00:00Z', 'stale'], [3, '2026-11-18T00:00:00Z', 'active']]) {
      s.claims.push({ id: 'd-' + i, field: 'deadline', value: { kind: 'at', at }, freshness, state: 'known' });
      s.decisions[i].applied_claim_ids = ['d-' + i];
      if (i === 3) continue;
      s.claims.push({ id: 'l-' + i, field: 'migration_lead_seconds', value: 1, freshness, state: 'known' });
      s.decisions[i].applied_claim_ids.push('l-' + i);
    }
  });
  f.instance.setLanguage('ja');
  const timeline = f.root.innerHTML.match(/id="migration-timeline"[\s\S]*?<\/section>/)?.[0];
  assert.ok(f.root.innerHTML.indexOf('id="migration-timeline"') < f.root.innerHTML.indexOf('id="migration-conclusion"'));
  assert.doesNotMatch(timeline, /<h2>/);
  assert.match(timeline, /評価日時.*JST/s);
  assert.match(timeline, /評価日時点で着手目安を超過/);
  assert.match(timeline, /評価日時点で着手目安に到来/);
  assert.match(timeline, /評価日時点で移行期限に到来/);
  assert.doesNotMatch(timeline, /移行期限を超過/);
  assert.doesNotMatch(timeline, /2026-11-(13|18)T00:00:00/);
  assert.doesNotMatch(fixture(1).root.innerHTML, /id="migration-timeline"/);
});

test('timeline and card distinguish start-by and migration deadline statuses at each boundary', () => {
  for (const [asOf, startStatus, deadlineStatus] of [
    ['2026-11-18T23:59:59Z', '', ''],
    ['2026-11-19T00:00:00Z', 'reached', ''],
    ['2026-11-19T00:00:01Z', 'passed', ''],
    ['2026-11-20T00:00:00Z', 'passed', 'reached'],
    ['2026-11-20T00:00:01Z', 'passed', 'passed'],
  ]) {
    const f = fixture(1,false,false,s=>{
      s.as_of=asOf;
      s.claims=[
        {id:'deadline',field:'deadline',value:{kind:'at',at:'2026-11-20T00:00:00Z',scope_reason:'organization_policy',reference_id:'policy'},state:'known',freshness:'active'},
        {id:'lead',field:'migration_lead_seconds',value:86400,state:'known',freshness:'active'},
      ];
      s.decisions[0].applied_claim_ids=['deadline','lead'];
    });
    for (const [locale, startReached, startPassed, deadlineReached, deadlinePassed] of [
      ['en','Start-by date reached at assessment','Start-by date passed at assessment','Migration deadline reached at assessment','Migration deadline passed at assessment'],
      ['ja','評価日時点で着手目安に到来','評価日時点で着手目安を超過','評価日時点で移行期限に到来','評価日時点で移行期限を超過'],
    ]) {
      f.instance.setLanguage(locale);
      const html=f.root.innerHTML;
      const timeline=html.match(/id="migration-timeline"[\s\S]*?<\/section>/)[0];
      const card=html.match(/<article class="decision-card"[\s\S]*?<\/article>/)[0];
      for (const rendered of [timeline, card]) {
        assert.equal(rendered.includes(startReached), startStatus === 'reached');
        assert.equal(rendered.includes(startPassed), startStatus === 'passed');
        assert.equal(rendered.includes(deadlineReached), deadlineStatus === 'reached');
        assert.equal(rendered.includes(deadlinePassed), deadlineStatus === 'passed');
      }
    }
  }
});

test('card details show only active goal-month declarations with per-value record counts', () => {
  const f=fixture(4,false,false,s=>{
    s.claims=[
      {id:'first',field:'migration_goal_month',value:'2027-03',state:'known',freshness:'active'},
      {id:'second',field:'migration_goal_month',value:'2028-05',state:'known',freshness:'active'},
      {id:'stale',field:'migration_goal_month',value:'2029-09',state:'known',freshness:'stale'},
      {id:'other-target',field:'migration_goal_month',value:'2031-11',state:'known',freshness:'active'},
    ];
    [['first'],['first'],['second'],['stale']].forEach((ids,i)=>{s.decisions[i].applied_claim_ids=ids;});
    return [{members:[0,1,2,3]}];
  });
  for(const [locale, label, records] of [['en','Migration goal month','records'],['ja','移行の目標年月','記録']]) {
    f.instance.setLanguage(locale);
    const html=f.root.innerHTML;
    const details=html.match(/<section class="card-declarations"[\s\S]*?<\/section>/)?.[0] || '';
    assert.ok(details.includes(label));
    assert.ok(details.includes('2027-03 (2 / 4 '+records+')'));
    assert.ok(details.includes('2028-05 (1 / 4 '+records+')'));
    assert.doesNotMatch(html, /2029-09|2031-11|id="migration-timeline"|No purpose or planning declarations shown|用途・利用状況・計画の申告なし/);
  }
});

test('card shows a specific first check and keeps blocker labels in its detail', () => {
  const f = fixture(2, false, false, s => {
    s.claims = [{ id: 'use', field: 'use_case', value: 'signature', state: 'known', freshness: 'active' }];
    s.decisions[0].applied_claim_ids = ['use'];
    s.decisions[0].action.blocking_prerequisites = ['peer'];
    s.decisions[0].missing_inputs = ['technical.encoding'];
    return [{ members: [0, 1] }];
  });
  f.instance.setLanguage('ja');
  assert.match(f.root.innerHTML, /次に進める条件/);
  assert.match(f.root.innerHTML, /検出・保存された根拠/);
  assert.match(f.root.innerHTML, /署名 \(1 \/ 2 記録\)/);
  assert.match(f.root.innerHTML, /technical.encoding/);
  assert.match(f.root.innerHTML, /class="decision-reasoning"[^>]* hidden>.*未対応の条件.*接続先/s);
  assert.match(f.root.innerHTML, /確認に使う場所|src\/file-0.java/);
  const empty = fixture(0).root.innerHTML;
  assert.doesNotMatch(empty, /data-urgency-group|id="migration-timeline"|data-owner-copy/);
  assert.match(empty, /No assessment targets/);
});

test('a plan card keeps distinct partial declarations and identifies recorded locations separately', () => {
  const f = fixture(3, false, false, s => {
    s.claims = [
      { id: 'sign', field: 'use_case', value: 'signature', state: 'known', freshness: 'active' },
      { id: 'encrypt', field: 'use_case', value: 'encryption', state: 'known', freshness: 'active' },
    ];
    s.decisions[0].applied_claim_ids = ['sign'];
    s.decisions[1].applied_claim_ids = ['encrypt'];
    s.decisions[0].action.kind = 'plan';
    return [{ members: [0, 1, 2] }];
  });
  const card = f.root.innerHTML.split('class="decision-card"')[1].split('</article>')[0];
  assert.match(card, /<h4>[^<]*RSA.*Signing<\/h4>/);
  assert.match(card, /Signing \(1 \/ 3 records\).*encryption \(1 \/ 3 records\)/);
  assert.match(card, /Recorded locations.*src\/file-0.java.*View detection locations/s);
});

test('timeline limits date pairs to the earliest three and counts overdue cards only once', () => {
  const f = fixture(6, false, false, s => {
    s.claims = [];
    const dates = ['2026-11-10','2026-11-11','2026-11-12','2026-11-13','2026-11-14','2026-11-15'];
    s.decisions.forEach((d,i) => {
      d.urgency = 'urgent';
      const deadline = {id:'d-' + i, field:'deadline', value:{kind:'at',at:dates[i]+'T00:00:00Z'}, state:'known', freshness:'active'};
      const lead = {id:'l-' + i, field:'migration_lead_seconds', value:0, state:'known', freshness:'active'};
      s.claims.push(deadline,lead);
      // The last two records have only one half of their respective pair.
      d.applied_claim_ids = i === 4 ? [deadline.id] : i === 5 ? [lead.id] : [deadline.id,lead.id];
    });
    return [{members:[0,1]}, {members:[2]}, {members:[3]}, {members:[4]}, {members:[5]}];
  });
  assert.match(f.root.innerHTML, /Past the start-by date<\/dt><dd>3 items<\/dd>.*With a start-by date<\/dt><dd>3 \/ 5 items<\/dd>/s);
  const timeline = f.root.innerHTML.match(/id="migration-timeline"[\s\S]*?<\/section>/)[0];
  assert.equal((timeline.match(/class="timeline-track"/g) || []).length, 3);
  assert.match(timeline, /1 more date pairs/);
  assert.doesNotMatch(timeline, /2026-11-(13|14|15)T00:00:00/);
});

test('a blocker-free report has no checklist or empty checklist notice',()=>{
  const f=fixture(1);
  assert.doesNotMatch(f.root.innerHTML,/work-checklist|migration-checklist|No requests/);
  f.instance.setLanguage('ja');
  assert.doesNotMatch(f.root.innerHTML,/work-checklist|migration-checklist|確認依頼はありません/);
});
