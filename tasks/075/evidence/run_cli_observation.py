"""Author-run bounded U08 command recorder; not a human speed benchmark."""
import datetime, hashlib, json, pathlib, subprocess, sys, time
ROOT = pathlib.Path(__file__).resolve().parents[3]
BASE = ROOT / "tasks/075/evidence"
CASE, ROUTE = sys.argv[1:]
assert CASE in ("P1", "O1") and ROUTE in ("M", "C")
OUT = BASE / f"{CASE}-{ROUTE}"
assert not OUT.exists(), "Preserve any prior attempt; do not overwrite"
OUT.mkdir()
CTX = OUT / "migration-context.json"
CLI = ROOT / "target/debug/pqc-scan"
INPUT = BASE / "input.inventory.json"
ASOF = "2026-11-18T00:00:00Z"
freeze = json.loads((BASE / "FREEZE.json").read_text())
assert hashlib.sha256(CLI.read_bytes()).hexdigest() == freeze["cli_sha256"]
# Explicit author transcription; no generation/validation helper writes manual JSON.
prov = {"source_kind":"operator_claim", "actor_id":"owner", "asserted_at":"2026-11-17T00:00:00Z"}
if CASE == "O1": prov["valid_until"] = "2027-11-17T00:00:00Z"
claims = {
 "applicability":{"state":"known","value":"applicable","provenance":prov},
 "use_case":{"state":"known","value":"signature","provenance":prov},
 "usage_state":{"state":"known","value":"active","provenance":prov},
 "business_critical":{"state":"unknown","provenance":prov} if CASE == "P1" else {"state":"known","value":True,"provenance":prov},
}
if CASE == "P1": claims["signature_verification_until"] = {"state":"unknown","provenance":prov}
else:
 claims["support"] = {
  "runtime":{"state":"known","value":True,"provenance":prov},
  "provider":{"state":"known","value":True,"provenance":prov},
  "protocol":{"state":"known","value":True,"provenance":prov},
  "parameters":{"state":"known","value":True,"provenance":prov},
  "jose":{"state":"known","value":True,"provenance":prov},
  "verifiers":{"state":"known","value":True,"provenance":prov},
  "pki":{"state":"known","value":True,"provenance":prov},
  "peer":{"state":"known","value":False,"provenance":prov}}
manual = {"format":"pqc-scan.migration-context","version":1,"organization_id":"organization",
 "input":{"format":"inventory-v1","sha256":"ea072032ff581bb287e010de83d5c9be0f72debdce0962925dfa3c5dbf29f81f"},
 "systems":[{"id":"system-1"}],"bindings":[{"system_id":"system-1",
 "target_id":"migration_target-v1-114701d84e5655c8e85a185e3d38612b5764dbe9a416e76facefd000102676ad","claims":claims}]}
commands=[]
def run(args):
 r=subprocess.run([str(x) for x in args],cwd=ROOT,capture_output=True,text=True)
 commands.append({"command":[str(x) for x in args],"exit":r.returncode,"stdout":r.stdout,"stderr":r.stderr})
 return r.returncode
started=datetime.datetime.now(datetime.timezone.utc).isoformat(); start=time.monotonic_ns()
if ROUTE == "M":
 CTX.write_text(json.dumps(manual,indent=2)+"\n")
else:
 args=[CLI,"context-template",INPUT,"--input-format","inventory-v1","--as-of",ASOF,"--out",CTX,"--target","1",
 "--set",'applicability="applicable"',"--set",'use_case="signature"',"--set",'usage_state="active"',"--actor","owner","--asserted-at","2026-11-17T00:00:00Z"]
 if CASE == "P1": args += ["--unknown","business_critical","--unknown","signature_verification_until"]
 else:
  args += ["--valid-until","2027-11-17T00:00:00Z","--set","business_critical=true"]
  for field in ["runtime","provider","protocol","parameters","jose","verifiers","pki"]: args += ["--set",f"support.{field}=true"]
  args += ["--set","support.peer=false"]
 run(args)
if CTX.exists(): run([CLI,"assess",INPUT,"--input-format","inventory-v1","--context",CTX,"--as-of",ASOF,"--lang","ja","--out-dir",OUT/"reports"])
elapsed=(time.monotonic_ns()-start)/1e9
result={"id":f"{CASE}-{ROUTE}","started_at":started,"elapsed_seconds":elapsed,"timing":"monotonic context write/generation + actual CLI assess execution; excludes author drafting/human typing","attempted":True,"completed":bool(commands) and all(c["exit"]==0 for c in commands),"commands":commands,"operator":freeze["operator"],"source_sha256":freeze["source_sha256"],"cli_sha256":freeze["cli_sha256"],"input_sha256":freeze["input_sha256"],"unknown_answers":2 if CASE=="P1" else 0,"omitted_answers_of_18_supported_leaves":13 if CASE=="P1" else 6,"known_answers":3 if CASE=="P1" else 12,"errors":sum(c["exit"]!=0 for c in commands),"retries":0}
result["artifact_sha256"]={str(x.relative_to(BASE)):hashlib.sha256(x.read_bytes()).hexdigest() for x in OUT.rglob("*") if x.is_file()}
(OUT/"observation.json").write_text(json.dumps(result,indent=2)+"\n")
print(json.dumps({k:v for k,v in result.items() if k not in ("commands","artifact_sha256")},indent=2))
