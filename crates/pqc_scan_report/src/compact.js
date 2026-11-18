(() => {
  const initialized = new WeakSet();
  function initialize(root) {
  if (!root || !root.hasAttribute('data-compact-report') || initialized.has(root)) return;
  initialized.add(root);
  const ja = root.lang === 'ja';
  const choose = (en, jp) => ja ? jp : en;
  const controls = root.querySelector('[data-report-controls]');
  const mode = controls.querySelector('[data-mode]');
  const search = controls.querySelector('[data-search]');
  const severity = controls.querySelector('[data-severity]');
  const risk = controls.querySelector('[data-risk]');
  const ruleFilter = controls.querySelector('[data-rule-filter]');
  const fileFilter = controls.querySelector('[data-file-filter]');
  const rules = Array.from(root.querySelectorAll('[data-rule]'));
  const rows = new Map(rules.map(rule => [rule, Array.from(rule.querySelectorAll('[data-finding-id]'))]));
  const tree = root.querySelector('[data-file-tree]');
  const toggles = root.querySelector('[data-panel-toggles]');
  const filterToggle = root.querySelector('[data-toggle-filters]');
  const treeToggle = root.querySelector('[data-toggle-tree]');
  const sidebar = root.querySelector('[data-report-sidebar]');
  function panels(filtersVisible, treeVisible) {
    controls.hidden = !filtersVisible;
    if (tree) tree.hidden = !treeVisible;
    if (sidebar) sidebar.hidden = !filtersVisible && !treeVisible;
    if (filterToggle) filterToggle.setAttribute('aria-pressed', String(filtersVisible));
    if (treeToggle) treeToggle.setAttribute('aria-pressed', String(treeVisible));
  }
  if (toggles) toggles.hidden = false;
  panels(true, true);
  const pages = new Map();
  let page = 0;
  let visibleRules = [];
  const matches = row => {
    const inventory = row.dataset.inventory === 'true';
    return (mode.value === 'all' || (mode.value === 'inventory' ? inventory : !inventory))
      && (!severity.value || row.dataset.severity === severity.value)
      && (!risk.value || row.dataset.risk === risk.value)
      && (!ruleFilter.value || row.closest('[data-rule]').dataset.ruleId === ruleFilter.value)
      && (!fileFilter.value || row.dataset.file === fileFilter.value)
      && row.dataset.search.toLocaleLowerCase().includes(search.value.trim().toLocaleLowerCase());
  };
  function refresh() {
    for (const button of root.querySelectorAll('[data-tree-file]')) {
      button.setAttribute('aria-pressed', String(button.dataset.treeFile === fileFilter.value));
    }
    let count = 0;
    visibleRules = [];
    for (const rule of rules) {
      const all = rows.get(rule);
      const matched = all.filter(matches);
      count += matched.length;
      if (matched.length) visibleRules.push(rule);
      const last = Math.max(0, Math.ceil(matched.length / 20) - 1);
      const p = Math.min(pages.get(rule) || 0, last);
      pages.set(rule, p);
      const onPage = new Set(matched.slice(p * 20, (p + 1) * 20));
      all.forEach(row => { row.hidden = !onPage.has(row); });
      rule.querySelector('[data-match-count]').textContent = `${matched.length} / ${all.length}`;
      rule.querySelector('[data-occ-pager]').hidden = matched.length <= 20;
      rule.querySelector('[data-occ-prev]').disabled = p === 0;
      rule.querySelector('[data-occ-next]').disabled = p === last;
      rule.querySelector('[data-occ-status]').textContent = `${matched.length ? p * 20 + 1 : 0}-${Math.min((p + 1) * 20, matched.length)} / ${matched.length}`;
    }
    const last = Math.max(0, Math.ceil(visibleRules.length / 20) - 1);
    page = Math.min(page, last);
    rules.forEach(rule => { const i = visibleRules.indexOf(rule); rule.hidden = i < page * 20 || i >= (page + 1) * 20; });
    root.querySelector('[data-report-status]').textContent = choose(
      `${visibleRules.length} / ${rules.length} rule groups; ${count} / ${Array.from(rows.values()).reduce((n,r) => n+r.length,0)} findings match the current view.`,
      `表示対象：${visibleRules.length} / ${rules.length}ルール、${count} / ${Array.from(rows.values()).reduce((n,r) => n+r.length,0)}箇所`);
    root.querySelector('[data-empty]').hidden = count !== 0 || rules.length === 0;
    root.querySelector('[data-group-pager]').hidden = visibleRules.length <= 20;
    root.querySelector('[data-prev]').disabled = page === 0;
    root.querySelector('[data-next]').disabled = page === last;
    root.querySelector('[data-page]').textContent = `${visibleRules.length ? page * 20 + 1 : 0}-${Math.min((page + 1) * 20, visibleRules.length)} / ${visibleRules.length}`;
  }
  function reset() { mode.value = 'all'; search.value = ''; severity.value = ''; risk.value = ''; ruleFilter.value = ''; fileFilter.value = ''; page = 0; pages.clear(); }
  controls.hidden = false;
  controls.addEventListener('input', () => {
    if (severity.value === 'info' || risk.value === 'quantum-safe') mode.value = 'all';
    page = 0; pages.clear(); refresh();
  });
  root.addEventListener('click', event => {
    const button = event.target.closest('button');
    if (!button || !root.contains(button)) return;
    if (button.matches('[data-toggle-filters]')) { panels(controls.hidden, tree && !tree.hidden); return; }
    if (button.matches('[data-toggle-tree]')) { panels(!controls.hidden, tree && tree.hidden); return; }
    if (button.matches('[data-tree-file], [data-tree-clear]')) {
      fileFilter.value = button.dataset.treeFile || ''; page = 0; pages.clear();
    } else if (button.matches('[data-reset]')) reset();
    else if (button.matches('[data-prev]')) page--;
    else if (button.matches('[data-next]')) page++;
    else if (button.matches('[data-occ-prev], [data-occ-next]')) {
      const rule = button.closest('[data-rule]');
      pages.set(rule, (pages.get(rule) || 0) + (button.matches('[data-occ-next]') ? 1 : -1));
    } else return;
    refresh();
  });
  function revealHash() {
    let id;
    try { id = decodeURIComponent(location.hash.slice(1)); } catch { return; }
    const target = document.getElementById(id);
    if (!target || !root.contains(target)) return;
    const rule = target.closest('[data-rule]');
    if (!rule) return;
    reset();
    page = Math.floor(rules.indexOf(rule) / 20);
    const occurrence = target.closest('[data-finding-id]');
    if (occurrence) pages.set(rule, Math.floor(rows.get(rule).indexOf(occurrence) / 20));
    refresh();
    for (let element = target; element && element !== root; element = element.parentElement) {
      if (element.tagName === 'DETAILS') element.open = true;
    }
    (target.querySelector('summary') || target).focus();
    target.scrollIntoView({block: 'nearest'});
  }
  // Print the full report, then restore the reader's open/closed state.
  let beforePrint = null;
  window.addEventListener('beforeprint', () => {
    if (beforePrint) return;
    beforePrint = Array.from(root.querySelectorAll('details'), detail => [detail, detail.open]);
    beforePrint.forEach(([detail]) => { detail.open = true; });
  });
  window.addEventListener('afterprint', () => {
    if (!beforePrint) return;
    beforePrint.forEach(([detail, open]) => { detail.open = open; });
    beforePrint = null;
  });
  const fields = {'data-mode': mode, 'data-search': search, 'data-severity': severity,
    'data-risk': risk, 'data-rule-filter': ruleFilter, 'data-file-filter': fileFilter};
  root.addEventListener('pqc-report-capture', event => {
    Object.assign(event.detail, {panels: [!controls.hidden, tree && !tree.hidden], page, pages: rules.map(rule => [rule.id, pages.get(rule) || 0]),
      fields: Object.fromEntries(Object.entries(fields).map(([key, input]) => [key, input.value]))});
  });
  root.addEventListener('pqc-report-restore', event => {
    const state = event.detail;
    if (!state.fields) return;
    Object.entries(fields).forEach(([key, input]) => { input.value = state.fields[key]; });
    if (state.panels) panels(...state.panels);
    page = state.page;
    pages.clear();
    for (const [id, p] of state.pages) {
      const rule = rules.find(rule => rule.id === id);
      if (rule) pages.set(rule, p);
    }
    refresh();
  });
  window.addEventListener('hashchange', revealHash);
  refresh();
  if (location.hash) revealHash();
  }
  document.addEventListener('pqc-report-view', event => initialize(event.detail));
  initialize(document.querySelector('[data-compact-report]'));
})();
