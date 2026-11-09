const {test} = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

// A small DOM test double exercises the embedded script without opening a browser.
class Element {
  constructor(tag='div', attrs={}) { this.tagName=tag.toUpperCase(); this.attrs=attrs; this.children=[]; this.parentElement=null; this.hidden=false; this.disabled=false; this.value=''; this.open=false; this.textContent=''; this.listeners={}; this.dataset={}; for(const [key,value]of Object.entries(attrs)) if(key.startsWith('data-')) this.dataset[key.slice(5).replace(/-([a-z])/g,(_,c)=>c.toUpperCase())]=value; }
  append(...children) { for(const child of children){child.parentElement=this;this.children.push(child);}return this; }
  matches(selector) { return selector.split(',').some(s=>{s=s.trim();return s.startsWith('[')?Object.hasOwn(this.attrs,s.slice(1,-1)):this.tagName.toLowerCase()===s;}); }
  querySelectorAll(selector) { return this.children.flatMap(c=>[...(c.matches(selector)?[c]:[]),...c.querySelectorAll(selector)]); }
  querySelector(selector) { return this.querySelectorAll(selector)[0]||null; }
  closest(selector) { for(let p=this;p;p=p.parentElement)if(p.matches(selector))return p;return null; }
  contains(child){for(let p=child;p;p=p.parentElement)if(p===this)return true;return false;}
  addEventListener(type,listener){this.listeners[type]=listener;}
  focus(){this.focused=true;}
  scrollIntoView(){this.scrolled=true;}
}
const el=(tag,key,value='')=>new Element(tag,{[key]:value});
function harness() {
  const root=el('main','data-compact-report');
  const controls=el('section','data-report-controls');
  for(const key of ['mode','search','severity','risk']) controls.append(el(key==='search'?'input':'select',`data-${key}`));
  controls.querySelector('[data-mode]').value='candidate';
  controls.append(el('button','data-reset'));
  root.append(controls,el('p','data-report-status'),el('p','data-empty'),new Element('nav').append(el('button','data-prev'),el('span','data-page'),el('button','data-next')));
  root.children[3].attrs['data-group-pager']='';
  const rules=[];
  for(let g=0;g<25;g++){
    const rule=new Element('details',{'data-rule':'',id:`rule-${g}`});
    rule.append(new Element('summary'),el('span','data-match-count'),el('nav','data-occ-pager').append(el('button','data-occ-prev'),el('span','data-occ-status'),el('button','data-occ-next')));
    const list=el('div','data-occurrence-list');
    for(let i=0;i<25;i++)list.append(new Element('details',{'data-finding-id':`f-${g}-${i}`,'data-inventory':String(g>=21),'data-severity':g>=21?'info':'high','data-risk':g>=21?'quantum-safe':'quantum-vulnerable','data-search':`rule-${g} source-${g}.java`,id:`finding-${g*25+i}`}).append(new Element('summary')));
    rule.append(list);root.append(rule);rules.push(rule);
  }
  const location={hash:''},window={listeners:{},addEventListener(k,f){this.listeners[k]=f;}};
  const document={documentElement:{lang:'en'},querySelector:s=>root.matches(s)?root:root.querySelector(s),getElementById:id=>[root,...root.querySelectorAll('details')].find(e=>e.attrs.id===id)};
  vm.runInNewContext(fs.readFileSync(path.join(__dirname,'../crates/pqc_scan_report/src/compact.js'),'utf8'),{document,window,location});
  const click=key=>root.listeners.click({target:root.querySelector(`[data-${key}]`)});
  const input=()=>controls.listeners.input();
  return {root,controls,rules,click,input,location,window,document};
}

test('default is compact and occurrence/group pagination preserves every record',()=>{
  const h=harness();
  assert.equal(h.rules.filter(r=>!r.hidden).length,20);
  assert.equal(h.rules[0].querySelectorAll('[data-finding-id]').filter(r=>!r.hidden).length,20);
  assert.ok(h.rules.every(r=>!r.open));
  h.click('occ-next');
  assert.equal(h.rules[0].querySelectorAll('[data-finding-id]').filter(r=>!r.hidden).length,5);
  h.click('next');assert.equal(h.rules.filter(r=>!r.hidden).length,1);
  h.click('reset');assert.equal(h.rules.filter(r=>!r.hidden).length,20);
  assert.equal(h.root.querySelectorAll('[data-finding-id]').length,625);
});
test('inventory, search, empty filters and resetting report correct counts',()=>{
  const h=harness(),get=k=>h.controls.querySelector(`[data-${k}]`);
  get('mode').value='inventory';h.input();assert.equal(h.rules.filter(r=>!r.hidden).length,4);
  assert.ok(h.root.querySelector('[data-report-status]').textContent.includes('100 / 625'));
  get('search').value='source-24.java';h.input();assert.deepEqual(h.rules.filter(r=>!r.hidden),[h.rules[24]]);
  get('severity').value='critical';h.input();assert.equal(h.root.querySelector('[data-empty]').hidden,false);
  h.click('reset');assert.equal(h.root.querySelector('[data-empty]').hidden,true);
  get('severity').value='info';h.input();assert.equal(get('mode').value,'all');assert.equal(h.rules.filter(r=>!r.hidden).length,4);
});
test('deep links reveal inventory findings outside both pagers',()=>{
  const h=harness();h.location.hash='#finding-624';h.window.listeners.hashchange();
  const target=h.document.getElementById('finding-624');
  assert.equal(h.rules[24].hidden,false);assert.equal(target.hidden,false);assert.equal(target.open,true);assert.equal(h.rules[24].open,true);assert.equal(target.scrolled,true);
  h.location.hash='#%broken';assert.doesNotThrow(()=>h.window.listeners.hashchange());
});
