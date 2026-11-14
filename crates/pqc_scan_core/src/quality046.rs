//! Finite, seeded properties and a real counterexample-reduction control.
use super::*;

#[test]
fn quality046_reducer_preserves_failure_and_reaches_one_minimal_input() {
    let failure = |xs: &[u8]| xs.contains(&6);
    let reduced = minimize(vec![0, 5, 6, 5, 0], failure);
    assert_eq!(reduced, vec![6]);
    assert!(failure(&reduced));
    assert!(!failure(&[]));
}

const KINDS: [&str; 8] = [
    "complete",
    "partial",
    "truncated",
    "failed",
    "unsupported",
    "notapplicable",
    "unknown",
    "read",
];
const SEEDS: [u64; 3] = [46, 20261113, 0x5eed_cafe];
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn text(&mut self) -> String {
        let values = ["", "a", "bc", ":", "|", "日本", "😀", "\0", "/", "é"];
        let n = (self.next() % 5) as usize;
        (0..n)
            .map(|_| values[(self.next() % values.len() as u64) as usize])
            .collect()
    }
}
fn minimize(mut value: Vec<u8>, fails: impl Fn(&[u8]) -> bool) -> Vec<u8> {
    assert!(fails(&value));
    let mut index = 0;
    while index < value.len() {
        let mut trial = value.clone();
        trial.remove(index);
        if fails(&trial) {
            value = trial;
            index = 0;
        } else {
            index += 1;
        }
    }
    value
}
fn scan(kinds: &[u8], threads: usize) -> ScanDocument {
    let fixture = Fixture::new();
    let mut config = fixture.config();
    config.threads = threads;
    for (i, kind) in kinds.iter().enumerate() {
        let path = config
            .root
            .join(format!("f{i}/{}.txt", KINDS[*kind as usize]));
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, "HIT").unwrap();
    }
    scan_observed_with_reader(
        &config,
        &fixture.rules(),
        &[Arc::new(States)],
        &read,
        Some(&crate::NativeOptions::new(Some("generated/046")).unwrap()),
        crate::FileSelection::default(),
    )
    .unwrap()
}
fn semantic(doc: &ScanDocument) -> Value {
    let mut legacy = serde_json::to_value(doc.legacy()).unwrap();
    legacy.as_object_mut().unwrap().remove("generated_at");
    let mut diagnostics = serde_json::to_value(doc.diagnostics_report()).unwrap();
    diagnostics.as_object_mut().unwrap().remove("generated_at");
    let mut native = serde_json::to_value(doc.native().unwrap()).unwrap();
    native.as_object_mut().unwrap().remove("run");
    // Inventory v1 defines observation IDs as run-local. Alpha-rename only those
    // IDs and their typed references; keep findings/assets/occurrence keys exact.
    doc.native().unwrap().validate().unwrap();
    let ids: BTreeMap<String, String> = native["observations"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, o)| {
            (
                o["id"].as_str().unwrap().to_string(),
                format!("observation046:{i}"),
            )
        })
        .collect();
    for o in native["observations"].as_array_mut().unwrap() {
        o["id"] = json!(ids[o["id"].as_str().unwrap()]);
    }
    for a in native["legacy_associations"].as_array_mut().unwrap() {
        for id in a["observation_ids"].as_array_mut().unwrap() {
            *id = json!(ids[id.as_str().unwrap()]);
        }
    }
    for id in native["unassociated_observation_ids"]
        .as_array_mut()
        .unwrap()
    {
        *id = json!(ids[id.as_str().unwrap()]);
    }
    for loss in native["legacy_loss_records"].as_array_mut().unwrap() {
        loss["observation_id"] = json!(ids[loss["observation_id"].as_str().unwrap()]);
    }
    json!({"legacy":legacy,"diagnostics":diagnostics,"native":native})
}
fn check(kinds: &[u8]) -> std::result::Result<Value, String> {
    let doc = scan(kinds, 1);
    let actual = serde_json::to_value(doc.coverage()).unwrap();
    let count = |kind: u8| kinds.iter().filter(|x| **x == kind).count() as u64;
    let expected = [
        ("selected_files", kinds.len() as u64),
        ("read_ok_files", kinds.len() as u64 - count(7)),
        ("read_failed_files", count(7)),
        ("analyzed_files", count(0)),
        ("partial_files", count(1) + count(2)),
        ("analysis_failed_files", count(3)),
        ("unsupported_files", count(4)),
        ("not_applicable_files", count(5)),
        ("unknown_files", count(6)),
    ];
    for (name, n) in expected {
        if actual[name] != n {
            return Err(format!("{name}: expected {n}, actual {}", actual[name]));
        }
    }
    if doc.legacy().findings.len() as u64 != count(0) + count(1) + count(2) {
        return Err("findings not retained".into());
    }
    if (count(1) + count(2) + count(3) + count(4) + count(6) + count(7) > 0)
        && doc.coverage().is_complete()
    {
        return Err("incomplete inspection became complete".into());
    }
    let left = semantic(&doc);
    let repeated = scan(kinds, 2);
    let right = semantic(&repeated);
    if left != right {
        save(
            "semantic046-debug.json",
            &json!({"left":left,"right":right}),
        );
        return Err("semantic repeat/thread/root drift".into());
    }
    let native = doc.native().unwrap();
    native.validate().map_err(|e| e.to_string())?;
    let value = serde_json::to_value(native).unwrap();
    let back: crate::inventory::NativeScan = serde_json::from_value(value.clone()).unwrap();
    back.validate().unwrap();
    if serde_json::to_value(back).unwrap() != value {
        return Err("native roundtrip drift".into());
    }
    for o in &value["observations"].as_array().unwrap()[..] {
        if o["assurance"] != "unknown" || o["usage"] != "unknown" {
            return Err("legacy assurance was fabricated".into());
        }
    }
    if !value["observations"].as_array().unwrap().is_empty() {
        let mut future = value.clone();
        let token = format!("future046_{}", kinds.len());
        future["observations"][0]["method"] = json!(token);
        future["assets"][0]["kind"] = json!("future_asset_046");
        let back: crate::inventory::NativeScan = serde_json::from_value(future.clone()).unwrap();
        back.validate().unwrap();
        let again = serde_json::to_value(back).unwrap();
        if again["observations"][0]["method"] != future["observations"][0]["method"]
            || again["assets"][0]["kind"] != "future_asset_046"
        {
            return Err("future vocabulary was guessed away".into());
        }
        future["observations"][0]["asset_ids"] = json!(["missing046"]);
        let broken: crate::inventory::NativeScan = serde_json::from_value(future).unwrap();
        if broken.validate().is_ok() {
            return Err("broken reference accepted".into());
        }
    }
    Ok(
        json!({"input":kinds,"counts":actual,"findingIds":doc.legacy().findings.iter().map(|f|&f.finding_id).collect::<Vec<_>>(),"semanticRepeat":true,"nativeRoundtrip":true,"unknownPreserved":true,"brokenReferenceRejectedWhenPresent":true}),
    )
}

#[test]
fn quality046_generated_pipeline_statistics_semantics_and_unknown_are_preserved() {
    let mut results = vec![];
    for seed in SEEDS {
        let mut rng = Rng(seed);
        for index in 0..64 {
            let kinds = if index == 0 {
                vec![]
            } else if index <= 8 {
                vec![(index - 1) as u8]
            } else {
                let n = (rng.next() % 9) as usize;
                (0..n).map(|_| (rng.next() % 8) as u8).collect()
            };
            match check(&kinds) {
                Ok(result) => results.push(json!({"seed":seed,"case":index,"result":result})),
                Err(reason) => {
                    let reduced = minimize(kinds.clone(), |input| check(input).is_err());
                    save(
                        "property046-failure.json",
                        &json!({"seed":seed,"case":index,"original":kinds,"reduced":reduced,"reason":reason}),
                    );
                    panic!("seed {seed} case {index}: {reason}; reduced={reduced:?}");
                }
            }
        }
    }
    save(
        "pipeline046.json",
        &json!({"seeds":SEEDS,"caseCount":results.len(),"results":results,"canonicalization":"remove legacy/diagnostic generated_at and native run metadata; alpha-rename run-scoped observation IDs and all typed references; keep finding/asset IDs, occurrence keys and all remaining facts exact"}),
    );
}

#[test]
fn quality046_generated_id_boundaries_are_stable_unique_and_unknown_is_not_guessed() {
    let mut seen = BTreeMap::new();
    let mut generated = vec![];
    let mut boundaries = 0;
    for seed in SEEDS {
        let mut rng = Rng(seed);
        for index in 0..128 {
            let a = rng.text();
            let b = if index == 0 {
                "a".repeat(128)
            } else {
                rng.text()
            };
            let c = rng.text();
            let tuples = [
                vec![a.clone(), format!("{b}{c}")],
                vec![format!("{a}{b}"), c.clone()],
                vec![a.clone()],
                vec![a.clone(), String::new()],
            ];
            let mut ids = vec![];
            for tuple in tuples {
                let refs: Vec<_> = tuple.iter().map(String::as_str).collect();
                let id = crate::identity::native_id("asset", &refs);
                assert_eq!(id, crate::identity::native_id("asset", &refs));
                assert_eq!(id.len(), "asset-v1-".len() + 64);
                if let Some(old) = seen.insert(id.clone(), tuple.clone()) {
                    assert_eq!(old, tuple, "distinct tuples collapsed");
                }
                let other = crate::identity::native_id("observation", &refs);
                assert_ne!(
                    id.rsplit('-').next(),
                    other.rsplit('-').next(),
                    "domain must change digest, not only prefix"
                );
                ids.push(id);
            }
            if !b.is_empty() {
                assert_ne!(ids[0], ids[1]);
                boundaries += 1;
            }
            assert_ne!(ids[2], ids[3]);
            let unknown = if index % 2 == 0 {
                format!("unrecognized046_{seed}_{index}_{a}")
            } else {
                format!("RSA_future046_{seed}_{index}_{a}")
            };
            assert!(crate::observation::AlgorithmSpec::recognized(&unknown).is_none());
            let assessment = crate::assessment::assess_algorithm(&unknown, &BTreeMap::new());
            assert_eq!(assessment.identifier, "unknown");
            assert_eq!(assessment.construction, "unknown");
            assert_eq!(assessment.standardization, "unknown");
            assert!(!assessment.parameters_known);
            assert_eq!(
                assessment.quantum,
                crate::assessment::QuantumExposure::Unknown
            );
            assert_eq!(assessment.purpose, crate::assessment::Purpose::Unknown);
            assert!(crate::observation::AlgorithmSpec::recognized("RSA").is_some());
            generated.push(
                json!({"seed":seed,"case":index,"parts":[a,b,c],"ids":ids,"unknownInput":unknown}),
            );
        }
    }
    // Explicit byte/character and framing edges supplement the generated cases.
    for n in [0, 1, 127, 128, 129] {
        let a = "a".repeat(n);
        let id = crate::identity::native_id("asset", &[&a, "日本😀"]);
        assert_eq!(id, crate::identity::native_id("asset", &[&a, "日本😀"]));
    }
    save(
        "identity046.json",
        &json!({"seeds":SEEDS,"generatedCases":generated.len(),"distinctIds":seen.len(),"concatenationBoundaryChecks":boundaries,"results":generated,"limits":"finite collision checks, not a proof of cryptographic collision resistance"}),
    );
}

#[test]
fn quality046_shrinks_real_unknown_scan_against_a_deliberately_wrong_success_rule() {
    // Negative control only: the intentionally wrong rule is not product code.
    let fails = |kinds: &[u8]| {
        let doc = scan(kinds, 1);
        doc.legacy().findings.is_empty() && !doc.coverage().is_complete()
    };
    let original = vec![5, 6, 5];
    assert!(fails(&original));
    let reduced = minimize(original.clone(), fails);
    assert_eq!(reduced, vec![6]);
    assert!(fails(&reduced));
    assert!(!fails(&[]));
    let actual = scan(&reduced, 1);
    assert_eq!(actual.coverage().status(), crate::InspectionStatus::Unknown);
    save(
        "shrinking046.json",
        &json!({"negativeControl":"wrong hypothetical rule: zero findings means completed","original":original,"reduced":reduced,"minimality":"no single element can be removed while preserving failure","actualCoverage":actual.coverage(),"productFailure":false}),
    );
}
