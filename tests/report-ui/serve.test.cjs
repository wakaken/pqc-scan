const {test}=require('node:test');
const assert=require('node:assert/strict');
const {readFileSync}=require('node:fs');
const {runInNewContext}=require('node:vm');
const source=readFileSync(require('node:path').join(__dirname,'../../crates/pqc_scan_cli/src/serve.js'),'utf8');
const settle=()=>new Promise(resolve=>setImmediate(resolve));
function fixture() {
 const node=()=>({children:[],listeners:{},disabled:false,value:'ja',textContent:'',dataset:{},classList:{toggle(){}},addEventListener(t,f){this.listeners[t]=f;},replaceChildren(){this.children=[];},append(...children){this.children.push(...children);}});
 const names=['language','feedback','root','selected','up','folders','folder-note','scan','jobs'];const ui=Object.fromEntries(names.map(n=>[n,node()]));
 const window={listeners:{},addEventListener(t,f){this.listeners[t]=f;}};
 const document={getElementById:id=>ui[id],createElement:node,querySelectorAll:()=>[],documentElement:{lang:'ja'}};
 let childrenResolve,childrenReject;let jobs=[];const posts=[],timers=[],location={href:''};
 const fetch=async (route,options)=>{
   if(route==='api/jobs')return {ok:true,json:async()=>({jobs})};
   if(route==='api/directories?path=')return {ok:true,json:async()=>({root:'/projects',directories:[{name:'next',path:'next'}],omitted:0})};
   if(route.startsWith('api/directories?path=next'))return new Promise((resolve,reject)=>{childrenResolve=value=>resolve({ok:true,json:async()=>value});childrenReject=reject;});
   posts.push([route,options]);return {ok:true,json:async()=>({id:7})};
 };
 runInNewContext(source,{window,document,fetch,location,setTimeout:f=>timers.push(f),Error,encodeURIComponent});
 return {ui,posts,timers,location,window,document,setJobs:v=>jobs=v,resolveFolder:()=>childrenResolve({root:'/projects',directories:[],omitted:0}),rejectFolder:()=>childrenReject(new Error('unavailable'))};
}
test('pending folder navigation blocks old-folder scans through polling and language changes',async()=>{
 const f=fixture();await settle();assert.equal(f.ui.scan.disabled,false);
 f.ui.folders.children[0].children[0].listeners.click();assert.equal(f.ui.scan.disabled,true);
 await f.timers.shift()();assert.equal(f.ui.scan.disabled,true);
 f.ui.language.value='en';f.ui.language.listeners.change();assert.equal(f.ui.scan.disabled,true);
 await f.ui.scan.listeners.click();assert.equal(f.posts.length,0);
 f.resolveFolder();await settle();assert.equal(f.ui.selected.textContent,'next');assert.equal(f.ui.scan.disabled,false);
 await f.ui.scan.listeners.click();assert.equal(f.posts.length,1);assert.equal(f.posts[0][0],'api/scan?path=next');assert.equal(f.posts[0][1].method,'POST');
 f.setJobs([{id:7,path:'next',status:'ready',coverage:'partial',report:'reports/7/report.html'}]);await f.timers.shift()();assert.equal(f.location.href,'reports/7/report.html');
 assert.ok(f.timers.length>0,'polling resumes after history returns to the cached start page');
 f.ui.language.value='en';f.window.listeners.pageshow();assert.equal(f.document.documentElement.lang,'en');
 f.setJobs([{id:7,path:'next',status:'ready',report:'reports/7/report.html'},{id:8,path:'next',status:'running'}]);await f.timers.shift()();assert.equal(f.ui.scan.disabled,true);
 f.setJobs([{id:8,path:'next',status:'ready',report:'reports/8/report.html'}]);await f.timers.shift()();assert.equal(f.ui.scan.disabled,false);
});
test('folder and scan failures remain explicit and allow retry without false zero results',async()=>{
 const f=fixture();await settle();f.ui.folders.children[0].children[0].listeners.click();f.rejectFolder();await settle();
 assert.match(f.ui.feedback.textContent,/フォルダを開けません/);assert.equal(f.ui.selected.textContent,'/');assert.equal(f.ui.scan.disabled,false);
 await f.ui.scan.listeners.click();f.setJobs([{id:7,path:'',status:'failed'}]);await f.timers.shift()();assert.equal(f.location.href,'');assert.equal(f.ui.scan.disabled,false);assert.match(f.ui.feedback.textContent,/失敗/);
});

test('history-restored language selection matches the labels and document language',async()=>{
 const f=fixture();await settle();f.ui.language.value='en';f.window.listeners.pageshow();assert.equal(f.document.documentElement.lang,'en');
});
