const {test}=require('node:test');
const assert=require('node:assert/strict');
const {readFileSync}=require('node:fs');
const {runInNewContext}=require('node:vm');
const path=require('node:path');
const source=readFileSync(path.join(__dirname,'../../crates/pqc_scan_cli/src/serve.js'),'utf8');
const html=readFileSync(path.join(__dirname,'../../crates/pqc_scan_cli/src/serve.html'),'utf8');
async function fixture(search='') {
 const node=()=>({children:[],listeners:{},disabled:false,value:'',textContent:'',dataset:{},attributes:{},open:false,files:[],classList:{toggle(){}},
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
 const window={listeners:{},addEventListener(t,f){this.listeners[t]=f;},dispatchEvent(){}};
 const document={getElementById:id=>ui[id]||null,createElement:node,querySelectorAll:selector=>selector==='[data-en]'?translated:[],body:node(),documentElement:{lang:'ja',dataset:{theme:'light'}}};
 const state={jobs:[],nextResolve:null,nextReject:null};
 const requests=[],timers=[],location={href:'http://127.0.0.1:8787/session/'+search,search};
 const history={replaceState(_state,_title,url){const next=new URL(url);location.href=next.href;location.search=next.search;}};
 const fetch=async(route,options)=>{
  requests.push([route,options?.method||'GET']);
  if(route==='api/config')return {ok:true,json:async()=>({migration:false})};
  if(route==='api/jobs')return {ok:true,json:async()=>({jobs:state.jobs})};
  if(route==='api/directories?path=')return {ok:true,json:async()=>({root:'/projects',directories:[{name:'next',path:'next'}],omitted:0})};
  if(route==='api/directories?path=next')return new Promise((resolve,reject)=>{state.nextResolve=()=>resolve({ok:true,json:async()=>({root:'/projects',directories:[],omitted:0})});state.nextReject=()=>reject(Error('denied'));});
  if(route.startsWith('api/scan?path=') && options?.method==='POST')return {ok:true,json:async()=>({id:7})};
  throw Error('Unexpected request: '+route);
 };
 const context={window,document,fetch,location,history,URL,URLSearchParams,
  CustomEvent:class {constructor(type,options){this.type=type;this.detail=options?.detail;}},
  localStorage:{setItem(){}},setTimeout:f=>timers.push(f)};
 const flush=async()=>{for(let i=0;i<3;i++)await new Promise(resolve=>setImmediate(resolve));};
 runInNewContext(source,context);await flush();
 return {ui,state,requests,location,window,document,flush,
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
