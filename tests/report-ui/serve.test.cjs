const {test}=require('node:test');
const assert=require('node:assert/strict');
const {readFileSync}=require('node:fs');
const {runInNewContext}=require('node:vm');
const path=require('node:path');
const source=readFileSync(path.join(__dirname,'../../crates/pqc_scan_cli/src/serve.js'),'utf8');
const html=readFileSync(path.join(__dirname,'../../crates/pqc_scan_cli/src/serve.html'),'utf8');
async function fixture(search='', {migration=false, jobs=[], directories=[{name:'next',path:'next'}]}={}) {
 const node=tag=>({tag,children:[],listeners:{},disabled:false,value:'',textContent:'',dataset:{},attributes:{},open:false,files:[],classList:{toggle(){}},
  addEventListener(t,f){this.listeners[t]=f;},setAttribute(k,v){this.attributes[k]=v;},removeAttribute(k){delete this.attributes[k];},
  replaceChildren(...children){this.children=children;},append(...children){this.children.push(...children);},focus(){this.focused=true;},
  showModal(){this.open=true;},close(){this.open=false;},click(){if(!this.disabled)return this.listeners.click?.({detail:1});}});
 // Use actual shell IDs: a missing/renamed production element must not silently
 // become a successful mock lookup. Dynamic elements still use createElement.
 const ui={};
 for(const match of html.matchAll(/<[^>]*\bid="([^"]+)"[^>]*>/g)) {
  ui[match[1]]=node();ui[match[1]].hidden=/\shidden(?:\s|>)/.test(match[0]);
 }
 const translated=[];
 for(const match of html.matchAll(/<[^>]*\bdata-en="[^"]*"[^>]*>/g)) {
  const id=match[0].match(/\bid="([^"]+)"/),n=id ? ui[id[1]] : node();
  n.dataset.en=match[0].match(/data-en="([^"]*)"/)[1];n.dataset.ja=match[0].match(/data-ja="([^"]*)"/)[1];
  n.textContent=n.dataset.ja;translated.push(n);
 }
 ui.language.value='ja';ui.steps.children=[node(),node(),node()];
 const events=[];
 const window={listeners:{},addEventListener(t,f){this.listeners[t]=f;},dispatchEvent(event){events.push(event);this.listeners[event.type]?.(event);}};
 const document={getElementById:id=>ui[id]||null,createElement:node,querySelectorAll:selector=>selector==='[data-en]'?translated:[],body:{append(script){script.onload()}},documentElement:{lang:'ja',dataset:{theme:'light'}}};
 const state={jobs,nextResolve:null,nextReject:null};
 let nextJob=7;
 const requests=[],timers=[],location={href:'http://127.0.0.1:8787/session/'+search,search};
 const history={replaceState(_state,_title,url){const next=new URL(url);location.href=next.href;location.search=next.search;}};
 const fetch=async(route,options)=>{
  requests.push([route,options?.method||'GET']);
  if(route==='api/config')return {ok:true,json:async()=>({migration})};
  if(route==='api/jobs')return {ok:true,json:async()=>({jobs:state.jobs})};
  if(route==='api/directories?path=')return {ok:true,json:async()=>({root:'/projects',directories,omitted:0})};
  if(route==='api/directories?path=next')return new Promise((resolve,reject)=>{state.nextResolve=()=>resolve({ok:true,json:async()=>({root:'/projects',directories:[],omitted:0})});state.nextReject=()=>reject(Error('denied'));});
  if(route.startsWith('api/scan?path=') && options?.method==='POST')return {ok:true,json:async()=>({id:nextJob++})};
  throw Error('Unexpected request: '+route);
 };
 const context={window,document,fetch,location,history,URL,URLSearchParams,
  CustomEvent:class {constructor(type,options){this.type=type;this.detail=options?.detail;}},
  localStorage:{setItem(){}},setTimeout:f=>timers.push(f)};
 const flush=async()=>{for(let i=0;i<3;i++)await new Promise(resolve=>setImmediate(resolve));};
 runInNewContext(source,context);await flush();
 return {ui,state,requests,location,window,document,events,flush,
  async tick(){assert.ok(timers.length,'poll must remain scheduled');await timers.shift()();await flush();},
  posts(){return requests.filter(([,method])=>method==='POST');}};
}
test('pending folder navigation blocks old-folder scans through polling and language changes',async()=>{
 const f=await fixture();assert.equal(f.ui.scan.disabled,true,'nothing is selected initially');
 f.ui['folder-path'].children[0].click();assert.equal(f.ui.scan.disabled,false);
 f.ui.folders.children[0].children[1].click();assert.equal(f.ui.scan.disabled,true);
 await f.tick();assert.equal(f.ui.scan.disabled,true);
 f.ui.language.value='en';f.ui.language.listeners.change();assert.equal(f.ui.scan.disabled,true);
 await f.ui.scan.listeners.click();assert.equal(f.posts().length,0,'even a direct event must not scan the previous root');
 f.state.nextResolve();await f.flush();
 assert.equal(f.ui['folder-path'].children.at(-1).textContent,'next');
 f.ui['clear-folders'].click();f.ui['folder-path'].children.at(-1).click();
 assert.equal(f.ui.selected.textContent,'Selected: next');assert.equal(f.ui.scan.disabled,false);
 await f.ui.scan.click();assert.deepEqual(f.posts(),[['api/scan?path=next&max_file_size_mib=2','POST']]);
 assert.equal(f.ui['scan-progress'].open,true);
 f.state.jobs=[{id:7,path:'next',status:'ready',coverage:'partial',report:'reports/7/report.html'}];await f.tick();
 const report=new URL(f.location.href);assert.equal(report.pathname,'/session/reports/7/report.html');assert.equal(report.searchParams.get('lang'),'en');
 assert.equal(f.ui['scan-progress'].open,false);
 f.ui.language.value='ja';f.window.listeners.pageshow();assert.equal(f.document.documentElement.lang,'ja');
 f.state.jobs.push({id:8,path:'next',status:'running'});await f.tick();assert.equal(f.ui.scan.disabled,true);assert.equal(f.ui['scan-progress'].open,true);
 f.state.jobs=[{id:8,path:'next',status:'ready',report:'reports/8/report.html'}];await f.tick();assert.equal(f.ui.scan.disabled,false);assert.equal(f.ui['scan-progress'].open,false);
});
test('folder and scan failures remain explicit and allow retry without false zero results',async()=>{
 const f=await fixture();f.ui['folder-path'].children[0].click();
 f.ui.folders.children[0].children[1].click();f.state.nextReject();await f.flush();
 assert.match(f.ui.feedback.textContent,/フォルダを開けません/);assert.equal(f.ui.selected.textContent,'選択中：/projects');assert.equal(f.ui.scan.disabled,false);
 const before=f.location.href;await f.ui.scan.click();assert.equal(f.ui['scan-progress'].open,true);
 f.state.jobs=[{id:7,path:'',status:'failed'}];await f.tick();
 assert.equal(f.location.href,before);assert.equal(f.ui.scan.disabled,false);assert.equal(f.ui['scan-progress'].open,false);assert.match(f.ui.feedback.textContent,/失敗/);
 assert.equal(f.ui['input-screen'].hidden,false);
});
test('history-restored language selection matches labels and document language',async()=>{
 const f=await fixture('?lang=en&theme=light');assert.equal(f.document.documentElement.lang,'en');assert.equal(f.ui.scan.textContent,'Scan selected folders');
 f.ui.language.value='ja';f.window.listeners.pageshow();assert.equal(f.document.documentElement.lang,'ja');assert.match(f.ui.scan.textContent,/スキャン/);
 assert.equal(new URL(f.location.href).searchParams.get('lang'),'ja');
});

function ready(id, path) {
  return {id,path,display_name:path.split('/').at(-1),status:'ready',
    coverage:'complete_within_scope',report:`reports/${id}/report.html`,migration_input:true};
}

function actions(card) {
  return card.children.find(element=>element.className==='job-actions')?.children || [];
}

test('completed history opens the matching report directly and dispatches migration for the same folder', async()=>{
  const jobs=[
    ready(1,'first/api'),ready(2,'second/api'),
    {id:3,path:'stale',status:'failed'},
    {id:4,kind:'saved',status:'ready',display_name:'inventory.json',migration_input:true},
  ];
  const f=await fixture('?lang=en',{migration:true,jobs});
  const [imported,failed,second,first]=f.ui.jobs.children;
  const [scan,migrate]=actions(first);
  assert.equal(scan.tag,'a');
  assert.equal(new URL(scan.href).pathname,'/session/reports/1/report.html');
  assert.equal(new URL(scan.href).searchParams.get('lang'),'en');
  assert.equal(scan.target,'_blank');
  assert.match(scan.attributes['aria-label'],/first\/api/);
  assert.match(actions(second)[0].href,/reports\/2\/report.html/);
  assert.deepEqual(actions(failed),[]);
  assert.deepEqual(actions(imported).map(action=>action.tag),['button']);
  f.ui.theme.click();
  const dark=actions(f.ui.jobs.children.at(-1))[0];
  assert.equal(new URL(dark.href).searchParams.get('theme'),'dark');
  migrate.click();
  assert.equal(f.events.at(-1).type,'pqc-migration-job');
  assert.equal(f.events.at(-1).detail.id,1);
  assert.equal(f.events.at(-1).detail.display_path,'first/api');
  assert.equal(f.ui['migration-screen'].hidden,false);
  f.window.dispatchEvent({type:'pqc-migration-back'});
  assert.equal(f.ui['result-screen'].hidden,false);
  assert.match(f.ui['result-report'].href,/reports\/1\/report.html/);
});

test('three completed folders stay separately actionable, in selection order, across migration and reload', async()=>{
  const paths=['first/api','second/api','third'];
  const directories=paths.map(path=>({name:path.split('/').at(-1),path}));
  const f=await fixture('',{migration:true,directories});
  for(const row of f.ui.folders.children)row.children[0].click();
  await f.ui.scan.click();
  for(const [index,path] of paths.entries()){
    f.state.jobs.push(ready(7+index,path));
    await f.tick();
  }
  assert.deepEqual(f.posts().map(([route])=>new URL(route,f.location.href).searchParams.get('path')),paths);
  assert.equal(f.ui['result-screen'].hidden,false);
  assert.equal(f.ui['result-single'].hidden,true);
  assert.equal(f.ui['batch-results'].hidden,false);
  const cards=f.ui['batch-results'].children;
  assert.deepEqual(cards.map(card=>new URL(actions(card)[0].href).pathname),paths.map((_,i)=>`/session/reports/${i+7}/report.html`));
  assert.match(actions(cards[0])[0].attributes['aria-label'],/first\/api/);
  assert.match(actions(cards[1])[0].attributes['aria-label'],/second\/api/);
  actions(cards[1])[1].click();
  assert.equal(f.events.at(-1).detail.id,8);
  assert.equal(f.events.at(-1).detail.display_path,'second/api');
  f.window.dispatchEvent({type:'pqc-migration-back'});
  assert.equal(f.ui['batch-results'].hidden,false);
  const restored=await fixture(new URL(f.location.href).search,{migration:true,jobs:f.state.jobs});
  assert.deepEqual(restored.ui['batch-results'].children.map(card=>new URL(actions(card)[0].href).pathname),
    paths.map((_,i)=>`/session/reports/${i+7}/report.html`));
});

test('batch results show a failure without report actions and exclude older history', async()=>{
  const paths=['one','two','three'];
  const f=await fixture('',{jobs:[ready(2,'older')],directories:paths.map(path=>({name:path,path}))});
  for(const row of f.ui.folders.children)row.children[0].click();
  await f.ui.scan.click();
  f.state.jobs.push(ready(7,'one'));
  await f.tick();
  f.state.jobs.push({id:8,path:'two',status:'failed'});
  await f.tick();
  f.state.jobs.push(ready(9,'three'));
  await f.tick();
  const cards=f.ui['batch-results'].children;
  assert.deepEqual(cards.map(card=>card.children[0].textContent),paths);
  assert.deepEqual(cards.map(card=>actions(card).map(action=>action.tag)),[['a'],[],['a']]);
  assert.match(cards[1].children[1].textContent,/失敗/);
  assert.ok(f.ui.jobs.children.some(card=>card.children[0].textContent==='older'));
  assert.equal(f.ui['result-title'].textContent,'フォルダごとのスキャン結果');
});

test('a report return URL restores just its own job; stale batch IDs do not select a different report', async()=>{
  const jobs=[ready(1,'older'),ready(2,'latest')];
  const single=await fixture('?result=1&lang=en',{jobs});
  assert.equal(single.ui['result-path'].textContent,'older');
  assert.match(single.ui['result-report'].href,/reports\/1\/report.html/);
  assert.equal(single.ui['batch-results'].hidden,true);
  const unavailable=await fixture('?results=1,999&lang=en',{jobs});
  assert.equal(unavailable.ui['result-screen'].hidden,true);
  assert.match(unavailable.ui.feedback.textContent,/unavailable/);
});

test('a root scan and its same-named child are identifiable in history', async()=>{
  const root={...ready(1,''),display_name:'api'};
  const child=ready(2,'api');
  const f=await fixture('',{migration:true,jobs:[root,child]});
  const [childCard,rootCard]=f.ui.jobs.children;
  assert.equal(rootCard.children[0].textContent,'api');
  assert.match(rootCard.children[1].textContent,/スキャンの起点/);
  assert.match(actions(rootCard)[0].attributes['aria-label'],/スキャンの起点/);
  assert.doesNotMatch(actions(childCard)[0].attributes['aria-label'],/スキャンの起点/);
});
