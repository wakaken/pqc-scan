const {test} = require('node:test');
const assert = require('node:assert/strict');
const {readFileSync} = require('node:fs');
const {runInNewContext} = require('node:vm');
const source = readFileSync(require('node:path').join(__dirname,'../../crates/pqc_scan_report/src/presentation.js'),'utf8');
function fixture({blocked=false, saved=null, query='', date='2026-11-18T00:30:00Z'}={}) {
  const input=()=>({value:'',listeners:{},addEventListener(t,f){this.listeners[t]=f;}});
  const language=input(),theme=input();
  const document={documentElement:{dataset:{},lang:'ja'},title:'',dispatchEvent(e){this.events.push(e);},events:[]};
  const settings={hidden:true,querySelector(s){return s==='[data-language]'?language:theme;}};
  const view=lang=>({lang,time:{dateTime:date,textContent:'HOST +09:00'},details:[{id:'finding-20',open:lang==='ja'},{id:'inspection-scope',open:lang==='ja'},{id:'shared-group-0',open:lang==='ja'}],events:[],
    querySelector(){return {textContent:lang==='ja'?'日本語レポート':'English report'};},
    querySelectorAll(s){return s==='[data-local-time]'?[this.time]:this.details;},
    dispatchEvent(e){this.events.push(e);if(e.type==='pqc-report-capture')Object.assign(e.detail,{page:2,fields:{'data-file-filter':'src/a.py'}});},
    replaceWith(next){document.active=next;}});
  const ja=view('ja'),en=view('en');document.active=ja;
  const alternate={lang:'en',content:{firstElementChild:en,append(view){this.firstElementChild=view;}}};
  document.querySelector=s=>s==='[data-settings]'?settings:s==='[data-report-translation]'?alternate:document.active;
  const writes=[];
  const localStorage={getItem(){if(blocked)throw Error('blocked');return saved;},setItem(k,v){if(blocked)throw Error('blocked');writes.push([k,v]);}};
  class CustomEvent {constructor(type,{detail}){this.type=type;this.detail=detail;}}
  runInNewContext(source,{document,localStorage,CustomEvent,Intl,Date,URLSearchParams,location:{search:query}});
  return {document,settings,language,theme,ja,en,alternate,writes,
    changeLanguage(value){language.value=value;language.listeners.change();},
    changeTheme(value){theme.value=value;theme.listeners.change();}};
}
test('language swaps one active view and preserves investigated occurrence and state',()=>{
 const f=fixture();f.changeLanguage('en');
 assert.equal(f.document.active,f.en);assert.equal(f.alternate.content.firstElementChild,f.ja);
 assert.equal(f.document.documentElement.lang,'en');assert.equal(f.document.title,'English report');
 assert.ok(f.en.details.every(d=>d.open));
 f.en.details[1].open=false;
 const state=f.en.events.find(e=>e.type==='pqc-report-restore').detail;
 assert.equal(state.page,2);assert.equal(state.fields['data-file-filter'],'src/a.py');
 f.changeLanguage('ja');assert.equal(f.document.active,f.ja);assert.equal(f.ja.details[1].open,false);
});
test('only English or Japanese can be selected, including rule-guide query navigation',()=>{
 const f=fixture({query:'?lang=en'});assert.equal(f.language.value,'en');
 f.changeLanguage('unsupported');assert.equal(f.document.active,f.en);
 const unknown=fixture({query:'?lang=unsupported'});assert.equal(unknown.document.active,unknown.ja);
});
test('time uses the viewing environment timezone and selected language',()=>{
 const f=fixture();const zone=new Intl.DateTimeFormat().resolvedOptions().timeZone;
 const expected=new Intl.DateTimeFormat('ja-JP',{dateStyle:'medium',timeStyle:'long'}).format(new Date('2026-11-18T00:30:00Z'));
 assert.equal(f.ja.time.textContent,`${expected} (${zone})`);
 f.changeLanguage('en');assert.match(f.en.time.textContent,new RegExp(zone.replaceAll('/','\\/')));
 assert.equal(f.ja.time.dateTime,'2026-11-18T00:30:00Z');
});
test('invalid times retain explicit server offset, with no uncaught Intl error',()=>{
 const f=fixture({date:'not-a-date'});assert.equal(f.ja.time.textContent,'HOST +09:00');
});
test('explicit theme persists, invalid theme defaults light, blocked storage stays usable',()=>{
 const f=fixture({saved:'dark'});assert.equal(f.document.documentElement.dataset.theme,'dark');
 f.changeTheme('light');assert.equal(f.document.documentElement.dataset.theme,'light');
 assert.deepEqual(f.writes,[['pqc-scan-theme','light']]);
 const blocked=fixture({blocked:true});blocked.changeTheme('dark');assert.equal(blocked.document.documentElement.dataset.theme,'dark');assert.equal(blocked.settings.hidden,false);
 assert.equal(fixture({saved:'unknown'}).document.documentElement.dataset.theme,'light');
});
