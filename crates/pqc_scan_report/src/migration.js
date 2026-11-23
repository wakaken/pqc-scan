/* Human views of immutable saved decisions. No policy evaluation in JavaScript. */
(() => {
  function mount(payload, initialView, standalone = false, artifactBase = "", hasScanReport = false) {
    const standaloneExport = standalone && document.documentElement?.dataset?.savedStandalone === "true";
    const abort = new AbortController(),
      events = { signal: abort.signal };
    const nodes = payload.archive.nodes,
      rootRef = payload.archive.root;
    const h = (v) =>
      String(v ?? "")
        .replace(
          /[\u0000-\u001f\u007f-\u009f]/g,
          (c) => `\\u{${c.charCodeAt(0).toString(16)}}`,
        )
        .replace(
          /[&<>"']/g,
          (c) =>
            ({
              "&": "&amp;",
              "<": "&lt;",
              ">": "&gt;",
              '"': "&quot;",
              "'": "&#39;",
            })[c],
        );
    function value(ref) {
      const n = nodes[ref];
      if (n[0] === "o") {
        const o = Object.create(null);
        for (let i = 0; i < n[1].length; i += 2)
          o[value(n[1][i])] = value(n[1][i + 1]);
        return o;
      }
      if (n[0] === "a") return n[1].map(value);
      return n[0] === "z" ? null : n[1];
    }
    function child(ref, key) {
      const edges = nodes[ref][1];
      for (let i = 0; i < edges.length; i += 2)
        if (nodes[edges[i]][1] === key) return edges[i + 1];
      return null;
    }
    const refs = (key) => nodes[child(rootRef, key)]?.[1] || [];
    const decisions = refs("decisions"),
      targets = refs("targets"),
      claims = refs("claims"),
      sources = refs("sources"),
      candidates = refs("candidates");
    const byId = (list) =>
      new Map(
        list.map((ref, i) => [value(child(ref, "id")), { ref, index: i }]),
      );
    const targetById = byId(targets),
      claimById = byId(claims),
      candidateById = byId(candidates);
    const field = (key) => value(child(rootRef, key));
    // Build report-only workflows from decisions with the same applied context.
    // Keep the archive and the original evidence/decision indices intact.
    const scopeClaims = new Map();
    for (const [id, {ref}] of claimById) {
      const claim = value(ref);
      if (claim.state === "known" && claim.freshness === "active") {
        scopeClaims.set(id, {field: claim.field, value: JSON.stringify(claim.value)});
      }
    }
    const supportedUses = new Set(["jwt", "signature", "pki", "key_establishment", "encryption", "hashing"]);
    const phaseActions = new Set(["plan", "unblock", "migrate", "monitor"]);
    const candidateConditions = new Map();
    function candidateCondition(id) {
      if (!candidateConditions.has(id)) {
        const c = value(candidateById.get(id).ref);
        candidateConditions.set(id, [c.identifier, c.purpose, c.status, c.parameter_selection,
          c.prerequisites.map(p => [p.name, p.claimed_support]), c.validation_steps]);
      }
      return candidateConditions.get(id);
    }
    function scopeFor(d) {
      if (!d.system_id || d.information === "conflicting") return null;
      const fields = new Map();
      for (const id of d.applied_claim_ids) {
        const claim = scopeClaims.get(id);
        if (!claim) continue;
        if (fields.has(claim.field) && fields.get(claim.field) !== claim.value) return null;
        fields.set(claim.field, claim.value);
      }
      const use = JSON.parse(fields.get("use_case") ?? "null");
      const usage = JSON.parse(fields.get("usage_state") ?? "null");
      const kind = d.action.kind;
      const excluded = kind === "none" && d.urgency === "not_applicable" &&
        (fields.get("applicability") === '"not_applicable"' || ["unused", "test_only"].includes(usage));
      if (!excluded && !(phaseActions.has(kind) && supportedUses.has(use) && ["active", "planned"].includes(usage))) return null;
      const candidates = d.action.candidate_ids.map(candidateCondition);
      // An explicit exclusion is the same decision for a known algorithm and
      // an unidentified reference. Preserve both reason sets in the evidence.
      const reasons = excluded ? d.reason_codes.filter(code =>
        !["reference_only", "unknown_primitive", "unknown_parameter_encoding"].includes(code)) : d.reason_codes;
      const key = JSON.stringify([d.system_id, kind, d.urgency, d.information, d.readiness,
        reasons, d.missing_inputs, d.action.blocking_prerequisites, d.action.validation_steps, candidates,
        [...fields].sort(([a], [b]) => a.localeCompare(b, "en"))]);
      const constraints = Object.fromEntries(["signature_verification_until", "confidentiality_until", "historical_ciphertext"]
        .filter(field => fields.has(field)).map(field => [field, JSON.parse(fields.get(field))]));
      return {key, use, kind, constraints};
    }
    const groups = payload.groups.map(() => ({members: []}));
    const aliases = new Map(), scopeIndices = new Map(), scopes = new Map();
    for (const [original, group] of payload.groups.entries()) {
      const partitions = new Map();
      for (const member of group.members) {
        const d = decisions[member];
        // Most inventory records have no binding and cannot supply a workflow.
        const system = child(d, "system_id");
        const scope = system !== null && value(system) ? scopeFor(value(d)) : null;
        const key = scope?.key ?? null;
        if (!partitions.has(key)) partitions.set(key, {scope, members: []});
        partitions.get(key).members.push(member);
      }
      for (const [part, {scope, members}] of [...partitions.values()].entries()) {
        const index = part === 0 ? original : groups.length;
        if (part > 0) groups.push({members: []});
        const primary = scope ? scopeIndices.get(scope.key) ?? index : index;
        if (scope && primary === index) {
          scopeIndices.set(scope.key, index);
          scopes.set(index, scope);
        }
        if (primary !== index) aliases.set(index, primary);
        for (const member of members) groups[primary].members.push(member);
      }
    }
    const scanReference = (id) => Object.hasOwn(payload.scanReferences || {}, id)
      ? payload.scanReferences[id] : undefined;
    const scanRule = (source) => scanReference(source.observation_id)?.rule;
    const importRule = (rule) => rule === "GO_TLS_RSA_IMPORT";
    const orderLocation = (a, b) => a.localeCompare(b, "en", { numeric: true });
    const unbindable = new Set(payload.unbindable || []);
    const relatedSourceRules = new Map();
    const allDeclarations = new Map();
    const groupAlgorithms = [],
      groupFacts = [];
    for (const group of groups) {
      const names = new Set(),
        purposes = new Map(), roles = new Set(), reasons = new Set(),
        paths = new Set(), dependencyPaths = new Set(), referencePaths = new Set(), savedPaths = new Set(), sourceFiles = new Set(), rules = new Set(), declarations = new Map(), planning = new Map();
      let developmentDeclarations = 0, dependencyRecords = 0, sourceRecords = 0, unbindableRecords = 0, libraryReferences = 0;
      for (const i of group.members) {
        const targetId = value(child(decisions[i], "target_id")),
          ref = targetById.get(targetId).ref,
          tech = child(ref, "technical"),
          source = child(ref, "source_ref"),
          name = value(child(tech, "identifier"));
        names.add(name);
        const purpose = value(child(tech, "purpose"));
        purposes.set(purpose, (purposes.get(purpose) || 0) + 1);
        roles.add(value(child(source, "role")));
        for (const item of value(child(decisions[i], "reason_codes"))) reasons.add(item);
        if (unbindable.has(targetId)) unbindableRecords++;
        const pathRef = child(source, "path");
        const sourcePath = pathRef === null ? "" : value(pathRef);
        const provenance = child(source, "dependency_provenance");
        const methodRef = child(child(ref, "observation"), "method");
        const method = methodRef === null ? "" : value(methodRef);
        const isDependency = (provenance !== null && value(provenance) !== null) ||
          method === "dependency_report" ||
          /(?:^|[/\\])(?:package(?:-lock)?\.json|pnpm-lock\.yaml|yarn\.lock|pom\.xml|Cargo\.(?:toml|lock)|go\.(?:mod|sum)|Gemfile(?:\.lock)?|requirements\.txt|pyproject\.toml)$/.test(sourcePath);
        const isSource = !isDependency && sourcePath && ["ast_pattern", "lexical_match", "configuration_match"].includes(method);
        if (isDependency) dependencyRecords++;
        if (isSource) {
          sourceRecords++;
          sourceFiles.add(sourcePath);
        }
        const observationId = child(source, "observation_id");
        const rule = observationId === null ? null : scanReference(value(observationId))?.rule;
        if (rule) rules.add(rule);
        if (isSource && importRule(rule)) libraryReferences++;
        const systemRef = child(decisions[i], "system_id");
        const system = systemRef === null ? null : value(systemRef);
        if (system && isSource && rule) {
          const key = JSON.stringify([system, sourcePath]);
          if (!relatedSourceRules.has(key)) relatedSourceRules.set(key, new Set());
          relatedSourceRules.get(key).add(rule);
        }
        if (provenance !== null && value(provenance) === "manifest_dev_declared") developmentDeclarations++;
        for (const key of ["path", "detail_path"]) {
          const id = child(source, key);
          if (id !== null && value(id)) {
            const line = key === "path" ? child(source, "line") : null;
            const lineValue = line !== null ? value(line) : null;
            const location = value(id) + (lineValue ? ":" + lineValue : "");
            paths.add(location);
            (isDependency ? dependencyPaths : isSource ? referencePaths : savedPaths).add(location);
          }
        }
        const seen = new Set(), overviewSeen = new Set(), plan = {};
        for (const id of value(child(decisions[i], "applied_claim_ids")) || []) {
          const claimRef = claimById.get(id)?.ref;
          if (claimRef === undefined) continue;
          const fieldName = value(child(claimRef, "field"));
          if (!["use_case", "usage_state", "migration_target", "deadline", "migration_lead_seconds", "migration_goal_month"].includes(fieldName)) continue;
          const c = value(claimRef);
          if (c.state !== "known" || c.freshness !== "active") continue;
          if (["deadline", "migration_lead_seconds"].includes(fieldName)) plan[fieldName] = c.value;
          const key = JSON.stringify([fieldName, c.value]);
          if (seen.has(key)) continue;
          seen.add(key);
          const entry = declarations.get(key) || { field: fieldName, value: c.value, count: 0 };
          entry.count++;
          declarations.set(key, entry);
          // Count each record once per displayed value. The card and archive
          // preserve the original deadline provenance.
          const overviewValue = fieldName === "deadline"
            ? c.value.kind === "at" ? {kind:"at", at:Date.parse(c.value.at)} : {kind:"none"}
            : c.value;
          const overviewKey = JSON.stringify([fieldName, overviewValue]);
          if (!overviewSeen.has(overviewKey)) {
            const overviewEntry = allDeclarations.get(overviewKey) || {field:fieldName, value:overviewValue, count:0};
            overviewEntry.count++;
            allDeclarations.set(overviewKey, overviewEntry);
            overviewSeen.add(overviewKey);
          }
        }
        if (Object.keys(plan).length) {
          const key = JSON.stringify([plan.deadline || null, plan.migration_lead_seconds ?? null]), entry = planning.get(key) || { ...plan, count: 0 };
          entry.count++; planning.set(key, entry);
        }
      }
      groupAlgorithms.push([...names]);
      groupFacts.push({paths: [...paths], dependencyPaths: [...dependencyPaths], referencePaths: [...referencePaths], savedPaths: [...savedPaths], sourceFiles, rules, roles, purposes, reasons, libraryReferences, libraryReferencesOnly: libraryReferences === group.members.length, sourceRecords, declarations: [...declarations.values()], planning: [...planning.values()], developmentDeclarations, dependencyRecords, unbindableRecords});
    }

    let active,
      page = 0,
      otherExpanded = false,
      expandedCards = new Set(),
      viewer = null,
      detail = null,
      detailPage = 0,
      detailQuery = "",
      recordPage = 0,
      visibleLocations = new Map();
    const size = 20,
      bound = new WeakSet();
    const ja = () => active.lang === "ja",
      t = (en, jp) => (ja() ? jp : en);
    const word = (code) =>
      code === "resolve_declared_blocking_prerequisites"
        ? t("Resolve the declared blocking requirements.", "申告された未対応の条件を解消する。")
        : payload.words[code]?.[ja() ? "ja" : "en"] || code || t("Unentered", "未入力");
    const state = (type, code) =>
      (type === "information" && code === "sufficient" ? t("Required information available", "判断に必要な情報は揃っています") : null) ||
      payload.states[ja() ? "ja" : "en"][`${type}:${code}`] || word(code);
    const n = (v) => Number(v).toLocaleString(ja() ? "ja-JP" : "en-US");
    const algorithm = (s) =>
      s === "unknown" || !s ? t("Unidentified", "未特定") : s;
    const decision = (i) => value(decisions[i]);
    const target = (d) => value(targetById.get(d.target_id).ref);
    const recordLocation = (tgt) =>
      (tgt.source_ref.path ||
        tgt.source_ref.detail_path ||
        word(tgt.source_ref.role)) +
      (tgt.source_ref.line ? ":" + tgt.source_ref.line : "");
    const list = (values, translate = true) => {
      const lines = [...new Set(values.map(v => translate ? word(v) : v))];
      return lines.length ? `<ul>${lines.map(line => `<li>${h(line)}</li>`).join("")}</ul>` : "";
    };
    const section = (title, content, level = 3) =>
      content ? `<h${level}>${h(title)}</h${level}>${content}` : "";
    function serialize(ref) {
      const node = nodes[ref];
      if (node[0] === "o") {
        const pairs = [];
        for (let i = 0; i < node[1].length; i += 2)
          pairs.push(serialize(node[1][i]) + ":" + serialize(node[1][i + 1]));
        return "{" + pairs.join(",") + "}";
      }
      if (node[0] === "a") return "[" + node[1].map(serialize).join(",") + "]";
      if (node[0] === "n") return node[1];
      return node[0] === "z" ? "null" : JSON.stringify(node[1]);
    }
    function download() {
      const url = URL.createObjectURL(
        new Blob([serialize(rootRef)], { type: "application/json" }),
      );
      const a = document.createElement("a");
      a.href = url;
      a.download = "migration.assessment.v1.json";
      a.click();
      setTimeout(() => URL.revokeObjectURL(url), 10000);
    }
    function dateText(raw) {
      const date = new Date(raw);
      return Number.isNaN(date.getTime()) ? String(raw) : new Intl.DateTimeFormat(ja() ? "ja-JP" : "en-GB", {
        timeZone: "Asia/Tokyo", year: "numeric", month: "2-digit", day: "2-digit",
        hour: "2-digit", minute: "2-digit", second: "2-digit", hourCycle: "h23",
      }).format(date);
    }
    function durationText(raw) {
      let rest = Number(raw);
      if (!Number.isSafeInteger(rest) || rest < 0) return String(raw);
      const parts = [];
      for (const [seconds, en, jp] of [[86400,"days","日"],[3600,"hours","時間"],[60,"minutes","分"],[1,"seconds","秒"]]) {
        const amount = Math.floor(rest / seconds); rest %= seconds;
        if (amount || seconds === 1 && !parts.length) parts.push(n(amount) + t(" " + (amount === 1 ? en.slice(0, -1) : en), jp));
      }
      return parts.join(t(" ", ""));
    }
    function bindingNotice(count = 1, heading = 3) {
      return `<section class="binding-limit"><h${heading}>${t("Tool binding limitation", "ツール上の対応付け制約")}</h${heading}><p>${t("Records that cannot be bound", "対応付けできない記録")}: ${n(count)}</p><p>${t("Adding answers in this screen cannot resolve this limitation. The scanner could not uniquely identify these records. Inspect the listed source and configuration, and use separately identifiable evidence where available; the detection/binding issue may need investigation.", "この画面で回答を追加しても解消できません。検出された記録を一意に対応付けできないためです。記録された参照箇所・設定を調べ、対応付け可能な別の根拠を確認してください。検出・対応付け処理の調査が必要な場合もあります。")}</p></section>`;
    }
    function planningHtml(i) {
      const rows = groupFacts[i].planning;
      if (!rows.length) return "";
      return `<section class="planning-basis"><h6>${t("Applied planning conditions (declared)", "判定に使った期限・期間（申告）")}</h6>${rows.slice(0,5).map(p => {
        const deadline = p.deadline, lead = p.migration_lead_seconds;
        const parts = [];
        if (deadline?.kind === "none") parts.push(t("No deadline declared", "期限なしと申告"));
        if (deadline?.kind === "at") {
          const label = t("Migration deadline", "移行期限");
          const date = dateText(deadline.at) + " " + t("(Japan time)", "（日本時間）");
          const basis = deadline.scope_reason === "contractual_requirement"
            ? t("Contractual requirement", "契約上の要件") : t("Organization policy", "組織の方針");
          const status = deadlineStatus(epochSecond(deadline.at));
          parts.push([`${label}: ${date}`, basis, deadline.reference_id, status].filter(Boolean).join(" · "));
        }
        if (lead !== undefined) parts.push(`${t("Migration lead time", "移行に必要な期間")}: ${durationText(lead)}`);
        return `<p>${h(parts.join(" / "))} (${n(p.count)} / ${n(groups[i].members.length)} ${t("records", "記録")})</p>`;
      }).join("")}${rows.length > 5 ? `<p>+${n(rows.length-5)} ${t("variants (see evidence)", "種類（根拠で確認）")}</p>` : ""}</section>`;
    }
    function evaluationTime() {
      const raw = field("as_of"), date = new Date(raw);
      const formatted = Number.isNaN(date.getTime()) ? raw : new Intl.DateTimeFormat(ja() ? "ja-JP" : "en-GB", {
        timeZone: "Asia/Tokyo", year: "numeric", month: "2-digit", day: "2-digit",
        hour: "2-digit", minute: "2-digit", second: "2-digit", hourCycle: "h23",
      }).format(date);
      return `<time datetime="${h(raw)}" title="${h(raw)}">${t("Evaluation time", "評価日時")}: ${h(formatted)} ${t("(Japan time, UTC+09:00)", "（日本時間）")}</time>`;
    }
    function declarationHtml(i, fieldName, label) {
      const entries = groupFacts[i].declarations.filter(c => c.field === fieldName);
      if (!entries.length) return "";
      const values = entries.slice(0, 5).map(c => {
        const v = fieldName === "migration_target"
          ? [word(c.value.use_case), c.value.algorithm, c.value.parameters].filter(Boolean).join(" / ")
          : word(c.value);
        return h(v) + ` (${n(c.count)} / ${n(groups[i].members.length)} ${t("records", "記録")})`;
      });
      return `<p class="declared-fact"><strong>${label}:</strong> ${values.join(" · ")}${entries.length > 5 ? ` +${n(entries.length - 5)} ${t("more variants (see evidence)", "種類（根拠で確認）")}` : ""}</p>`;
    }
    function locationsHtml(paths, abbreviated = false) {
      if (!paths.length) return "";
      const shown = [...paths].sort(orderLocation).slice(0, 3);
      const basename = path => path.split(/[/\x5c]/).at(-1);
      const files = shown.map(path => path.replace(/:\d+$/, ""));
      const labels = shown.map((path, i) =>
        abbreviated && !path.startsWith("/") &&
        !files.some(file => file !== files[i] && basename(file) === basename(files[i]))
          ? basename(path) : path);
      const remaining = paths.length - shown.length;
      return `<ul class="check-locations">${shown.map((path, i) =>
        `<li><code title="${h(path)}">${h(labels[i])}</code></li>`).join("")}${remaining ? `<li>${h(t(n(remaining) + " more locations in the list", "ほか" + n(remaining) + "箇所は一覧に掲載"))}</li>` : ""}</ul>`;
    }

    // Display arithmetic on the exact pairs of active declarations already used
    // by the assessment. Both instants use the evaluator's whole-second precision.
    const epochSecond = date => Math.floor(Date.parse(date) / 1000);
    const asOf = epochSecond(field("as_of"));
    const startDates = groupFacts.map(facts => facts.planning.flatMap(p => {
      if (p.deadline?.kind !== "at" || p.migration_lead_seconds === undefined) return [];
      const lead = Number(p.migration_lead_seconds);
      const deadline = epochSecond(p.deadline.at);
      const due = deadline - lead;
      if (!Number.isSafeInteger(lead) || lead < 0 || !Number.isFinite(asOf) ||
          !Number.isFinite(new Date(due * 1000).getTime())) return [];
      return [{ due, deadline, lead, count: p.count }];
    }).sort((a, b) => a.due - b.due || a.deadline - b.deadline));
    const startStatus = due => due < asOf ? t("Start-by date passed at assessment", "評価日時点で着手目安を超過") :
      due === asOf ? t("Start-by date reached at assessment", "評価日時点で着手目安に到来") : "";
    const deadlineStatus = deadline => deadline < asOf ? t("Migration deadline passed at assessment", "評価日時点で移行期限を超過") :
      deadline === asOf ? t("Migration deadline reached at assessment", "評価日時点で移行期限に到来") : "";
    const timestamp = seconds => {
      const label = dateText(seconds * 1000);
      return `<time datetime="${new Date(seconds * 1000).toISOString()}" aria-label="${h(label)}">${h(label)}</time>`;
    };
    function startDue(i, compact = false) {
      const dates = new Map();
      for (const row of startDates[i]) dates.set(row.due, (dates.get(row.due) || 0) + row.count);
      if (!dates.size) return "";
      const total = groups[i].members.length;
      if (compact) {
        const [due, count] = dates.entries().next().value;
        const status = startStatus(due);
        return `<p class="start-chip${due < asOf ? " overdue" : ""}">${t("Start by (JST)", "着手目安（日本時間）")} ${timestamp(due)}${status ? " · " + h(status) : ""}${count < total ? ` · ${n(count)} / ${n(total)} ${t("records", "記録")}` : ""}</p>`;
      }
      return `<div class="start-due"><strong>${t("Start by (from declared dates, Japan time)", "着手期限の目安（申告から逆算・日本時間）")}</strong>${[...dates].slice(0, 3).map(([due, count]) => {
        const status = startStatus(due);
        const coverage = count < total ? ` · ${n(count)} / ${n(total)} ${t("records", "記録")}` : "";
        return `<p>${timestamp(due)}${h(status ? " · " + status : "")}${coverage}</p>`;
      }).join("")}${dates.size > 3 ? `<p>${t("See declarations for the remaining dates.", "残りの期限は申告欄で確認できます。")}</p>` : ""}</div>`;
    }
    function responseChecks(d, facts, names) {
      const uses = new Set(facts.declarations.filter(c => c.field === "use_case").map(c => c.value));
      const jwt = uses.size === 1 && uses.has("jwt") && facts.sourceFiles.size > 0;
      const checks = [];
      const hasRelatedRule = rule => facts.rules.has(rule) || Boolean(d.system_id &&
        [...facts.sourceFiles].some(path => relatedSourceRules.get(JSON.stringify([d.system_id, path]))?.has(rule)));
      if (d.action.kind === "none") {
        checks.push(t("Reassess when a caller starts using this reference to choose a live cryptographic operation, or when its use enters the declared scope.", "呼び出し元がこの参照を使って実処理の暗号方式を選ぶようになったときや、利用が申告対象に入ったときに再評価する。"));
      } else if (d.action.kind === "monitor") {
        checks.push(uses.has("encryption")
          ? t("Compare the effective cipher, mode and key size with defaults and application overrides. Reassess when any of them changes.", "既定値とアプリ側の上書きを照合し、実際に使う方式・モード・鍵長が変わったときに再評価する。")
          : t("Reassess when the selected algorithm, parameters or business use changes.", "実際に選ぶ方式・設定値・業務用途が変わったときに再評価する。"));
      } else if (d.action.kind === "unblock") {
        const libraryCheck = t("Follow the library reference to the code or configuration that uses it. Check whether the deployed application reaches that path.", "ライブラリ名の参照から利用箇所をたどり、配備先でその処理が使われるか確認する。");
        if (facts.libraryReferencesOnly) {
          checks.push(libraryCheck);
        } else if (jwt && facts.sourceFiles.size > 1) {
          checks.push(t("Locate the signer and any JWT header validation in the listed files. Compare the signer's alg with the verifier's accepted values for the same token flow.", "下記のファイルから署名処理とヘッダー検証を探し、同じトークン経路で、署名時の alg と検証側の許可値を照合する。"));
        } else if (jwt && ["API_PY_RSA_MODULE", "TS_PY_RSA_USAGE"].some(hasRelatedRule)) {
          checks.push(t("Trace the configured signing key to the algorithm selection. Distinguish key generation from the signing path actually used.", "設定される署名鍵の種類から方式を選ぶ処理をたどり、鍵生成の補助処理と実際に署名する経路を区別する。"));
        } else if (jwt && names.filter(name => name !== "unknown").length > 1) {
          checks.push(t("Follow these alg references to the signing provider and compare the enabled signing algorithms with the verifier's accepted values.", "方式名の参照から署名プロバイダーの選択をたどり、有効な署名方式と検証側の許可値を照合する。"));
        } else if (jwt) {
          checks.push(t("Trace where the token is sent. Where present, use the signing call or aud as a starting point. Check its accepted alg values and key types against the issuer's configuration.", "トークンの送信先をたどる。署名呼び出しや aud があれば手掛かりにして、受け付ける alg と鍵の種類を発行側の設定と照合する。"));
        }
        if (!facts.libraryReferencesOnly && facts.libraryReferences && facts.paths.length === 1) {
          checks.push(libraryCheck);
        }
        if (d.missing_inputs.includes("technical.identity") || d.missing_inputs.includes("technical.parameters")) {
          const rsa = names.some(name => /^(RSA(?:-|$)|(?:RS|PS)\d)/.test(name));
          checks.push(rsa
            ? t("Check the key actually configured for this operation. For RSA, record the public key size.", "この処理に設定される鍵を確認し、RSA なら公開鍵のビット数を記録する。")
            : t("Check the effective algorithm and parameters in this environment.", "この環境で実際に使う方式とパラメーターを確認する。"));
        }
      }
      return checks.map((text, index) => ({
        text,
        paths: index === 0 && (jwt || facts.libraryReferencesOnly) ? facts.referencePaths : [],
      }));
    }
    const requirementLabels = {
      runtime: ["Runtime", "実行環境"],
      provider: ["Crypto provider", "プロバイダー"],
      parameters: ["Cryptographic parameters", "暗号パラメーター"],
      peer: ["Peer support", "接続先"],
      protocol: ["Protocol", "プロトコル"],
      jose: ["JWT implementation", "JWT実装"],
      verifiers: ["Token verification", "トークン検証"],
      pki: ["Certificate infrastructure", "証明書基盤"],
    };
    const requirementLabel = code => Object.hasOwn(requirementLabels, code) ? t(...requirementLabels[code]) : word(code);
    function nextChecks(d, facts, total) {
      if (facts.unbindableRecords === total) {
        return [{ text: t("Review the listed evidence and look for a record that can be linked to a use.", "参照箇所を調べ、用途を対応付けられる別の記録を探す。"), paths: facts.paths }];
      }
      const missingTechnicalFacts = d.missing_inputs.includes("technical.identity") ||
        d.missing_inputs.includes("technical.parameters");
      if (d.action.kind !== "investigate" || !d.missing_inputs.length) return [];
      if (!missingTechnicalFacts) {
        const checks = [];
        if (d.missing_inputs.includes("technical.collision_sensitive_use")) {
          checks.push({ text: t("Check whether the hash is used where collisions matter, such as signature verification or integrity checks.", "署名の検証や改ざん検知など、衝突耐性が必要な処理でこのハッシュを使うか確認する。"), paths: facts.paths });
        }
        if (d.missing_inputs.includes("observation.selected_use")) {
          checks.push({ text: t("Check the connection record for the settings actually negotiated.", "接続時の記録で、双方が選んだ設定を確認する。"), paths: facts.savedPaths });
        }
        if (d.missing_inputs.includes("technical.encoding")) {
          checks.push({ text: t("Inspect the original artifact for the encoded parameters.", "元の証拠から、符号化されているパラメーターを確認する。"), paths: facts.paths });
        }
        if (d.missing_inputs.some(field => field.startsWith("support."))) {
          checks.push({ text: t("Check support for the proposed configuration with the runtime, provider or peer responsible for it.", "移行候補の設定に対応しているか、該当する実行環境・プロバイダー・接続先で確認する。"), paths: [] });
        }
        return checks;
      }
      const checks = [];
      if (facts.sourceRecords) {
        checks.push({
          title: t("Check the referenced code", "参照箇所から実行時の設定をたどる"),
          text: t("Use these locations to find the active configuration. Check which algorithm and parameters the application selects at runtime.", "以下の箇所から適用される設定を探し、実行時に選ばれる方式と設定値を確認する。"),
          paths: facts.referencePaths,
        });
      }
      if (facts.dependencyRecords) {
        checks.push({
          title: t("Check dependency use", "依存関係の利用箇所を確認する"),
          text: t("Use the enabled build features and call sites to find where the dependency is used. Check the configured algorithm and parameters there.", "ビルドで有効な機能と呼び出し元から利用箇所を探し、そこで指定される暗号方式と設定値を確認する。"),
          paths: facts.dependencyPaths,
        });
      }
      if (facts.sourceRecords + facts.dependencyRecords < total) {
        checks.push({
          title: t("Review the saved evidence", "保存された根拠を確認する"),
          text: t("Check the original evidence for the algorithm and parameters. For a connection, use a record of the actual negotiation.", "元の記録で方式と設定値を確認する。通信の場合は、実際の接続で選ばれた設定を確認する。"),
          paths: facts.savedPaths,
        });
      }
      return checks;
    }
    function workChecks(i, d) {
      return d.action.kind === "investigate"
        ? nextChecks(d, groupFacts[i], groups[i].members.length)
        : responseChecks(d, groupFacts[i], groupAlgorithms[i]);
    }
    function workTitle(i, d, checks) {
      const scope = scopes.get(i);
      if (scope) return scope.use ? word(scope.use) : t("Declared exclusion", "対象外の申告");
      const tg = target(d), facts = groupFacts[i], total = groups[i].members.length;
      const classicalConcern = d.action.kind === "investigate" && d.reason_codes.includes("known_classical_concern");
      let title = state("action", d.action.kind);
      if (classicalConcern) {
        title = t("Investigate the detected cryptographic concern", "検出された暗号方式の懸念を調査する");
      } else if (checks.some(check => check.title)) {
        if (facts.dependencyRecords === total) title = t("Dependency references", "依存関係の参照");
        else if (facts.dependencyRecords) title = t("Dependency references and other evidence", "依存関係の参照と関連する根拠");
        else if (facts.sourceRecords === total && tg.source_ref.role === "reference-only") title = t("Cryptographic references", "暗号に関わる参照箇所");
        else title = t("Cryptographic configuration", "暗号方式・設定の確認");
      }
      const algorithms = groupAlgorithms[i].filter(name => name !== "unknown").slice(0, 3).join(" / ");
      const declaration = facts.declarations.find(c => c.field === "use_case" && c.count === total);
      const purpose = tg.technical.purpose === "unknown"
        ? declaration ? word(declaration.value) + t(" (declared)", "（申告）") : ""
        : word(tg.technical.purpose);
      const subject = [algorithms, purpose].filter(Boolean).join(" · ");
      if (["unblock", "monitor", "none"].includes(d.action.kind)) {
        if (subject) title = subject;
        else if (d.action.kind === "none") title = t("Recorded references", "記録された参照");
        if (facts.libraryReferences) title = `${title} · ${t("Includes a library reference", "ライブラリ名の参照あり")}`;
      } else if (subject) {
        title = `${title} · ${subject}`;
      }
      return title;
    }
    function decisionFields(action, condition) {
      return `<dl class="decision-fields">
        <div><dt>${t("What to do", "やること")}</dt><dd>${h(action)}</dd></div>
        <div><dt>${t("When to proceed", "次に進める条件")}</dt><dd>${h(condition)}</dd></div>
      </dl>`;
    }
    function investigationCondition(d, facts, total) {
      if (facts.unbindableRecords === total) return t(
        "Reassess when a distinguishable record can be linked to the actual use.", "実際の用途に結び付けられる記録が見つかったら、再評価する。");
      if (d.missing_inputs.some(input => input.startsWith("technical.") || input === "observation.selected_use")) return t(
        "Reassess with the effective algorithm, settings and use confirmed from the code, configuration or observation.", "コード・設定・観測記録から実際の方式と利用箇所を確認できたら、再評価する。");
      if (d.missing_inputs.some(input => input.startsWith("support.")) || d.action.blocking_prerequisites.length) return t(
        "Reassess once the support evidence covers the required configuration and affected environments.", "必要な設定と配備先について、対応状況を示す根拠が揃ったら再評価する。");
      if (d.action.kind === "monitor" || d.action.kind === "none") return t(
        "Reassess if the selected settings or declared scope changes.", "使用する設定や申告した対象範囲が変わった時点で、再評価する。");
      return t("Reassess after confirming where the evidence applies and how it is used.", "根拠が示す箇所と、実際にどの用途で使うかを確認できたら、再評価する。");
    }
    function phases(scope, d) {
      const jwt = scope.use === "jwt";
      const encryption = scope.use === "encryption";
      const pki = scope.use === "pki";
      const connection = scope.use === "key_establishment";
      const hash = scope.use === "hashing";
      if (scope.kind === "none") return [[
        t("Check the declared exclusion", "申告した対象外の範囲を確認する"),
        t("Review the declaration and check whether any live processing outside that scope relies on the listed references.", "対象外の申告を確認し、記録された参照が、申告範囲外の実処理に使われていないか確認する。"),
        t("Reassess if a caller starts using a reference in scope or the declaration changes.", "実処理で使うことが分かった場合や申告範囲が変わった場合は、再評価する。"),
      ]];
      if (scope.kind === "monitor") return [[
        encryption ? t("Monitor the active encryption configuration", "暗号化設定を継続確認する")
          : t("Monitor the active settings", "利用中の設定を継続確認する"),
        encryption ? t("Compare the code defaults and application overrides. Record the cipher, mode and key size that the application actually uses.", "コードの既定値とアプリ側の上書きを照合し、実際に使う暗号方式・モード・鍵長を記録する。")
          : t("Compare the recorded evidence with the effective algorithm and parameters in the declared scope.", "記録された箇所をたどり、申告した用途で実際に選ぶ方式と設定値を照合する。"),
        encryption ? t("If the effective settings differ from the recorded evidence or an override changes, reassess with the actual values.", "記録と実際の設定が異なる場合や、上書きで設定が変わった場合は、その値で再評価する。")
          : t("Reassess when the effective settings or use changes.", "実際に使う方式・設定値・用途が変わった時点で再評価する。"),
      ]];
      const current = jwt ? [
        t("Identify the current issuance and verification paths", "現在の発行・検証経路を特定する"),
        t("Follow the signing locations to the effective configuration. Identify the services that verify the issued tokens.", "署名箇所から実際の設定をたどり、発行されたトークンを検証するサービスを洗い出す。"),
        t("Begin compatibility testing after identifying the current signing algorithm, each verifier's accepted algorithms and key source, and choosing the candidate algorithm and key format to test.", "現在の署名方式と、検証先ごとの許可方式・公開鍵の取得元を調べ、試験する候補方式と鍵形式が決まれば互換性試験へ進む。"),
      ] : [
        t("Identify the current configuration and affected systems", "現在の設定と影響範囲を特定する"),
        connection ? t("Trace the connection configuration and identify the peers and the negotiated groups or settings.", "接続設定をたどり、接続先と、実際に交渉されるグループ・設定を確認する。")
          : pki ? t("Trace certificate issuance, trust stores and the consumers that validate the certificate chain.", "発行元・信頼ストア・証明書チェーンを検証する利用先を確認する。")
          : encryption || hash ? t("Trace the algorithm and parameter selection, and identify operations that read or compare existing data.", "方式・設定値を選ぶ箇所と、既存データを読み出す、または照合する処理を確認する。")
          : t("Trace the signing settings and identify the components that verify the resulting signatures.", "署名設定をたどり、生成された署名を検証する処理を確認する。"),
        t("Start testing after identifying the effective configuration and affected systems and choosing the candidate and its parameters.", "実際の設定と影響範囲を特定し、試験する候補方式と設定値が決まれば互換性試験へ進む。"),
      ];
      const compatibility = jwt ? [
        t("Can the verifiers accept the proposed algorithm?", "候補方式を検証先が受け入れられるか"),
        t("Issue tokens with the proposed algorithm in a test environment. Try the new and existing tokens at each verifier.", "検証環境で候補方式のトークンを発行し、各検証先で新方式と既存方式の両方を試す。"),
        t("Verify the new tokens at every affected verifier and keep existing valid tokens usable during the transition.", "対象の全検証先で候補方式を検証でき、切り替え中も有効な既存のトークンを使えること。"),
      ] : [
        t("Does the proposed configuration work with the affected systems?", "候補方式で対象システムが動作するか"),
        connection ? t("Test negotiation with each affected peer and verify the connection settings on both ends.", "対象の各接続先と相互接続試験を行い、両端で成立した接続設定を確認する。")
          : pki ? t("Test certificate issuance, distribution and chain verification with representative clients.", "検証環境で証明書を発行・配布し、代表的なクライアントでチェーン検証を試す。")
          : encryption || hash ? t("Test the candidate with new data and verify that existing data can still be read or matched.", "候補方式を新規データで試し、既存データも引き続き読み出し・照合できるかを確認する。")
          : t("Test signatures with the proposed configuration and verify both new and existing signatures at their consumers.", "候補方式で署名し、利用先で新旧両方の署名を検証する。"),
        t("Pass the tests in the required environments and peers, keeping existing connections or data usable where required.", "必要な実行環境・利用先で試験に合格し、移行中に必要な既存の接続やデータも使えること。"),
      ];
      const remainingSupport = [...new Set([...d.action.blocking_prerequisites,
        ...d.missing_inputs.filter(input => input.startsWith("support.")).map(input => input.slice(8))])];
      const support = remainingSupport.map(requirementLabel);
      const protection = [
        scope.constraints.signature_verification_until && t(
          "Keep existing signatures verifiable until " + localDate(scope.constraints.signature_verification_until) + " as declared.",
          "申告された " + localDate(scope.constraints.signature_verification_until) + " まで既存の署名を検証できること。"),
        scope.constraints.confidentiality_until && t(
          "Confirm a protection plan through " + localDate(scope.constraints.confidentiality_until) + " as declared.",
          "申告された " + localDate(scope.constraints.confidentiality_until) + " までデータを保護する計画を確認できること。"),
        scope.constraints.historical_ciphertext === true && t(
          "Confirm how to protect historical ciphertext.", "過去の暗号文の保護方法を確認できること。"),
      ].filter(Boolean);
      if (protection.length) compatibility[2] += " " + protection.join(" ");
      if (remainingSupport.length || scope.kind === "unblock") {
        const label = support.length ? t(" (" + support.join(", ") + ")", "（" + support.join("、") + "）") : "";
        compatibility[1] += t(" Also verify the pending requirements" + label + ".", " 未対応の条件" + label + "も試験で確かめる。");
        compatibility[2] += t(
          " Proceed to staged rollout once open requirements" + label + " have been resolved and verified.",
          " 未対応の条件を解消し、その根拠を確認できたら段階切り替えへ進む。");
      } else {
        compatibility[2] += t(" Then proceed to staged rollout.", " 確認できたら段階切り替えへ進む。");
      }
      return [current, compatibility, [
        t("Can the rollout be expanded?", "切り替え対象を広げられるか"),
        t("Start with a limited scope. Monitor relevant errors and test how to restore the previous configuration.", "限定した範囲で切り替えてエラーを監視し、旧設定へ戻す手順を試す。"),
        t("Expand the rollout after the agreed error criteria are met and rollback has been tested.", "エラーが合意した基準内に収まり、復旧手順を確認できたら、対象を段階的に広げる。"),
      ]];
    }
    function workflowPhases(scope, d, location) {
      const labels = [
        t("Start here", "最初に着手"),
        t("After identifying the scope", "影響範囲の特定後"),
        t("After compatibility testing", "互換性試験の合格後"),
      ];
      const steps = phases(scope, d);
      return steps.map(([title, action, condition], index) => `<section class="workflow-phase" data-migration-phase="${index + 1}">
        <h5>${steps.length > 1 ? `<span class="workflow-phase-label">${labels[index]}</span>` : ""}${h(title)}</h5>
        ${decisionFields(action, condition)}
        ${index === 0 ? location : ""}
      </section>`).join("");
    }
    function card(group, i) {
      const d = decision(group.members[0]);
      const facts = groupFacts[i], total = group.members.length;
      const scope = scopes.get(i);
      const checks = workChecks(i, d);
      const title = workTitle(i, d, checks);
      const primaryReason = d.reason_codes.find(code => !["reference_only", "collection_incomplete", "unknown_primitive", "unknown_parameter_encoding"].includes(code));
      const names = groupAlgorithms[i];
      const observedPurposes = [...facts.purposes].map(([purpose, count]) =>
        (purpose === "unknown" ? t("Unidentified", "未特定") : word(purpose)) +
        (facts.purposes.size > 1 ? ` (${n(count)} ${t(count === 1 ? "record" : "records", "記録")})` : ""));
      const observedPurpose = `${t("Observed purpose", "検出結果の用途")}: ${h(observedPurposes.join(" / "))}`;
      const observedAlgorithms = `${t("Observed algorithm", "検出した方式")}: ${names.slice(0, 5).map(name => h(algorithm(name))).join(" · ")}${names.length > 5 ? ` +${n(names.length - 5)}` : ""}`;
      const start = startDue(i);
      const declarations = [
        declarationHtml(i, "use_case", t("Declared purpose", "申告された用途")),
        declarationHtml(i, "usage_state", t("Declared usage", "申告された利用状況")),
        declarationHtml(i, "migration_target", t("Proposed configuration (declared)", "検討中の方式・設定（申告）")),
        declarationHtml(i, "migration_goal_month", t("Migration goal month", "移行の目標年月")),
        planningHtml(i),
      ].join("");
      const steps = checks.length ? checks : d.action.validation_steps.map(code => ({text: word(code), paths: []}));
      const first = steps[0] || {text: t("Reassess when facts, scope or declarations change.", "事実・対象範囲・申告が変わった際に再評価してください。"), paths: []};
      const paths = first.paths.length ? first.paths : facts.paths;
      const location = paths.length ? `<div class="recorded-locations"><strong>${first.paths.length ? t("Check locations", "確認箇所") : t("Recorded locations", "記録された場所")}</strong>${locationsHtml(paths, true)}</div>` : "";
      const checkHtml = ({title, text, paths}) => `<div class="check-step">${title ? `<strong>${h(title)}</strong>` : ""}<p>${h(text)}</p>${locationsHtml(paths)}</div>`;
      const blockers = [...new Set(d.action.blocking_prerequisites)];
      const recordRoles = [...facts.roles].map(word);
      const recordMeta = `${h(recordRoles.join(" / "))} · ${t(n(facts.paths.length) + " recorded locations", "記録された場所: " + n(facts.paths.length) + "か所")} · ${n(total)} ${t("records", "記録")}`;
      const expanded = expandedCards.has(i);
      const tag = scope ? "section" : "article";
      return `<${tag} class="${scope ? "migration-workflow" : "decision-card"}" id="work-${i + 1}" tabindex="-1">
        <div class="group-heading"><h4>${h(title)}</h4>${!scope || scopes.size > 1 || timelinePairs.length > 1 ? startDue(i, true) : ""}</div>
        ${scope ? workflowPhases(scope, d, location) : `<div class="next-checks">
          ${decisionFields(first.text, investigationCondition(d, facts, total))}
          ${location}
        </div>`}
        <div class="card-actions">
          <button class="evidence-open" data-work="${i}">${t("View detection locations", "検出箇所を見る")}</button>
          <button type="button" class="card-details-toggle" data-work-details="${i}" aria-controls="work-details-${i + 1}" aria-expanded="${expanded}">${t("Assessment details", "判定の詳細")}</button>
        </div>
        <div class="decision-reasoning" id="work-details-${i + 1}"${expanded ? "" : " hidden"}>
          <section><h5>${t("Decision reasons", "判定理由")}</h5>
            ${primaryReason ? `<p class="decision-basis">${h(word(primaryReason))}</p>` : ""}
            <p>${h(state("information", d.information))} · ${h(state("readiness", d.readiness))}</p>
            ${list([...facts.reasons].filter(code => code !== primaryReason))}
            ${blockers.length ? `<p><strong>${t("Blocking requirements", "未対応の条件")}</strong> ${h(blockers.map(requirementLabel).join(t(", ", "、")))}</p>` : ""}
            ${d.missing_inputs.length ? `<h6>${t("Facts still needed", "判断に必要な情報")}</h6>${list(d.missing_inputs)}` : ""}
          </section>
          ${declarations ? `<section class="card-declarations"><h5>${t("Applied declarations and planning", "申告された前提と計画")}</h5>${declarations}${start}</section>` : ""}
          <section><h5>${t("Detected and saved evidence", "検出・保存された根拠")}</h5>
            <p>${recordMeta}${facts.dependencyRecords ? ` · ${t(n(facts.dependencyRecords) + " dependency records", "依存関係 " + n(facts.dependencyRecords) + "件")}` : ""}</p>
            <p>${observedPurpose}</p>
            <p>${observedAlgorithms}</p>
            ${facts.developmentDeclarations ? `<p>${t("Declared development dependencies", "開発用として宣言された依存関係")}: ${n(facts.developmentDeclarations)} / ${n(total)} ${t("records", "記録")}</p>` : ""}
          </section>
          ${facts.unbindableRecords ? bindingNotice(facts.unbindableRecords, 5) : ""}
          ${(scope ? steps.length : steps.length > 1) || (checks.length && d.action.validation_steps.length) ? `<section><h5>${t("Additional checks", "追加の確認")}</h5>
            ${steps.slice(scope ? 0 : 1).map(checkHtml).join("")}
            ${checks.length ? list(d.action.validation_steps) : ""}
          </section>` : ""}
        </div>
      </${tag}>`;
    }
    const urgencyRank = ["urgent", "plan", "undetermined", "monitor", "not_applicable"];
    const urgencySymbol = {urgent: "▲", plan: "◆", undetermined: "？", monitor: "●", not_applicable: "—"};
    const workUrgencies = groups.map(g => g.members.length ? value(child(decisions[g.members[0]], "urgency")) : null);
    const workOrder = workUrgencies.flatMap((code, i) => code ? [i] : []).sort((a, b) =>
      urgencyRank.indexOf(workUrgencies[a]) - urgencyRank.indexOf(workUrgencies[b]) ||
      Number(scopes.has(b)) - Number(scopes.has(a)) ||
      Number(groupFacts[a].libraryReferencesOnly) - Number(groupFacts[b].libraryReferencesOnly));
    const firstOtherUrgent = workOrder.find(i => !scopes.has(i) && workUrgencies[i] === "urgent");
    const isOtherCheck = i => !scopes.has(i) && workUrgencies[i] !== "urgent";
    const timePairs = new Map();
    for (const i of workOrder) {
      for (const row of startDates[i]) timePairs.set(JSON.stringify([row.due, row.deadline]), row);
    }
    const timelinePairs = [...timePairs.values()].sort((a, b) => a.due - b.due || a.deadline - b.deadline);
    function declarationSummary() {
      const entries = [...allDeclarations.values()];
      const fields = [
        ["use_case", t("Purpose", "用途")],
        ["usage_state", t("Use", "利用状況")],
        ["migration_target", t("Candidate", "検討方式")],
        ["deadline", t("Migration deadline", "移行期限")],
        ["migration_lead_seconds", t("Lead time", "準備期間")],
      ];
      return fields.flatMap(([key, label]) => {
        const values = entries.filter(c => c.field === key);
        if (!values.length) return [];
        const shown = values.slice(0, 3).map(({value: v, count}) => {
          const result = key === "migration_target" ? [word(v.use_case), v.algorithm, v.parameters].filter(Boolean).join(" / ") :
            key === "deadline" ? v.kind === "at" ? dateText(v.at) + " JST" : t("None declared", "期限なしと申告") :
            key === "migration_lead_seconds" ? durationText(v) : word(v);
          return `<li><span>${h(result)}</span><small>${t(n(count) + (count === 1 ? " record" : " records"), n(count) + "記録")}</small></li>`;
        });
        const remaining = values.length - shown.length;
        const extra = remaining ? t(n(remaining) + " more values in card details", "ほか" + n(remaining) + "種類はカードの詳細へ") : "";
        return [`<div><dt>${label}</dt><dd><ul>${shown.join("")}</ul>${extra ? `<p>${extra}</p>` : ""}</dd></div>`];
      }).join("");
    }
    function conclusion(counts) {
      const highest = urgencyRank.find(code => counts.has(code));
      let headline = t("No assessment targets were found in this input.", "この入力から評価対象を検出できませんでした。");
      let timing = "";
      if (highest === "urgent" || highest === "plan") {
        headline = highest === "urgent" ? t("Check the urgent items first.", "早急な対応の項目から確認してください。")
          : t("Review the actions that need a plan.", "計画が必要な項目から確認してください。");
        const cardDates = workOrder.filter(i => workUrgencies[i] === highest).map(i => startDates[i][0]?.due).filter(due => due !== undefined);
        const overdue = cardDates.filter(due => due < asOf).length;
        const total = counts.get(highest);
        const passed = overdue ? `<div><dt>${t("Past the start-by date", "着手目安を過ぎた項目")}</dt><dd>${t(n(overdue) + (overdue === 1 ? " item" : " items"), n(overdue) + "件")}</dd></div>` : "";
        const dated = cardDates.length > 0 && cardDates.length < total
          ? `<div><dt>${t("With a start-by date", "着手目安あり")}</dt><dd>${n(cardDates.length)} / ${n(total)} ${t("items", "件")}</dd></div>` : "";
        if (passed || dated) timing = `<dl class="conclusion-timing">${passed}${dated}</dl>`;
      } else if (highest === "undetermined") {
        headline = t("Start with the first check on each item to establish urgency.", "各項目の最初の確認から、緊急度の判断に必要な事実を集めてください。");
      } else if (highest) {
        headline = t("Review when to reassess each item.", "各項目の再評価のきっかけを確認してください。");
      }
      const outcome = field("coverage").collection_outcome;
      const collectionText = outcome === "unknown" ? t("Collection status is unknown", "収集状況は不明") : word(outcome);
      const collection = outcome && !["complete", "completed"].includes(outcome)
        ? `<p class="collection-status">${h(collectionText)}</p>` : "";
      const declarations = declarationSummary();
      const otherUrgent = scopes.size && firstOtherUrgent !== undefined
        ? `<p class="conclusion-jump"><a href="#work-${firstOtherUrgent + 1}" data-check-work="${firstOtherUrgent}">${t("Go to urgent investigations", "早急な調査項目へ")}</a></p>` : "";
      return `<section class="migration-conclusion${scopes.size ? " compact-conclusion" : ""}" id="migration-conclusion"><div class="conclusion-heading"><h2>${t("Conclusion", "結論")}</h2>
        ${counts.size ? `<ul class="urgency-chips">${urgencyRank.filter(code => counts.has(code)).map(code =>
          `<li class="urgency-chip ${code}"><span aria-hidden="true">${urgencySymbol[code]}</span> ${h(state("urgency", code))}: ${n(counts.get(code))}</li>`).join("")}</ul>` : ""}</div>
        ${scopes.size ? `<div class="conclusion-meta">${timing}${collection}${otherUrgent}</div>` :
          `<p class="migration-headline">${h(headline)}</p>${timing}${collection}`}
        ${declarations ? `<details class="summary-declarations"><summary>${t("Applied declarations", "反映した申告")}</summary>
          <p class="declaration-population">${t("Assessed records", "評価対象")} ${t(n(decisions.length) + " records", n(decisions.length) + "記録")}</p>
          <dl class="declaration-summary">${declarations}</dl>
        </details>` : allDeclarations.size ? "" : `<p class="group-meta">${t("No purpose or planning declarations shown", "用途・利用状況・計画の申告なし")}</p>`}
      </section>`;
    }
    function timeline() {
      if (!timelinePairs.length) return "";
      const rows = timelinePairs.slice(0, 3);
      const min = Math.min(asOf, rows[0].due);
      const max = Math.max(asOf, ...rows.map(row => row.deadline));
      const scale = instant => (100 * (instant - min) / (max - min || 1)).toFixed(3);
      const assessment = `${t("Evaluation time", "評価日時")} ${timestamp(asOf)} JST`;
      return `<section id="migration-timeline" class="migration-timeline" aria-label="${t("Migration schedule", "移行日程")}">
        <div class="timeline-evaluation">${assessment}</div>
        <ol>${rows.map(({due, deadline, lead}) => {
          const start = startStatus(due), end = deadlineStatus(deadline);
          return `<li class="timeline-period${due < asOf ? " start-passed" : ""}${deadline < asOf ? " deadline-passed" : ""}">
          <div class="timeline-track" aria-hidden="true" style="--start:${scale(due)}%;--end:${scale(deadline)}%;--assessment:${scale(asOf)}%"><span></span><b class="assessed-mark"></b></div>
          <dl class="timeline-dates">
            <div class="timeline-start"><dt>${t("Start by", "着手目安")}</dt><dd>
              ${timestamp(due)}${start ? `<small class="timeline-date-status">${h(start)}</small>` : ""}</dd></div>
            <div class="timeline-lead"><dt>${t("Lead time", "準備期間")}</dt><dd>${h(durationText(lead))}</dd></div>
            <div class="timeline-deadline"><dt>${t("Migration deadline", "移行期限")}</dt><dd>
              ${timestamp(deadline)}${end ? `<small class="timeline-date-status">${h(end)}</small>` : ""}</dd></div>
          </dl>
        </li>`;
        }).join("")}</ol>${timelinePairs.length > rows.length ? `<p>${t(`${n(timelinePairs.length - rows.length)} more date pairs in the work items.`, `ほか${n(timelinePairs.length - rows.length)}組は各項目の申告欄で確認できます。`)}</p>` : ""}
      </section>`;
    }
    function urgencySections(indices) {
      const sections = new Map();
      for (const i of indices) {
        const code = workUrgencies[i];
        if (!sections.has(code)) sections.set(code, []);
        sections.get(code).push(i);
      }
      return [...sections].map(([code, items]) => `<section class="urgency-section" data-urgency-group="${h(code)}">
        <header><h3><span aria-hidden="true">${urgencySymbol[code] || ""}</span> ${h(state("urgency", code))} · ${n(items.length)} ${workOrder.length > size ? t("on this page", "件（このページ）") : t("", "件")}</h3></header>
        ${items.map(i => card(groups[i], i)).join("")}</section>`).join("");
    }
    function workList(indices) {
      if (!scopes.size) return urgencySections(indices);
      const other = indices.filter(isOtherCheck);
      const main = urgencySections(indices.filter(i => !isOtherCheck(i)));
      if (!other.length) return main;
      const remaining = workOrder.filter(isOtherCheck).length;
      const summary = t("Other checks", "ほかの確認項目") + ` · ${n(remaining)} ${t("items", "件")}`;
      return `${main}<details data-other-work${otherExpanded ? " open" : ""}>
        <summary>${h(summary)}</summary>${urgencySections(other)}</details>`;
    }
    function workCounts() {
      const counts = new Map();
      for (const i of workOrder) {
        const code = workUrgencies[i];
        counts.set(code, (counts.get(code) || 0) + 1);
      }
      return counts;
    }
    function render() {
      page = Math.max(0, Math.min(page, Math.ceil(workOrder.length / size) - 1));
      if (standalone && payload.label) document.title = `${payload.label} | ${t("Migration review", "移行レポート")} | pqc-scan`;
      const counts = workCounts();
      active.innerHTML = `<header class="top"><div><h1>${t("Migration response work items", "移行に向けた確認事項")}</h1>${payload.label ? `<p class="report-scope">${h(payload.label)}</p>` : ""}</div>${timelinePairs.length ? "" : evaluationTime()}</header>
        ${timeline()}
        ${conclusion(counts)}
        <section id="decisions" class="migration-work"><h2>${t("Migration decisions", "移行の判断")}</h2>
          ${workList(workOrder.slice(page * size, (page + 1) * size))}
        </section>
        ${workOrder.length > size ? `<div class="target-pagination"><span>${n(page * size + 1)}-${n(Math.min((page + 1) * size, workOrder.length))} / ${n(workOrder.length)}</span><div><button data-page="-1" ${page === 0 ? "disabled" : ""}>${t("Previous", "前へ")}</button><button data-page="1" ${(page + 1) * size >= workOrder.length ? "disabled" : ""}>${t("Next", "次へ")}</button></div></div>` : ""}
        <footer class="migration-links"><button data-download>${t("Save assessment data", "評価データを保存")}</button></footer><dialog class="migration-dialog" aria-label="${t("Detection locations", "検出箇所")}"><div data-dialog-body></div></dialog>`;
    }
    function localDate(raw) {
      return raw ? `${dateText(raw)} ${t("(Japan time)", "（日本時間）")}` : t("Unspecified", "指定なし");
    }
    function claimValue(c) {
      if (c.value === null || c.value === undefined) return word("unknown");
      if (typeof c.value === "boolean")
        return c.field.startsWith("support.")
          ? c.value
            ? t("Declared supported", "申告上は対応")
            : t("Declared unsupported", "申告上は未対応")
          : c.value
            ? t("Yes", "はい")
            : t("No", "いいえ");
      if (["signature_verification_until", "confidentiality_until"].includes(c.field)) return localDate(c.value);
      if (c.field === "migration_target")
        return [word(c.value.use_case), c.value.algorithm, c.value.parameters]
          .filter(Boolean)
          .join(" / ");
      if (c.field === "deadline")
        return c.value.kind === "none"
          ? t("No deadline declared", "期限なしと申告")
          : `${dateText(c.value.at)} ${t("(Japan time)", "（日本時間）")} / ${c.value.scope_reason === "contractual_requirement" ? t("Contractual requirement", "契約上の要件") : t("Organization policy", "組織の方針")} / ${c.value.reference_id}`;
      if (c.field === "migration_lead_seconds")
        return `${durationText(c.value)} (${n(c.value)} ${t("seconds", "秒")})`;
      return word(c.value);
    }
    function row(label, text) {
      return `<tr><th scope="row">${h(label)}</th><td>${h(text)}</td></tr>`;
    }
    function claimHtml(c, i, prefix = "") {
      const freshness = {
        active: t("Active declared interval", "申告期間内"),
        stale: t("Expired declaration", "申告は失効"),
        not_yet_effective: t("Not yet effective", "申告の開始前"),
        unknown: t("Expiry unknown", "有効期限不明"),
      };
      return `<section id="${prefix}claim-${i + 1}"><h3>${h(word(c.field))}</h3><p>${h(claimValue(c))} · ${h(freshness[c.freshness] || word(c.freshness))}</p><p>${h(c.provenance?.actor_id || "")} · ${h(localDate(c.provenance?.asserted_at))}</p><details><summary>${t("Reference identity", "参照情報")}</summary><table><tbody>${row(t("Claim ID", "申告ID"), c.id)}${row(t("Declared source", "申告の由来"), c.provenance?.source_kind === "operator_claim" ? t("Operator declaration", "利用者による申告") : t("Declared external review (unverified)", "外部確認を受けたとの申告（未検証）"))}${row(t("Valid until", "有効期限"), localDate(c.provenance?.valid_until))}${row(t("Reference", "参照先"), c.provenance?.reference_id || t("Unspecified", "指定なし"))}${row(t("Context pointer", "入力内の場所"), c.context_pointer || "")}</tbody></table></details></section>`;
    }
    function methodLabel(method) {
      const labels = {
        lexical_match: t("Text match", "文字列一致"),
        ast_pattern: t("Syntax pattern", "構文解析"),
        configuration_match: t("Configuration match", "設定値との一致"),
        dependency_report: t("Dependency inventory", "依存関係の読み取り"),
        certificate_parse: t("Certificate parsing", "証明書の解析"),
        private_key_marker: t("Key marker", "鍵ファイルの目印"),
        legacy_adapter: t("Imported scan finding", "既存のスキャン所見"),
        tls_saved_observation: t("Saved TLS observation", "保存したTLS観測"),
        ssh_saved_offers: t("Saved SSH offers", "保存したSSHの方式提示"),
        ssh_saved_kex: t("Saved SSH exchange", "保存したSSHの鍵交換"),
      };
      return Object.hasOwn(labels, method) ? labels[method] : word(method);
    }
    function appliedClaims(d) {
      return d.applied_claim_ids.map(id => claimById.get(id)).filter(Boolean);
    }
    function recordVariants(members) {
      const variants = new Map();
      for (const i of members) {
        const d = decision(i), tg = target(d);
        const {observation_id, ...location} = tg.source_ref;
        const {method, assurance, ...observation} = tg.observation;
        const {id, target_id, applied_claim_ids, action, ...judgment} = d;
        const {candidate_ids, ...response} = action;
        // Rule identity, method and assurance can differ at one location.
        // Show them together when the underlying facts and assessment agree.
        const claimsUsed = appliedClaims(d).map(({ref}) => {
          const {id, context_pointer, ...claim} = value(ref);
          return JSON.stringify(claim);
        }).sort();
        const key = JSON.stringify([location, tg.technical, observation, tg.limitations,
          unbindable.has(tg.id), judgment, response, claimsUsed, candidate_ids.map(candidateCondition)]);
        if (!variants.has(key)) variants.set(key, []);
        variants.get(key).push(i);
      }
      return [...variants.values()];
    }
    function recordHtml(members, separateLocation) {
      const i = members[0];
      const d = decision(i),
        tg = target(d);
      const applied = appliedClaims(d);
      const records = members.map(index => index === i ? tg : target(decision(index)));
      const methods = [...new Set(records.map(record => methodLabel(record.observation.method)))].join(" · ");
      const rules = [...new Set(records.map(record => scanRule(record.source_ref)).filter(Boolean))].join(" · ");
      const assurances = [...new Set(records.map(record => word(record.observation.assurance)))].join(" · ");
      const scanIds = [...new Set(records.flatMap(record => scanReference(record.source_ref.observation_id)?.findings || []))]
        .filter(id => /^finding-v1-[a-f0-9]{64}$/.test(id));
      const scanLinks = !standaloneExport && hasScanReport ? scanIds
        .map((id, index) => `<a href="${h(artifactBase)}report.html?lang=${ja() ? "ja" : "en"}#scan-match-${id}" target="_blank" rel="noopener">${t("Open scan finding", "スキャンの検出箇所を開く")}${scanIds.length > 1 ? " " + n(index + 1) : ""}</a>`)
        .join(" · ") : "";
      let html = `<section id="target-${i + 1}"><h3>${h(algorithm(tg.technical.identifier))}</h3>${separateLocation ? "" : `<p class="path">${h(recordLocation(tg))}</p>`}${scanLinks ? `<p>${scanLinks}</p>` : ""}<p>${h(state("urgency", d.urgency))} · ${h(state("information", d.information))} · ${h(state("readiness", d.readiness))}</p>${section(t("Decision reasons", "判定理由"), list(d.reason_codes), 4)}${unbindable.has(tg.id) ? bindingNotice() : ""}<table><tbody>${row(t("Observation method", "検出方法"), methods)}${rules ? row(t("Scan rule ID", "スキャンのルールID"), rules) : ""}${row(t("Role", "役割"), word(tg.source_ref.role))}${tg.source_ref.dependency_provenance ? row(t("Dependency declaration", "依存関係の宣言"), tg.source_ref.dependency_provenance === "manifest_dev_declared" ? t("Declared development dependency", "開発用依存関係として宣言") : word(tg.source_ref.dependency_provenance)) : ""}${row(t("Observed purpose", "検出結果の用途"), word(tg.technical.purpose))}${row(t("Assessment system ID", "評価上のシステムID"), d.system_id || t("Unbound", "未対応付け"))}${row(t("Quantum assessment", "量子計算に対する評価"), word(tg.technical.quantum))}${row(t("Evidence assurance", "確認できている範囲"), assurances)}${row(t("Collection", "収集状況"), word(tg.observation.collection_outcome))}${row(t("Parameter encoding", "パラメーターの符号化"), word(tg.technical.encoding_assurance))}${(tg.technical.classical_concerns || []).map((c) => row(t("Classical concern", "従来暗号の懸念"), word(c))).join("")}${(tg.technical.components || []).map((c) => row(t("Hybrid component", "ハイブリッド構成要素"), `${word(c.role)} / ${algorithm(c.assessment.identifier)} / ${word(c.assessment.quantum)}`)).join("")}${tg.technical.combiner_assessment ? row(t("Combiner assessment", "結合処理の確認範囲"), word(tg.technical.combiner_assessment)) : ""}</tbody></table>${section(t("Evidence limitations", "根拠の制約"), list(tg.limitations.filter(code => !d.reason_codes.some(reason => word(reason) === word(code)))), 4)}${section(t("Next checks", "次に確認すること"), list(d.action.validation_steps), 4)}${section(t("Claims used", "判断に使った申告"), applied.map(entry => claimHtml(value(entry.ref), entry.index, `target-${i + 1}-`)).join(""), 4)}`;
      if (d.action.candidate_ids.length)
        html +=
          `<h4>${t("Candidates: integration unverified", "検討候補（組込みは未検証）")}</h4>` +
          d.action.candidate_ids
            .map((id) => {
              const c = value(candidateById.get(id).ref);
              const declared = applied.map(entry => value(entry.ref)).filter(claim =>
                claim.field === "migration_target" && claim.freshness === "active" && claim.state === "known" &&
                claim.value?.algorithm === c.identifier && (claim.value?.use_case === c.purpose || (claim.value?.use_case === "jwt" && c.purpose === "signature"))
              );
              const setting = declared.map(claim => claim.value.parameters).filter(Boolean);
              const summary = setting.length
                ? t("Declared configuration", "申告済みの設定") + " " + setting.join(" / ")
                : word(c.parameter_selection);
              return `<details id="candidate-${candidateById.get(id).index + 1}"><summary>${h(c.identifier)} · ${h(summary)}</summary>${c.status ? `<p>${h(word(c.status))}</p>` : ""}${setting.length ? `<p>${t("Check that the declared configuration works in the target environment.", "申告された設定が対象の環境で動作するか検証してください。")}</p>` : ""}<table><tbody>${c.prerequisites.map((p) => row(word(p.name), p.claimed_support === null ? word("unknown") : p.claimed_support ? t("Declared supported", "申告上は対応") : t("Declared unsupported", "申告上は未対応"))).join("")}</tbody></table>${list(c.validation_steps.filter(step => !d.action.validation_steps.includes(step)))}</details>`;
            })
            .join("");
      return html + "</section>";
    }
    function sourceHtml(s, i) {
      const external = /^https?:\/\//.test(s.uri || "");
      const declared = s.id.startsWith("context:");
      const description = declared
        ? t("Reference supplied with a declaration", "申告に添えられた資料")
        : external ? t("Reference used for the technical assessment", "技術評価で参照した資料")
          : t("Local assessment basis", "ツール内の判定根拠");
      return `<section id="source-${i + 1}"><p>${h(description)}${external ? ` · <a href="${h(s.uri)}" target="_blank" rel="noreferrer noopener">${t("Open reference", "資料を開く")}</a>` : ""}</p><details><summary>${t("Reference information", "参照情報")}</summary><p><code>${h(s.id)}</code></p><p>${h(s.edition)}</p><p>${h(s.clause)}</p><p>${h(s.scope_limit)}</p></details></section>`;
    }
    const closeButton = () =>
      `<button class="dialog-close" data-close aria-label="${t("Close", "閉じる")}">×</button>`;
    function locationGroups(rows) {
      const locations = new Map();
      for (const i of rows) {
        const tg = target(decision(i));
        const { path, line } = tg.source_ref;
        const key = path && Number(line) > 0 ? JSON.stringify([path, line]) : i;
        if (!locations.has(key)) locations.set(key, { label: recordLocation(tg), members: [] });
        locations.get(key).members.push(i);
      }
      return [...locations.values()].sort((a, b) => orderLocation(a.label, b.label));
    }
    function evidenceRow(group) {
      const records = group.members.map(i => target(decision(i)));
      const preview = values => {
        const unique = [...new Set(values)];
        return h(unique.slice(0, 4).join(" · ")) + (unique.length > 4 ? ` +${n(unique.length - 4)}` : "");
      };
      return `<tr><td>${preview(records.map(tg => algorithm(tg.technical.identifier)))}</td><td class="path">${h(group.label)}</td><td>${preview(records.map(tg => methodLabel(tg.observation.method)))}</td><td>${preview(records.map(tg => scanRule(tg.source_ref)).filter(Boolean))}</td><td><button data-target="${group.members[0]}">${t("Details", "詳細")}</button></td></tr>`;
    }
    function renderDialog() {
      const dialog = active.querySelector("dialog"),
        body = dialog.querySelector("[data-dialog-body]");
      const title =
        viewer.kind === "claims"
          ? t("Declarations", "申告内容")
          : viewer.kind === "sources"
            ? t("Sources", "出典")
            : t("Detection locations", "検出箇所");
      dialog.setAttribute("aria-label", title);
      if (detail !== null) {
        const multiple = detail.variants.length > 1;
        const location = multiple ? `<p class="path">${h(recordLocation(target(decision(detail.first))))}</p>` : "";
        const variants = detail.variants.slice(recordPage * size, (recordPage + 1) * size);
        body.innerHTML = `<div class="dialog-head"><button data-list>${t("Back to list", "一覧に戻る")}</button>${closeButton()}</div>${location}${variants.map(members => recordHtml(members, multiple)).join("")}${
          detail.variants.length > size ? dialogPager(recordPage, detail.variants.length, "data-record-page") : ""}`;
        return;
      }
      let rows =
        viewer.kind === "work"
          ? groups[viewer.index].members
          : viewer.kind === "target"
            ? [viewer.index]
            : viewer.kind === "claims"
              ? claims.map((_, i) => i)
              : sources.map((_, i) => i);
      const evidenceView = viewer.kind === "work" || viewer.kind === "target";
      if (evidenceView) {
        const needle = detailQuery.toLowerCase();
        rows = locationGroups(rows);
        if (needle) rows = rows.filter(group => group.members.some(i => {
          const tg = target(decision(i));
          return [tg.technical.identifier, algorithm(tg.technical.identifier), group.label, scanRule(tg.source_ref)]
            .join(" ").toLowerCase().includes(needle);
        }));
      }
      detailPage = Math.max(
        0,
        Math.min(detailPage, Math.ceil(rows.length / size) - 1),
      );

      const selected = rows.slice(detailPage * size, (detailPage + 1) * size);
      visibleLocations = new Map(evidenceView ? selected.map(group => [group.members[0], group.members]) : []);
      body.innerHTML = `<div class="dialog-head"><h2>${title}</h2>${closeButton()}</div>${
        evidenceView
          ? `<input type="search" maxlength="512" data-detail-search aria-label="${t("Search records", "記録を検索")}" placeholder="${t("Algorithm or file", "方式・ファイル名")}" value="${h(detailQuery)}"><div class="table-scroll"><table class="detection-location-table"><colgroup><col class="location-algorithm"><col><col class="location-method"><col class="location-rule"><col class="location-action"></colgroup><thead><tr><th>${t("Algorithm", "暗号方式")}</th><th>${t("Location", "場所")}</th><th>${t("Observation method", "検出方法")}</th><th>${t("Scan rule ID", "スキャンのルールID")}</th><th></th></tr></thead><tbody>${
              selected.map(evidenceRow).join("") ||
              `<tr><td colspan="5">${t("No records match.", "一致する記録はありません。")}</td></tr>`
            }</tbody></table></div>`
          : selected
              .map((i) =>
                viewer.kind === "claims"
                  ? claimHtml(value(claims[i]), i)
                  : sourceHtml(value(sources[i]), i),
              )
              .join("")
      }${dialogPager(detailPage, rows.length, "data-detail-page")}`;
    }
    function dialogPager(index, total, attribute) {
      const range = total ? n(index * size + 1) + "-" + n(Math.min((index + 1) * size, total)) : "0";
      return `<div class="target-pagination"><span>${range} / ${n(total)}</span><div><button ${attribute}="-1" ${index === 0 ? "disabled" : ""}>${t("Previous", "前へ")}</button><button ${attribute}="1" ${(index + 1) * size >= total ? "disabled" : ""}>${t("Next", "次へ")}</button></div></div>`;
    }
    function selectDetails(members) {
      detail = {first: members[0], variants: recordVariants(members)};
      recordPage = 0;
    }
    function open(kind, index, record = null) {
      viewer = { kind, index };
      detail = null;
      if (record !== null) selectDetails([record]);
      detailPage = 0;
      detailQuery = "";
      renderDialog();
      active.querySelector("dialog").showModal();
    }
    function showWork(index) {
      index = aliases.get(index) ?? index;
      const position = workOrder.indexOf(index);
      if (position < 0) return;
      const nextPage = Math.floor(position / size);
      if (page !== nextPage) {
        page = nextPage;
        render();
      }
      const card = active.querySelector(`#work-${index + 1}`);
      const disclosure = card?.closest?.("details[data-other-work]");
      if (disclosure?.hasAttribute("data-other-work")) disclosure.open = true;
      card?.focus();
      card?.scrollIntoView?.({block: "start"});
    }
    function jump() {
      const match = /^#(target|evidence|claim|source|work)-(\d+)$/.exec(
        location.hash,
      );
      if (!match) return;
      const i = Number(match[2]) - 1;
      if (!Number.isSafeInteger(i) || i < 0) return;
      if (match[1] === "work") showWork(i);
      else if (["target", "evidence"].includes(match[1]) && i < decisions.length)
        open("target", i, i);
      else if (match[1] === "claim" && i < claims.length) {
        open("claims");
        detailPage = Math.floor(i / size);
        renderDialog();
      } else if (match[1] === "source" && i < sources.length) {
        open("sources");
        detailPage = Math.floor(i / size);
        renderDialog();
      }
    }
    function attach(view) {
      if (!view?.classList.contains("migration")) return;
      active = view;
      render();
      if (bound.has(view)) return;
      bound.add(view);
      view.addEventListener("toggle", event => {
        if (event.target.hasAttribute("data-other-work")) otherExpanded = event.target.open;
      }, { ...events, capture: true });
      let backdrop = false;
      view.addEventListener(
        "pointerdown",
        (event) => {
          const d = view.querySelector("dialog"),
            r = d.getBoundingClientRect();
          backdrop =
            event.target === d &&
            (event.clientX < r.left ||
              event.clientX > r.right ||
              event.clientY < r.top ||
              event.clientY > r.bottom);
        },
        events,
      );
      view.addEventListener(
        "click",
        (event) => {
          const d = view.querySelector("dialog");
          const link = event.target.closest("a[data-check-work]");
          if (link?.dataset.checkWork !== undefined && !event.metaKey && !event.ctrlKey && !event.shiftKey && !event.altKey) {
            event.preventDefault();
            showWork(Number(link.dataset.checkWork));
            return;
          }
          if (backdrop && event.target === d) {
            d.close();
            backdrop = false;
            return;
          }
          backdrop = false;
          const el = event.target.closest("button");
          if (!el) return;
          if (el.dataset.work !== undefined)
            open("work", Number(el.dataset.work));
          else if (el.hasAttribute("data-close")) d.close();
          else if (el.hasAttribute("data-download")) download();
          else if (el.dataset.workDetails !== undefined) {
            const i = Number(el.dataset.workDetails);
            const panel = view.querySelector(`#work-details-${i + 1}`);
            if (!panel) return;
            panel.hidden = !panel.hidden;
            if (panel.hidden) expandedCards.delete(i);
            else expandedCards.add(i);
            el.setAttribute("aria-expanded", String(!panel.hidden));
          } else if (el.dataset.page) {
            page += Number(el.dataset.page);
            render();
          } else if (el.dataset.target !== undefined) {
            const members = visibleLocations.get(Number(el.dataset.target));
            if (!members) return;
            selectDetails(members);
            renderDialog();
            active.querySelector("dialog [data-list]").focus();
          } else if (el.hasAttribute("data-list")) {
            const first = detail.first;
            detail = null;
            renderDialog();
            active.querySelector(`dialog [data-target="${first}"]`)?.focus();
          } else if (el.dataset.recordPage) {
            recordPage += Number(el.dataset.recordPage);
            renderDialog();
            active.querySelector("dialog [data-list]").focus();
          } else if (el.dataset.detailPage) {
            detailPage += Number(el.dataset.detailPage);
            renderDialog();
          }
        },
        events,
      );
      view.addEventListener(
        "input",
        (event) => {
          if (event.target.hasAttribute("data-detail-search")) {
            detailQuery = event.target.value;
            detailPage = 0;
            renderDialog();
            const control = active.querySelector("[data-detail-search]");
            control.focus();
          }
        },
        events,
      );
    }
    if (standalone) {
      document.addEventListener(
        "pqc-report-view",
        (event) => {
          attach(event.detail);
          jump();
        },
        events,
      );
      window.addEventListener("hashchange", jump, events);
    }
    attach(initialView);
    if (standalone) jump();
    return {
      setLanguage(language) {
        active.lang = language === "en" ? "en" : "ja";
        render();
      },
      destroy() {
        abort.abort();
        active.replaceChildren();
      },
    };
  }
  window.PqcMigrationReport = Object.freeze({ mount });
  const block = document.getElementById("migration-shared-data");
  if (block)
    mount(
      JSON.parse(block.textContent),
      document.querySelector("[data-report-view]"),
      true,
      "",
      document.querySelector("[data-scan-report]") !== null,
    );
})();
