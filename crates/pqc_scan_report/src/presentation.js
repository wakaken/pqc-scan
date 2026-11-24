(() => {
  const settings = document.querySelector('[data-settings]');
  const alternate = document.querySelector('[data-report-translation]');
  let active = document.querySelector('[data-report-view]');
  if (!settings || !alternate || !active) return;
  const language = settings.querySelector('[data-language]');
  const theme = settings.querySelector('[data-theme-select]');
  function localTime() {
    const locale = active.lang === 'ja' ? 'ja-JP' : 'en-US';
    for (const time of active.querySelectorAll('[data-local-time]')) {
      try {
        const formatter = new Intl.DateTimeFormat(locale, {dateStyle: 'medium', timeStyle: 'long'});
        time.textContent = `${formatter.format(new Date(time.dateTime))} (${formatter.resolvedOptions().timeZone})`;
      } catch { /* Keep the generation host's explicit offset if Intl is unavailable. */ }
    }
  }
  function switchLanguage(value) {
    if (!['en', 'ja'].includes(value)) return;
    if (active.lang !== value && alternate.lang === value) {
      const state = {};
      active.dispatchEvent(new CustomEvent('pqc-report-capture', {detail: state}));
      const opened = new Set(Array.from(active.querySelectorAll('details[id]')).filter(d => d.open).map(d => d.id));
      const incoming = alternate.content.firstElementChild;
      const outgoing = active;
      active.replaceWith(incoming);
      alternate.content.append(outgoing);
      alternate.lang = outgoing.lang;
      active = incoming;
      document.dispatchEvent(new CustomEvent('pqc-report-view', {detail: active}));
      active.dispatchEvent(new CustomEvent('pqc-report-restore', {detail: state}));
      active.querySelectorAll('details[id]').forEach(d => { d.open = opened.has(d.id); });
    }
    language.value = active.lang;
    document.documentElement.lang = active.lang;
    const scope = active.querySelector('.report-scope')?.textContent;
    document.title = (scope ? scope + ' | ' : '') + active.querySelector('h1').textContent;
    localTime();
    rememberURL();
  }
  function rememberURL() {
    try {
      const url = new URL(location.href);
      url.searchParams.set('theme', document.documentElement.dataset.theme);
      url.searchParams.set('lang', document.documentElement.lang);
      history.replaceState(null, '', url);
    } catch { /* Standalone exports also work without history updates. */ }
  }
  function applyTheme(value) {
    if (!['light', 'dark'].includes(value)) value = 'light';
    theme.value = value;
    document.documentElement.dataset.theme = value;
  }
  language.addEventListener('change', () => switchLanguage(language.value));
  theme.addEventListener('change', () => {
    applyTheme(theme.value);
    rememberURL();
    try { localStorage.setItem('pqc-scan-theme', theme.value); } catch { /* Optional preference. */ }
  });
  applyTheme(document.documentElement.dataset.theme);
  let requested;
  try { requested = new URLSearchParams(location.search).get('lang'); } catch { /* CLI default. */ }
  switchLanguage(requested || active.lang);
  settings.hidden = false;
})();
