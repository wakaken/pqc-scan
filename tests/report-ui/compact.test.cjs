// Dependency-free controller regression tests. Rust tests cover emitted markup;
// actual browser acceptance separately covers native details, layout and printing.
const {test} = require('node:test');
const assert = require('node:assert/strict');
const {readFileSync} = require('node:fs');
const {runInNewContext} = require('node:vm');
const source = readFileSync(require('node:path').join(__dirname, '../../crates/pqc_scan_report/src/compact.js'), 'utf8');

function report(spec, hash = '', modal = false) {
  const node = (dataset = {}) => ({dataset, hidden: false, open: false, value: '', textContent: '', listeners: {},
    addEventListener(type, fn) { this.listeners[type] = fn; },
    setAttribute(name,value) { this[name]=value; },
    focus() { this.focused = true; }, scrollIntoView() { this.scrolled = true; }});
  const selectors = ['data-mode','data-search','data-severity','data-risk','data-rule-filter','data-file-filter','data-show-tests','data-reset','data-report-status','data-empty','data-group-pager','data-prev','data-next','data-page','data-file-tree','data-panel-toggles','data-toggle-filters','data-toggle-tree','data-report-sidebar'];
  const ui = Object.fromEntries(selectors.map(s => [s, node()]));
  ui['data-mode'].value = 'candidate';
  ui['data-show-tests'].checked = true;
  const root = node(), controls = node();
  const dialog = modal ? node() : null, body = node(), close = node();
  body.replaceChildren = (...children) => { body.children = children; };
  body.append = child => body.children.push(child);
  if (dialog) {
    dialog.querySelector = selector => selector === '[data-close-rule-dialog]' ? close : body;
    dialog.showModal = () => { dialog.open = true; };
    dialog.close = () => { dialog.open = false; dialog.listeners.close?.(); };
    dialog.contains = link => link.inDialog === true;
  }
  const copy = {removeAttribute(){},querySelectorAll:()=>[]};
  const explanations = modal ? {content:{querySelectorAll:()=>[{cloneNode:()=>copy}]}} : null;
  root.hasAttribute = name => name === 'data-compact-report';
  root.lang = 'en';
  controls.hidden = true;
  root.querySelector = s => s === '[data-rule-dialog]' ? dialog : s === '[data-rule-explanations]' ? explanations : s === '[data-report-controls]' ? controls : ui[s.slice(1,-1)];
  controls.querySelector = root.querySelector;
  const rules = spec.map((rows, g) => {
    const rule = node({ruleId: `RULE_${g}`});
    rule.id = `rule-${g}`; rule.tagName = 'DETAILS'; rule.parentElement = root;
    rule.closest = () => rule;
    const parts = Object.fromEntries(['data-match-count','data-occ-pager','data-occ-prev','data-occ-next','data-occ-status'].map(s => [s,node()]));
    rule.parts = parts;
    rule.rows = rows.map((data,i) => {
      const row = node({rules:JSON.stringify([`RULE_${g}`]),inventory:'false',severity:'high',risk:'quantum-vulnerable',file:'a.py',search:`RULE_${g} a.py`,...data});
      row.id = `finding-${g}-${i}`; row.tagName = 'DETAILS'; row.parentElement = rule;
      row.closest = s => s === '[data-rule]' ? rule : row;
      row.querySelector = () => row;
      if (data.matchedRuleId) {
        row.matchedDetails = {...node(), tagName:'DETAILS', parentElement:row};
        row.matchedRule = {...node(), id:data.matchedRuleId, parentElement:row.matchedDetails,
          closest:s=>s==='[data-rule]' ? rule : row, querySelector:()=>null};
      }
      return row;
    });
    rule.querySelector = s => parts[s.slice(1,-1)] || rule;
    rule.querySelectorAll = () => rule.rows;
    return rule;
  });
  const treeFiles = [...new Set(rules.flatMap(r=>r.rows.map(x=>x.dataset.file)))].map(file=>node({treeFile:file}));
  const details = rules.flatMap(r => [r,...r.rows.flatMap(row =>
    row.matchedRule ? [row, row.matchedDetails, row.matchedRule] : [row])]);
  root.querySelectorAll = s => s === '[data-rule]' ? rules : s === '[data-tree-file]' ? treeFiles : details;
  root.contains = n => n === root || details.includes(n);
  const window = node();
  const document = {addEventListener(){},documentElement:{lang:'en'},querySelector:()=>root,getElementById:id=>details.find(n=>n.id===id)};
  runInNewContext(source,{document,window,location:{hash}});
  return {ui,rules,controls,window,root,treeFiles,dialog,close,
    link(help, inDialog=false) {
      const anchor={dataset:{ruleHelp:help},inDialog,focus(){this.focused=true;},getAttribute:()=>help, matches:s=>s==='a'||s==='[data-rule-help]'&&!inDialog};
      root.contains=n=>n===anchor||details.includes(n);
      root.listeners.click({target:{closest:s=>s==='a'?anchor:null},preventDefault(){}});return anchor;
    },
    treeClick(file) {
      const button={dataset:{treeFile:file},closest:()=>button,matches:s=>s.includes('[data-tree-file]')};
      root.contains=n=>n===button||details.includes(n);root.listeners.click({target:{closest:()=>button}});
    },
    input(key,value) { ui[key].value=value; controls.listeners.input(); },
    click(key,rule) {
      const button = {closest:()=>rule,matches:s=>s.split(', ').includes(`[${key}]`)};
      root.contains = n => n === button || details.includes(n);
      root.listeners.click({target:{closest:()=>button}});
    }};
}
const rows = n => Array.from({length:n},()=>({}));
const visible = r => r.rows.filter(x=>!x.hidden);

test('combined exact rule/file, case-insensitive search and reset retain all data', () => {
  const r = report([rows(25),[{file:'b.py',search:'RULE_1 B.PY'}]]);
  assert.equal(r.controls.hidden,false);
  assert.equal(visible(r.rules[0]).length,20);
  r.input('data-rule-filter','RULE_1');
  assert.equal(r.rules[0].hidden,true);
  r.input('data-file-filter','a.py');
  assert.equal(r.ui['data-empty'].hidden,false);
  r.click('data-reset');
  assert.equal(r.ui['data-rule-filter'].value,'');
  assert.equal(r.ui['data-file-filter'].value,'');
  assert.equal(r.ui['data-mode'].value,'all');
  r.input('data-search',' b.py ');
  assert.equal(visible(r.rules[1]).length,1);
  assert.equal(r.rules[0].hidden,true);
});

test('inventory, severity and risk compose without changing global records', () => {
  const r=report([[{}, {inventory:'true',severity:'info',risk:'quantum-safe'}]]);
  assert.equal(visible(r.rules[0]).length,1);
  r.input('data-mode','inventory');
  assert.equal(visible(r.rules[0])[0].dataset.severity,'info');
  r.input('data-severity','info');
  assert.equal(r.ui['data-mode'].value,'all');
  r.input('data-risk','non-quantum-risk');
  assert.equal(r.ui['data-empty'].hidden,false);
  assert.equal(r.rules[0].rows.length,2);
});

test('occurrence and group pagination clamp and restart after filter changes', () => {
  const r=report([rows(41),...Array.from({length:21},()=>rows(1))]);
  assert.equal(r.rules[20].hidden,true);
  r.click('data-occ-next',r.rules[0]);
  assert.equal(visible(r.rules[0]).length,20);
  assert.equal(r.rules[0].parts['data-occ-status'].textContent,'21-40 / 41');
  r.click('data-occ-next',r.rules[0]);
  assert.equal(visible(r.rules[0]).length,1);
  r.click('data-next');
  assert.equal(r.rules[20].hidden,false);
  r.input('data-rule-filter','RULE_0');
  assert.equal(r.rules[0].hidden,false);
  assert.equal(r.rules[0].parts['data-occ-status'].textContent,'1-20 / 41');
});

test('deep links clear every filter and reveal the correct occurrence page', () => {
  const r=report([rows(41)],'#finding-0-40');
  assert.equal(r.rules[0].open,true);
  assert.equal(r.rules[0].rows[40].open,true);
  assert.equal(r.rules[0].rows[40].hidden,false);
  assert.equal(r.rules[0].rows[40].focused,true);
  assert.equal(r.ui['data-mode'].value,'all');
});

test('print expands all details once and restores original state', () => {
  const r=report([rows(25)]);
  r.rules[0].open=true;
  r.window.listeners.beforeprint(); r.window.listeners.beforeprint();
  assert.equal(r.rules[0].rows.every(x=>x.open),true);
  r.window.listeners.afterprint();
  assert.equal(r.rules[0].open,true);
  assert.equal(r.rules[0].rows.every(x=>!x.open),true);
  // Printing must not mutate the screen filter/page state.
  assert.equal(visible(r.rules[0]).length,20);
});

test('empty report hides the filtered-no-matches message', () => {
  const r=report([]);
  assert.equal(r.ui['data-empty'].hidden,true);
  assert.match(r.ui['data-report-status'].textContent,/0 \/ 0/);
});

// Translation swaps retain the same filters and pagination rather than silently
// dropping the user's investigation context.
test('capture and restore keep exact filter and page state across language views', () => {
  const a=report([rows(45)]);
  a.input('data-file-filter','a.py');
  a.click('data-occ-next',a.rules[0]);
  const state={}; a.root.listeners['pqc-report-capture']({detail:state});
  const b=report([rows(45)]); b.root.listeners['pqc-report-restore']({detail:state});
  assert.equal(b.ui['data-file-filter'].value,'a.py');
  assert.equal(b.rules[0].parts['data-occ-status'].textContent,'21-40 / 45');
});

test('file tree composes with filters, selected count stays global and panels toggle independently',()=>{
 const r=report([[{file:'src/a.py'},{file:'src/a.py'},{file:'src/b.py',severity:'medium'}]]);
 r.treeClick('src/a.py');assert.equal(r.ui['data-file-filter'].value,'src/a.py');assert.equal(visible(r.rules[0]).length,2);
 assert.equal(r.treeFiles.find(x=>x.dataset.treeFile==='src/a.py')['aria-pressed'],'true');
 r.input('data-severity','medium');assert.equal(visible(r.rules[0]).length,0);assert.equal(r.ui['data-empty'].hidden,false);
 r.controls.open=false; r.ui['data-file-tree'].open=false;
 assert.equal(r.controls.hidden,false);assert.equal(r.ui['data-file-tree'].hidden,false);
 assert.equal(r.ui['data-report-sidebar'].hidden,false);
 r.treeClick('');assert.equal(r.ui['data-file-filter'].value,'');assert.equal(visible(r.rules[0]).length,1);
 const state={};r.root.listeners['pqc-report-capture']({detail:state});
 r.controls.open=true;r.ui['data-file-tree'].open=true;r.root.listeners['pqc-report-restore']({detail:state});
 assert.equal(r.controls.open,false);assert.equal(r.ui['data-file-tree'].open,false);
});

 test('merged supporting rules remain selectable without duplicating the location', () => {
  const r=report([[{rules:JSON.stringify(['RULE_0','API_JOSE_RSA_ALG']),search:'RULE_0 API_JOSE_RSA_ALG tokens.py',file:'tokens.py'}],[{}]]);
  r.input('data-rule-filter','API_JOSE_RSA_ALG');
  assert.equal(visible(r.rules[0]).length,1);
  assert.equal(r.rules[1].hidden,true);
  r.input('data-search','api_jose_rsa_alg');
  assert.equal(visible(r.rules[0]).length,1);
  r.input('data-file-filter','other.py');
  assert.equal(r.ui['data-empty'].hidden,false);
});

test('test results are included by default, compose with filters, restore across languages and reset',()=>{
 const r=report([[{file:'tests/a.py',test:'true',search:'key tests/a.py'},{file:'src/a.py',test:'false',search:'key src/a.py'}]]);
 assert.equal(visible(r.rules[0]).length,2);
 r.ui['data-show-tests'].checked=false;r.controls.listeners.input();
 assert.equal(visible(r.rules[0]).length,1);assert.equal(visible(r.rules[0])[0].dataset.file,'src/a.py');
 r.input('data-search','tests/');assert.equal(r.ui['data-empty'].hidden,false);
 const state={};r.root.listeners['pqc-report-capture']({detail:state});
 const other=report([[{test:'true',search:'tests/a.py'},{}]]);other.root.listeners['pqc-report-restore']({detail:state});
 assert.equal(other.ui['data-show-tests'].checked,false);
 r.click('data-reset');assert.equal(r.ui['data-show-tests'].checked,true);assert.equal(visible(r.rules[0]).length,2);
});

test('offline rule help preserves filters on close and reveals targets even for the same hash',()=>{
 const r=report([[{test:'true'}],[{}]],'#rule-0',true);
 r.ui['data-show-tests'].checked=false;r.controls.listeners.input();
 r.input('data-rule-filter','RULE_1');assert.equal(r.rules[0].hidden,true);
 const opener=r.link('all');assert.equal(r.dialog.open,true);assert.equal(r.close.focused,true);
 r.close.listeners.click();assert.equal(r.dialog.open,false);assert.equal(opener.focused,true);
 assert.equal(r.ui['data-rule-filter'].value,'RULE_1');assert.equal(r.ui['data-show-tests'].checked,false);
 r.link('all');r.link('#rule-0',true);
 assert.equal(r.dialog.open,false);assert.equal(r.rules[0].hidden,false);assert.equal(r.rules[0].open,true);
 assert.equal(r.ui['data-rule-filter'].value,'');assert.equal(r.ui['data-show-tests'].checked,true);assert.equal(r.rules[0].focused,true);
});

test('a matched-rule hash opens and focuses that individual finding inside a merged occurrence', () => {
  const id = 'scan-match-finding-v1-' + 'b'.repeat(64);
  const r = report([[{matchedRuleId:id}]], '#' + id);
  const occurrence = r.rules[0].rows[0];
  assert.equal(occurrence.open,true);
  assert.equal(occurrence.matchedDetails.open,true);
  assert.equal(occurrence.matchedRule.focused,true);
});
