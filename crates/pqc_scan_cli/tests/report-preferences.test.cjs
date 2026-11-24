const { test } = require('node:test');
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const vm = require('node:vm');
const path = require('node:path');
const themeScript = readFileSync(path.join(__dirname, '../../pqc_scan_report/src/theme-init.js'), 'utf8');
function theme(search, stored, blocked = false, initial = 'light') {
  const document = { documentElement: { dataset: { theme: initial } } };
  vm.runInNewContext(themeScript, { document, URLSearchParams, location: { search }, localStorage: {
    getItem() { if (blocked) throw Error('disabled'); return stored; }
  } });
  return document.documentElement.dataset.theme;
}
test('explicit valid theme wins; invalid input/storage cannot become a theme', () => {
  assert.equal(theme('?theme=dark', 'light'), 'dark');
  assert.equal(theme('?theme=light', 'dark'), 'light');
  assert.equal(theme('?theme=invalid', 'dark'), 'dark');
  assert.equal(theme('', 'invalid'), 'light');
  assert.equal(theme('?theme=dark', null, true), 'dark');
  assert.equal(theme('', null, true), 'light');
  assert.equal(theme('', null, true, 'dark'), 'dark');
});
test('session return preserves the exact result and current preferences; external links stay unchanged', () => {
  const root = 'http://127.0.0.1:8787/session/';
  const back = { href: root + '?result=12', textContent: '' };
  const internal = { href: root + 'reports/12/report.rules.html#rule' };
  const external = { href: 'https://example.org/report.html' };
  const other = { href: root + 'reports/11/report.html' };
  const listeners = {};
  const document = {
    documentElement: { lang: 'ja', dataset: { theme: 'dark' } },
    getElementById: () => back,
    querySelectorAll: () => [back, internal, external, other],
    addEventListener: (event, fn) => { listeners[event] = fn; },
  };
  vm.runInNewContext(readFileSync(path.join(__dirname, '../src/report-session.js'), 'utf8'), {
    URL, document, queueMicrotask: fn => fn(), location: { href: root + 'reports/12/report.html', origin: 'http://127.0.0.1:8787' }
  });
  assert.equal(back.href, root + '?result=12&theme=dark&lang=ja');
  assert.equal(back.textContent, '← 結果に戻る');
  assert.equal(internal.href, root + 'reports/12/report.rules.html?theme=dark&lang=ja#rule');
  assert.equal(external.href, 'https://example.org/report.html');
  assert.equal(other.href, root + 'reports/11/report.html');
  document.documentElement.lang = 'en';
  document.documentElement.dataset.theme = 'light';
  listeners.change();
  assert.equal(back.href, root + '?result=12&theme=light&lang=en');
  assert.equal(back.textContent, '← Back to results');
});
