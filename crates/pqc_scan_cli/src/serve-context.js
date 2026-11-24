/* Authoring state only. Rust owns claim validation, binding and policy decisions. */
((scope) => {
  const clone = (value) => JSON.parse(JSON.stringify(value));
  const parameterSets = Object.freeze({
    "ML-DSA": ["ML-DSA-44", "ML-DSA-65", "ML-DSA-87"],
    "ML-KEM": ["ML-KEM-512", "ML-KEM-768", "ML-KEM-1024"],
    AES: ["AES-128", "AES-192", "AES-256"],
    "SHA-256": ["SHA-256"],
  });
  const familyFor = (purpose) =>
    ({
      signature: "ML-DSA",
      jwt: "ML-DSA",
      pki: "ML-DSA",
      key_establishment: "ML-KEM",
      encryption: "AES",
      hashing: "SHA-256",
    })[purpose] || "";
  const basisKey = (snapshot) =>
    JSON.stringify([
      snapshot.answers["use-case"],
      snapshot.candidate,
      snapshot.values["candidate-parameters"].trim(),
    ]);
  function active(snapshot, fields, required) {
    const s = clone(snapshot),
      purpose = s.answers["use-case"];
    if (s.answers.applicability === "not_applicable") {
      s.answers = { applicability: "not_applicable" };
      s.details = {};
      s.candidate = "";
      s.values["candidate-parameters"] = "";
      return s;
    }
    for (const field of fields)
      if (field.relevance && !field.relevance.split(",").includes(purpose)) {
        delete s.answers[field.id];
        delete s.details[field.id];
      }
    if (
      !["signature", "jwt", "pki"].includes(purpose) ||
      !["active", "planned"].includes(s.answers["usage-state"])
    ) {
      delete s.answers["business-critical"];
      delete s.details["business-critical"];
    }
    const complete =
      s.candidate === familyFor(purpose) &&
      !!s.candidate &&
      !!s.values["candidate-parameters"].trim();
    for (const id of Object.keys(s.answers))
      if (
        id.startsWith("support-") &&
        (!complete || !required.includes(id.slice(8)))
      )
        delete s.answers[id];
    return s;
  }
  class Groups {
    constructor() {
      this.items = [];
      this.serial = 0;
    }
    owner(id) {
      return this.items.find((g) => g.ids.includes(id));
    }
    saveNew(ids, snapshot) {
      if (
        !ids.length ||
        this.items.length >= 256 ||
        new Set(ids).size !== ids.length ||
        ids.some((id) => this.owner(id)) ||
        this.count() + ids.length > 4096
      )
        throw new Error("invalid_scope");
      const id = "g" + ++this.serial;
      this.items.push({
        id,
        name: snapshot.values["group-name"].trim() || id,
        ids: ids.slice(),
        common: clone(snapshot),
        exceptions: new Map(),
      });
      return id;
    }
    get(id) {
      return this.items.find((g) => g.id === id);
    }
    saveCommon(id, snapshot) {
      const g = this.get(id);
      g.common = clone(snapshot);
      g.name = snapshot.values["group-name"].trim() || g.name;
    }
    saveIndividual(group, id, snapshot) {
      const g = this.get(group);
      if (!g.ids.includes(id)) throw new Error("invalid_scope");
      g.exceptions.set(id, clone(snapshot));
    }
    remove(id) {
      const index = this.items.findIndex(g => g.id === id);
      if (index < 0) throw new Error("invalid_scope");
      return { index, group: this.items.splice(index, 1)[0] };
    }
    undo(removed) {
      const g = removed.group;
      if (this.get(g.id) || g.ids.some(id => this.owner(id)) ||
          this.count() + g.ids.length > 4096 || this.items.length >= 256)
        throw new Error("invalid_scope");
      this.items.splice(Math.min(removed.index, this.items.length), 0, g);
    }
    restore(group, id) {
      this.get(group).exceptions.delete(id);
    }
    count() {
      return this.items.reduce((n, g) => n + g.ids.length, 0);
    }
    effective() {
      return this.items.flatMap((g) =>
        g.ids.map((id) => ({
          id,
          snapshot: clone(g.exceptions.get(id) || g.common),
        })),
      );
    }
  }
  function timestamp(value) {
    if (!/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}(?::\d{2})?$/.test(value))
      throw new Error("invalid_time");
    return value.length === 16 ? value + ":00+09:00" : value + "+09:00";
  }
  // Gregorian calendar arithmetic; no fixed number of days per month.
  function calendarDuration(anchor, years, months) {
    if (!/^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}(?::\d{2})?$/.test(anchor) ||
        !/^(0|[1-9]\d*)$/.test(String(years)) ||
        !/^(0|[1-9]\d*)$/.test(String(months))) throw new Error("invalid_duration");
    years = Number(years); months = Number(months);
    if (years > 10 || months > 11) throw new Error("invalid_duration");
    const start = new Date(timestamp(anchor));
    const local = new Date(start.getTime() + 9 * 3600000);
    if (!Number.isFinite(start.getTime()) || local.toISOString().slice(0, 19) !== (anchor.length === 16 ? anchor + ":00" : anchor) || local.getUTCFullYear() < 1) throw new Error("invalid_duration");
    const day = local.getUTCDate();
    local.setUTCDate(1);
    local.setUTCMonth(local.getUTCMonth() + years * 12 + months);
    const endOfMonth = new Date(local.getTime());
    endOfMonth.setUTCMonth(endOfMonth.getUTCMonth() + 1);
    endOfMonth.setUTCDate(0);
    local.setUTCDate(Math.min(day, endOfMonth.getUTCDate()));
    if (local.getUTCFullYear() > 9999) throw new Error("invalid_duration");
    const seconds = (local.getTime() - 9 * 3600000 - start.getTime()) / 1000;
    if (!Number.isSafeInteger(seconds) || seconds < 0 || seconds > 315576000)
      throw new Error("invalid_duration");
    return { seconds, end: local.toISOString().slice(0, 19) };
  }
  const api = Object.freeze({
    clone,
    parameterSets,
    familyFor,
    basisKey,
    active,
    Groups,
    timestamp,
    calendarDuration,
  });
  if (typeof module === "object" && module.exports) module.exports = api;
  else scope.PqcContext = api;
})(typeof window === "object" ? window : undefined);
