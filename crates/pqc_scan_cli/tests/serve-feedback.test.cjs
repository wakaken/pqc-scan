const { test } = require('node:test');
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const vm = require('node:vm');
const path = require('node:path');
// Execute the bundled controller with its DOM/HTTP boundary substituted.
// Assertions concern visible states and requests, not private controller variables.
async function page(search = '', initialJobs = [], directories = {}) {
  const nodes = new Map(), timers = [], requests = [], windowListeners = {};
  function element(id = '') {
    return { id, hidden: id === 'saved-pane', disabled: false, value: id === 'language' ? 'ja' : '',
      textContent: '', open: false, children: [], dataset: {}, attributes: {}, listeners: {}, files: [],
      classList: { toggle() {} },
      setAttribute(k, v) { this.attributes[k] = v; }, removeAttribute(k) { delete this.attributes[k]; },
      addEventListener(k, fn) { this.listeners[k] = fn; },
      append(n) { this.children.push(n); }, replaceChildren(...n) { this.children = n; }, focus() { this.focused = true; },
      showModal() { this.open = true; }, close() { this.open = false; },
      click() { if (!this.disabled) return this.listeners.click?.({detail:1}); },
    };
  }
  const get = id => { if (!nodes.has(id)) nodes.set(id, element(id)); return nodes.get(id); };
  const state = { jobs: initialJobs, fail: false, directories, directoryRequest: null, nextId:1, startRequest:null, jobsRequest:null };
  const location = { href: 'http://127.0.0.1:8787/session/' + search, search };
  const document = { documentElement: { lang: 'ja', dataset: { theme: 'dark' } },
    getElementById: get, querySelectorAll: () => [], createElement: () => element(), body: element() };
  const context = { document, location, URL, URLSearchParams, CustomEvent: class {},
    history: { replaceState(_a, _b, url) { location.href = String(url); } },
    window: { addEventListener(k, fn) { windowListeners[k] = fn; }, dispatchEvent() {} },
    localStorage: { setItem() {} }, setTimeout: fn => timers.push(fn),
    fetch: async (route, options) => {
      requests.push([route, options?.method || 'GET']);
      if (route.startsWith('api/scan') && state.startRequest) return state.startRequest(route);
      if (route === 'api/jobs' && state.jobsRequest) return state.jobsRequest();
      if (route === 'api/jobs' && state.fail) throw Error('connection lost');
      if (route.startsWith('api/directories') && state.directoryRequest) return state.directoryRequest(route);
      const data = route === 'api/config' ? { migration: false }
        : route.startsWith('api/directories') ? { root: '/project', directories: state.directories[new URL(route, location.href).searchParams.get('path')] || [] }
        : route === 'api/jobs' ? { jobs: state.jobs }
        : { id: state.nextId++ };
      return { ok: true, json: async () => data };
    },
  };
  vm.runInNewContext(readFileSync(path.join(__dirname, '../src/serve.js'), 'utf8'), context);
  const flush = async () => { for (let i = 0; i < 3; i++) await new Promise(resolve => setImmediate(resolve)); };
  await flush();
  return { state, get, requests, location, flush, windowListeners, async tick() { assert.ok(timers.length); await timers.shift()(); await flush(); } };
}
test('spinner starts immediately, reports actual counts, and stops on scan failure with retry enabled', async () => {
  const p = await page();
  assert.equal(p.get('scan').disabled, true);
  p.get('folder-path').children[0].click();
  const click = p.get('scan').listeners.click();
  assert.equal(p.get('scan-progress').open, true);
  assert.equal(p.get('scan').attributes['aria-busy'], 'true');
  await click;
  p.state.jobs = [{ id: 1, status: 'running', progress: { phase: 'inspecting', processed: 3, total: 9 } }];
  await p.tick();
  assert.match(p.get('progress-label').textContent, /3 \/ 9件/);
  assert.equal(p.get('progress-bar').value, 3);
  assert.equal(p.get('progress-bar').max, 9);
  p.state.jobs = [{ id: 1, status: 'failed' }];
  await p.tick();
  assert.equal(p.get('scan-progress').open, false);
  assert.equal(p.get('scan').disabled, false);
  assert.equal(p.get('scan').attributes['aria-busy'], 'false');
  assert.match(p.get('feedback').textContent, /失敗/);
});
test('an exact ready result restores without rescanning; unavailable IDs do not select another job', async () => {
  const jobs = [{ id: 12, status: 'ready', report: 'reports/12/report.html', path: 'sub', coverage: 'complete_within_scope' }];
  const p = await page('?result=12&theme=dark&lang=en', jobs);
  assert.equal(p.get('result-screen').hidden, false);
  assert.equal(p.get('input-screen').hidden, true);
  assert.equal(p.get('result-path').textContent, 'sub');
  assert.equal(p.get('result-report').href, 'http://127.0.0.1:8787/session/reports/12/report.html?theme=dark&lang=en');
  assert.equal(p.requests.filter(r => r[1] === 'POST').length, 0);
  const missing = await page('?result=13&lang=en', jobs);
  assert.match(missing.get('feedback').textContent, /unavailable/);
  assert.equal(missing.get('input-screen').hidden, false);
  assert.equal(missing.requests.filter(r => r[1] === 'POST').length, 0);
});
test('connection failure removes stale progress and recovery clears the connection error', async () => {
  const p = await page('', [{ id: 1, status: 'running', progress: { phase: 'inspecting', processed: 3, total: 9 } }]);
  assert.equal(p.get('scan-progress').open, true);
  p.state.fail = true;
  await p.tick();
  assert.equal(p.get('scan-progress').open, true);
  assert.equal(p.get('progress-bar').hidden,true);
  assert.match(p.get('progress-error').textContent,/接続/);
  assert.match(p.get('feedback').textContent, /接続できません/);
  p.get('language').value = 'en';
  p.get('language').listeners.change();
  assert.equal(p.get('scan-progress').open, true, 'connection failure cannot unlock active work');
  assert.match(p.get('feedback').textContent, /Cannot reach the server/);
  p.state.fail = false;
  await p.tick();
  assert.equal(p.get('scan-progress').open, true);
  assert.equal(p.get('feedback').textContent, '');
});

test('restoring imported results does not browse the display label as a source folder',async()=>{
 const p=await page('?result=12&lang=en',[{id:12,status:'ready',kind:'saved',path:'Saved inventory',migration_input:true}]);
 assert.equal(p.get('result-screen').hidden,false);
 assert.equal(p.get('feedback').textContent,'');
 assert.deepEqual(p.requests.filter(r=>r[0].startsWith('api/directories')).map(r=>r[0]),['api/directories?path=']);
 assert.equal(p.requests.filter(r=>r[1]==='POST').length,0);
});


test('session results show source identity, Japan request time, explicit missing time and newest first', async () => {
 const p = await page('', [
  {id:2,kind:'saved',status:'ready',display_name:'保存.json',created_at_ms:Date.UTC(2026,10,20,0,0)},
  {id:1,status:'ready',path:'',display_name:'product-root'},
 ]);
 const rows=p.get('jobs').children;
 assert.equal(rows[0].children[0].textContent,'保存.json');
 assert.ok(rows[0].children.some(n=>n.textContent==='最新'));
 assert.ok(rows[0].children.some(n=>n.dateTime==='2026-11-20T00:00:00.000Z'));
 assert.equal(rows[1].children[0].textContent,'product-root');
 assert.ok(rows[1].children.some(n=>n.textContent==='日時未記録'));
});

const folders = { '': [{name:'alpha',path:'alpha'}, {name:'beta',path:'beta'}],
  alpha: [{name:'nested',path:'alpha/nested'}], 'alpha/nested': [] };
const row = (p, i) => p.get('folders').children[i].children[0];
const key = (name, extra = {}) => ({ key: name, preventDefault() { this.prevented = true; }, ...extra });
test('single click selects the child without browsing; scan submits the selected child path', async () => {
 const p = await page('', [], folders);
 assert.equal(p.get('scan').disabled, true);
 const first = row(p,0);
 first.click();
 assert.equal(row(p,0), first, 'selection must preserve the DOM target for double-click');
 assert.equal(first.attributes['aria-pressed'], 'true');
 assert.equal(row(p,1).attributes['aria-pressed'], 'false');
 assert.equal(p.requests.filter(([r])=>r.startsWith('api/directories')).length,1);
 assert.equal(p.get('selected').textContent,'選択中：alpha');
 assert.equal(p.get('scan').disabled,false);
 await p.get('scan').click();
 assert.deepEqual(p.requests.filter(([,m])=>m==='POST'),[['api/scan?path=alpha&max_file_size_mib=2','POST']]);
});
test('double-click opens, breadcrumbs select current/root, and history restores selection', async () => {
 const p = await page('', [], folders);
 row(p,0).click();
 row(p,0).listeners.dblclick(); await p.flush();
 assert.equal(p.get('selected').textContent,'選択中：alpha');
 assert.equal(p.get('folder-path').children.length,2);
 assert.equal(p.get('folder-forward').disabled,true);
 assert.equal(p.get('folder-back').disabled,false);
 row(p,0).click(); row(p,0).listeners.dblclick(); await p.flush();
 assert.equal(p.get('folders').children.length,0);
 assert.equal(p.get('scan').disabled,false,'an opened empty directory is selectable');
 p.get('folder-back').click(); await p.flush();
 assert.equal(p.get('selected').textContent,'選択中：2フォルダ');
 p.get('folder-forward').click(); await p.flush();
 assert.equal(p.get('folder-path').children.length,3);
 p.get('folder-path').children[0].click(); await p.flush();
 assert.equal(p.get('selected').textContent,'選択中：2フォルダ');
 assert.equal(p.get('folder-path').children.length,1);
 assert.equal(p.get('folder-forward').disabled,true,'new navigation truncates forward history');
 p.get('clear-folders').click(); p.get('folder-path').children[0].click();
 await p.get('scan').click();
 assert.ok(p.requests.some(([r,m])=>r==='api/scan?path=&max_file_size_mib=2' && m==='POST'));
});
test('keyboard selection, Enter, touch open button and parent shortcut navigate without selection confirmation',async()=>{
 const p=await page('',[],folders);
 row(p,0).listeners.keydown(key('ArrowDown'));
 assert.equal(p.get('selected').textContent,'');
 assert.equal(row(p,1).focused,true);
 row(p,1).listeners.keydown(key('Home'));
 assert.equal(p.get('selected').textContent,'');
 row(p,0).click();
 row(p,0).listeners.keydown(key('Enter')); await p.flush();
 assert.equal(p.get('folder-path').children.length,2);
 p.get('folders').children[0].children[1].click(); await p.flush();
 assert.equal(p.get('folder-path').children.length,3);
 p.get('folder-pane').listeners.keydown(key('ArrowUp',{altKey:true})); await p.flush();
 assert.equal(p.get('folder-path').children.length,2);
});
test('pending/failed directory navigation cannot scan a different folder and preserves successful history',async()=>{
 const p=await page('',[],folders); row(p,0).click();
 let reject;
 p.state.directoryRequest=()=>new Promise((_resolve,r)=>{reject=r;});
 p.get('folders').children[0].children[1].click();
 assert.equal(p.get('scan').disabled,true);
 assert.equal(p.get('folder-back').disabled,true);
 await p.get('scan').listeners.click();
 assert.equal(p.requests.filter(([,m])=>m==='POST').length,0);
 reject(Error('denied'));await p.flush();
 assert.equal(p.get('selected').textContent,'選択中：alpha');
 assert.equal(p.get('scan').disabled,false);
 assert.equal(p.get('folder-path').children.length,1);
 assert.equal(p.get('folder-back').disabled,true);
 assert.match(p.get('feedback').textContent,/開けません/);
});
test('folder names containing spaces, separators and Unicode retain their exact scan path',async()=>{
 const name='研究 & data#1'; const p=await page('',[],{'':[{name,path:name}]});
 assert.equal(row(p,0).children[1].textContent,name);
 row(p,0).click();await p.get('scan').click();
 assert.ok(p.requests.some(([r,m])=>r==='api/scan?path='+encodeURIComponent(name)+'&max_file_size_mib=2'&&m==='POST'));
});
test('a stale directory response cannot overwrite a newer successful navigation',async()=>{
 const p=await page('',[],folders);const pending=[];
 p.state.directoryRequest=()=>new Promise(resolve=>pending.push(resolve));
 const openAlpha=p.get('folders').children[0].children[1].listeners.click;
 const openBeta=p.get('folders').children[1].children[1].listeners.click;
 openAlpha();openBeta();
 pending[1]({ok:true,json:async()=>({root:'/project',directories:[]})});await p.flush();
 assert.equal(p.get('selected').textContent,'');
 pending[0]({ok:true,json:async()=>({root:'/project',directories:folders.alpha})});await p.flush();
 assert.equal(p.get('selected').textContent,'');
 assert.equal(p.get('folders').children.length,0);
 assert.equal(p.get('folder-path').children[1].textContent,'beta');
});

const posts = p => p.requests.filter(([,method])=>method==='POST').map(([route])=>route);
const ready = (id, path) => ({id,path,display_name:path,status:'ready',report:`reports/${id}/report.html`,coverage:'complete_within_scope'});
test('multiple checkbox selections survive navigation; remove/clear and deselection stay exact', async()=>{
 const p=await page('',[],folders);
 const check=p.get('folders').children[0].children[2].children[0];
 check.checked=true;check.listeners.change();
 row(p,1).click();
 assert.equal(p.get('selected-folders').children.length,2);
 assert.equal(check.checked,true);
 p.get('folders').children[0].children[1].click();await p.flush();
 assert.deepEqual(p.get('selected-folders').children.map(li=>li.children[0].textContent),['alpha','beta']);
 row(p,0).click();
 assert.deepEqual(p.get('selected-folders').children.map(li=>li.children[0].textContent),['alpha','beta','alpha/nested']);
 p.get('selected-folders').children[1].children[1].click();
 assert.deepEqual(p.get('selected-folders').children.map(li=>li.children[0].textContent),['alpha','alpha/nested']);
 p.get('clear-folders').click();assert.equal(p.get('scan').disabled,true);
 assert.equal(p.get('selected-folders').children.length,0);
 assert.equal(row(p,0).attributes['aria-pressed'],'false');
});
test('two scans are strictly sequential and remain individually reviewable without navigating early',async()=>{
 const p=await page('',[],folders);row(p,0).click();row(p,1).click();
 await p.get('scan').click();
 assert.deepEqual(posts(p),['api/scan?path=alpha&max_file_size_mib=2']);
 assert.equal(p.get('scan-progress').open,true);
 assert.match(p.get('progress-position').textContent,/1 \/ 2/);
 assert.equal(p.get('progress-folder').textContent,'alpha');
 p.state.jobs=[{id:1,path:'alpha',status:'running',progress:{phase:'inspecting',processed:3,total:7}}];
 await p.tick();assert.equal(posts(p).length,1);assert.equal(p.get('progress-bar').value,3);
 p.state.jobs=[ready(1,'alpha')];await p.tick();
 assert.deepEqual(posts(p),['api/scan?path=alpha&max_file_size_mib=2','api/scan?path=beta&max_file_size_mib=2']);
 assert.equal(p.get('scan-progress').open,true,'no unlock between folders');
 assert.match(p.get('progress-position').textContent,/2 \/ 2/);
 assert.equal(p.get('progress-folder').textContent,'beta');
 assert.equal(p.get('input-screen').hidden,false,'intermediate result cannot navigate');
 p.state.jobs=[ready(1,'alpha'),ready(2,'beta')];await p.tick();
 assert.equal(p.get('scan-progress').open,false);
 assert.equal(p.get('result-screen').hidden,false);
 assert.equal(p.get('result-single').hidden,true,'a batch must not select the last folder implicitly');
 assert.equal(p.get('batch-results').hidden,false);
 const cards=p.get('batch-results').children;
 assert.deepEqual(cards.map(card=>card.children[0].textContent),['alpha','beta']);
 const reportLink=card=>card.children.find(n=>n.className==='job-actions').children[0];
 for(const [index,card] of cards.entries()) {
  const link=reportLink(card);
  assert.equal(link.href,`http://127.0.0.1:8787/session/reports/${index+1}/report.html?theme=dark&lang=ja`);
  assert.equal(link.target,'_blank');
  assert.match(link.attributes['aria-label'],new RegExp(index===0?'alpha':'beta'));
 }
 assert.deepEqual(p.get('jobs').children.map(card=>card.children[0].textContent),['beta','alpha']);
 assert.equal(reportLink(p.get('jobs').children[1]).href,reportLink(cards[0]).href,'history preserves the same folder/report association');
 assert.match(p.get('feedback').textContent,/2フォルダ完了、0フォルダ失敗/);
 const url=new URL(p.location.href);
 assert.equal(url.searchParams.get('results'),'1,2');
 assert.equal(url.searchParams.has('result'),false);
});
test('modal cannot cancel; page changes, selection and duplicate submission are blocked until all work ends',async()=>{
 const p=await page('',[],folders);row(p,0).click();row(p,1).click();await p.get('scan').click();
 const cancel=key('Escape');p.get('scan-progress').listeners.cancel(cancel);assert.equal(cancel.prevented,true);
 const tab=key('Tab');p.get('scan-progress').listeners.keydown(tab);assert.equal(tab.prevented,true);assert.equal(p.get('cancel-scans').focused,true);
 const leave=key('');p.windowListeners.beforeunload(leave);assert.equal(leave.prevented,true);
 p.get('saved-tab').listeners.click();p.get('choose-input').listeners.click();
 p.get('clear-folders').listeners.click();row(p,0).listeners.click({detail:1});
 p.get('folders').children[0].children[1].listeners.click();await p.get('scan').listeners.click();
 assert.equal(p.get('saved-pane').hidden,true);
 assert.equal(p.get('selected-folders').children.length,2);
 assert.equal(p.requests.filter(([r])=>r.startsWith('api/directories')).length,1);
 assert.equal(posts(p).length,1);
});
test('failed folder still advances to the next and remains a failed result',async()=>{
 const p=await page('',[],folders);row(p,0).click();row(p,1).click();await p.get('scan').click();
 p.state.jobs=[{id:1,path:'alpha',status:'failed'}];await p.tick();assert.equal(posts(p).length,2);
 p.state.jobs.push(ready(2,'beta'));await p.tick();
 assert.equal(p.get('scan-progress').open,false);
 assert.match(p.get('feedback').textContent,/1フォルダ完了、1フォルダ失敗/);
 assert.ok(p.get('jobs').children[1].children.some(n=>/スキャンに失敗/.test(n.textContent)));
});
test('uncertain start stops remaining folders and polls before unlocking; accepted work is never posted twice',async()=>{
 const p=await page('',[],folders);row(p,0).click();row(p,1).click();
 p.state.startRequest=async()=>{throw Error('response lost');};await p.get('scan').click();
 assert.equal(p.get('scan-progress').open,true,'unknown start outcome stays locked');
 p.state.jobs=[{id:1,path:'alpha',status:'running'}];await p.tick();
 assert.equal(p.get('scan-progress').open,true,'accepted scan is still running');
 assert.equal(posts(p).length,1);
 assert.match(p.get('feedback').textContent,/残り2フォルダ/);
 p.state.jobs=[ready(1,'alpha')];await p.tick();
 assert.equal(p.get('scan-progress').open,false);assert.equal(posts(p).length,1);
 assert.equal(p.get('jobs').children.length,1);
});
test('lost polling keeps queue locked and resumes exactly once after recovery',async()=>{
 const p=await page('',[],folders);row(p,0).click();row(p,1).click();await p.get('scan').click();
 p.state.fail=true;await p.tick();assert.equal(p.get('scan-progress').open,true);
 assert.equal(posts(p).length,1);assert.match(p.get('progress-error').textContent,/接続/);
 p.state.fail=false;p.state.jobs=[ready(1,'alpha')];await p.tick();
 assert.equal(posts(p).length,2);assert.equal(p.get('progress-error').textContent,'');
 p.state.jobs.push(ready(2,'beta'));await p.tick();assert.equal(p.get('scan-progress').open,false);
});
test('selection beyond session capacity is rejected before any scan starts',async()=>{
 const p=await page('',Array.from({length:19},(_,i)=>ready(i+1,'old'+i)),folders);
 row(p,0).click();row(p,1).click();await p.get('scan').click();
 assert.equal(posts(p).length,0);assert.equal(p.get('scan-progress').open,false);
 assert.match(p.get('feedback').textContent,/残り件数/);
});
test('single default-mode scan retains automatic report navigation and unlocks at completion',async()=>{
 const p=await page('',[],folders);row(p,0).click();await p.get('scan').click();
 p.state.jobs=[ready(1,'alpha')];await p.tick();
 assert.match(p.location.href,/reports\/1\/report.html/);
 assert.equal(p.get('scan-progress').open,false);
 const leave=key('');p.windowListeners.beforeunload(leave);assert.equal(leave.prevented,undefined);
});

test('a jobs response issued before an uncertain start cannot unlock or resubmit accepted work',async()=>{
 const p=await page('',[],folders);row(p,0).click();row(p,1).click();
 let resolveOld;
 p.state.jobsRequest=()=>new Promise(resolve=>{resolveOld=resolve;});
 const oldPoll=p.tick();
 p.state.startRequest=async()=>{throw Error('lost response');};await p.get('scan').click();
 resolveOld({ok:true,json:async()=>({jobs:[]})});await oldPoll;
 assert.equal(p.get('scan-progress').open,true);
 assert.equal(p.get('scan').disabled,true);
 await p.get('scan').listeners.click();assert.equal(posts(p).length,1);
 p.state.jobsRequest=null;p.state.jobs=[{id:1,path:'alpha',status:'running'}];await p.tick();
 assert.equal(p.get('scan-progress').open,true);
 assert.equal(posts(p).length,1);
 p.state.jobs=[ready(1,'alpha')];await p.tick();
 assert.equal(p.get('scan-progress').open,false);
 assert.equal(row(p,0).disabled,false);
 p.get('folders').children[0].children[1].click();await p.flush();
 assert.equal(p.get('folder-path').children.length,2,'browsing works after unknown submission settles');
});
test('folder rows, checkboxes, breadcrumbs and browsing unlock after successful or all-failed batches',async()=>{
 for(const success of [true,false]) {
  const p=await page('',[],folders);row(p,0).click();row(p,1).click();await p.get('scan').click();
  const job=(id,path)=>success?ready(id,path):{id,path,status:'failed'};
  p.state.jobs=[job(1,'alpha')];await p.tick();
  p.state.jobs.push(job(2,'beta'));await p.tick();
  p.get('choose-input').click();
  assert.equal(row(p,0).disabled,false);
  assert.equal(p.get('folders').children[0].children[2].children[0].disabled,false);
  assert.equal(p.get('folder-path').children[0].disabled,false);
  p.get('folders').children[0].children[1].click();await p.flush();
  assert.equal(p.get('folder-path').children.length,2);
  assert.equal(p.get('folder-back').disabled,false);
 }
});
test('a delayed start response holds the modal and cannot create duplicate queue submissions',async()=>{
 const p=await page('',[],folders);row(p,0).click();row(p,1).click();
 let resolveStart;p.state.startRequest=()=>new Promise(resolve=>{resolveStart=resolve;});
 const start=p.get('scan').click();assert.equal(p.get('scan-progress').open,true);
 await p.tick();await p.get('scan').listeners.click();
 assert.equal(posts(p).length,1);assert.equal(p.get('scan-progress').open,true);
 resolveStart({ok:true,json:async()=>({id:1})});await start;
 assert.equal(posts(p).length,1);
});
test('selection is bounded to twenty exact unique paths and can be reduced',async()=>{
 const entries=Array.from({length:21},(_,i)=>({name:'folder'+i,path:'folder'+i}));
 const p=await page('',[],{'':entries});
 for(let i=0;i<21;i++)row(p,i).click();
 assert.equal(p.get('selected-folders').children.length,20);
 assert.equal(row(p,20).attributes['aria-pressed'],'false');
 assert.match(p.get('feedback').textContent,/20件まで/);
 row(p,0).click();row(p,20).click();
 assert.equal(p.get('selected-folders').children.length,20);
 assert.equal(row(p,0).attributes['aria-pressed'],'false');
 assert.equal(row(p,20).attributes['aria-pressed'],'true');
});

test('cancel stops every pending folder while preserving the running folder and its result',async()=>{
 const p=await page('',[],folders);row(p,0).click();row(p,1).click();await p.get('scan').click();
 p.get('cancel-scans').click();assert.equal(p.get('scan-progress').open,true);
 assert.equal(p.get('cancel-scans').disabled,true);
 p.state.jobs=[{id:1,path:'alpha',status:'running'}];await p.tick();assert.equal(posts(p).length,1);
 p.state.jobs=[ready(1,'alpha')];await p.tick();assert.equal(posts(p).length,1);
 assert.equal(p.get('scan-progress').open,false);assert.equal(p.get('result-path').textContent,'alpha');
 assert.match(p.get('feedback').textContent,/キャンセル/);assert.equal(p.get('jobs').children.length,1);
});

test('cancel during a pending POST still waits for that accepted job and never starts another',async()=>{
 const p=await page('',[],folders);row(p,0).click();row(p,1).click();let resolve;
 p.state.startRequest=()=>new Promise(r=>resolve=r);const start=p.get('scan').click();p.get('cancel-scans').click();
 resolve({ok:true,json:async()=>({id:1})});await start;
 p.state.jobs=[ready(1,'alpha')];await p.tick();assert.equal(posts(p).length,1);
 assert.equal(p.get('scan-progress').open,false);assert.match(p.get('feedback').textContent,/キャンセル/);
});

test('test exclusion is explicit, locked and captured for the entire sequential batch',async()=>{
 const p=await page('',[],folders);row(p,0).click();row(p,1).click();
 assert.equal(Boolean(p.get('exclude-tests').checked),false);
 p.get('exclude-tests').checked=true;await p.get('scan').click();
 assert.deepEqual(posts(p),['api/scan?path=alpha&exclude_tests=true&max_file_size_mib=2']);
 assert.equal(p.get('exclude-tests').disabled,true);
 p.get('exclude-tests').checked=false; // Even programmatic mutation cannot change the captured batch option.
 p.state.jobs=[ready(1,'alpha')];await p.tick();
 assert.deepEqual(posts(p),['api/scan?path=alpha&exclude_tests=true&max_file_size_mib=2','api/scan?path=beta&exclude_tests=true&max_file_size_mib=2']);
 p.state.jobs=[ready(1,'alpha'),ready(2,'beta')];await p.tick();
 assert.equal(p.get('exclude-tests').disabled,false);
});

test('size limit is validated before POST and frozen for the complete batch',async()=>{
 const p=await page('',[],folders);row(p,0).click();row(p,1).click();
 assert.equal(p.get('max-file-size-mib').value,'2');
 for(const value of ['', '0','65','2.5','not-a-number']){
  p.get('max-file-size-mib').value=value; await p.get('scan').listeners.click();
  assert.equal(posts(p).length,0); assert.match(p.get('file-size-error').textContent,/1〜64/);
 }
 p.get('max-file-size-mib').value='16';await p.get('scan').click();
 assert.deepEqual(posts(p),['api/scan?path=alpha&max_file_size_mib=16']);
 assert.equal(p.get('max-file-size-mib').disabled,true);
 p.get('max-file-size-mib').value='4';p.state.jobs=[ready(1,'alpha')];await p.tick();
 assert.deepEqual(posts(p),['api/scan?path=alpha&max_file_size_mib=16','api/scan?path=beta&max_file_size_mib=16']);
 p.state.jobs=[ready(1,'alpha'),ready(2,'beta')];await p.tick();assert.equal(p.get('max-file-size-mib').disabled,false);
});

test('live size validation disables scanning across polling and navigation until corrected',async()=>{
 const p=await page('',[],folders);row(p,0).click();const field=p.get('max-file-size-mib');
 for(const value of ['65','100','','0','2.5','2e1','abc']){
  field.value=value;field.listeners.input();
  assert.equal(p.get('scan').disabled,true);assert.equal(field.attributes['aria-invalid'],'true');
  assert.equal(p.get('file-size-error').hidden,false);
  await p.tick();assert.equal(p.get('scan').disabled,true,'poll cannot enable an invalid scan');
 }
 field.value='64';field.listeners.input();assert.equal(p.get('scan').disabled,false);
 assert.equal(p.get('file-size-error').hidden,true);assert.equal(field.attributes['aria-invalid'],'false');
 let prevented=false;field.listeners.keydown({key:'e',preventDefault(){prevented=true;}});assert.equal(prevented,true);
 prevented=false;field.listeners.beforeinput({data:'2e1',preventDefault(){prevented=true;}});assert.equal(prevented,true);
 prevented=false;field.listeners.paste({clipboardData:{getData:()=>'+2'},preventDefault(){prevented=true;}});assert.equal(prevented,true);
 prevented=false;field.listeners.keydown({key:'a',metaKey:true,preventDefault(){prevented=true;}});assert.equal(prevented,false);
 await p.get('scan').click();assert.deepEqual(posts(p),['api/scan?path=alpha&max_file_size_mib=64']);
});
