/* Apply before styles/large report content to avoid a light-theme flash. */
(() => {
  let value = new URLSearchParams(location.search).get('theme') || document.documentElement.dataset.exportTheme;
  if (!['light', 'dark'].includes(value)) {
    try { value = localStorage.getItem('pqc-scan-theme'); } catch { /* Optional preference. */ }
  }
  if (['light', 'dark'].includes(value)) document.documentElement.dataset.theme = value;
})();
