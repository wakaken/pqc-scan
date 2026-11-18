(() => {
  const $ = id => document.getElementById(id);
  let language = 'ja', folder = '', busy = false, starting = false, folderLoading = false, folderRequest = 0, selectedJob = null, currentJobs = [], listing = null;
  const t = (ja,en) => language === 'ja' ? ja : en;
  async function api(route, options) {
    const response = await fetch(route, options);
    const data = await response.json();
    if (!response.ok) throw new Error(data.error || 'Request failed');
    return data;
  }
  function feedback(text, error=false) { $('feedback').textContent=text; $('feedback').classList.toggle('error',error); }
  function renderFolders() {
    if (!listing) return;
    $('root').textContent=listing.root; $('selected').textContent=folder || '/'; $('up').disabled=!folder;
    $('folders').replaceChildren();
    for (const child of listing.directories) {
      const item=document.createElement('li'), button=document.createElement('button');
      button.className='folder';button.type='button';button.textContent=`▸ ${child.name}`;
      button.addEventListener('click',()=>loadFolder(child.path)); item.append(button);$('folders').append(item);
    }
    $('folder-note').textContent=listing.omitted ? t(`${listing.omitted}項目を表示できませんでした。`,`Could not list ${listing.omitted} entries.`) : listing.directories.length ? '' : t('サブフォルダはありません。','No subfolders.');
    $('scan').disabled=folderLoading||starting||busy;
  }
  async function loadFolder(path) {
    const request=++folderRequest; folderLoading=true; $('scan').disabled=true;
    try {const next=await api(`api/directories?path=${encodeURIComponent(path)}`);if(request!==folderRequest)return;listing=next;folder=path;folderLoading=false;renderFolders();}
    catch {if(request!==folderRequest)return;folderLoading=false;$('scan').disabled=starting||busy||!listing;feedback(t('フォルダを開けません。参照起点とアクセス権を確認してください。','Cannot open this folder. Check the root and access permissions.'),true);}
  }
  function renderJobs() {
    $('jobs').replaceChildren();
    if (!currentJobs.length) {$('jobs').textContent=t('まだスキャンしていません。','No scans yet.');return;}
    for(const job of currentJobs.slice().reverse()) {
      const row=document.createElement('div');row.className='job';
      const path=document.createElement('strong');path.className='path';path.textContent=job.path||'/';row.append(path);
      const status=document.createElement('p');
      status.textContent=job.status==='running'?t('スキャン中…','Scanning…'):job.status==='failed'?t('スキャンに失敗しました。フォルダとルールの読み取り可否を確認し、再実行してください。','Scan failed. Check folder and rule access, then retry.'):job.coverage==='complete_within_scope'?t('対象範囲の解析が完了しました。','Inspection completed within the selected scope.'):t('レポートを生成しました。一部の解析が未完了です。理由はレポートで確認してください。','Report ready. Some inspection is unfinished; review the reasons in the report.');row.append(status);
      if (job.status==='ready') {const a=document.createElement('a');a.href=job.report;a.textContent=t('レポートを開く','Open report');row.append(a);}
      $('jobs').append(row);
    }
  }
  async function poll() {
    try {
      currentJobs=(await api('api/jobs')).jobs;busy=currentJobs.some(j=>j.status==='running');$('scan').disabled=folderLoading||starting||busy||!listing;renderJobs();
      const finished=currentJobs.find(j=>j.id===selectedJob&&j.status!=='running');
      if(finished) {selectedJob=null;if(finished.status==='ready'){setTimeout(poll,1000);location.href=finished.report;return;}feedback(t('スキャンに失敗しました。対象フォルダを確認して再実行してください。','Scan failed. Check the selected folder and try again.'),true);}
    } catch {feedback(t('サーバーに接続できません。ターミナルで起動状態を確認してください。','Cannot reach the server. Check the terminal.'),true);}
    setTimeout(poll,1000);
  }
  $('scan').addEventListener('click',async()=>{
    if(folderLoading||starting||busy||!listing)return;starting=true;busy=true;$('scan').disabled=true;feedback(t('スキャンを開始しています…','Starting scan…'));
    try {const job=await api(`api/scan?path=${encodeURIComponent(folder)}`,{method:'POST'});selectedJob=job.id;starting=false;feedback(t('スキャン中です。完了するとレポートを開きます。','Scanning. The report opens when ready.'));}
    catch {starting=false;busy=false;$('scan').disabled=!listing;feedback(t('開始できませんでした。他のスキャンの完了を待つか、対象フォルダを確認してください。','Could not start. Wait for the active scan or check the folder.'),true);}
  });
  $('up').addEventListener('click',()=>loadFolder(folder.split('/').slice(0,-1).join('/')));
  function applyLanguage() {
    language=$('language').value==='en'?'en':'ja';document.documentElement.lang=language;
    for(const node of document.querySelectorAll('[data-en]'))node.textContent=node.dataset[language];
    renderFolders();renderJobs();
  }
  $('language').addEventListener('change',()=>{applyLanguage();feedback('');});
  window.addEventListener('pageshow',applyLanguage);
  loadFolder('');poll();
})();
