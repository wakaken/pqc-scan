const { test } = require("node:test");
const assert = require("node:assert/strict");
const {
  Groups,
  active,
  parameterSets,
  timestamp,
} = require("../src/serve-context.js");
const draft = (configuration = "ML-DSA-65") => ({
  answers: {
    applicability: "applicable",
    "use-case": "jwt",
    "usage-state": "?",
    "support-runtime": "true",
    "support-jose": "false",
    confidential: "true",
  },
  details: { "lead-time": "" },
  values: {
    "group-name": "auth",
    "candidate-parameters": configuration,
    actor: "owner",
  },
  candidate: "ML-DSA",
  parameterDrafts: { "ML-DSA": { choice: configuration, custom: "custom-X" } },
});
test("shared edits preserve individual full snapshots and explicit restore uses latest common values", () => {
  const groups = new Groups();
  const input = draft();
  const id = groups.saveNew(["1", "2"], input);
  input.answers["support-runtime"] = "false";
  assert.equal(groups.get(id).common.answers["support-runtime"], "true");
  groups.saveIndividual(id, "1", draft("ML-DSA-44"));
  groups.saveCommon(id, draft("ML-DSA-87"));
  assert.deepEqual(
    groups.effective().map((x) => x.snapshot.values["candidate-parameters"]),
    ["ML-DSA-44", "ML-DSA-87"],
  );
  groups.restore(id, "1");
  assert.deepEqual(
    groups.effective().map((x) => x.snapshot.values["candidate-parameters"]),
    ["ML-DSA-87", "ML-DSA-87"],
  );
});
test("scope is fixed, unique and bounded, never silently transferred to another group", () => {
  const groups = new Groups();
  groups.saveNew(["1"], draft());
  assert.throws(() => groups.saveNew(["1", "2"], draft()));
  assert.throws(() => groups.saveNew(["3", "3"], draft()));
  assert.throws(() => groups.saveIndividual("g1", "unknown", draft()));
  assert.throws(() =>
    groups.saveNew(
      Array.from({ length: 4096 }, (_, i) => String(i + 2)),
      draft(),
    ),
  );
  assert.equal(groups.count(), 1);
});
test("inactive values are omitted while raw draft and explicit unknown remain intact", () => {
  const raw = draft();
  const fields = [
    { id: "confidential", relevance: "encryption,key_establishment" },
  ];
  let applied = active(raw, fields, ["runtime"]);
  assert.equal(applied.answers["usage-state"], "?");
  assert.equal(applied.answers.confidential, undefined);
  assert.equal(applied.answers["support-jose"], undefined);
  assert.equal(applied.answers["support-runtime"], "true");
  raw.answers.applicability = "not_applicable";
  applied = active(raw, fields, ["runtime", "jose"]);
  assert.deepEqual(applied.answers, { applicability: "not_applicable" });
  assert.equal(applied.candidate, "");
  assert.equal(raw.answers["support-jose"], "false");
  raw.answers.applicability = "applicable";
  assert.equal(
    active(raw, fields, ["runtime", "jose"]).answers["support-jose"],
    "false",
  );
});
test("incomplete or unrelated configuration cannot carry support claims", () => {
  const raw = draft("");
  assert.equal(
    active(raw, [], ["runtime"]).answers["support-runtime"],
    undefined,
  );
  raw.values["candidate-parameters"] = "ML-KEM-768";
  raw.candidate = "ML-KEM";
  assert.equal(
    active(raw, [], ["runtime"]).answers["support-runtime"],
    undefined,
  );
});
test("presets name standard sets and dates retain the explicit Japan offset", () => {
  assert.deepEqual(parameterSets["ML-DSA"], [
    "ML-DSA-44",
    "ML-DSA-65",
    "ML-DSA-87",
  ]);
  assert.deepEqual(parameterSets["ML-KEM"], [
    "ML-KEM-512",
    "ML-KEM-768",
    "ML-KEM-1024",
  ]);
  assert.equal(timestamp("2026-11-19T09:30"), "2026-11-19T09:30:00+09:00");
  assert.equal(timestamp("2026-11-19T09:30:45"), "2026-11-19T09:30:45+09:00");
  assert.throws(() => timestamp("2026-11-19"));
});

test("business criticality is dormant without an active or planned signing purpose", () => {
  const raw=draft();raw.answers['business-critical']='true';
  assert.equal(active(raw,[],[]).answers['business-critical'],undefined);
  raw.answers['usage-state']='active';assert.equal(active(raw,[],[]).answers['business-critical'],'true');
  raw.answers['use-case']='encryption';assert.equal(active(raw,[],[]).answers['business-critical'],undefined);
  assert.equal(raw.answers['business-critical'],'true');
});
test("distinct use groups respect the256 system bound before another group is saved", () => {
  const groups=new Groups();for(let i=1;i<=256;i++)groups.saveNew([String(i)],draft());
  assert.throws(()=>groups.saveNew(['257'],draft()));assert.equal(groups.items.length,256);assert.equal(groups.count(),256);
});


test("deleting a use releases membership; undo restores exact overrides without touching other groups", () => {
 const store = new Groups();
 const a = store.saveNew(["1", "2"], draft());
 const b = store.saveNew(["3"], draft("ML-DSA-87"));
 store.saveIndividual(a, "2", draft("ML-DSA-44"));
 const before = store.effective();
 const removed = store.remove(a);
 assert.equal(store.owner("1"), undefined);
 assert.equal(store.count(), 1);
 assert.equal(store.get(b).common.values["candidate-parameters"], "ML-DSA-87");
 store.undo(removed);
 assert.deepEqual(store.effective(), before);
 const again = store.remove(a);
 store.saveNew(["2"], draft());
 assert.throws(() => store.undo(again), /invalid_scope/);
 assert.equal(store.owner("1"), undefined, "conflict cannot partially restore membership");
});

test("calendar years/months use actual Japan calendar days and clamp leap/month ends", () => {
 const { calendarDuration } = require("../src/serve-context.js");
 assert.deepEqual(calendarDuration("2028-01-31T00:00:00", 0, 1), {seconds:29*86400,end:"2028-02-29T00:00:00"});
 assert.deepEqual(calendarDuration("2028-02-29T12:34:56", 1, 0), {seconds:365*86400,end:"2029-02-28T12:34:56"});
 assert.equal(calendarDuration("2026-01-01T00:00:00", 0, 1).seconds,31*86400);
 assert.equal(calendarDuration("2026-04-01T00:00:00", 0, 1).seconds,30*86400);
 assert.equal(calendarDuration("2026-01-01T00:00:00", 0, 0).seconds,0);
 for (const [anchor,y,m] of [["2026-02-30T00:00:00",0,1],["2026-01-01T00:00:00",-1,0],["2026-01-01T00:00:00",0,12],["2026-01-01T00:00:00",1.5,0],["2026-01-01T00:00:00",10,1],["9999-12-01T00:00:00",0,1],["0000-01-01T00:00:00",1,0]]) assert.throws(()=>calendarDuration(anchor,y,m), /invalid_duration/);
});


test("migration interface supplies each literal controller element dependency", () => {
 const { readFileSync } = require("node:fs");
 const { join } = require("node:path");
 const source = readFileSync(join(__dirname,"../src/serve-migration.js"),"utf8");
 const html = readFileSync(join(__dirname,"../src/serve-migration.html"),"utf8") + readFileSync(join(__dirname,"../src/serve.html"),"utf8");
 const supplied = new Set([...html.matchAll(/id="([\w-]+)"/g), ...source.matchAll(/\.id = "([\w-]+)"/g)].map(m=>m[1]));
 for (const [,id] of source.matchAll(/\$\("([\w-]+)"\)/g)) assert.ok(supplied.has(id), `missing bundled element ${id}`);
});
