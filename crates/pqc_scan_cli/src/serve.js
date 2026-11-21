/* Loopback session shell. All scans and decisions use the Rust services. */
(() => {
  const $ = (id) => document.getElementById(id);
  let language = "ja",
    folder = "",
    listing = null,
    chosenFolder = null,
    folderRequest = 0,
    folderLoading = false,
    busy = false,
    starting = false,
    connectionFailed = false;
  const selectedFolders = new Set();
  let batch = null, selectionSignature = "", scanEpoch = 0, folderControlsLocked = false;
  const locked = () => Boolean(batch || busy || starting);
  let folderHistory = [], folderHistoryIndex = -1;
  let selectedJob = null,
    currentJobs = [],
    activeJob = null,
    migrationEnabled = false,
    migrationReady = false,
    view = "input",
    historySignature = "";
  const preferences = new URLSearchParams(location.search);
  let restoreJob = /^\d+$/.test(preferences.get("result") || "")
    ? Number(preferences.get("result")) : null;
  if (["ja", "en"].includes(preferences.get("lang")))
    $("language").value = preferences.get("lang");
  const t = (ja, en) => (language === "ja" ? ja : en);
  const text = (tag, value, className = "") => {
    const n = document.createElement(tag);
    n.textContent = value;
    n.className = className;
    return n;
  };
  async function api(route, options) {
    const r = await fetch(route, options);
    const data = await r.json();
    if (!r.ok) throw new Error(data.error || "invalid_request");
    return data;
  }
  function feedback(message, error = false) {
    $("feedback").textContent = message;
    $("feedback").classList.toggle("error", error);
    $("feedback").setAttribute("role", error ? "alert" : "status");
  }
  function connectionError() {
    feedback(t(
      "サーバーに接続できません。ターミナルを確認してください。",
      "Cannot reach the server. Check the terminal.",
    ), true);
  }
  function stage(next, focus = true) {
    if (locked()) return;
    view = next;
    $("steps").hidden = next === "migration";
    for (const name of ["input", "result", "migration"])
      $(name + "-screen").hidden = name !== next;
    const index = ["input", "result", "migration"].indexOf(next);
    [...$("steps").children].forEach((n, i) => {
      n.classList.toggle("done", i < index);
      if (i === index) n.setAttribute("aria-current", "step");
      else n.removeAttribute("aria-current");
    });
    $("history").hidden = !currentJobs.length || next === "migration";
    if (focus)
      $(
        next === "input"
          ? "input-title"
          : next === "result"
            ? "result-title"
            : "migration-screen",
      ).focus();
  }
  function selectFolder(path, selected = !selectedFolders.has(path)) {
    if (folderLoading || locked()) return;
    if (selected && !selectedFolders.has(path) && selectedFolders.size >= 20) {
      feedback(t("一度に選べるフォルダは20件までです。", "Select at most 20 folders at a time."), true);
      updateFolderSelection(); return;
    }
    if (selected) selectedFolders.add(path);
    else selectedFolders.delete(path);
    chosenFolder = path;
    if (folderHistoryIndex >= 0) folderHistory[folderHistoryIndex].selected = path;
    updateFolderSelection();
  }
  function validFileSize() {
    const field = $("max-file-size-mib");
    const value = Number(field.value);
    const valid = /^[0-9]+$/.test(field.value) && Number.isInteger(value) && value >= 1 && value <= 64;
    field.setAttribute("aria-invalid", String(!valid));
    const error = $("file-size-error");
    const message = valid ? "" : t("1〜64の整数で入力してください。", "Enter a whole number from 1 to 64.");
    if (error.textContent !== message) error.textContent = message;
    error.hidden = valid;
    return valid;
  }
  function updateFolderSelection() {
    if (listing && folderControlsLocked !== locked()) { renderFolders(); return; }
    const count = selectedFolders.size;
    $("selected").textContent = count === 0
      ? ""
      : count === 1 ? t("選択中：", "Selected: ") + ([...selectedFolders][0] || listing?.root || "/")
      : t(`選択中：${count}フォルダ`, `Selected: ${count} folders`);
    const rows = [...$("folders").children].map(li => li.children[0]);
    const selectedIndex = rows.findIndex(row => row.dataset.path === chosenFolder);
    rows.forEach((row, i) => {
      const selected = selectedFolders.has(row.dataset.path);
      row.classList.toggle("selected", selected);
      row.setAttribute("aria-pressed", String(selected));
      row.tabIndex = i === (selectedIndex < 0 ? 0 : selectedIndex) ? 0 : -1;
      $("folders").children[i].children[2].children[0].checked = selected;
    });
    const signature = JSON.stringify([language, listing?.root, [...selectedFolders]]);
    if (signature !== selectionSignature) {
      selectionSignature = signature;
      $("selected-folders").replaceChildren();
      for (const path of selectedFolders) {
        const item = document.createElement("li"), label = text("span", path || listing?.root || "/", "mono"),
          remove = text("button", t("解除", "Remove"), "btn small-btn");
        remove.type = "button";
        remove.setAttribute("aria-label", t((path || listing?.root || "/") + "の選択を解除", "Deselect " + (path || listing?.root || "/")));
        remove.addEventListener("click", () => {
          selectFolder(path, false);
          $("clear-folders").focus();
        });
        item.append(label); item.append(remove); $("selected-folders").append(item);
      }
    }
    $("clear-folders").disabled = !count || locked();
    $("exclude-tests").disabled = locked();
    $("max-file-size-mib").disabled = locked();
    const sizeValid = validFileSize();
    $("scan").disabled = folderLoading || locked() || !count || !listing || connectionFailed || !sizeValid;

  }
  function renderFolders() {
    if (!listing) return;
    folderControlsLocked = locked();
    $("root").textContent = listing.root;
    $("folder-back").disabled = folderLoading || locked() || folderHistoryIndex <= 0;
    $("folder-forward").disabled = folderLoading || locked() || folderHistoryIndex >= folderHistory.length - 1;
    $("folder-path").replaceChildren();
    const segments = [listing.root.split("/").filter(Boolean).pop() || "/", ...folder.split("/").filter(Boolean)];
    segments.forEach((name, i) => {
      const path = folder.split("/").filter(Boolean).slice(0, i).join("/");
      const b = text("button", name, "folder-crumb");
      b.type = "button";
      b.disabled = folderLoading || locked();
      if (i === segments.length - 1) b.setAttribute("aria-current", "location");
      b.addEventListener("click", () => path === folder ? selectFolder(folder) : loadFolder(path));
      $("folder-path").append(b);
    });
    $("folders").setAttribute("aria-busy", String(folderLoading));
    $("folders").replaceChildren();
    for (const child of listing.directories) {
      const li = document.createElement("li"),
        b = text("button", "", "folder"),
        icon = text("span", "📁", "folder-icon"),
        label = text("span", child.name),
        open = text("button", "›", "folder-open"),
        check = document.createElement("input"),
        checkLabel = document.createElement("label");
      checkLabel.className = "folder-select-label";
      checkLabel.append(check);
      check.type = "checkbox";
      check.className = "folder-select";
      check.setAttribute("aria-label", t(child.path + "を選択", "Select " + child.path));
      check.disabled = folderLoading || locked();
      check.addEventListener("change", () => selectFolder(child.path, check.checked));
      icon.setAttribute("aria-hidden", "true");
      b.append(icon); b.append(label);
      b.type = open.type = "button";
      b.dataset.path = child.path;
      b.disabled = open.disabled = folderLoading || locked();
      b.addEventListener("click", event => { if (event.detail !== 2) selectFolder(child.path); });
      b.addEventListener("dblclick", () => { selectFolder(child.path, true); loadFolder(child.path); });
      b.addEventListener("keydown", event => {
        const rows = [...$("folders").children].map(row => row.children[0]);
        const index = rows.indexOf(b);
        let next;
        if (event.key === "ArrowDown") next = Math.min(index + 1, rows.length - 1);
        else if (event.key === "ArrowUp" && !event.altKey) next = Math.max(index - 1, 0);
        else if (event.key === "Home") next = 0;
        else if (event.key === "End") next = rows.length - 1;
        else if (!event.altKey && (event.key === "Enter" || event.key === "ArrowRight")) {
          event.preventDefault(); loadFolder(child.path); return;
        }
        if (next !== undefined) {
          event.preventDefault();
          if (locked()) return;
          chosenFolder = rows[next].dataset.path; updateFolderSelection(); rows[next].focus();
        }
      });
      open.setAttribute("aria-label", t(child.name + "を開く", "Open " + child.name));
      open.title = t("フォルダを開く", "Open folder");
      open.addEventListener("click", () => loadFolder(child.path));
      li.append(b); li.append(open); li.append(checkLabel);
      $("folders").append(li);
    }
    $("folder-note").textContent = listing.omitted
      ? t(`${listing.omitted}項目を表示できませんでした。`, `Could not list ${listing.omitted} entries.`)
      : listing.directories.length ? "" : t("サブフォルダはありません。上のパスから現在のフォルダを選べます。", "No subfolders. Select the current folder using the path above.");
    updateFolderSelection();
  }
  async function loadFolder(path, historyIndex = null) {
    if (locked()) return;
    const request = ++folderRequest;
    const firstLoad = !listing;
    folderLoading = true;
    if (listing) renderFolders();
    $("scan").disabled = true;
    try {
      const next = await api("api/directories?path=" + encodeURIComponent(path));
      if (request !== folderRequest) return;
      listing = next;
      folder = path;
      if (historyIndex !== null) {
        folderHistoryIndex = historyIndex;
        chosenFolder = folderHistory[historyIndex].selected;
      } else {
        chosenFolder = firstLoad ? null : path;
        folderHistory.splice(folderHistoryIndex + 1);
        folderHistory.push({path, selected: chosenFolder});
        folderHistoryIndex = folderHistory.length - 1;
      }
      folderLoading = false;
      renderFolders();
      if (!firstLoad) $("folder-path").children[$("folder-path").children.length - 1].focus();
    } catch {
      if (request !== folderRequest) return;
      folderLoading = false;
      if (listing) renderFolders();
      feedback(t("フォルダを開けません。アクセス権を確認してください。", "Cannot open this folder. Check access permissions."), true);
    }
  }
  function renderResult() {
    if (!activeJob) return;
    const j = activeJob;
    $("result-title").textContent =
      j.kind === "saved"
        ? t("保存した結果を読み込みました", "Saved results loaded")
        : t("スキャン完了", "Scan complete");
    $("result-path").textContent =
      j.display_name || (j.kind === "saved"
        ? t("保存したスキャン結果", "Saved scan results")
        : j.path || listing?.root || "/");
    $("result-status").textContent =
      j.kind === "saved"
        ? ""
        : j.coverage === "complete_within_scope"
          ? t("対象範囲の解析が完了", "Inspection completed within scope")
          : t(
              "一部の解析が未完了です。理由はスキャンレポートで確認してください。",
              "Some inspection is unfinished. See the scan report for reasons.",
            );
    $("scan-report-card").hidden = !j.report;
    if (j.report) $("result-report").href = reportURL(j.report);
    $("migration-card").hidden = !j.migration_input;
    $("result-migration").disabled = !migrationReady;
  }
  function showResult(job, focus = true) {
    if (locked()) return;
    activeJob = job;
    const url = new URL(location.href);
    url.searchParams.set("result", job.id);
    history.replaceState(null, "", url);
    renderResult();
    stage("result", focus);
    feedback("");
  }
  function renderJobs() {
    const signature = JSON.stringify([currentJobs, language]);
    if (historySignature === signature) return;
    historySignature = signature;
    $("jobs").replaceChildren();
    for (const [index, job] of currentJobs.slice().sort((a, b) => b.id - a.id).entries()) {
      const row = text("div", "", "job");
      row.append(
        text(
          "strong",
          job.display_name || (job.kind === "saved"
            ? t("保存したスキャン結果", "Saved scan results")
            : job.path || listing?.root?.split("/").filter(Boolean).pop() || t("名前未記録", "Name not recorded")),
          "path",
        ),
      );
      if (job.kind !== "saved" && job.path && job.path !== job.display_name) row.append(text("p", job.path, "mono small"));
      row.append(
        text(
          "p",
          job.status === "running"
            ? t("スキャン中…", "Scanning…")
            : job.status === "failed"
              ? t(
                  "スキャンに失敗しました。フォルダとルールを確認し、再実行してください。",
                  "Scan failed. Check folder and rules, then retry.",
                )
              : t("完了", "Ready"),
        ),
      );
      if (index === 0) row.append(text("span", t("最新", "Latest"), "latest-tag"));
      const time = text("time", Number.isSafeInteger(job.created_at_ms)
        ? new Date(job.created_at_ms).toLocaleString(language === "ja" ? "ja-JP" : "en-US", { timeZone: "Asia/Tokyo", hour12: false }) + t("（日本時間）", " (Japan time)")
        : t("日時未記録", "Time not recorded"), "job-time");
      if (Number.isSafeInteger(job.created_at_ms)) time.dateTime = new Date(job.created_at_ms).toISOString();
      row.append(time);
      if (job.status === "ready") {
        const b = text(
          "button",
          t("結果を確認", "Review results"),
          "btn small-btn",
        );
        b.type = "button";
        b.addEventListener("click", () => showResult(job));
        row.append(b);
      }
      $("jobs").append(row);
    }
    $("history").hidden = !currentJobs.length || view === "migration";
  }
  function reportURL(path) {
    const url = new URL(path, location.href);
    url.searchParams.set("theme", document.documentElement.dataset.theme);
    url.searchParams.set("lang", language);
    return url.href;
  }
  function renderProgress(progress, running) {
    running = Boolean(running || batch);
    const dialog = $("scan-progress");
    $("scan").setAttribute("aria-busy", String(running));
    if (running && !dialog.open) dialog.showModal();
    else if (!running && dialog.open) dialog.close();
    if (!running) return;
    const job = currentJobs.find(j => j.status === "running");
    $("progress-folder").textContent = batch
      ? (batch.paths[batch.index] || listing?.root || "/")
      : job?.path || job?.display_name || listing?.root || "/";
    $("progress-position").textContent = batch
      ? t(`${batch.index + 1} / ${batch.paths.length}フォルダ`, `Folder ${batch.index + 1} of ${batch.paths.length}`)
      : t("スキャン中", "Scanning");
    $("progress-error").textContent = connectionFailed
      ? t("接続を確認しています。復旧するまで、このページを開いたままお待ちください。", "Checking the connection. Keep this page open while it reconnects.")
      : batch?.startError !== undefined ? t("開始状況を確認しています。", "Checking whether the scan started.") : "";
    if (connectionFailed) {
      $("progress-label").textContent = t("進捗を取得できません。", "Progress is unavailable.");
      $("progress-bar").hidden = true;
      return;
    }
    $("cancel-scans").disabled = !batch || batch.cancelled;
    if (batch?.cancelled) $("progress-error").textContent = t("残りの実行を中止しました。実行中のフォルダが終了するまでお待ちください。", "Remaining scans stopped. Waiting for the active folder to finish.");
    const phase = progress?.phase;
    const count = phase === "inspecting" && progress.total > 0;
    const label = count
      ? t(`ファイルを解析中 · ${progress.processed.toLocaleString("ja-JP")} / ${progress.total.toLocaleString("ja-JP")}件`,
          `Inspecting files · ${progress.processed.toLocaleString("en-US")} / ${progress.total.toLocaleString("en-US")}`)
      : phase === "aggregating" ? t("結果を集計中…", "Aggregating results…")
      : phase === "writing" ? t("レポートを作成中…", "Writing reports…")
      : phase === "inspecting" ? t("ファイルの解析が終了しました…", "File inspection finished…")
      : t("対象ファイルを探しています…", "Finding files…");
    if ($("progress-label").textContent !== label) $("progress-label").textContent = label;
    const bar = $("progress-bar");
    bar.hidden = !count;
    if (count) { bar.max = progress.total; bar.value = progress.processed; }
  }
  function finishBatch(startError = null) {
    const finished = batch;
    batch = null;
    renderProgress(null, busy || starting);
    if (!finished) return;
    const ready = finished.results.filter(j => j.status === "ready");
    const failed = finished.results.length - ready.length;
    const remaining = finished.paths.length - finished.index;
    const cancelled = finished.cancelled;
    if (!cancelled && startError === null && finished.paths.length === 1 && ready.length && !migrationEnabled) {
      location.href = reportURL(ready[0].report);
      return;
    }
    if (ready.length) showResult(ready[ready.length - 1]);
    else stage("input");
    $("history").open = true;
    feedback(cancelled ? t(`残りのスキャンをキャンセルしました。完了${ready.length}件、失敗${failed}件。実行済みの結果は履歴に残ります。`, `Remaining scans cancelled. ${ready.length} ready, ${failed} failed. Existing results remain in history.`) : startError !== null
      ? t(`スキャンの開始を確認できませんでした：${startError || listing?.root || "/"}。残り${remaining}フォルダの追加実行を停止しました。開始済みの処理と完了済みの結果は、下の一覧で確認できます。`,
          `Could not confirm scan start for ${startError || listing?.root || "/"}. Stopped further requests for ${remaining} remaining folders. Accepted scans and completed results remain listed below.`)
      : t(`スキャン終了：${ready.length}フォルダ完了、${failed}フォルダ失敗。フォルダごとの結果を下の一覧で確認できます。`,
          `Finished: ${ready.length} folders ready, ${failed} failed. Review each folder in the results below.`),
      Boolean(startError !== null || failed));
    updateFolderSelection();
  }
  async function startNextFolder() {
    if (!batch || batch.cancelled || starting || batch.jobId !== null) return;
    scanEpoch++;
    starting = true;
    busy = true;
    renderProgress(null, true);
    try {
      const job = await api("api/scan?path=" + encodeURIComponent(batch.paths[batch.index]) + (batch.excludeTests ? "&exclude_tests=true" : "") + "&max_file_size_mib=" + batch.maxFileSizeMiB, {method: "POST"});
      batch.jobId = job.id;
    } catch {
      // A lost response may follow an accepted request. Never retry a POST automatically.
      scanEpoch++;
      const path = batch.paths[batch.index];
      batch.startError = path;
      renderProgress(null, true);
    } finally {
      starting = false;
      renderFolders();
    }
  }
  async function poll() {
    const epoch = scanEpoch;
    try {
      const jobs = (await api("api/jobs")).jobs;
      // A response sampled before a submission/failure cannot establish its outcome.
      if (epoch !== scanEpoch) { setTimeout(poll, 1000); return; }
      currentJobs = jobs;
      if (connectionFailed) feedback("");
      connectionFailed = false;
      busy = currentJobs.some(j => j.status === "running");
      if (restoreJob !== null) {
        const restored = currentJobs.find(j => j.id === restoreJob);
        if (restored?.status === "running") { selectedJob = restored.id; restoreJob = null; }
        else if (!busy) {
          restoreJob = null;
          if (restored?.status === "ready") { showResult(restored, false); if (restored.kind !== "saved") loadFolder(restored.path || ""); }
          else feedback(t("このセッションでは結果を開けません。", "These results are unavailable in this session."), true);
        }
      }
      renderJobs();
      if (batch && batch.startError !== undefined) finishBatch(batch.startError);
      if (batch && batch.jobId !== null) {
        const finished = currentJobs.find(j => j.id === batch.jobId && j.status !== "running");
        if (finished) {
          batch.results.push(finished);
          batch.index++;
          batch.jobId = null;
          if (batch.cancelled || batch.index === batch.paths.length) finishBatch();
        }
      }
      if (batch && batch.jobId === null && !busy) await startNextFolder();
      const finished = currentJobs.find(j => j.id === selectedJob && j.status !== "running");
      if (finished) {
        selectedJob = null;
        if (finished.status === "ready") {
          if (migrationEnabled) showResult(finished);
          else location.href = reportURL(finished.report);
        } else feedback(t("スキャンに失敗しました。対象フォルダを確認して再実行してください。", "Scan failed. Check the selected folder and retry."), true);
      }
      renderProgress(currentJobs.find(j => j.status === "running")?.progress, busy || starting);
      updateFolderSelection();
    } catch {
      if (epoch !== scanEpoch) { setTimeout(poll, 1000); return; }
      connectionFailed = true;
      renderProgress(null, busy || starting);
      connectionError();
    }
    setTimeout(poll, 1000);
  }
  $("scan-progress").addEventListener("cancel", event => event.preventDefault());
  $("scan-progress").addEventListener("keydown", event => {
    if (event.key === "Tab") {
      event.preventDefault();
      ($("cancel-scans").disabled ? $("scan-progress") : $("cancel-scans")).focus();
    }
  });
  $("cancel-scans").addEventListener("click", () => {
    if (!batch) return;
    batch.cancelled = true;
    $("cancel-scans").disabled = true;
    renderProgress(currentJobs.find(j => j.status === "running")?.progress, true);
  });
  window.addEventListener("beforeunload", event => {
    if (locked()) { event.preventDefault(); event.returnValue = ""; }
  });
  $("clear-folders").addEventListener("click", () => {
    if (locked()) return;
    selectedFolders.clear(); chosenFolder = null; updateFolderSelection();
  });
  const fileSizeField = $("max-file-size-mib");
  fileSizeField.addEventListener("input", updateFolderSelection);
  fileSizeField.addEventListener("keydown", event => {
    if (!event.ctrlKey && !event.metaKey && !event.altKey && event.key.length === 1 && !/^[0-9]$/.test(event.key)) event.preventDefault();
  });
  fileSizeField.addEventListener("beforeinput", event => {
    if (event.data && !/^[0-9]+$/.test(event.data)) event.preventDefault();
  });
  fileSizeField.addEventListener("paste", event => {
    const value = event.clipboardData?.getData("text");
    if (value && !/^[0-9]+$/.test(value)) event.preventDefault();
  });
  $("scan").addEventListener("click", async () => {
    if (folderLoading || locked() || !listing || !selectedFolders.size || connectionFailed) return;
    if (selectedFolders.size > 20 - currentJobs.length) {
      feedback(t("選択したフォルダ数が、このセッションの残り件数を超えています。選択を減らすか、サーバーを起動し直してください。", "The selected folders exceed this session's remaining capacity. Select fewer folders or restart the server."), true);
      return;
    }
    const maxFileSizeMiB = Number($("max-file-size-mib").value);
    if (!validFileSize()) {
      $("max-file-size-mib").focus();
      return;
    }
    batch = {maxFileSizeMiB, paths: [...selectedFolders], excludeTests: $("exclude-tests").checked, index: 0, jobId: null, results: []};
    feedback("");
    await startNextFolder();
  });
  $("folder-back").addEventListener("click", () => {
    if (!folderLoading && folderHistoryIndex > 0) loadFolder(folderHistory[folderHistoryIndex - 1].path, folderHistoryIndex - 1);
  });
  $("folder-forward").addEventListener("click", () => {
    if (!folderLoading && folderHistoryIndex < folderHistory.length - 1) loadFolder(folderHistory[folderHistoryIndex + 1].path, folderHistoryIndex + 1);
  });
  $("folder-pane").addEventListener("keydown", event => {
    if (folderLoading || locked() || !event.altKey) return;
    if (event.key === "ArrowLeft" || event.key === "ArrowRight") {
      event.preventDefault(); $(event.key === "ArrowLeft" ? "folder-back" : "folder-forward").click();
    } else if (event.key === "ArrowUp" && folder) {
      event.preventDefault(); loadFolder(folder.split("/").slice(0, -1).join("/"));
    }
  });
  $("choose-input").addEventListener("click", () => {
    if (locked()) return;
    const url = new URL(location.href); url.searchParams.delete("result");
    history.replaceState(null, "", url);
    stage("input");
  });
  $("result-migration").addEventListener("click", () => {
    if (locked() || !migrationReady || !activeJob) return;
    stage("migration", false);
    window.dispatchEvent(
      new CustomEvent("pqc-migration-job", {
        detail: { ...activeJob, display_path: $("result-path").textContent },
      }),
    );
  });
  function inputMode(saved) {
    if (locked()) return;
    $("folder-pane").hidden = saved;
    $("saved-pane").hidden = !saved;
    for (const [id, on] of [
      ["folder-tab", !saved],
      ["saved-tab", saved],
    ]) {
      $(id).classList.toggle("active", on);
      $(id).setAttribute("aria-pressed", String(on));
    }
    $("input-title").textContent = saved
      ? t("保存した結果を読み込む", "Load saved results")
      : t("スキャンするフォルダを選ぶ", "Choose a folder to scan");
  }
  $("folder-tab").addEventListener("click", () => inputMode(false));
  $("saved-tab").addEventListener("click", () => inputMode(true));
  $("saved-file").addEventListener("change", () => {
    $("saved-next").disabled = !$("saved-file").files.length;
  });
  $("saved-next").addEventListener("click", async () => {
    const f = $("saved-file").files[0];
    if (locked() || !f || !migrationReady) return;
    if (f.size > 32 * 1024 * 1024) {
      feedback(
        t(
          "32MiB以下のスキャン結果を選んでください。",
          "Choose scan results up to32MiB.",
        ),
        true,
      );
      return;
    }
    $("saved-next").disabled = true;
    try {
      const data = await api("api/migration/import", {
        method: "POST",
        headers: { "Content-Type": "application/json", "X-Pqc-Filename": encodeURIComponent(f.name) },
        body: await f.arrayBuffer(),
      });
      showResult(data);
    } catch {
      feedback(
        t(
          "読み込めません。保存したスキャン結果（scan.inventory.v1.json）を選んでください。",
          "Cannot load this file. Choose saved scan results (scan.inventory.v1.json).",
        ),
        true,
      );
    } finally {
      $("saved-next").disabled = !$("saved-file").files.length;
    }
  });
  function theme() {
    const light = document.documentElement.dataset.theme === "light";
    $("theme").textContent = light
      ? t("ダーク表示", "Dark theme")
      : t("ライト表示", "Light theme");
  }
  $("theme").addEventListener("click", () => {
    document.documentElement.dataset.theme =
      document.documentElement.dataset.theme === "light" ? "dark" : "light";
    try { localStorage.setItem("pqc-scan-theme", document.documentElement.dataset.theme); } catch { /* Optional preference. */ }
    const url = new URL(location.href);
    url.searchParams.set("theme", document.documentElement.dataset.theme);
    history.replaceState(null, "", url);
    theme();
    renderResult();
  });
  function applyLanguage() {
    language = $("language").value === "en" ? "en" : "ja";
    document.documentElement.lang = language;
    const url = new URL(location.href);
    url.searchParams.set("lang", language);
    history.replaceState(null, "", url);
    for (const n of document.querySelectorAll("[data-en]"))
      n.textContent = n.dataset[language];
    theme();
    renderFolders();
    renderJobs();
    renderResult();
    renderProgress(currentJobs.find(j => j.status === "running")?.progress, busy || starting);
    inputMode(!$("saved-pane").hidden);
    if (connectionFailed) connectionError();
    window.dispatchEvent(new CustomEvent("pqc-language", { detail: language }));
  }
  $("language").addEventListener("change", () => {
    applyLanguage();
    if (!connectionFailed) feedback("");
  });
  window.addEventListener("pageshow", applyLanguage);
  window.addEventListener("pqc-migration-back", () => {
    if (activeJob) showResult(activeJob);
  });
  window.addEventListener("pqc-open-report", event => {
    if (locked()) return;
    window.open(reportURL(event.detail), "_blank", "noopener");
  });
  async function start() {
    try {
      const c = await api("api/config");
      migrationEnabled = c.migration;
      $("max-file-size-mib").value = String(c.max_file_size_mib ?? 2);
      $("migration-step").hidden = !migrationEnabled;
      $("saved-tab").hidden = !migrationEnabled;
      if (migrationEnabled) {
        for (const file of ["context-model.js", "report-view.js", "migration.js"])
          await new Promise((resolve, reject) => {
            const s = document.createElement("script");
            s.src = file;
            s.onload = resolve;
            s.onerror = reject;
            document.body.append(s);
          });
        migrationReady = true;
        renderResult();
      }
    } catch {
      feedback(
        t(
          "画面を読み込めません。ページを開き直してください。",
          "Cannot load the interface. Reopen the page.",
        ),
        true,
      );
    }
    loadFolder("");
    poll();
  }
  applyLanguage();
  start();
})();
