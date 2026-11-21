// Session-bound context authoring. Validation and evaluation remain in Rust.
(() => {
  const root = document.getElementById("migration-screen");
  root.tabIndex = -1;
  let language = document.documentElement.lang === "en" ? "en" : "ja",
    job = null,
    input = null,
    version = 0,
    loading = false,
    requestSequence = 0,
    total = 0,
    allTotal = 0,
    controller = null,
    preview = null,
    downloadUrl = null,
    imported = null;
  const tr = (ja, en) => (language === "ja" ? ja : en);
  const t = tr;
  const ready = (async () => {
    const response = await fetch("migration-form.html");
    if (!response.ok) throw new Error("interface_unavailable");
    const template = document.createElement("template");
    template.innerHTML = await response.text();
    root.append(template.content.cloneNode(true));
    initialize();
  })();
  window.addEventListener("pqc-language", (e) => {
    language = e.detail;
    ready.then(() => localize()).catch(() => {});
  });
  window.addEventListener("pqc-migration-job", (e) => {
    ready
      .then(() => loadJob(e.detail))
      .catch(() => {
        root.textContent = t(
          "画面を読み込めません。ページを開き直してください。",
          "Cannot load the interface. Reopen this page.",
        );
      });
  });
  let localize = () => {},
    loadJob = () => {};
  function initialize() {
    const $ = (id) => document.getElementById(id);
    const form = $("migration-form"),
      editor = $("migration-editor"),
      review = $("review");
    const fields = [...form.querySelectorAll("[data-answer]")];
    const supportFields = fields.filter((n) => n.id.startsWith("support-"));
    const applicabilityRow = $("applicability").closest(".field");
    const scopeOptions = document.createElement("details");
    scopeOptions.className = "answer-help scope-options";
    const scopeOptionTitle = document.createElement("summary");
    scopeOptionTitle.dataset.ja = "対象範囲の申告・対象外にする";
    scopeOptionTitle.dataset.en = "Scope declaration / exclude from review";
    scopeOptions.append(scopeOptionTitle, applicabilityRow, $("not-applicable-note"));
    const exclude = document.createElement("button");
    exclude.type = "button";
    exclude.className = "btn small-btn";
    exclude.dataset.ja = "この範囲を対象外と申告";
    exclude.dataset.en = "Declare this scope out of review";
    exclude.addEventListener("click", () => {
      $("applicability").value = "not_applicable";
      scopeOptions.open = true;
      updateQuestions(); resetConfirmation(); stash();
    });
    scopeOptions.append(exclude);
    $("support-details").before(scopeOptions);
    $("business-critical").closest(".field").dataset.relevant =
      "signature,jwt,pki";
    const controls = [
      "actor",
      "asserted-at",
      "valid-until",
      "candidate-parameters",
      "candidate-custom",
      "group-name",
      "lead-years",
      "lead-months",
      "lead-anchor",
      "deadline-reason", "deadline-issuer", "deadline-document", "deadline-edition", "deadline-clause",
    ];
    // Standard names only, not deployment or compatibility recommendations.
    const parameterSets = PqcContext.parameterSets;
    let targets = [],
      roleLabels = {};
    const targetById = new Map(),
      store = new PqcContext.Groups();
    const pageSize = 20,
      selected = new Set(),
      groups = store.items,
      drafts = new Map();
    let page = 0,
      mode = "new",
      groupId = null,
      individualId = null,
      supportBasis = "",
      supportHadKnown = false,
      editorRevision = 0,
      navigationSequence = 0;
    let parameterFamily = "",
      parameterDrafts = {};
    const japanNow = (roundUp = false) =>
      new Date(
        (roundUp ? Math.ceil(Date.now() / 1000) * 1000 : Date.now()) +
          9 * 3600000,
      )
        .toISOString()
        .slice(0, 19);
    const candidateFor = (purpose) =>
      ({
        signature: "ML-DSA",
        jwt: "ML-DSA",
        pki: "ML-DSA",
        key_establishment: "ML-KEM",
        encryption: "AES",
        hashing: "SHA-256",
      })[purpose] || "";
    const currentGroup = () => groups.find((g) => g.id === groupId);
    const owner = (id) => groups.find((g) => g.ids.includes(id));
    const draftKey = () =>
      mode === "new"
        ? "new"
        : mode === "individual"
          ? `target:${individualId}`
          : `group:${groupId}`;
    // Three views share one authoring session; report navigation preserves drafts.
    const overviewPanel = document.createElement("section"),
      contextStage = document.createElement("div"),
      reportPanel = document.createElement("section");
    overviewPanel.id = "migration-overview";
    contextStage.id = "migration-context-stage";
    reportPanel.id = "migration-response";
    reportPanel.hidden = true;
    const heading = root.querySelector(".migration-heading"),
      importControls = root.querySelector(".import-controls");
    contextStage.append(heading, importControls, editor, review);
    root.append(overviewPanel, contextStage, reportPanel);
    const stageNav = document.createElement("nav");
    stageNav.className = "migration-stages";
    stageNav.setAttribute("aria-label", t("移行の確認内の画面", "Views within migration review"));
    root.prepend(stageNav);
    const style = document.createElement("link");
    style.rel = "stylesheet";
    style.href = "report-view.css";
    document.head.append(style);
    let summary = null,
      stageView = "overview",
      mountedReport = null,
      scopeData = null,
      scopeSequence = 0,
      scopePending = false,
      filteredCohorts = [],
      removedUses = [];
    const scopeKeys = new Set();
    const cohortPanel = document.createElement("section");
    cohortPanel.className = "panel migration-panel cohort-panel";
    const cohortTitle = textNode("h2", ""),
      moduleLabel = document.createElement("label"),
      moduleSelect = document.createElement("select"),
      cohortRows = document.createElement("div");
    moduleLabel.className = "field";
    moduleLabel.append(textNode("span", ""), moduleSelect);
    cohortPanel.append(cohortTitle, moduleLabel, cohortRows);
    const oldTargets = form.querySelector(".targets-panel");
    const recordsDialog = document.createElement("dialog");
    recordsDialog.className = "scope-record-dialog";
    const closeRecords = button(
      "×",
      () => recordsDialog.close(),
      "dialog-close",
    );
    closeRecords.setAttribute("aria-label", t("閉じる", "Close"));
    recordsDialog.append(closeRecords, oldTargets);
    form.prepend(cohortPanel);
    form.append(recordsDialog);
    const formBody = document.createElement("div");
    formBody.className = "use-form-body";
    const groupPanel = $("groups-panel"),
      draftEditor = $("draft-editor");
    const applyButton = $("save-group");
    formBody.append(cohortPanel, draftEditor, groupPanel);
    const scopeAside = document.createElement("aside");
    scopeAside.className = "panel migration-panel scope-aside";
    const scopeHeading = textNode("h2", ""),
      scopeCount = textNode("strong", "0", "scope-count"),
      scopeNote = textNode("p", "", "small muted"),
      scopeError = textNode("p", "", "form-error");
    const recordsButton = button(
      "",
      () => {
        recordsDialog.showModal();
        $("target-search").focus();
      },
      "text-link",
    );
    const addUse = button(
      "",
      () => {
        if (saveDraft(false)) {
          scopeKeys.clear();
          scopeData = null;
          renderCohorts();
          refreshScopeSummary();
        }
      },
      "btn small-btn",
    );
    scopeAside.append(
      scopeHeading,
      scopeCount,
      scopeNote,
      recordsButton,
      scopeError,
      applyButton,
      addUse,
    );
    const formLayout = document.createElement("div");
    formLayout.className = "use-form-layout";
    formLayout.append(formBody, scopeAside);
    form.prepend(formLayout);
    let outsideDown = false;
    recordsDialog.addEventListener("pointerdown", (event) => {
      const r = recordsDialog.getBoundingClientRect();
      outsideDown =
        event.target === recordsDialog &&
        (event.clientX < r.left ||
          event.clientX > r.right ||
          event.clientY < r.top ||
          event.clientY > r.bottom);
    });
    recordsDialog.addEventListener("click", (event) => {
      if (outsideDown && event.target === recordsDialog) recordsDialog.close();
      outsideDown = false;
    });
    const progressPanel = document.createElement("dialog");
    progressPanel.className = "migration-generation panel";
    progressPanel.hidden = true;
    progressPanel.tabIndex = -1;
    progressPanel.setAttribute("aria-label", t("確認事項を準備しています", "Preparing review items"));
    progressPanel.addEventListener("cancel", e => e.preventDefault());
    progressPanel.addEventListener("keydown", e => { if (e.key === "Tab") { e.preventDefault(); progressPanel.focus(); } });
    progressPanel.setAttribute("aria-modal", "true");
    progressPanel.setAttribute("aria-live", "polite");
    const spinner = textNode("span", "", "spinner"),
      phaseLabel = textNode("span", ""),
      generationBar = document.createElement("progress");
    spinner.setAttribute("aria-hidden", "true");
    generationBar.setAttribute(
      "aria-label",
      t("レポート生成", "Report generation"),
    );
    progressPanel.append(spinner, phaseLabel, generationBar);
    root.insertBefore(progressPanel, overviewPanel);
    const locationNames = {
      source: ["ソースファイル", "Source files"],
      tests: ["テスト用フォルダ内", "Inside test folders"],
      dependency: ["依存関係の宣言", "Dependency declarations"],
      other: ["その他の記録", "Other records"],
    };
    const purposeNames = {
      signature: ["署名・認証", "Signing / authentication"],
      key_establishment: ["鍵の共有", "Key establishment"],
      encryption: ["暗号化", "Encryption"],
      hash: ["ハッシュ", "Hashing"],
      unknown: ["用途未特定", "Purpose unidentified"],
    };
    const loc = (key) => t(...(locationNames[key] || locationNames.other));
    function stage(next) {
      if (loading) return;
      navigationSequence++;
      stageView = next;
      overviewPanel.hidden = next !== "overview";
      contextStage.hidden = next !== "input";
      reportPanel.hidden = next !== "report";
      review.hidden = true;
      heading.querySelector("h1").textContent = t(
        "用途と利用状況を入力",
        "Enter purpose and usage",
      );
      stageNav.replaceChildren(textNode("strong", t("移行の確認", "Migration review"), "nav-context"));
      stageNav.setAttribute("aria-label", t("移行の確認内の画面", "Views within migration review"));
      for (const [i, name] of [
        t("概要", "Overview"),
        t("用途を確認", "Confirm use"),
        t("対応項目", "Response work items"),
      ].entries()) {
        const key = ["overview", "input", "report"][i],
          control = button(name, () => {
            if (key === "report") {
              stage("report");
              if (!mountedReport) {
                reportPanel.replaceChildren(textNode("h1", t("移行に向けた確認事項", "Migration review items")), textNode("p", t("用途の入力内容を確認してから、確認事項を作成します。", "Review your use information before preparing the items.")), button(t("確認事項を作成", "Prepare review items"), () => {
                  stage("input");
                  if (selectedIds().length && $("context-mode").value === "generate") saveDraft();
                  else reviewRequest($("context-mode").value === "generate" && !store.count());
                }, "btn primary"));
                const labels = [["移行に向けた確認事項", "Migration review items"], ["用途の入力内容を確認してから、確認事項を作成します。", "Review your use information before preparing the items."], ["確認事項を作成", "Prepare review items"]];
                [...reportPanel.children].forEach((node, i) => { node.dataset.ja = labels[i][0]; node.dataset.en = labels[i][1]; });
              }
            } else stage(key);
          });
        if (key === next) control.setAttribute("aria-current", "page");
        stageNav.append(control);
      }
      (next === "overview"
        ? overviewPanel
        : next === "input"
          ? heading
          : reportPanel
      ).tabIndex = -1;
      (next === "overview"
        ? overviewPanel
        : next === "input"
          ? heading
          : reportPanel
      ).focus();
    }
    function renderOverview() {
      if (!summary) return;
      overviewPanel.replaceChildren(
        textNode("h1", t("スキャン結果の概要", "Scan overview")),
      );
      const known = Object.entries(summary.algorithms)
          .filter(([a]) => a !== "unknown")
          .reduce((n, [, count]) => n + count, 0),
        metrics = document.createElement("div");
      metrics.className = "review-metrics";
      for (const [label, count] of [
        [t("検出された記録", "Saved records"), summary.total],
        [t("方式名が分かる記録", "Records with algorithm names"), known],
        [
          t("方式名が未特定の記録", "Records with unidentified algorithms"),
          summary.total - known,
        ],
      ]) {
        const item = document.createElement("div");
        item.append(
          textNode("p", label, "small muted"),
          textNode(
            "strong",
            count.toLocaleString(language === "ja" ? "ja-JP" : "en-US"),
          ),
        );
        metrics.append(item);
      }
      overviewPanel.append(metrics);
      const columns = document.createElement("div");
      columns.className = "review-distributions";
      for (const [title, entries, isLocation] of [
        [
          t("暗号方式", "Algorithms"),
          Object.entries(summary.algorithms).sort((a, b) => b[1] - a[1]),
          false,
        ],
        [
          t("記録の場所", "Record locations"),
          Object.entries(summary.locations),
          true,
        ],
      ]) {
        const panel = document.createElement("section");
        panel.className = "panel migration-panel";
        panel.append(textNode("h2", title));
        const other = document.createElement("details");
        other.append(textNode("summary", t("ほかの方式", "Other algorithms")));
        entries.forEach(([name, count], i) => {
          const row = document.createElement("div");
          row.className = "overview-distribution-row";
          row.append(
            textNode(
              "span",
              isLocation
                ? loc(name)
                : name === "unknown"
                  ? t("未特定", "Unidentified")
                  : name,
            ),
            textNode(
              "span",
              count.toLocaleString(language === "ja" ? "ja-JP" : "en-US"),
            ),
          );
          if (!isLocation && i >= 6) other.append(row);
          else panel.append(row);
        });
        if (other.children.length > 1) panel.append(other);
        if (isLocation)
          panel.append(
            textNode(
              "p",
              t(
                "フォルダ名などで分類しています。実運用で使うかは未確認です。",
                "Location hints do not establish deployed use.",
              ),
              "small muted",
            ),
          );
        columns.append(panel);
      }
      overviewPanel.append(columns);
      const actions = document.createElement("div");
      actions.className = "overview-actions";
      actions.append(
        button(
          t("入力せずにレポートを見る", "View report without context"),
          () => reviewRequest(true),
          "text-link",
        ),
        button(t("次へ", "Next"), () => stage("input"), "btn primary"),
      );
      overviewPanel.append(actions);
    }
    function cancelScope() {
      scopeSequence++;
      scopePending = false;
    }
    function renderCohorts() {
      cohortPanel.hidden = mode !== "new";
      if (!summary) return;
      cohortTitle.textContent = t("記録の範囲を選択", "Select record scope");
      moduleLabel.firstChild.textContent = t(
        "場所で絞り込む",
        "Filter by location",
      );
      const previous = moduleSelect.value;
      moduleSelect.replaceChildren(
        new Option(t("すべての場所", "All locations"), ""),
      );
      for (const [name, count] of Object.entries(summary.modules))
        if (name) moduleSelect.append(new Option(`${name} (${count})`, name));
      moduleSelect.value = previous;
      const focusedKey = cohortRows.contains(document.activeElement) ? document.activeElement.value : null;
      cohortRows.replaceChildren();
      for (const cohort of filteredCohorts) {
        const label = document.createElement("label"),
          box = document.createElement("input"),
          name = textNode(
            "span",
            `${t(...(purposeNames[cohort.purpose] || purposeNames.unknown))} · ${cohort["role_" + language] || cohort.role} / ${loc(cohort.location)}`,
          ),
          count = textNode(
            "span",
            t(
              `${cohort.eligible}件に入力（全${cohort.total}件、入力済み${cohort.owned || 0}件）`,
              `${cohort.eligible} eligible (${cohort.total} records; ${cohort.owned || 0} assigned)`,
            ),
            "small muted",
          );
        label.className = "cohort-choice";
        box.type = "checkbox";
        box.value = cohort.key;
        box.checked = scopeKeys.has(cohort.key);
        box.disabled = mode !== "new" || loading || cohort.eligible === 0;
        box.addEventListener("change", () => {
          box.checked
            ? scopeKeys.add(cohort.key)
            : scopeKeys.delete(cohort.key);
          refreshScope().catch(reportError);
        });
        label.append(box, name, count);
        cohortRows.append(label);
      }
      if (!filteredCohorts.length) cohortRows.append(textNode("p", t("この場所には入力できる記録がありません。対象を特定できない記録や入力済みの記録は選択に含めません。", "No eligible records at this location. Ambiguous and already assigned records are excluded."), "hint"));
      if (focusedKey) [...cohortRows.querySelectorAll("input")].find(box => box.value === focusedKey)?.focus();
      moduleSelect.disabled = mode !== "new" || loading;
      refreshScopeSummary();
    }
    function refreshScopeSummary() {
      const count = selectedIds().length;
      scopeHeading.textContent = t(
        "この回答を適用する範囲",
        "Scope for these answers",
      );
      scopeCount.textContent = `${count.toLocaleString()}${t("件", " records")}`;
      scopeNote.textContent = scopePending
        ? t("範囲を確認中…", "Checking scope…")
        : scopeData && mode === "new"
          ? t(
              `選択範囲${scopeData.total}件、対象を特定できない記録${scopeData.ambiguous}件、別の用途で入力済み${scopeData.owned}件。`,
              `Selected ${scopeData.total}; ambiguous ${scopeData.ambiguous}; assigned to other uses ${scopeData.owned}.`,
            )
          : "";
      recordsButton.textContent = t(
        "記録を見る・一部を分ける",
        "Inspect records / split scope",
      );
      addUse.textContent = t("別の用途を追加", "Add another use");
      const tooMany =
        mode === "new" &&
        (store.count() + count > 4096 || groups.length >= 256);
      scopeError.textContent = tooMany
        ? t(
            "入力できる合計上限を超えています。範囲を絞ってください。",
            "The total scope limit is exceeded. Narrow the selection.",
          )
        : "";
      $("save-group").textContent = t("次へ", "Next");
      $("save-group").disabled = loading || scopePending || !count || tooMany;
      addUse.disabled = $("save-group").disabled;
    }
    async function refreshScope(keepSelection = false) {
      const seq = ++scopeSequence,
        v = version;
      scopePending = true;
      if (!keepSelection) selected.clear();
      scopeData = null;
      resetConfirmation();
      render();
      try {
        const result = await api(`api/migration/scope?job=${job.id}`, {
          input_sha256: input.sha256,
          keys: keepSelection ? [] : [...scopeKeys],
          module: moduleSelect.value || null,
          excluded: store.effective().map((r) => Number(r.id)),
        });
        if (seq !== scopeSequence || v !== version) return;
        if (result.input.sha256 !== input.sha256)
          throw new Error("input_binding_mismatch");
        filteredCohorts = result.cohorts.filter(c => c.eligible > 0).map(c => ({ ...summary.cohorts.find(s => s.key === c.key), ...c }));
        const visible = new Set(filteredCohorts.filter(c => c.eligible > 0).map(c => c.key));
        for (const key of scopeKeys) if (!visible.has(key)) scopeKeys.delete(key);
        if (!keepSelection) scopeData = result;
        for (const row of keepSelection ? [] : result.targets) {
          const r = {
            ...row,
            boundId: row.id,
            id: String(row.number),
            rawPath: row.path,
            path:
              (row.path || t("保存済みの記録", "Saved record")) +
              (row.line ? ":" + row.line : ""),
          };
          targetById.set(r.id, r);
          selected.add(r.id);
        }
        return true;
      } catch (error) {
        if (seq === scopeSequence && v === version) throw error;
      } finally {
        if (seq === scopeSequence && v === version) {
          renderCohorts();
          scopePending = false;
          render();
          updateQuestions();
        }
      }
    }
    moduleSelect.addEventListener("change", () => {
      scopeKeys.clear();
      filteredCohorts = [];
      cohortRows.replaceChildren(textNode("p", t("範囲を読み込み中…", "Loading scope…"), "hint"));
      refreshScope().catch(reportError);
    });
    function generationProgress(data) {
      const labels = {
        loading: ["保存データを読み込み中", "Loading saved evidence"],
        preview: ["入力を検証中", "Validating input"],
        evaluating: ["移行判断を評価中", "Evaluating migration decisions"],
        validating: ["出力内容を検証中", "Validating output"],
        json: ["評価データを生成中", "Writing assessment data"],
        markdown: ["説明を生成中", "Generating descriptions"],
        html: ["HTMLを生成中", "Generating HTML"],
        writing: ["レポートを保存中", "Saving reports"],
        context: ["移行情報を保存中", "Saving context"],
        opening: ["画面を準備中", "Preparing the view"],
      };
      progressPanel.hidden = false;
      if (!progressPanel.open) progressPanel.showModal();
      phaseLabel.textContent = t(...(labels[data.phase] || labels.loading));
      if (data.phase === "writing") {
        generationBar.max = data.total;
        generationBar.value = data.completed;
        phaseLabel.textContent += ` (${data.completed}/${data.total})`;
      } else generationBar.removeAttribute("value");
    }
    async function generateReport() {
      if (!preview) return;
      const v = version,
        id = job.id;
      controller = null;
      $("cancel-review").hidden = true;
      generationProgress({ phase: "loading" });
      const started = await api(`api/migration/start?job=${id}`, {
        token: preview.token,
        language,
      });
      const deadline = Date.now() + 15 * 60 * 1000;
      let result;
      while (Date.now() < deadline) {
        if (v !== version) throw new Error("stale_generation");
        const response = await fetch(`api/migration/progress?job=${id}`, {
          headers: { "X-Pqc-Migration-Token": started.token },
        });
        const status = await response.json();
        if (!response.ok) throw new Error(status.error || "generation_failed");
        if (status.status === "failed") throw new Error(status.error);
        if (status.status === "ready") {
          result = status.result;
          break;
        }
        generationProgress(status);
        await new Promise((resolve) => setTimeout(resolve, 150));
      }
      if (!result) throw new Error("generation_timeout");
      generationProgress({ phase: "opening" });
      const response = await fetch(result.report);
      if (!response.ok) throw new Error("report_unavailable");
      const html = await response.text(),
        documentView = new DOMParser().parseFromString(html, "text/html"),
        block = documentView.getElementById("migration-shared-data");
      if (!block) throw new Error("report_unavailable");
      mountedReport?.destroy();
      reportPanel.replaceChildren();
      const tools = document.createElement("div");
      tools.className = "response-tools";
      tools.append(
        button(t("用途を修正", "Edit use"), () => {
          if (groups.length) switchEditor("group", groups.at(-1).id);
          stage("input");
        }),
      );
      tools.firstElementChild.dataset.ja = "用途を修正";
      tools.firstElementChild.dataset.en = "Edit use";
      const saved = document.createElement("a");
      saved.className = "btn";
      saved.href = result.report;
      saved.addEventListener("click", (event) => {
        event.preventDefault();
        const copy = new DOMParser().parseFromString(html, "text/html");
        copy.querySelector("#pqc-session-back")?.closest("nav")?.remove();
        for (const script of copy.querySelectorAll(
          'script[src="../../report-session.js"]',
        ))
          script.remove();
        copy.documentElement.dataset.theme =
          document.documentElement.dataset.theme;
        for (const tree of [copy, ...[...copy.querySelectorAll("template")].map(n => n.content)])
          for (const link of tree.querySelectorAll('a[href="migration.assessment.md"], a[href="migration.assessment.v1.json"]')) link.remove();
        copy.documentElement.dataset.savedStandalone = "true";
        copy.documentElement.dataset.exportTheme =
          document.documentElement.dataset.theme;
        copy.documentElement.lang = language;
        const active = copy.querySelector("[data-report-view]"),
          alternate = copy.querySelector("[data-report-translation]");
        if (active.lang !== language && alternate.lang === language) {
          const incoming = alternate.content.firstElementChild;
          active.replaceWith(incoming);
          alternate.content.append(active);
          alternate.lang = active.lang;
        }
        const blob = new Blob(
          ["<!doctype html>\n" + copy.documentElement.outerHTML],
          { type: "text/html;charset=utf-8" },
        );
        if (blob.size > 64 * 1024 * 1024) {
          reportError(new Error("resource_limit"));
          return;
        }
        const url = URL.createObjectURL(blob),
          link = document.createElement("a");
        link.href = url;
        link.download = "migration.assessment.html";
        link.click();
        setTimeout(() => URL.revokeObjectURL(url), 10000);
      });
      saved.download = "migration.assessment.html";
      saved.dataset.ja = "レポートを保存（HTML）";
      saved.dataset.en = "Save report (HTML)";
      saved.textContent = t("レポートを保存（HTML）", "Save report (HTML)");
      tools.append(saved);
      const openReport = document.createElement("a");
      openReport.className = "btn";
      openReport.target = "_blank"; openReport.rel = "noopener";
      const reportURL = new URL(result.report, location.href);
      reportURL.searchParams.set("theme", document.documentElement.dataset.theme);
      reportURL.searchParams.set("lang", language);
      openReport.href = reportURL.href;
      openReport.addEventListener("click", () => {
        reportURL.searchParams.set("theme", document.documentElement.dataset.theme);
        reportURL.searchParams.set("lang", language);
        openReport.href = reportURL.href;
      });
      openReport.dataset.ja = "レポートを別タブで開く";
      openReport.dataset.en = "Open report in a new tab";
      openReport.textContent = t(openReport.dataset.ja, openReport.dataset.en);
      tools.append(openReport);
      if (result.context) {
        const contextLink = document.createElement("a");
        contextLink.className = "btn";
        contextLink.href = `reports/${id}/migration-context.json`;
        contextLink.download = "migration-context.json";
        contextLink.dataset.ja = "入力内容を保存（JSON）";
        contextLink.dataset.en = "Save input (JSON)";
        contextLink.textContent = t("入力内容を保存（JSON）", "Save input (JSON)");
        tools.append(contextLink);
      }
      const body = document.createElement("div");
      body.className = "migration";
      body.lang = language;
      reportPanel.append(tools, body);
      mountedReport = PqcMigrationReport.mount(
        JSON.parse(block.textContent),
        body,
        false,
        `reports/${id}/`,
      );
      setBusy(false);
      if (progressPanel.open) progressPanel.close();
      progressPanel.hidden = true;
      stage("report");
    }

    $("as-of").value = japanNow(true);
    function textNode(tag, text, className = "") {
      const n = document.createElement(tag);
      n.textContent = text;
      if (className) n.className = className;
      return n;
    }
    function button(text, callback, className = "btn small-btn") {
      const n = textNode("button", text, className);
      n.type = "button";
      n.addEventListener("click", callback);
      return n;
    }
    function read() {
      rememberParameters();
      return {
        answers: Object.fromEntries(fields.map((n) => [n.id, n.value])),
        details: Object.fromEntries(
          fields
            .filter((n) => $(n.id + "-value"))
            .map((n) => [n.id, $(n.id + "-value").value]),
        ),
        values: Object.fromEntries(controls.map((id) => [id, $(id).value])),
        candidate: $("candidate").value,
        audience: form.querySelector("input[name=audience]:checked").value,
        supportBasis,
        parameterDrafts: JSON.parse(JSON.stringify(parameterDrafts)),
      };
    }
    function stash() {
      if (mode === "new" && !selected.size) return;
      const value = read(),
        g = currentGroup(),
        baseline =
          mode === "new"
            ? null
            : mode === "group"
              ? g.common
              : g.exceptions.get(individualId) || g.common;
      if (baseline && JSON.stringify(value) === JSON.stringify(baseline))
        drafts.delete(draftKey());
      else drafts.set(draftKey(), value);
    }
    function resetConfirmation() {
      editorRevision++;
      $("context-reviewed").checked = false;
      $("show-migration-report").disabled = true;
      $("form-error").hidden = true;
      invalidate();
    }
    function selectedIds() {
      return mode === "new"
        ? [...selected]
        : mode === "individual"
          ? [individualId]
          : currentGroup().ids.slice();
    }
    function basisKey() {
      return JSON.stringify([
        $("use-case").value,
        $("candidate").value,
        $("candidate-parameters").value.trim(),
      ]);
    }
    function hasKnownSupport() {
      return supportFields.some(
        (n) => n.value === "true" || n.value === "false",
      );
    }
    function hasActiveKnownSupport() {
      if (
        $("applicability").value === "not_applicable" ||
        !$("candidate").value ||
        !$("candidate-parameters").value.trim()
      )
        return false;
      const relevant = requiredSupport($("use-case").value);
      return supportFields.some(
        (n) =>
          relevant.has(n.id.slice(8)) &&
          (n.value === "true" || n.value === "false"),
      );
    }
    function rememberParameters() {
      if (parameterFamily)
        parameterDrafts[parameterFamily] = {
          choice: $("parameter-choice").value,
          custom: $("candidate-custom").value,
        };
    }
    function syncParameters() {
      const custom = $("parameter-choice").value === "__custom__";
      $("candidate-custom").hidden = !custom;
      $("custom-parameter-note").hidden = !custom;
      $("candidate-parameters").value = custom
        ? $("candidate-custom").value.trim()
        : $("parameter-choice").value;
      rememberParameters();
    }
    function rebuildParameters() {
      rememberParameters();
      parameterFamily = $("candidate").value;
      const state = parameterDrafts[parameterFamily] || {
        choice: "",
        custom: "",
      };
      $("parameter-choice").replaceChildren(
        new Option(t("未定", "Undecided"), ""),
      );
      for (const name of parameterSets[parameterFamily] || [])
        $("parameter-choice").append(new Option(name, name));
      $("parameter-choice").append(
        new Option(
          t("その他（手入力）", "Other (enter manually)"),
          "__custom__",
        ),
      );
      $("parameter-choice").value = state.choice;
      $("candidate-custom").value = state.custom;
      syncParameters();
    }
    function updateCandidate(value = $("candidate").value) {
      const offered = candidateFor($("use-case").value);
      $("candidate").replaceChildren(new Option(t("未定", "Undecided"), ""));
      if (offered) $("candidate").append(new Option(offered, offered));
      $("candidate").value = value === offered ? value : "";
      $("candidate").disabled = false;
      rebuildParameters();
    }
    function requiredSupport(purpose) {
      const names = new Set();
      for (const id of selectedIds())
        for (const name of targetById.get(id)?.support_fields?.[
          purpose || "unspecified"
        ] || [])
          names.add(name);
      return names;
    }
    function effectiveSnapshot(snapshot) {
      return PqcContext.active(
        snapshot,
        fields.map((n) => ({
          id: n.id,
          relevance: n.closest("[data-relevant]")?.dataset.relevant,
        })),
        [...requiredSupportForSnapshot(snapshot)],
      );
    }
    function requiredSupportForSnapshot(snapshot) {
      const names = new Set();
      for (const row of targetById.values())
        for (const name of row.support_fields?.[
          snapshot.answers["use-case"] || "unspecified"
        ] || [])
          names.add(name);
      return names;
    }
    function completeBasis() {
      return !!$("candidate").value && !!$("candidate-parameters").value.trim();
    }
    function chooseSchedule(id) {
      if (id === "lead-time" && $(id).value) { $("goal-month").value = ""; $("goal-month-value").value = ""; }
      if (id === "goal-month" && $(id).value) { $("lead-time").value = ""; $("lead-time-value").value = ""; }
    }
    function updateDuration() {
      if ($("lead-time").value !== "specified") return;
      try {
        const anchor = $("lead-anchor").value + "T00:00:00";
        const result = PqcContext.calendarDuration(anchor, $("lead-years").value, $("lead-months").value);
        $("lead-time-value").value = String(result.seconds);
        $("duration-note").textContent = t(
          `${$("lead-anchor").value}から${result.end.slice(0, 10)}までの${result.seconds / 86400}日（${result.seconds}秒）。`,
          `${$("lead-anchor").value} to ${result.end.slice(0, 10)}: ${result.seconds / 86400} days (${result.seconds} seconds).`);
      } catch {
        $("lead-time-value").value = "";
        $("duration-note").textContent = t("基準日と0〜10年・0〜11か月を入力してください。換算結果は上限315,576,000秒以内にしてください。", "Enter an anchor, 0–10 years and 0–11 months, within 315,576,000 seconds.");
      }
    }
    function updateQuestions() {
      const purpose = $("use-case").value,
        out = $("applicability").value === "not_applicable",
        relevant = requiredSupport(purpose);
      for (const n of fields)
        if (!n.id.startsWith("support-")) {
          const row = n.closest(".field"),
            rel = row.dataset.relevant;
          row.hidden =
            n.id !== "applicability" &&
            (out || (!!rel && !rel.split(",").includes(purpose)));
        }
      for (const detail of form.querySelectorAll("[data-detail-for]"))
        detail.hidden = $(detail.dataset.detailFor).value !== "specified";
      $("business-critical").closest(".field").hidden =
        out ||
        !["signature", "jwt", "pki"].includes(purpose) ||
        !["active", "planned"].includes($("usage-state").value);
      $("actor").closest("section").hidden = !(
        Object.values(effectiveSnapshot(read()).answers).some(Boolean) ||
        completeBasis()
      );
      $("support-details").hidden = out;
      $("deadline-details").hidden = out;
      scopeOptions.open ||= out;
      exclude.hidden = out;
      $("lead-time").closest("details").hidden = out;
      form.querySelector(".audience-options").hidden = out;
      $("not-applicable-note").hidden = !out;
      const candidate = $("candidate").value,
        parameters = $("candidate-parameters").value.trim(),
        complete = !!candidate && !!parameters;
      $("candidate-field").hidden = out || !candidateFor(purpose);
      $("parameters-field").hidden = out || !candidate;
      $("candidate-note").textContent = candidate
        ? `${candidate} / ${parameters || t("設定未定", "Configuration undecided")}`
        : candidateFor(purpose)
          ? t("上の「検討する方式」を選ぶと設定を入力できます。未定のままでも進められます。", "Choose a candidate above to enter its configuration, or continue undecided.")
          : t("先に「用途」を選ぶと、方式・設定・対応状況の順に入力できます。", "First choose a purpose; then enter a candidate, configuration and support details in order.");
      $("basis-incomplete").hidden = complete || !candidate;
      $("support-empty").hidden = out || !complete || relevant.size > 0;
      $("support-questions").hidden = out || !complete;
      for (const row of form.querySelectorAll("[data-support]")) {
        const control = row.querySelector("select");
        row.hidden = !relevant.has(row.dataset.support);
        for (const option of control.options) option.disabled = false;
      }
      updateDuration();
      $("support-basis-warning").hidden = !(
        hasActiveKnownSupport() && supportBasis !== basisKey()
      );
      supportHadKnown = hasActiveKnownSupport();
    }
    function load(snapshot) {
      const s = snapshot || {
        answers: {},
        details: {},
        values: {},
        candidate: "",
        audience: "personal",
        supportBasis: "",
      };
      for (const n of fields) n.value = s.answers[n.id] || "";
      for (const n of fields)
        if ($(n.id + "-value"))
          $(n.id + "-value").value = s.details[n.id] || "";
      for (const id of controls) $(id).value = s.values[id] || "";
      $("lead-years").value ||= "0";
      $("lead-months").value ||= "0";
      $("lead-anchor").value ||= $("as-of").value.slice(0, 10);
      form.querySelector(
        `input[name=audience][value="${s.audience}"]`,
      ).checked = true;
      parameterFamily = "";
      parameterDrafts = JSON.parse(JSON.stringify(s.parameterDrafts || {}));
      if (s.candidate && !parameterDrafts[s.candidate]) {
        const v = s.values["candidate-parameters"] || "";
        parameterDrafts[s.candidate] = {
          choice: !v
            ? ""
            : (parameterSets[s.candidate] || []).includes(v)
              ? v
              : "__custom__",
          custom: v,
        };
      }
      updateCandidate(s.candidate);
      supportHadKnown = hasKnownSupport();
      supportBasis = supportHadKnown ? s.supportBasis || "" : "";
      updateQuestions();
      resetConfirmation();
    }
    function switchEditor(next, group = null, target = null, saveDraft = true) {
      if (saveDraft) stash();
      scopeKeys.clear();
      scopeData = null;
      cancelScope();
      mode = next;
      groupId = group;
      individualId = target;
      const g = currentGroup(),
        baseline =
          next === "new"
            ? null
            : next === "individual"
              ? g.exceptions.get(target) || g.common
              : g.common;
      load(drafts.get(draftKey()) || baseline);
      render();
    }
    function visibleTargets() {
      return targets;
    }
    function renderTable() {
      const filtered = visibleTargets();
      const rows = filtered;
      $("target-rows").replaceChildren();
      const chosen = new Set(selectedIds());
      for (const t of rows) {
        const row = document.createElement("tr"),
          cell = document.createElement("td"),
          g = owner(t.id),
          box = document.createElement("input");
        box.type = "checkbox";
        box.checked = chosen.has(t.id);
        box.disabled = loading || mode !== "new" || !!g || !t.selectable;
        box.setAttribute(
          "aria-label",
          `${t.algorithm === "unknown" ? tr("未特定", "Unidentified") : t.algorithm} · ${t.path}`,
        );
        box.addEventListener("change", () => {
          if (box.checked && selected.size + store.count() >= 4096) {
            box.checked = false;
            fail(
              tr(
                "選択できる対象は合計4096件までです。",
                "Select at most4096 targets in total.",
              ),
              $("target-search"),
            );
            return;
          }
          cancelScope();
          box.checked ? selected.add(t.id) : selected.delete(t.id);
          scopeKeys.clear();
          scopeData = null;
          for (const b of cohortRows.querySelectorAll("input"))
            b.checked = false;
          resetConfirmation();
          render();
        });
        cell.append(box);
        row.append(cell);
        row.append(
          textNode(
            "td",
            t.algorithm === "unknown"
              ? tr("未特定", "Unidentified")
              : t.algorithm,
          ),
          textNode("td", t["role_" + language] || t.role),
          textNode("td", t.path, "mono"),
        );
        const assignment = document.createElement("td");
        if (g) {
          assignment.append(textNode("span", g.name, "group-row-name"));
          const b = button(
            g.exceptions.has(t.id)
              ? tr("個別設定を編集", "Edit individual context")
              : tr("個別に修正", "Edit individually"),
            () => {
              recordsDialog.close();
              switchEditor("individual", g.id, t.id);
              $("draft-title").focus();
            },
            "row-link",
          );
          b.setAttribute(
            "aria-label",
            tr(`${t.path}を個別に修正`, `Edit ${t.path} individually`),
          );
          assignment.append(b);
        } else
          assignment.append(
            textNode(
              "span",
              t.selectable
                ? tr("未入力", "Unanswered")
                : tr("対応付けできません（回答では解消できない検出上の制約）", "Cannot bind: a detection limitation that answers cannot resolve"),
              "muted",
            ),
          );
        if (!t.selectable) assignment.append(textNode("small", tr("参照箇所と設定を調査し、対応付け可能な別の根拠を確認してください。検出・対応付け処理の調査が必要な場合もあります。", "Inspect the source/configuration and separately identifiable evidence. The detection/binding issue may need investigation."), "hint"));
        row.append(assignment);
        $("target-rows").append(row);
      }
      if (!rows.length) {
        const td = textNode(
          "td",
          t("一致する対象はありません。", "No matching targets."),
          "muted",
        );
        td.colSpan = 5;
        const tr = document.createElement("tr");
        tr.append(td);
        $("target-rows").append(tr);
      }
      const available = rows.filter((t) => !owner(t.id) && t.selectable);
      $("select-page").disabled =
        loading || mode !== "new" || !available.length;
      $("select-page").checked =
        available.length > 0 && available.every((t) => selected.has(t.id));
      $("select-page").indeterminate =
        available.some((t) => selected.has(t.id)) && !$("select-page").checked;
      $("target-range").textContent = total
        ? `${page * pageSize + 1}–${Math.min((page + 1) * pageSize, total)} / ${total}`
        : "0";
      $("targets-prev").disabled = loading || page === 0;
      $("targets-next").disabled = loading || (page + 1) * pageSize >= total;
      const hidden = [...chosen].filter(
        (id) => !rows.some((t) => t.id === id),
      ).length;
      $("selection-count").textContent = t(
        `選択${chosen.size}件${hidden ? "（このページ外" + hidden + "件）" : ""}`,
        `${chosen.size} selected${hidden ? " (" + hidden + " outside this page)" : ""}`,
      );
      $("clear-selection").disabled = mode !== "new";
    }
    const deleteDialog = document.createElement("dialog");
    deleteDialog.className = "scope-record-dialog";
    deleteDialog.setAttribute("aria-labelledby", "delete-use-title");
    let deleteUseId = null;
    const deleteTitle = textNode("h2", "");
    deleteTitle.id = "delete-use-title";
    const deleteDescription = textNode("p", "", "hint");
    const cancelDelete = button("", () => deleteDialog.close());
    const confirmDelete = button("", () => {
      if (loading || scopePending || !deleteUseId) return;
      stash();
      removedUses.push(store.remove(deleteUseId));
      if (groupId === deleteUseId) switchEditor("new", null, null, false);
      cancelScope(); scopeKeys.clear(); scopeData = null;
      resetConfirmation(); render();
      refreshScope(true).catch(reportError);
      deleteDialog.close();
      message("用途を削除しました。入力済みの用途から元に戻せます。", "Use deleted. Undo is available under saved uses.");
      $("groups-panel").focus();
    }, "btn danger");
    const deleteActions = textNode("div", "", "toolbar");
    deleteActions.append(cancelDelete, confirmDelete);
    deleteDialog.append(deleteTitle, deleteDescription, deleteActions);
    root.append(deleteDialog);
    $("groups-panel").tabIndex = -1;
    function askDelete(g) {
      if (loading || scopePending) return;
      deleteUseId = g.id;
      deleteTitle.textContent = t(`「${g.name}」を削除しますか`, `Delete “${g.name}”?`);
      deleteDescription.textContent = t(
        `${g.ids.length}件の対象への入力と個別設定${g.exceptions.size}件を外します。削除後に元に戻せます。保存済みのファイルは削除しません。`,
        `Remove input for ${g.ids.length} targets, including ${g.exceptions.size} individual overrides. You can undo this. Existing saved files are retained.`);
      cancelDelete.textContent = t("キャンセル", "Cancel");
      confirmDelete.textContent = t("用途を削除", "Delete use");
      deleteDialog.showModal(); cancelDelete.focus();
    }
    function renderGroups() {
      $("groups-panel").hidden = groups.length === 0 && removedUses.length === 0;
      $("groups-list").replaceChildren();
      for (const g of groups) {
        const row = document.createElement("div");
        row.className = "saved-group";
        row.append(
          textNode("strong", g.name),
          textNode(
            "span",
            t(
              `${g.ids.length}件${g.exceptions.size ? " · 個別設定" + g.exceptions.size + "件" : ""}`,
              `${g.ids.length} targets${g.exceptions.size ? " · " + g.exceptions.size + " individual" : ""}`,
            ),
            "small muted",
          ),
          button(t("共通の内容を編集", "Edit shared context"), () => {
            switchEditor("group", g.id);
            $("draft-title").focus();
          }),
        );
        const remove = button(t("削除", "Delete"), () => askDelete(g), "btn small-btn danger");
        remove.disabled = loading || scopePending;
        row.append(remove);
        $("groups-list").append(row);
      }
      for (const removed of removedUses) {
        const row = textNode("div", "", "undo-use");
        row.append(textNode("span", t(`削除済み：${removed.group.name}（${removed.group.ids.length}件）`, `Deleted: ${removed.group.name} (${removed.group.ids.length} targets)`)));
        const undo = button(t("元に戻す", "Undo"), () => {
          if (loading || scopePending) return;
          try {
            if (removed.group.ids.some(id => selected.has(id))) throw new Error("invalid_scope");
            store.undo(removed);
            removedUses = removedUses.filter(r => r !== removed);
            cancelScope(); scopeKeys.clear(); scopeData = null;
            resetConfirmation(); render(); refreshScope(true).catch(reportError);
            message("用途と個別設定を元に戻しました。", "Use and individual overrides restored.");
          } catch {
            message("同じ対象が別の用途か編集中の選択に含まれています。重複を解消してから元に戻してください。", "Some targets belong to another use or the current selection. Resolve the overlap before undoing.", true);
          }
        });
        undo.disabled = loading || scopePending;
        row.append(undo);
        $("groups-list").append(row);
      }
      $("group-coverage").textContent = t(
        `入力グループに含まれる対象：${store.count()} / ${allTotal}件`,
        `${store.count()} / ${allTotal} targets in groups`,
      );
    }
    function render() {
      cohortPanel.hidden = mode !== "new";
      renderTable();
      renderGroups();
      const ids = selectedIds(),
        g = currentGroup();
      $("draft-editor").hidden = false;
      $("draft-title").textContent =
        mode === "individual"
          ? t(
              `個別に修正：${targetById.get(individualId).path}`,
              `Edit individually: ${targetById.get(individualId).path}`,
            )
          : mode === "group"
            ? g.name
            : t("用途の情報", "Use context");
      $("group-name-field").hidden = mode === "individual";
      $("restore-common").hidden = mode !== "individual";
      $("individual-note").hidden = mode !== "individual";
      $("draft-note").textContent =
        mode === "group" && g.exceptions.size
          ? t(
              "個別設定がある対象は変更しません。",
              "Targets with individual context are unchanged.",
            )
          : "";
      refreshScopeSummary();
      for (const box of cohortRows.querySelectorAll("input"))
        box.disabled =
          mode !== "new" ||
          loading ||
          filteredCohorts.find((c) => c.key === box.value)?.eligible === 0;
      moduleSelect.disabled = mode !== "new" || loading;
    }
    function fail(message, control) {
      $("form-error").textContent = message;
      $("form-error").hidden = false;
      for (let p = control?.parentElement; p; p = p.parentElement)
        if (p.tagName === "DETAILS") p.open = true;
      control?.focus();
      return false;
    }
    function validateDraft() {
      if (!$("as-of").value || !$("as-of").validity.valid)
        return fail(
          t("評価日時を入力してください。", "Enter an evaluation time."),
          $("as-of"),
        );
      const ids = selectedIds();
      if (
        mode === "new" &&
        (store.count() + ids.length > 4096 || groups.length >= 256)
      )
        return fail(
          t(
            "入力できる合計上限を超えています。",
            "The total scope limit is exceeded.",
          ),
          cohortTitle,
        );
      if (!ids.length)
        return fail(
          t(
            "入力を適用する対象を選んでください。",
            "Select the targets for this context.",
          ),
          $("target-search"),
        );
      const active = effectiveSnapshot(read()),
        purpose = active.answers["use-case"],
        out = active.answers.applicability === "not_applicable",
        expected = {
          signature: "signature",
          jwt: "signature",
          pki: "signature",
          key_establishment: "key_establishment",
          encryption: "encryption",
          hashing: "hash",
        };
      if (
        purpose &&
        purpose !== "?" &&
        ids.some((id) => {
          const p = targetById.get(id).purpose;
          return p !== "unknown" && p !== expected[purpose];
        })
      )
        return fail(
          t(
            "役割に合わない用途が含まれています。対象を分けて選んでください。",
            "The purpose conflicts with some targets. Select separate groups.",
          ),
          $("use-case"),
        );
      if (
        !out &&
        $("candidate").value &&
        $("parameter-choice").value === "__custom__" &&
        !$("candidate-custom").value.trim()
      )
        return fail(
          t(
            "その他の設定を入力してください。",
            "Enter the custom configuration.",
          ),
          $("candidate-custom"),
        );
      if (hasActiveKnownSupport() && supportBasis !== basisKey())
        return fail(
          t(
            "方式・設定が変わっています。対応状況を回答し直してください。",
            "The algorithm or configuration changed. Reset support answers.",
          ),
          $("reset-support"),
        );
      if (!out && active.answers.deadline === "specified") {
        if (!["organization_policy", "contractual_requirement"].includes(active.values["deadline-reason"])) return fail(t("期限の根拠を選んでください。", "Select the deadline basis."), $("deadline-reason"));
        for (const name of ["issuer", "document", "edition", "clause"]) {
          const id = "deadline-" + name;
          if (!/^[!-~]{1,128}$/.test(active.values[id] || "")) return fail(t("期限の根拠の識別名を、空白なしの半角英数字・記号で入力してください。", "Enter visible ASCII reference identifiers without spaces."), $(id));
        }
      }
      for (const n of fields)
        if (active.answers[n.id] === "specified") {
          const d = $(n.id + "-value");
          if (!d.value || !d.validity.valid)
            return fail(
              t(
                n.dataset.label + "を確認してください。",
                "Check the date or duration.",
              ),
              d,
            );
        }
      if (
        (Object.values(active.answers).some(Boolean) ||
          (active.candidate && active.values["candidate-parameters"].trim())) &&
        (!$("actor").value.trim() ||
          !$("asserted-at").value ||
          !$("asserted-at").validity.valid)
      )
        return fail(
          t(
            "入力者の別名と申告日時を入力してください。",
            "Enter an actor alias and assertion time.",
          ),
          !$("actor").value.trim() ? $("actor") : $("asserted-at"),
        );
      if (
        $("actor").value &&
        !/^[a-z][a-z0-9._-]{0,63}$/.test($("actor").value)
      )
        return fail(
          t(
            "入力者の別名は英小文字で始め、英小文字・数字・.・_・-で入力してください。",
            "Actor aliases must start with a lowercase letter and contain only lowercase letters, digits, dots, underscores or hyphens.",
          ),
          $("actor"),
        );
      if (
        $("valid-until").value &&
        (!$("valid-until").validity.valid ||
          ($("asserted-at").value &&
            $("valid-until").value <= $("asserted-at").value))
      )
        return fail(
          t(
            "有効期限は申告日時より後にしてください。",
            "Expiry must be after assertion time.",
          ),
          $("valid-until"),
        );
      return true;
    }
    function saveDraft(proceed = true) {
      if (scopePending || loading) return false;
      resetConfirmation();
      if (!validateDraft()) return false;
      const snap = read(),
        key = draftKey(),
        wasNew = mode === "new";
      if (mode === "new") store.saveNew([...selected], snap);
      else if (mode === "group") store.saveCommon(groupId, snap);
      else store.saveIndividual(groupId, individualId, snap);
      drafts.delete(key);
      if (mode === "new") {
        selected.clear();
        drafts.delete("new");
      }
      switchEditor("new", null, null, false);
      scopeKeys.clear();
      scopeData = null;
      if (proceed) generationProgress({phase: "preview"});
      const pending = refreshScope(!wasNew), sequence = scopeSequence,
        v = version, revision = editorRevision, navigation = navigationSequence;
      pending.then(async applied => {
        if (proceed && applied && sequence === scopeSequence && v === version &&
            revision === editorRevision && navigation === navigationSequence && mode === "new")
          await reviewRequest(false);
      }).catch(reportError).finally(() => {
        if (proceed && !loading) { if (progressPanel.open) progressPanel.close(); progressPanel.hidden = true; }
      });
      return true;
    }
    async function showReview(without) {
      await reviewRequest(without);
    }
    form.addEventListener("submit", (e) => {
      e.preventDefault();
      saveDraft();
    });
    form.addEventListener("input", (e) => {
      resetConfirmation();
      if (e.target.id === "candidate-custom") {
        syncParameters();
        updateQuestions();
      }
      if (["lead-years", "lead-months", "lead-anchor"].includes(e.target.id)) updateDuration();
      if (e.target.closest("#draft-editor")) stash();
    });
    form.addEventListener("change", (e) => {
      resetConfirmation();
      chooseSchedule(e.target.id);
      if (e.target.id === "use-case") updateCandidate();
      if (e.target.id === "candidate") rebuildParameters();
      if (e.target.id === "parameter-choice") syncParameters();
      if (e.target.id?.startsWith("support-")) {
        const active = hasActiveKnownSupport();
        if (!hasKnownSupport()) supportBasis = "";
        else if (active && !supportHadKnown) supportBasis = basisKey();
      }
      updateQuestions();
      if (e.target.closest("#draft-editor")) stash();
    });
    $("assertion-now").addEventListener("click", () => {
      $("asserted-at").value = japanNow();
      resetConfirmation();
      stash();
    });
    $("reset-support").addEventListener("click", () => {
      for (const n of supportFields) n.value = "";
      supportBasis = "";
      supportHadKnown = false;
      updateQuestions();
      resetConfirmation();
      stash();
    });
    $("as-of").addEventListener("input", resetConfirmation);
    $("restore-common").addEventListener("click", () => {
      store.restore(groupId, individualId);
      drafts.delete(draftKey());
      load(currentGroup().common);
      render();
    });
    let searchTimer;
    $("target-search").addEventListener("input", () => {
      clearTimeout(searchTimer);
      searchTimer = setTimeout(() => loadPage(0).catch(reportError), 200);
    });
    $("role-filter").addEventListener("change", () =>
      loadPage(0).catch(reportError),
    );
    $("target-search").addEventListener("keydown", (e) => {
      if (e.key === "Enter") e.preventDefault();
    });
    $("targets-prev").addEventListener("click", () =>
      loadPage(page - 1).catch(reportError),
    );
    $("targets-next").addEventListener("click", () =>
      loadPage(page + 1).catch(reportError),
    );
    $("select-page").addEventListener("change", () => {
      cancelScope();
      const available = visibleTargets().filter(
        (target) => !owner(target.id) && target.selectable,
      );
      const added = available.filter((target) => !selected.has(target.id));
      if (
        $("select-page").checked &&
        store.count() + selected.size + added.length > 4096
      ) {
        renderTable();
        fail(
          t(
            "入力できる合計上限を超えています。",
            "The total scope limit is exceeded.",
          ),
          $("target-search"),
        );
        return;
      }
      for (const target of available)
        $("select-page").checked
          ? selected.add(target.id)
          : selected.delete(target.id);
      scopeKeys.clear();
      scopeData = null;
      for (const box of cohortRows.querySelectorAll("input"))
        box.checked = false;
      resetConfirmation();
      render();
    });
    $("clear-selection").addEventListener("click", () => {
      cancelScope();
      selected.clear();
      scopeKeys.clear();
      scopeData = null;
      drafts.delete("new");
      load(null);
      renderCohorts();
      render();
    });
    $("new-group").addEventListener("click", () => {
      selected.clear();
      scopeKeys.clear();
      scopeData = null;
      switchEditor("new");
      renderCohorts();
    });
    $("review-all").addEventListener("click", () => showReview(false));
    $("skip-context").addEventListener("click", (e) => {
      e.preventDefault();
      showReview(true);
    });
    $("edit-context").addEventListener("click", () => {
      review.hidden = true;
      editor.hidden = false;
      $("skip-context").hidden = false;
      resetConfirmation();
      $("target-search").focus();
    });
    $("context-reviewed").addEventListener("change", () => {
      $("show-migration-report").disabled = !$("context-reviewed").checked;
    });
    function invalidate() {
      if (!controller) $("cancel-review").hidden = true;
      preview = null;
      if (mountedReport) {
        mountedReport.destroy();
        mountedReport = null;
      }
      $("context-reviewed").checked = false;
      $("show-migration-report").disabled = true;
      if (downloadUrl) {
        URL.revokeObjectURL(downloadUrl);
        downloadUrl = null;
      }
      $("download-context").hidden = true;
    }
    function message(ja, en, error = false) {
      const n = $("migration-status");
      n.dataset.ja = ja;
      n.dataset.en = en;
      n.textContent = t(ja, en);
      n.classList.toggle("error", error);
      n.setAttribute("role", error ? "alert" : "status");
      if (error) n.focus();
    }
    function reportError(error) {
      if (error.name === "AbortError") return;
      if (
        [
          "generation_failed",
          "report_unavailable",
          "generation_timeout",
        ].includes(error.message)
      ) {
        message(
          "レポートを生成できませんでした。保存先と接続を確認して再試行してください。",
          "Report generation failed. Check the destination and connection, then retry.",
          true,
        );
        return;
      }
      if (error.message === "resource_limit") {
        message(
          "対象数またはデータ量が上限を超えています。範囲を絞って再試行してください。",
          "The target or data limit was exceeded. Narrow the scope and retry.",
          true,
        );
        return;
      }
      message(
        "入力を適用できません。対象、日時、値と保存ファイルの形式を確認してください。",
        "Cannot apply the input. Check targets, dates, values and saved-file format.",
        true,
      );
    }
    function setBusy(value) {
      loading = value;
      root.setAttribute("aria-busy", String(value));
      for (const b of root.querySelectorAll("button,input,select")) {
        if (value) {
          b.dataset.wasDisabled = String(b.disabled);
          b.disabled = true;
        } else if (b.dataset.wasDisabled !== undefined) {
          b.disabled = b.dataset.wasDisabled === "true";
          delete b.dataset.wasDisabled;
        }
      }
      if (value && controller) $("cancel-review").disabled = false;
      if (!value) {
        renderTable();
        $("show-migration-report").disabled =
          !preview || !$("context-reviewed").checked;
      }
    }
    async function api(route, body) {
      const response = await fetch(
        route,
        body === undefined
          ? {}
          : {
              method: "POST",
              headers: { "Content-Type": "application/json" },
              body: JSON.stringify(body),
              ...(route.startsWith("api/migration/preview")
                ? {
                    headers: {
                      "Content-Type": "application/json",
                      "X-Pqc-Compact-Preview": "1",
                    },
                  }
                : {}),
              signal: controller?.signal,
            },
      );
      const data = await response.json();
      if (!response.ok) throw new Error(data.error || "invalid_request");
      return data;
    }
    async function loadPage(next) {
      const sequence = ++requestSequence,
        v = version;
      loading = true;
      renderTable();
      try {
        const data = await api(
          `api/migration/targets?job=${job.id}&page=${next}&limit=20&search=${encodeURIComponent($("target-search").value)}&role=${encodeURIComponent($("role-filter").value)}`,
        );
        if (sequence !== requestSequence || v !== version) return;
        if (input && input.sha256 !== data.input.sha256) {
          groups.splice(0);
          selected.clear();
          drafts.clear();
          targetById.clear();
          load(null);
          invalidate();
          throw new Error("input_binding_mismatch");
        }
        input = data.input;
        total = data.total;
        allTotal = data.all_total;
        page = data.page;
        targets = data.targets.map((r) => ({
          ...r,
          rawPath: r.path,
          boundId: r.id,
          id: String(r.number),
          path:
            (r.path || t("保存済みの記録", "Saved record")) +
            (r.line ? ":" + r.line : ""),
        }));
        for (const row of targets) targetById.set(row.id, row);
        roleLabels = data.role_labels;
        renderRoleFilter();
        render();
      } finally {
        if (sequence === requestSequence) {
          loading = false;
          renderTable();
        }
      }
    }
    function renderRoleFilter() {
      const value = $("role-filter").value;
      $("role-filter").replaceChildren(
        new Option(t("すべての役割", "All roles"), ""),
      );
      for (const [role, labels] of Object.entries(roleLabels))
        $("role-filter").append(new Option(labels[language], role));
      $("role-filter").value = value;
    }
    function renderSource() {
      for (const n of root.querySelectorAll("[data-current-folder]"))
        n.textContent =
          job?.display_name || (job?.kind === "saved"
            ? t("保存したスキャン結果", "Saved scan results")
            : job?.display_path || job?.path || "/");
    }
    loadJob = async (next) => {
      if (loading) return;
      if (job?.id === next.id && input) {
        stage(stageView);
        root.focus();
        return;
      }
      version++;
      requestSequence++;
      job = next;
      input = null;
      selected.clear();
      groups.splice(0);
      store.serial = 0;
      targetById.clear();
      drafts.clear();
      targets = [];
      mode = "new";
      groupId = null;
      individualId = null;
      imported = null;
      scopeKeys.clear();
      scopeData = null;
      scopeSequence++;
      mountedReport?.destroy();
      mountedReport = null;
      $("context-mode").value = "generate";
      root.querySelector('input[name="context-input-method"][value="generate"]').checked = true;
      removedUses = [];
      moduleSelect.value = "";
      $("context-file").value = "";
      $("context-import").hidden = true;
      $("generated-editor").hidden = false;
      $("migration-editor").hidden = false;
      review.hidden = true;
      $("skip-context").hidden = false;
      $("target-search").value = "";
      $("role-filter").value = "";
      $("as-of").value = japanNow(true);
      renderSource();
      load(null);
      generationProgress({phase:"loading"});
      try {
        summary = await api(`api/migration/overview?job=${job.id}`);
        await loadPage(0);
        renderOverview();
        await refreshScope();
        stage("overview");
        localize();
        root.focus();
      } catch (e) {
        reportError(e);
      } finally {
        if (progressPanel.open) progressPanel.close();
        progressPanel.hidden = true;
      }
    };
    const fieldNames = {
      "use-case": "use_case",
      "usage-state": "usage_state",
      "business-critical": "business_critical",
      "confidentiality-until": "confidentiality_until",
      "historical-ciphertext": "historical_ciphertext",
      "signature-until": "signature_verification_until",
      "lead-time": "migration_lead_seconds",
      "goal-month": "migration_goal_month",
    };
    function serializeSnapshot(snapshot, ids, systemId) {
      const required = new Set();
      for (const id of ids)
        for (const name of targetById.get(id)?.support_fields?.[
          snapshot.answers["use-case"] || "unspecified"
        ] || [])
          required.add(name);
      const active = PqcContext.active(
        snapshot,
        fields.map((n) => ({
          id: n.id,
          relevance: n.closest("[data-relevant]")?.dataset.relevant,
        })),
        [...required],
      );
      const answers = [], references = [];
      const deadlineRef = `deadline-${systemId}-${ids[0]}`;
      for (const [id, v] of Object.entries(active.answers)) {
        if (!v) continue;
        const field = id.startsWith("support-")
          ? "support." + id.slice(8)
          : fieldNames[id] || id;
        if (v === "?") {
          answers.push({ field, state: "unknown" });
          continue;
        }
        let value = id === "deadline" && v === "none" ? { kind: "none" } : v;
        if (v === "true" || v === "false") value = v === "true";
        if (v === "specified") {
          const raw = active.details[id];
          if (id === "lead-time") {
            value = Number(raw);
            if (
              raw === "" ||
              !Number.isSafeInteger(value) ||
              value < 0 ||
              value > 315576000
            )
              throw new Error("invalid_duration");
          } else if (id === "goal-month") value = raw;
          else if (id === "deadline") {
            value = { kind: "at", at: PqcContext.timestamp(raw), reference_id: deadlineRef, scope_reason: active.values["deadline-reason"] };
            references.push({ id: deadlineRef, issuer_id: active.values["deadline-issuer"], document_id: active.values["deadline-document"], edition: active.values["deadline-edition"], clause: active.values["deadline-clause"], scope_id: systemId });
          } else value = PqcContext.timestamp(raw);
        }
        answers.push({ field, state: "known", value });
      }
      if (active.candidate && active.values["candidate-parameters"].trim())
        answers.push({
          field: "migration_target",
          state: "known",
          value: {
            use_case: active.answers["use-case"],
            algorithm: active.candidate,
            parameters: active.values["candidate-parameters"].trim(),
          },
        });
      return {
        system_id: systemId,
        references,
        selected: ids.map(Number),
        answers,
        ...(answers.length
          ? {
              actor: active.values.actor,
              asserted_at: PqcContext.timestamp(active.values["asserted-at"]),
              ...(active.values["valid-until"]
                ? {
                    valid_until: PqcContext.timestamp(
                      active.values["valid-until"],
                    ),
                  }
                : {}),
            }
          : {}),
      };
    }
    function requestBody(without) {
      const common = {
        input_sha256: input.sha256,
        as_of: PqcContext.timestamp($("as-of").value),
      };
      if (without) return { mode: "without_context", ...common };
      if ($("context-mode").value === "import_context") {
        if (!imported) throw new Error("missing_context");
        return { mode: "import_context", ...common, context: imported };
      }
      const parts = [];
      for (const group of groups) {
        const common = group.ids.filter((id) => !group.exceptions.has(id));
        if (common.length)
          parts.push(
            serializeSnapshot(group.common, common, "system-" + group.id),
          );
        for (const [id, snapshot] of group.exceptions)
          parts.push(serializeSnapshot(snapshot, [id], "system-" + group.id));
      }
      return {
        mode: "generate_groups",
        ...common,
        organization_id: "organization",
        references: parts.flatMap(p => p.references),
        groups: parts.map(({ references, ...part }) => part),
      };
    }
    function claimLabel(key) {
      if (key === "migration_target")
        return t("検討する方式・設定", "Candidate algorithm / configuration");
      const id =
        Object.keys(fieldNames).find((k) => fieldNames[k] === key) ||
        key.replace("support.", "support-");
      const control = $(id);
      return (
        control?.closest(".field")?.querySelector("span")?.textContent || key
      );
    }
    function formatDate(value) {
      return value
        ? new Date(value).toLocaleString(
            language === "ja" ? "ja-JP" : "en-GB",
            { timeZone: "Asia/Tokyo", hour12: false },
          ) + t("（日本時間）", " (Japan time)")
        : t("有効期限なし", "No expiry supplied");
    }
    function claimValue(claim, key) {
      if (claim.state === "unknown") return t("分からない", "Unknown");
      const value = claim.value;
      if (value && typeof value === "object") {
        if (value.algorithm) return `${value.algorithm} / ${value.parameters}`;
        if (value.kind === "none") return t("期限指定なし", "No deadline");
        if (value.kind === "at")
          return `${formatDate(value.at)} · ${value.reference_id} · ${value.scope_reason === "contractual_requirement" ? t("契約上の要件", "Contractual requirement") : t("組織の方針", "Organization policy")}`;
        return JSON.stringify(value);
      }
      if (key === "migration_lead_seconds")
        return value % 86400 === 0
          ? t(`${value / 86400}日`, `${value / 86400} days`)
          : t(`${value}秒`, `${value} seconds`);
      if (typeof value === "boolean") {
        const id = key.startsWith("support.")
          ? key.replace("support.", "support-")
          : Object.keys(fieldNames).find((k) => fieldNames[k] === key) || key;
        return (
          [...($(id)?.options || [])].find((o) => o.value === String(value))
            ?.textContent || t(value ? "はい" : "いいえ", value ? "Yes" : "No")
        );
      }
      if (
        ["confidentiality_until", "signature_verification_until"].includes(key)
      )
        return formatDate(value);
      const match = fields
        .flatMap((n) => [...n.options])
        .find((n) => n.value === value);
      return match?.textContent || String(value);
    }
    function renderCanonical(data) {
      const area = $("review-answers");
      area.replaceChildren();
      $("review-target").textContent = String(data.scope.length);
      $("review-as-of").textContent = formatDate(data.as_of);
      if (data.context === null) {
        $("review-warning").hidden = true;
        area.append(
          textNode("p", t("追加情報なし", "Without additional context")),
        );
        return;
      }
      const context = JSON.parse(data.context),
        scope = document.createElement("details");
      scope.className = "review-scope";
      scope.append(
        textNode("summary", t("適用先を確認", "Review exact targets")),
      );
      for (const row of data.scope)
        scope.append(
          textNode(
            "p",
            `${row.number}. ${row.algorithm} · ${row.path || row.role}${row.line ? ":" + row.line : ""}`,
            "small mono",
          ),
        );
      area.append(scope);
      const table = document.createElement("table"),
        head = document.createElement("tr");
      for (const title of [
        t("項目・範囲", "Field and scope"),
        t("入力内容", "Declaration"),
        t("入力者・日時・期限", "Provenance and dates"),
      ])
        head.append(textNode("th", title));
      table.append(head);
      function rows(set, scope, prefix = "") {
        for (const [key, c] of Object.entries(set || {})) {
          if (key === "support") {
            rows(c, scope, "support.");
            continue;
          }
          const row = document.createElement("tr");
          row.append(
            textNode("td", scope + " / " + claimLabel(prefix + key)),
            textNode("td", claimValue(c, prefix + key)),
            textNode(
              "td",
              `${c.provenance.source_kind === "external_review_claim" ? t("外部からの申告（未検証）", "External claim (unverified)") : t("利用者の申告（未検証）", "Operator claim (unverified)")} · ${c.provenance.actor_id}\n${formatDate(c.provenance.asserted_at)}\n${formatDate(c.provenance.valid_until)}${c.provenance.reference_id ? " · " + c.provenance.reference_id : ""}`,
            ),
          );
          table.append(row);
        }
      }
      rows(
        context.organization_defaults,
        t("組織の既定値", "Organization defaults"),
      );
      for (const system of context.systems) rows(system.defaults, system.id);
      const shared = new Map();
      for (const binding of context.bindings) {
        const key = JSON.stringify([binding.system_id, binding.claims]);
        if (!shared.has(key))
          shared.set(key, {
            claims: binding.claims,
            system: binding.system_id,
            numbers: [],
          });
        shared
          .get(key)
          .numbers.push(
            data.scope.find((r) => r.id === binding.target_id)?.number ||
              binding.target_id,
          );
      }
      for (const group of shared.values()) {
        const heading = document.createElement("tr"),
          cell = document.createElement("th");
        cell.colSpan = 3;
        const detail = document.createElement("details");
        detail.append(
          textNode(
            "summary",
            t(
              `${group.numbers.length}件に適用`,
              `Applies to ${group.numbers.length} targets`,
            ),
          ),
          textNode("p", group.numbers.join(", ")),
        );
        cell.append(detail);
        heading.append(cell);
        table.append(heading);
        rows(group.claims, group.system);
      }
      const scroll = document.createElement("div");
      scroll.className = "review-table-scroll";
      scroll.append(table);
      area.append(scroll);
      if (context.references?.length) {
        area.append(
          textNode(
            "h3",
            t(
              "申告に付随する資料参照（未検証）",
              "Declared references (unverified)",
            ),
          ),
        );
        for (const r of context.references)
          area.append(
            textNode(
              "p",
              [
                r.id,
                r.issuer_id,
                r.document_id,
                r.edition,
                r.clause,
                r.scope_id,
                r.https_uri,
              ]
                .filter(Boolean)
                .join(" · "),
            ),
          );
      }
      const warnings = [];
      if (drafts.size)
        warnings.push(
          t("未保存の入力は含めていません。", "Unsaved drafts are excluded."),
        );
      if (data.purpose_conflicting)
        warnings.push(
          t(
            "用途と技術情報に矛盾があります。",
            "Declared purpose conflicts with technical facts.",
          ),
        );
      const freshness = {
        active: t("申告期間内", "Within declaration interval"),
        stale: t("有効期限切れ", "Expired"),
        not_yet_effective: t("申告日時より前", "Before assertion time"),
        unknown: t("有効性が不明", "Freshness unknown"),
      };
      if (data.resolved?.length) {
        const details = document.createElement("details");
        details.append(
          textNode("summary", t("申告の有効性", "Declaration freshness")),
        );
        for (const row of data.resolved)
          details.append(
            textNode(
              "p",
              `${row.target_number}. ${claimLabel(row.field)} · ${freshness[row.freshness] || row.freshness}`,
            ),
          );
        area.append(details);
      }
      $("review-warning").textContent = warnings.join(" ");
      $("review-warning").hidden = !warnings.length;
    }
    async function reviewRequest(without) {
      if (loading || !input || scopePending) return;
      message("", "");
      resetConfirmation();
      stash();
      if (!$("as-of").value)
        return fail(
          t("評価日時を入力してください。", "Enter an evaluation time."),
          $("as-of"),
        );
      if (!without && $("context-mode").value === "generate" && !groups.length)
        return fail(
          t(
            "用途を入力するか、入力せずに進んでください。",
            "Enter a use or continue without context.",
          ),
          cohortTitle,
        );
      const v = version;
      controller = new AbortController();
      setBusy(true);
      generationProgress({ phase: "preview" });
      try {
        preview = await api(
          `api/migration/preview?job=${job.id}`,
          requestBody(without),
        );
        if (v !== version) return;
        await generateReport();
      } catch (error) {
        if (v === version) {
          invalidate();
          reportError(error);
          stageView = "input";
          overviewPanel.hidden = true;
          contextStage.hidden = false;
          reportPanel.hidden = true;
        }
      } finally {
        if (v === version) {
          controller = null;
          setBusy(false);
          if (progressPanel.open) progressPanel.close();
      progressPanel.hidden = true;
        }
      }
    }
    $("cancel-review").addEventListener("click", async () => {
      const token = preview?.token;
      if (controller) {
        version++;
        controller.abort();
        controller = null;
        setBusy(false);
      }
      invalidate();
      review.hidden = true;
      editor.hidden = false;
      $("skip-context").hidden = false;
      $("context-mode").disabled = false;
      $("context-import").hidden = $("context-mode").value !== "import_context";
      $("cancel-review").hidden = true;
      if (token)
        try {
          await api(`api/migration/cancel?job=${job.id}`, { token });
        } catch (e) {
          reportError(e);
          return;
        }
      message("入力の確認を取り消しました。", "Context review cancelled.");
    });
    $("migration-back").addEventListener("click", (e) => {
      e.preventDefault();
      if (loading) return;
      window.dispatchEvent(new CustomEvent("pqc-migration-back"));
    });
    for (const radio of root.querySelectorAll('input[name="context-input-method"]')) radio.addEventListener("change", () => {
      $("context-mode").value = radio.value;
      $("context-mode").dispatchEvent(new Event("change"));
    });
    $("context-mode").addEventListener("change", () => {
      invalidate();
      const importing = $("context-mode").value === "import_context";
      cancelScope();
      $("context-import").hidden = !importing;
      $("generated-editor").hidden = importing;
    });
    $("context-file").addEventListener("change", async () => {
      invalidate();
      imported = null;
      const file = $("context-file").files[0],
        v = version;
      if (!file) return;
      if (file.size > 1024 * 1024) {
        reportError(new Error("resource_limit"));
        return;
      }
      try {
        const bytes = await file.arrayBuffer();
        if (v !== version || file !== $("context-file").files[0]) return;
        imported = new TextDecoder("utf-8", {
          fatal: true,
          ignoreBOM: true,
        }).decode(bytes);
      } catch (e) {
        reportError(e);
      }
    });
    $("import-review").addEventListener("click", () => reviewRequest(false));
    $("edit-context").addEventListener("click", () => {
      $("context-mode").disabled = false;
      $("context-import").hidden = $("context-mode").value !== "import_context";
    });
    localize = () => {
      progressPanel.setAttribute("aria-label", t("確認事項を準備しています", "Preparing review items"));
      closeRecords.setAttribute("aria-label", t("閉じる", "Close"));
      generationBar.setAttribute(
        "aria-label",
        t("レポート生成", "Report generation"),
      );
      for (const n of root.querySelectorAll("[data-en]"))
        n.textContent = n.dataset[language];
      for (const attr of ["aria-label", "placeholder"])
        for (const n of root.querySelectorAll(`[data-${attr}-en]`))
          n.setAttribute(attr, n.getAttribute(`data-${attr}-${language}`));
      if (input) {
        for (const row of targetById.values())
          if (!row.rawPath)
            row.path =
              t("保存済みの記録", "Saved record") +
              (row.line ? ":" + row.line : "");
        updateCandidate($("candidate").value);
        renderRoleFilter();
        renderSource();
        render();
        updateQuestions();
        renderOverview();
        renderCohorts();
        mountedReport?.setLanguage(language);
        if (!loading) stage(stageView);
      }
    };
    localize();
    load(null);
    render();
    contextStage.hidden = true;
    overviewPanel.hidden = false;
  }
})();
