/* Human views of immutable saved decisions. No policy evaluation in JavaScript. */
(() => {
  function mount(payload, initialView, standalone = false, artifactBase = "") {
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
      sourceById = byId(sources),
      candidateById = byId(candidates);
    const field = (key) => value(child(rootRef, key));
    const groups = payload.groups;
    const unbindable = new Set(payload.unbindable || []);
    const groupAlgorithms = [],
      groupFacts = [],
      recordSearch = [];
    for (const group of groups) {
      const names = new Set(),
        paths = [], declarations = new Map(), planning = new Map();
      let developmentDeclarations = 0, dependencyRecords = 0, unbindableRecords = 0;
      for (const i of group.members) {
        const targetId = value(child(decisions[i], "target_id")),
          ref = targetById.get(targetId).ref,
          tech = child(ref, "technical"),
          source = child(ref, "source_ref"),
          name = value(child(tech, "identifier"));
        names.add(name);
        if (unbindable.has(targetId)) unbindableRecords++;
        const pathRef = child(source, "path");
        const sourcePath = pathRef === null ? "" : value(pathRef);
        if (child(source, "dependency_provenance") !== null || /(?:^|[/\\])(?:package(?:-lock)?\.json|pnpm-lock\.yaml|yarn\.lock|pom\.xml|Cargo\.(?:toml|lock)|go\.(?:mod|sum)|Gemfile(?:\.lock)?|requirements\.txt|pyproject\.toml)$/.test(sourcePath)) dependencyRecords++;
        const provenance = child(source, "dependency_provenance");
        if (provenance !== null && value(provenance) === "manifest_dev_declared") developmentDeclarations++;
        for (const key of ["path", "detail_path"]) {
          const id = child(source, key);
          if (id !== null) {
            const line = key === "path" ? child(source, "line") : null;
            const lineValue = line !== null ? value(line) : null;
            paths.push(value(id) + (lineValue ? ":" + lineValue : ""));
          }
        }
        const seen = new Set(), plan = {};
        for (const id of value(child(decisions[i], "applied_claim_ids")) || []) {
          const claimRef = claimById.get(id)?.ref;
          if (claimRef === undefined) continue;
          const fieldName = value(child(claimRef, "field"));
          if (!["use_case", "usage_state", "migration_target", "deadline", "migration_lead_seconds"].includes(fieldName)) continue;
          const c = value(claimRef);
          if (c.state !== "known" || c.freshness !== "active") continue;
          if (["deadline", "migration_lead_seconds"].includes(fieldName)) plan[fieldName] = c.value;
          const key = JSON.stringify([fieldName, c.value]);
          if (seen.has(key)) continue;
          seen.add(key);
          const entry = declarations.get(key) || { field: fieldName, value: c.value, count: 0 };
          entry.count++;
          declarations.set(key, entry);
        }
        if (Object.keys(plan).length) {
          const key = JSON.stringify([plan.deadline || null, plan.migration_lead_seconds ?? null]), entry = planning.get(key) || { ...plan, count: 0 };
          entry.count++; planning.set(key, entry);
        }
      }
      groupAlgorithms.push([...names]);
      groupFacts.push({paths: [...new Set(paths)], declarations: [...declarations.values()], planning: [...planning.values()], developmentDeclarations, dependencyRecords, unbindableRecords});
      recordSearch.push(
        [
          ...names,
          ...paths,
          ...[...declarations.values()].flatMap(c => {
            const values = typeof c.value === "object" ? Object.values(c.value || {}) : [c.value];
            return values.flatMap(v => [v, payload.words[v]?.en || "", payload.words[v]?.ja || ""]);
          }),
          ...(names.has("unknown") ? ["未特定 Unidentified"] : []),
        ]
          .join(" ")
          .toLowerCase(),
      );
    }

    let active,
      page = 0,
      query = "",
      urgency = "",
      viewer = null,
      detail = null,
      detailPage = 0,
      detailQuery = "";
    const size = 20,
      bound = new WeakSet();
    const ja = () => active.lang === "ja",
      t = (en, jp) => (ja() ? jp : en);
    const word = (code) =>
      payload.words[code]?.[ja() ? "ja" : "en"] ||
      code ||
      t("Unentered", "未入力");
    const state = (type, code) =>
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
    const list = (values, translate = true) =>
      `<ul>${values.map((v) => `<li>${h(translate ? word(v) : v)}</li>`).join("")}</ul>`;
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
        if (amount || seconds === 1 && !parts.length) parts.push(n(amount) + t(" " + en, jp));
      }
      return parts.join(t(" ", ""));
    }
    function bindingNotice(count = 1) {
      return `<section class="binding-limit"><h3>${t("Tool binding limitation", "ツール上の対応付け制約")}</h3><p>${t("Records that cannot be bound", "対応付けできない記録")}: ${n(count)}</p><p>${t("Adding answers in this screen cannot resolve this limitation. The scanner could not uniquely identify these records. Inspect the listed source and configuration, and use separately identifiable evidence where available; the detection/binding issue may need investigation.", "この画面で回答を追加しても解消できません。検出された記録を一意に対応付けできないためです。下記の参照箇所・設定を調べ、対応付け可能な別の根拠を確認してください。検出・対応付け処理の調査が必要な場合もあります。")}</p></section>`;
    }
    function planningHtml(i) {
      const rows = groupFacts[i].planning;
      if (!rows.length) return "";
      return `<section class="planning-basis"><h3>${t("Applied planning conditions (declared)", "判定に使った期限・期間（申告）")}</h3>${rows.slice(0,5).map(p => {
        const deadline = p.deadline, lead = p.migration_lead_seconds;
        const parts = [];
        if (deadline) parts.push(deadline.kind === "none" ? t("No deadline declared", "期限なしと申告") : `${t("Deadline", "期限")}: ${dateText(deadline.at)} ${t("(Japan time)", "（日本時間）")} · ${deadline.scope_reason === "contractual_requirement" ? t("Contractual requirement", "契約上の要件") : t("Organization policy", "組織の方針")} · ${deadline.reference_id}`);
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
    function card(group, i) {
      const d = decision(group.members[0]), tg = target(d);
      const identityMissing = d.missing_inputs.includes("technical.identity"),
        parametersMissing = d.missing_inputs.includes("technical.parameters"),
        referenceOnly = d.reason_codes.includes("reference_only");
      const concreteInvestigation = d.action.kind === "investigate" && (identityMissing || parametersMissing);
      const facts = groupFacts[i], blockedBinding = facts.unbindableRecords === group.members.length;
      const title = d.action.kind === "investigate" && d.reason_codes.includes("known_classical_concern")
        ? t("Investigate the detected cryptographic concern", "検出された暗号方式の懸念を調査する")
        : concreteInvestigation
        ? identityMissing ? t("Identify the implementation's cryptography", "実装で使う暗号方式を特定する")
          : t("Check the implementation's parameters", "実装の暗号設定を確認する")
        : state("action", d.action.kind);
      const next = blockedBinding ? t("Inspect the listed evidence and the scanner binding limitation above; ordinary purpose or support answers cannot be attached to these records.", "上記の対応付け制約と参照箇所を調査してください。この記録には通常の用途・対応状況の回答を追加できません。") : concreteInvestigation
        ? referenceOnly && facts.dependencyRecords === group.members.length
          ? t("Locate where the referenced dependency is used, then check its call sites and configuration for the actual algorithm and parameters.", "該当する依存関係の利用箇所を探し、呼び出し元と設定から、実際の暗号方式・設定値を確認してください。")
          : referenceOnly
            ? t("Inspect the referenced source and its call sites or configuration to identify the actual algorithm and parameters.", "参照しているソースと呼び出し元・設定を調べ、実際の暗号方式・設定値を確認してください。")
            : t("Inspect the listed implementation and configuration to establish the missing algorithm or parameter facts.", "下記の実装・設定を調べ、不足している暗号方式や設定値を確認してください。")
        : d.action.kind === "investigate" && d.missing_inputs.length
          ? t("Investigate the missing facts above and obtain the relevant technical evidence or declarations.", "上記の未確認事項を調べ、確認できた根拠や必要な申告を補ってください。")
          : null;
      const primaryReason = d.reason_codes.find(code => !["reference_only", "collection_incomplete", "unknown_primitive", "unknown_parameter_encoding"].includes(code));
      const paths = groupFacts[i].paths;
      return `<article class="decision-card" id="work-${i + 1}"><div class="group-heading"><h2>${h(title)}</h2><span class="state ${h(d.urgency)}">${h(state("urgency", d.urgency))}</span></div><p class="group-meta">${h(word(tg.source_ref.role))} · ${n(group.members.length)} ${t("records", "記録")} · ${t(n(paths.length) + " recorded locations", "記録された場所: " + n(paths.length) + "か所")}</p><p class="group-meta">${t("Observed purpose", "検出結果の用途")}: ${h(tg.technical.purpose === "unknown" ? t("Unidentified", "未特定") : word(tg.technical.purpose))} · ${t("Observed algorithm", "検出した方式")}: ${groupAlgorithms[i].slice(0, 5).map(name => h(algorithm(name))).join(" · ")}${groupAlgorithms[i].length > 5 ? ` +${groupAlgorithms[i].length - 5}` : ""}</p>${groupFacts[i].developmentDeclarations ? `<p class="group-meta">${t("Declared development dependencies", "開発用として宣言された依存関係")}: ${n(groupFacts[i].developmentDeclarations)} / ${n(group.members.length)} ${t("records", "記録")}</p>` : ""}${declarationHtml(i, "use_case", t("Declared purpose", "申告された用途"))}${declarationHtml(i, "usage_state", t("Declared usage", "申告された利用状況"))}${declarationHtml(i, "migration_target", t("Proposed configuration (declared)", "検討中の方式・設定（申告）"))}${groupFacts[i].declarations.length ? `<p class="muted">${t("Declarations do not establish the cryptography actually used by the implementation.", "申告内容と、実装から検出した情報は分けて表示しています。")}</p>` : ""}${paths.length ? `<p class="path"><strong>${t("Locations to inspect", "確認する場所")}:</strong> ${paths.slice(0, 3).map(h).join(" · ")}${paths.length > 3 ? ` +${n(paths.length - 3)}` : ""}</p>` : ""}${d.urgency !== "undetermined" && primaryReason ? `<p class="decision-basis">${h(word(primaryReason))}</p>` : ""}${planningHtml(i)}${facts.unbindableRecords ? bindingNotice(facts.unbindableRecords) : ""}${d.missing_inputs.length ? `<section class="missing-facts"><h3>${t("Facts still needed", "まだ確認できていないこと")}</h3>${list(d.missing_inputs)}${d.urgency === "undetermined" ? `<p>${t("These unresolved assessment inputs prevent an urgency classification; this is not a low-risk rating.", "判定に必要な確認が残っているため、緊急度は未判定です。低い緊急度という意味ではありません。")}</p>` : ""}</section>` : ""}<section class="next-checks"><h3>${t("Next checks", "次に確認すること")}</h3>${next ? `<p>${h(next)}</p>` : d.action.validation_steps.length ? list(d.action.validation_steps) : `<p>${t("Reassess when facts, scope or declarations change.", "事実・対象範囲・申告が変わった際に再評価してください。")}</p>`}</section>${d.action.blocking_prerequisites.length ? `<section class="decision-note blocker"><h3>${t("Blocking prerequisites", "対応を妨げている条件")}</h3>${list(d.action.blocking_prerequisites)}</section>` : ""}<details class="decision-reasoning"><summary>${t("Full assessment reasoning", "判断の詳細")}</summary><p>${h(state("information", d.information))} · ${h(state("readiness", d.readiness))}</p>${list(d.reason_codes)}${next ? list(d.action.validation_steps) : ""}</details><button class="evidence-open" data-work="${i}">${t("View evidence", "根拠を見る")}</button></article>`;
    }
    function controls() {
      if (groups.length <= 1) return "";
      const counts = new Map();
      for (const g of groups) {
        const code = decision(g.members[0]).urgency;
        counts.set(code, (counts.get(code) || 0) + 1);
      }
      return `<div class="migration-controls"><label>${t("Search algorithms, purposes or filenames", "方式・用途・ファイル名で検索")}<input type="search" maxlength="512" data-work-search value="${h(query)}" placeholder="${t("e.g. RSA, encryption, package.json", "例：RSA、暗号化、package.json")}"></label>${counts.size > 1 ? `<label>${t("Urgency", "緊急度")}<select data-work-urgency><option value="">${t("All", "すべて")}</option>${["urgent", "plan", "undetermined", "monitor", "not_applicable"].filter(code => counts.has(code)).map(code => `<option value="${code}" ${code === urgency ? "selected" : ""}>${h(state("urgency", code))} (${n(counts.get(code))} ${t("items", "項目")})</option>`).join("")}</select></label>` : `<p class="single-urgency">${t("All work items", "全確認項目の緊急度")}: ${h(state("urgency", [...counts.keys()][0]))}</p>`}</div>`;
    }
    function render() {
      const needle = query.trim().toLowerCase();
      const matches = groups.flatMap((g, i) => {
        const d = decision(g.members[0]),
          tg = target(d);
        const text = [
          state("action", d.action.kind),
          d.system_id,
          word(tg.source_ref.role),
          word(tg.technical.purpose),
          ...d.reason_codes.map(word),
        ]
          .join(" ")
          .toLowerCase();
        return (!urgency || d.urgency === urgency) &&
          (text.includes(needle) || recordSearch[i].includes(needle))
          ? [i]
          : [];
      });
      page = Math.max(0, Math.min(page, Math.ceil(matches.length / size) - 1));
      if (standalone && payload.label) document.title = `${payload.label} | ${t("Migration review", "移行レポート")} | pqc-scan`;
      active.innerHTML = `<header class="top"><div><p class="eyebrow">pqc-scan / ${t("Migration review", "移行レポート")}</p><h1>${t("Migration response work items", "移行に向けた確認事項")}</h1>${payload.label ? `<p class="report-scope">${h(payload.label)}</p>` : ""}</div>${evaluationTime()}</header><section class="summary"><div class="metric"><div class="metric-label">${t("Work items", "確認項目")}</div><strong class="metric-value">${n(groups.length)}</strong></div><div class="metric"><div class="metric-label">${t("Saved records", "保存された記録")}</div><strong class="metric-value">${n(decisions.length)}</strong></div><div class="metric"><div class="metric-label">${t("Urgent records", "早急な対応が必要な記録")}</div><strong class="metric-value">${n(decisions.reduce((a, ref) => a + (value(child(ref, "urgency")) === "urgent" ? 1 : 0), 0))}</strong></div><div class="metric"><div class="metric-label">${t("Records with undetermined urgency", "緊急度が未判定の記録")}</div><strong class="metric-value">${n(decisions.reduce((a, ref) => a + (value(child(ref, "urgency")) === "undetermined" ? 1 : 0), 0))}</strong></div></section>${controls()}<section id="decisions">${
        matches
          .slice(page * size, (page + 1) * size)
          .map((i) => card(groups[i], i))
          .join("") ||
        `<p>${decisions.length ? t("No matching work items.", "一致する確認項目はありません。") : t("No assessment targets were found in this input. This does not establish that cryptography is absent, safe, or needs no action.", "この入力から評価対象を検出できませんでした。暗号利用がない、安全、対応不要と確認できた結果ではありません。")}</p>`
      }</section><div class="target-pagination"><span>${matches.length ? n(page * size + 1) + "-" + n(Math.min((page + 1) * size, matches.length)) : "0"} / ${n(matches.length)}</span><div><button data-page="-1" ${page === 0 ? "disabled" : ""}>${t("Previous", "前へ")}</button><button data-page="1" ${(page + 1) * size >= matches.length ? "disabled" : ""}>${t("Next", "次へ")}</button></div></div><details class="assessment-scope"><summary>${t("Inspection scope and assessment conditions", "確認範囲と評価条件")}</summary><p>${h(word(field("coverage").collection_outcome))} · ${h(field("input").collection_time)}</p>${list(field("limitations"))}<dl>${[
        "catalog",
        "policy",
        "model",
        "input",
      ]
        .map(
          (k) =>
            `<dt>${h(k)}</dt><dd>${h(
              Object.values(field(k))
                .filter((v) => typeof v === "string")
                .join(" · "),
            )}</dd>`,
        )
        .join(
          "",
        )}</dl></details><footer class="migration-links"><button data-reference="claims">${t("Declarations", "申告内容")} (${n(claims.length)})</button><button data-reference="sources">${t("Sources", "出典")} (${n(sources.length)})</button><button data-download>${t("Save assessment data", "評価データを保存")}</button>${standaloneExport ? "" : `<a href="${h(artifactBase)}migration.assessment.md">Markdown</a>`}</footer><dialog class="migration-dialog" aria-label="${t("Evidence", "根拠")}"><div data-dialog-body></div></dialog>`;
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
    function claimHtml(c, i) {
      const freshness = {
        active: t("Active declared interval", "申告期間内"),
        stale: t("Expired declaration", "申告は失効"),
        not_yet_effective: t("Not yet effective", "申告の開始前"),
        unknown: t("Expiry unknown", "有効期限不明"),
      };
      return `<section id="claim-${i + 1}"><h3>${h(word(c.field))}</h3><p>${h(claimValue(c))} · ${h(freshness[c.freshness] || word(c.freshness))}</p><p>${h(c.provenance?.actor_id || "")} · ${h(c.provenance?.asserted_at || "")}</p><details><summary>${t("Reference identity", "参照情報")}</summary><table><tbody>${row(t("Claim ID", "申告ID"), c.id)}${row(t("Declared source", "申告の由来"), c.provenance?.source_kind === "operator_claim" ? t("Operator declaration", "担当者による申告") : t("Declared external review (unverified)", "外部確認を受けたとの申告（未検証）"))}${row(t("Valid until", "有効期限"), c.provenance?.valid_until || t("Unspecified", "指定なし"))}${row(t("Reference", "参照先"), c.provenance?.reference_id || t("Unspecified", "指定なし"))}${row(t("Context pointer", "入力内の場所"), c.context_pointer || "")}</tbody></table></details></section>`;
    }
    function recordHtml(i) {
      const d = decision(i),
        tg = target(d);
      let html = `<section id="target-${i + 1}"><h3>${h(algorithm(tg.technical.identifier))}</h3><p class="path">${h(recordLocation(tg))}</p><p>${h(state("urgency", d.urgency))} · ${h(state("information", d.information))} · ${h(state("readiness", d.readiness))}</p>${list(d.reason_codes)}${unbindable.has(tg.id) ? bindingNotice() : ""}<table><tbody>${row(t("Observation method", "検出方法"), word(tg.observation.method))}${row(t("Role", "役割"), word(tg.source_ref.role))}${tg.source_ref.dependency_provenance ? row(t("Dependency declaration", "依存関係の宣言"), tg.source_ref.dependency_provenance === "manifest_dev_declared" ? t("Declared development dependency", "開発用依存関係として宣言") : word(tg.source_ref.dependency_provenance)) : ""}${row(t("Observed purpose", "検出結果の用途"), word(tg.technical.purpose))}${row(t("Assessment system ID", "評価上のシステムID"), d.system_id || t("Unbound", "未対応付け"))}${row(t("Quantum assessment", "量子計算に対する評価"), word(tg.technical.quantum))}${row(t("Evidence assurance", "確認できている範囲"), word(tg.observation.assurance))}${row(t("Collection", "収集状況"), word(tg.observation.collection_outcome))}${row(t("Parameter encoding", "パラメーターの符号化"), word(tg.technical.encoding_assurance))}${(tg.technical.classical_concerns || []).map((c) => row(t("Classical concern", "従来暗号の懸念"), word(c))).join("")}${(tg.technical.components || []).map((c) => row(t("Hybrid component", "ハイブリッド構成要素"), `${word(c.role)} / ${algorithm(c.assessment.identifier)} / ${word(c.assessment.quantum)}`)).join("")}${tg.technical.combiner_assessment ? row(t("Combiner assessment", "結合処理の確認範囲"), word(tg.technical.combiner_assessment)) : ""}</tbody></table>${list(tg.limitations)}<h4>${t("Next checks", "次に確認すること")}</h4>${list(d.action.validation_steps)}<h4>${t("Claims used", "判断に使った申告")}</h4>${d.applied_claim_ids
        .map((id) => {
          const entry = claimById.get(id);
          return claimHtml(value(entry.ref), entry.index);
        })
        .join("")}`;
      if (d.action.candidate_ids.length)
        html +=
          `<h4>${t("Candidates: integration unverified", "検討候補（組込みは未検証）")}</h4>` +
          d.action.candidate_ids
            .map((id) => {
              const c = value(candidateById.get(id).ref);
              return `<details id="candidate-${candidateById.get(id).index + 1}"><summary>${h(c.identifier)} · ${h(word(c.parameter_selection))}</summary><table><tbody>${c.prerequisites.map((p) => row(word(p.name), p.claimed_support === null ? word("unknown") : p.claimed_support ? t("Declared supported", "申告上は対応") : t("Declared unsupported", "申告上は未対応"))).join("")}</tbody></table>${list(c.validation_steps)}<p>${h(c.id)}</p></details>`;
            })
            .join("");
      html += `<h4>${t("Sources", "出典")}</h4>${d.source_ids.map((id) => sourceHtml(value(sourceById.get(id).ref), sourceById.get(id).index)).join("")}<details id="evidence-${i + 1}"><summary>${t("Reference identities", "参照情報")}</summary><p>${h(tg.id)}<br>${h(d.id)}</p></details></section>`;
      return html;
    }
    function sourceHtml(s, i) {
      return `<section id="source-${i + 1}"><h3>${h(s.id)} · ${h(s.edition)}</h3><p>${h(s.clause)}</p><p>${h(s.scope_limit)}</p>${/^https?:\/\//.test(s.uri || "") ? `<a href="${h(s.uri)}" target="_blank" rel="noreferrer noopener">${h(s.uri)}</a>` : ""}</section>`;
    }
    const closeButton = () =>
      `<button class="dialog-close" data-close aria-label="${t("Close", "閉じる")}">×</button>`;
    function renderDialog() {
      const dialog = active.querySelector("dialog"),
        body = dialog.querySelector("[data-dialog-body]");
      if (detail !== null) {
        body.innerHTML = `<div class="dialog-head"><button data-list>${t("Back to list", "一覧に戻る")}</button>${closeButton()}</div>${recordHtml(detail)}`;
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
      const needle = detailQuery.toLowerCase();
      if (viewer.kind === "work" || viewer.kind === "target")
        rows = rows.filter((i) => {
          const tg = target(decision(i));
          return [
            tg.technical.identifier,
            algorithm(tg.technical.identifier),
            recordLocation(tg),
          ]
            .join(" ")
            .toLowerCase()
            .includes(needle);
        });
      detailPage = Math.max(
        0,
        Math.min(detailPage, Math.ceil(rows.length / size) - 1),
      );
      const title =
        viewer.kind === "claims"
          ? t("Declarations", "申告内容")
          : viewer.kind === "sources"
            ? t("Sources", "出典")
            : t("Evidence records", "根拠となる記録");
      const selected = rows.slice(detailPage * size, (detailPage + 1) * size);
      body.innerHTML = `<div class="dialog-head"><h2>${title}</h2>${closeButton()}</div>${
        ["work", "target"].includes(viewer.kind)
          ? `<input type="search" maxlength="512" data-detail-search aria-label="${t("Search records", "記録を検索")}" placeholder="${t("Algorithm or file", "方式・ファイル名")}" value="${h(detailQuery)}"><div class="table-scroll"><table><thead><tr><th>${t("Algorithm", "暗号方式")}</th><th>${t("Location", "場所")}</th><th>${t("Observation method", "検出方法")}</th><th></th></tr></thead><tbody>${
              selected
                .map((i) => {
                  const tg = target(decision(i));
                  return `<tr><td>${h(algorithm(tg.technical.identifier))}</td><td class="path">${h(recordLocation(tg))}</td><td>${h(word(tg.observation.method))}</td><td><button data-target="${i}">${t("Details", "詳細")}</button></td></tr>`;
                })
                .join("") ||
              `<tr><td colspan="4">${t("No records match.", "一致する記録はありません。")}</td></tr>`
            }</tbody></table></div>`
          : selected
              .map((i) =>
                viewer.kind === "claims"
                  ? claimHtml(value(claims[i]), i)
                  : sourceHtml(value(sources[i]), i),
              )
              .join("")
      }<div class="target-pagination"><span>${rows.length ? n(detailPage * size + 1) + "-" + n(Math.min((detailPage + 1) * size, rows.length)) : "0"} / ${n(rows.length)}</span><div><button data-detail-page="-1" ${detailPage === 0 ? "disabled" : ""}>${t("Previous", "前へ")}</button><button data-detail-page="1" ${(detailPage + 1) * size >= rows.length ? "disabled" : ""}>${t("Next", "次へ")}</button></div></div>`;
    }
    function open(kind, index, record = null) {
      viewer = { kind, index };
      detail = record;
      detailPage = 0;
      detailQuery = "";
      renderDialog();
      active.querySelector("dialog").showModal();
    }
    function jump() {
      const match = /^#(target|evidence|claim|source)-(\d+)$/.exec(
        location.hash,
      );
      if (!match) return;
      const i = Number(match[2]) - 1;
      if (!Number.isSafeInteger(i) || i < 0) return;
      if (["target", "evidence"].includes(match[1]) && i < decisions.length)
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
          else if (el.dataset.reference) open(el.dataset.reference);
          else if (el.hasAttribute("data-close")) d.close();
          else if (el.hasAttribute("data-download")) download();
          else if (el.dataset.page) {
            page += Number(el.dataset.page);
            render();
          } else if (el.dataset.target !== undefined) {
            detail = Number(el.dataset.target);
            renderDialog();
          } else if (el.hasAttribute("data-list")) {
            detail = null;
            renderDialog();
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
          if (event.target.hasAttribute("data-work-search")) {
            query = event.target.value;
            page = 0;
            render();
            const control = active.querySelector("[data-work-search]");
            control.focus();
            control.setSelectionRange(query.length, query.length);
          }
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
      view.addEventListener(
        "change",
        (event) => {
          if (event.target.hasAttribute("data-work-urgency")) {
            urgency = event.target.value;
            page = 0;
            render();
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
    );
})();
