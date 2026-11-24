/* Served-session navigation only. Offline exports remain self-contained. */
(() => {
  const back = document.getElementById('pqc-session-back');
  if (!back) return;
  const destination = new URL(back.href);
  const directory = new URL('.', location.href).pathname;
  function update() {
    const language = document.documentElement.lang === 'en' ? 'en' : 'ja';
    const theme = document.documentElement.dataset.theme === 'dark' ? 'dark' : 'light';
    back.textContent = language === 'ja' ? '← 結果に戻る' : '← Back to results';
    destination.searchParams.set('theme', theme);
    destination.searchParams.set('lang', language);
    back.href = destination.href;
    for (const link of document.querySelectorAll('a[href]')) {
      const url = new URL(link.href);
      if (url.origin === location.origin && url.pathname.startsWith(directory) && url.pathname.endsWith('.html')) {
        url.searchParams.set('theme', theme);
        url.searchParams.set('lang', language);
        link.href = url.href;
      }
    }
  }
  document.addEventListener('change', update);
  document.addEventListener('pqc-report-view', () => queueMicrotask(update));
  update();
})();
