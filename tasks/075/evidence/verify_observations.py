"""Post-observation equality/denominator recorder, excluded from measured runs."""
import hashlib, json, pathlib, shutil, subprocess, urllib.request
ROOT = pathlib.Path(__file__).resolve().parents[3]
BASE = ROOT / "tasks/075/evidence"
WEBROOT = pathlib.Path("/private/var/folders/vp/sb96fq311gl__cjkxvhswf040000gp/T/pqc-scan-web-032cfd85b96a61dd762286e3019fdc1246ddcdfbd28ff243727f437d028a6f9e")
URL = "http://127.0.0.1:62524/31537bed1e666b560f102a30bccec0b453dc425abec8a85063fdf1e7f5849331/"
freeze=json.loads((BASE/"FREEZE.json").read_text()); cli=ROOT/"target/debug/pqc-scan"
assert hashlib.sha256(cli.read_bytes()).hexdigest()==freeze["cli_sha256"]
records=[]; commands=[]; checks=[]
def post(route,data):
 req=urllib.request.Request(URL+route,data=json.dumps(data).encode(),headers={"Content-Type":"application/json","Origin":"http://127.0.0.1:62524"})
 with urllib.request.urlopen(req) as response: return json.load(response)
def assess(case,ctx,out):
 args=[str(cli),"assess",str(BASE/"input.inventory.json"),"--input-format","inventory-v1","--context",str(ctx),"--as-of",freeze["as_of"],"--lang","ja","--out-dir",str(out)]
 r=subprocess.run(args,cwd=ROOT,capture_output=True,text=True); commands.append({"command":args,"exit":r.returncode,"stdout":r.stdout,"stderr":r.stderr}); assert r.returncode==0
 return json.loads((out/"migration.assessment.v1.json").read_text())
for case,job in [("P1",1),("O1",2)]:
 m,c,w=[BASE/f"{case}-{route}" for route in "MCW"]
 for x in (WEBROOT/str(job)/"migration").iterdir():
  if x.name.startswith("migration.assessment"): shutil.copyfile(x,w/x.name)
 assert (WEBROOT/str(job)/"scan.inventory.v1.json").read_bytes()==(BASE/"input.inventory.json").read_bytes()
 assert (WEBROOT/str(job)/"migration/migration-context.json").read_bytes()==(w/"migration-context.json").read_bytes()
 mc,cc,wc=[json.loads((x/"migration-context.json").read_text()) for x in (m,c,w)]
 assert mc==cc==wc
 # Actual shared MigrationContext::to_bytes through existing ImportContext preview.
 canonical=post(f"api/migration/preview?job={job}",{"mode":"import_context","input_sha256":freeze["input_sha256"],"as_of":freeze["as_of"],"context":(m/"migration-context.json").read_text()})
 (m/"canonical-preview.json").write_text(json.dumps(canonical,indent=2)+"\n")
 (m/"canonical-context.json").write_text(canonical["context"])
 assert (m/"canonical-context.json").read_bytes()==(c/"migration-context.json").read_bytes()==(w/"migration-context.json").read_bytes()
 manual=assess(case,m/"canonical-context.json",m/"canonical-reports")
 assisted=json.loads((c/"reports/migration.assessment.v1.json").read_text())
 web=json.loads((w/"migration.assessment.v1.json").read_text())
 web_cli=assess(case,w/"migration-context.json",w/"cli-reports")
 assert manual==assisted==web==web_cli
 d=web["decisions"][0]
 assert d["urgency"]==("undetermined" if case=="P1" else "urgent")
 assert d["readiness"]==("unknown" if case=="P1" else "blocked")
 if case=="O1": assert d["action"]["blocking_prerequisites"]==["peer"]
 for route in "MCW":
  x=BASE/f"{case}-{route}"; ctx=json.loads((x/"migration-context.json").read_text()); cl=ctx["bindings"][0]["claims"]; leaves={k:v for k,v in cl.items() if k!="support"};leaves.update({"support."+k:v for k,v in cl.get("support",{}).items()})
  known=sum(v["state"]=="known" for v in leaves.values()); unknown=sum(v["state"]=="unknown" for v in leaves.values()); omitted=18-len(leaves)
  assert (known,unknown,omitted)==((3,2,13) if case=="P1" else (12,0,6))
  observation=json.loads((x/("browser-observation.json" if route=="W" else "observation.json")).read_text())
  md=(w/"migration.assessment.md" if route=="W" else x/"reports/migration.assessment.md").read_text()
  expected=["実際の利用状況と必要な暗号用途を確認してください。","不足・不明・失効している対象範囲の申告を確認し、更新してください。"] if case=="P1" else ["接続相手の対応"]
  assert all(text in md for text in expected)
  if route=="W": assert all(text in (w/"result.ax.txt").read_text() for text in expected)
  records.append({"id":f"{case}-{route}","attempted":True,"completed":True,"elapsed_seconds":observation["elapsed_seconds"],"timing":observation["timing"],"known":known,"unknown":unknown,"omitted_of_18_supported_leaves":omitted,"product_errors":observation.get("product_validation_errors",observation.get("errors",0)),"tool_errors":observation.get("selector_failures",0),"retries":observation["retries"],"urgency":d["urgency"],"information":d["information"],"readiness":d["readiness"],"next_action_checked":True,"next_action_text":expected})
 checks.append({"case":case,"manual_assisted_web_context_semantic_equal":True,"shared_canonical_bytes_equal":True,"complete_json_equal_after_manual_canonicalization":True,"actual_web_export_cli_json_equal":True,"same_inventory_bytes":True,"download_applied_bytes_equal":True,"decision":d})
summary={"source":freeze,"planned_runs":6,"attempted_runs":len(records),"completed_runs":sum(r["completed"] for r in records),"product_errors":sum(r["product_errors"] for r in records),"tool_errors":sum(r["tool_errors"] for r in records),"next_actions_checked":sum(r["next_action_checked"] for r in records),"records":records,"comparisons":checks,"post_measurement_commands":commands,"limitations":["One implementation-author AI, exposed synthetic fixtures, fixed route order; no human usefulness or speedup inference.","CLI timings exclude human drafting/typing; Web timing includes orchestration and selector recovery.","Raw manual context property order/format hashes and claim/decision IDs differ; preserved originals. Canonical verification uses actual shared exporter through import-preview and is outside timing.","Eighteen supported leaves include deadline (advanced-only) and all support fields; irrelevant omissions are not failures.","Two extra conformance controls reuse applicable task074 actual-browser cases, outside six-run denominator;071/072 and U09 remain unfinished."]}
(BASE/"OBSERVATIONS.json").write_text(json.dumps(summary,indent=2)+"\n")
print(json.dumps({k:v for k,v in summary.items() if k not in ("records","comparisons","post_measurement_commands","source")},indent=2))
