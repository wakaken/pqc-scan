const {test} = require('node:test');
const assert = require('node:assert/strict');
const {readFileSync} = require('node:fs');
const {join} = require('node:path');
const vm = require('node:vm');
const PqcContext = require('../src/serve-context.js');
const source = readFileSync(join(__dirname, '../src/serve-migration.js'), 'utf8');
function implementation(name) {
  const start = source.search(new RegExp(`    (?:async )?function ${name}\\(`));
  return start < 0 ? '' : source.slice(start, source.indexOf('\n    }', start) + 6);
}
function clock(mode = 'now') {
  const nodes = new Map(Object.entries({
    'evaluation-mode': {value: mode}, 'as-of': {value: '2026-11-24T09:00:00'},
    'evaluation-fixed': {}, 'evaluation-time-hint': {}, 'evaluation-time-warning': {}, 'assertion-time-warning': {},
    'context-mode': {value: 'generate'}, 'asserted-at': {value: ''},
  }));
  const c = {PqcContext, loading: false, input: {sha256: 'digest'}, groups: [], imported: null,
    $: id => nodes.get(id), t: ja => ja, japanNow: () => c.now,
    now: '2026-11-24T10:00:00', read: () => ({values: {'asserted-at': nodes.get('asserted-at').value}}),
    store: {effective: () => []},
  };
  vm.createContext(c);
  vm.runInContext(['evaluationTime', 'updateEvaluationTime', 'updateTimeWarning', 'requestBody'].map(implementation).join('\n'), c);
  return {c, nodes};
}
test('default evaluation uses creation time after later declarations, not form-open time', () => {
  const {c, nodes} = clock();
  nodes.get('asserted-at').value = '2026-11-24T09:59:59';
  assert.equal(c.requestBody(true).as_of, '2026-11-24T10:00:00+09:00');
  c.now = '2026-11-24T10:05:00';
  assert.equal(c.requestBody(true).as_of, '2026-11-24T10:05:00+09:00');
});
test('explicit evaluation time and imported context are preserved without rewriting claims or expiry', () => {
  const {c, nodes} = clock('fixed');
  const context = JSON.stringify({format: 'pqc-scan.migration-context', version: 1, bindings: [{claims: {}}]});
  c.imported = context; nodes.get('context-mode').value = 'import_context';
  const body = c.requestBody(false);
  assert.equal(body.as_of, '2026-11-24T09:00:00+09:00');
  assert.equal(body.context, context);
});
test('future declarations warn for fixed evaluation and clear when now is selected', () => {
  const {c, nodes} = clock('fixed');
  nodes.get('asserted-at').value = '2026-11-24T09:30:00';
  c.updateEvaluationTime();
  assert.equal(nodes.get('evaluation-fixed').hidden, false);
  assert.equal(nodes.get('evaluation-time-warning').hidden, false);
  assert.match(nodes.get('evaluation-time-warning').textContent, /申告日時.*評価日時/);
  nodes.get('evaluation-mode').value = 'now'; c.updateEvaluationTime();
  assert.equal(nodes.get('evaluation-fixed').hidden, true);
  assert.equal(nodes.get('evaluation-time-warning').hidden, true);
});
test('warnings also cover saved groups and imported declarations outside the current draft', () => {
  for (const imported of [false, true]) {
    const {c, nodes} = clock('fixed');
    if (imported) {
      nodes.get('context-mode').value = 'import_context';
      c.imported = JSON.stringify({bindings: [{claims: {use_case: {provenance: {asserted_at: '2026-11-24T01:00:00Z'}}}}]});
    } else c.groups = [{ids: ['1'], common: {values: {'asserted-at': '2026-11-24T10:00:00'}}, exceptions: new Map()}];
    c.updateTimeWarning(); assert.equal(nodes.get('evaluation-time-warning').hidden, false);
  }
});

test('import warnings exclude unused draft and groups and clear the inline draft warning', () => {
  const {c, nodes} = clock('fixed');
  nodes.get('asserted-at').value = '2026-11-24T10:00:00';
  c.groups = [{ids: ['1'], common: {values: {'asserted-at': '2026-11-24T10:00:00'}}, exceptions: new Map()}];
  c.updateTimeWarning(); assert.equal(nodes.get('evaluation-time-warning').hidden, false);
  nodes.get('context-mode').value = 'import_context';
  c.imported = JSON.stringify({bindings: [{claims: {use_case: {provenance: {asserted_at: '2026-11-23T00:00:00Z'}}}}]});
  c.updateTimeWarning();
  assert.equal(nodes.get('evaluation-time-warning').hidden, true);
  assert.equal(nodes.get('assertion-time-warning').hidden, true);
  nodes.get('context-mode').value = 'generate'; c.updateTimeWarning();
  assert.equal(nodes.get('evaluation-time-warning').hidden, false);
  assert.equal(nodes.get('assertion-time-warning').hidden, false);
});

async function exportPage(options = {}) {
  const nodes = new Map(), downloads = [], requests = [], timers = [];
  function node(tag, text = '') {
    return {tag, textContent: text, dataset: {}, children: [], handlers: {}, disabled: false,
      append(...v) {this.children.push(...v);}, replaceChildren(...v) {this.children = v;},
      get firstElementChild() {return this.children[0];},
      addEventListener(type, fn) {this.handlers[type] = fn;}, setAttribute() {}, remove() {},
      contains(control) {return this.children.some(n => n === control || n.contains(control));},
      click() {if (this.disabled) return; if (this.href?.startsWith('blob:')) downloads.push(blobs.get(this.href)); return this.handlers.click?.({preventDefault() {}});},
    };
  }
  const blobs = new Map();
  const raw = options.raw ?? '{ "format": "pqc-scan.migration-context", "version": 1, "bindings": [] }\n';
  const c = {preview: {token: 'preview'}, version: 1, navigationSequence: 1, job: {id: 7},
    controller: null, mountedReport: null, generationProgress() {}, setBusy() {}, stage() {},
    t: ja => ja, textNode: node, language: 'ja', reportPanel: node('section'),
    progressPanel: {open: false}, groups: [], switchEditor() {},
    $: id => {if (!nodes.has(id)) nodes.set(id, node('div')); return nodes.get(id);},
    document: {createElement: node, documentElement: {dataset: {theme: 'dark'}}, body: node('body')},
    location: {href: 'http://127.0.0.1:8787/session/'},
    URL: class extends URL {static createObjectURL(blob) {const id = 'blob:' + blobs.size; blobs.set(id, blob); return id;} static revokeObjectURL() {}},
    Blob, AbortController, setTimeout: (fn, ms) => {timers.push({fn, ms}); return timers.length;}, clearTimeout() {},
    DOMParser: class {parseFromString() {return {getElementById: () => ({textContent: '{}'}), querySelector: () => null};}},
    PqcMigrationReport: {mount: () => ({destroy() {}})},
    button: (text, callback) => {const n = node('button', text); n.addEventListener('click', callback); return n;},
    api: async () => ({token: 'running'}),
    fetch: async route => {
      requests.push(route);
      if (route.startsWith('api/')) return {ok: true, json: async () => ({status: 'ready', result: {context: true, report: 'reports/7/migration.assessment.html'}})};
      if (route.endsWith('.html')) return {ok: true, text: async () => '<html></html>'};
      if (options.delayed) await options.delayed;
      return {ok: !options.fail, headers: {get: () => options.mime ?? 'application/json; charset=utf-8'}, blob: async () => new Blob([raw], {type: 'application/json'})};
    },
  };
  vm.createContext(c); vm.runInContext(implementation('saveInput') + '\n' + implementation('generateReport'), c);
  await c.generateReport();
  const tools = c.reportPanel.children[0], control = tools.children.find(n => n.textContent === '入力内容を保存（JSON）');
  return {c, control, tools, downloads, requests, raw, timers};
}
test('input save is a button that downloads exact server context bytes through the working Blob path', async () => {
  const p = await exportPage();
  await p.control.click();
  assert.equal(p.control.tag, 'button'); assert.equal(p.downloads.length, 1);
  assert.equal(await p.downloads[0].text(), p.raw);
  assert.equal(p.requests.at(-1), 'reports/7/migration-context.json');
  assert.equal(p.control.disabled, false);
  assert.ok(p.tools.children.some(n => /保存を開始/.test(n.textContent)));
});
test('failed, wrong-type, malformed and oversized context exports show errors and produce no file', async () => {
  for (const options of [{fail: true}, {mime: 'text/html'}, {raw: 'not JSON'}, {raw: 'x'.repeat(1048577)}]) {
    const p = await exportPage(options); await p.control.click();
    assert.equal(p.downloads.length, 0); assert.equal(p.control.disabled, false);
    assert.ok(p.tools.children.some(n => /保存できません/.test(n.textContent)));
  }
});
test('pending export prevents duplicates and never downloads after a job or view change', async () => {
  for (const field of ['version', 'navigationSequence']) {
    let finish; const delayed = new Promise(resolve => {finish = resolve;});
    const p = await exportPage({delayed}); const pending = p.control.click();
    await p.control.click(); assert.equal(p.requests.filter(n => n.endsWith('migration-context.json')).length, 1);
    p.c[field]++; finish(); await pending; assert.equal(p.downloads.length, 0);
  }
});
