"""Offline development evaluator. Does not alter scanner CLI, rules or reports."""
import copy
import hashlib
import json
import pathlib
import re

LABELS = {"positive", "negative", "ambiguous", "unsupported"}


def canonical(value):
    return json.dumps(value, sort_keys=True, ensure_ascii=False, separators=(",", ":"), allow_nan=False)


def digest(value):
    return hashlib.sha256(canonical(value).encode()).hexdigest()


def bytes_digest(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()


def safe_relative(name):
    if not isinstance(name, str) or not name or "\\" in name:
        raise ValueError("Invalid relative path")
    path = pathlib.PurePosixPath(name)
    if path.is_absolute() or any(x in {"", ".", ".."} for x in name.split("/")) or ":" in name:
        raise ValueError("Unsafe relative path")
    return pathlib.Path(*path.parts)


def validate_corpus(data, repo):
    if data.get("version") != 1 or data.get("license") != "MIT":
        raise ValueError("Unsupported corpus version/license")
    paths, lineages, rules = set(), {}, {}
    for family, ids in data["families"].items():
        for rule in ids:
            if rule in rules:
                raise ValueError("Rule belongs to multiple families")
            rules[rule] = family
    for file in data["files"]:
        path = safe_relative(file["path"])
        source = repo / safe_relative(file["source"])
        if file["path"] in paths or file["split"] not in {"tuning", "holdout"}:
            raise ValueError("Duplicate input or invalid split")
        if any(p in {".git", "target", "docs", "dist", "node_modules"} for p in path.parts) or path.suffix == ".md":
            raise ValueError("Input would be excluded by scanner policy")
        if source.resolve() != source or not source.is_file() or bytes_digest(source) != file["sha256"]:
            raise ValueError("Source binding changed/unavailable")
        if file.get("license") != "MIT":
            raise ValueError("Input license must be reviewed")
        previous = lineages.setdefault(file["lineage"], file["split"])
        if previous != file["split"]:
            raise ValueError("Lineage crosses tuning/holdout split")
        paths.add(file["path"])
    ids, intervals = set(), {}
    for unit in data["units"]:
        if unit["id"] in ids or unit["path"] not in paths or unit["ruleId"] not in rules:
            raise ValueError("Invalid truth unit identity")
        if not (isinstance(unit["line"], int) and unit["line"] > 0
                and 0 < unit["startColumn"] < unit["endColumn"]):
            raise ValueError("Invalid truth unit span")
        key = (unit["path"], unit["ruleId"], unit["line"])
        for lo, hi in intervals.setdefault(key, []):
            if max(lo, unit["startColumn"]) < min(hi, unit["endColumn"]):
                raise ValueError("Overlapping truth units")
        intervals[key].append((unit["startColumn"], unit["endColumn"]))
        ids.add(unit["id"])
    return data


def load_truth(directory):
    data = json.loads((directory / "corpus.json").read_text())
    truth = json.loads((directory / "truth.json").read_text())
    if truth["corpusSha256"] != bytes_digest(directory / "corpus.json"):
        raise ValueError("Truth does not bind current corpus")
    if len(truth["labelSources"]) != 2:
        raise ValueError("Two independent label records are required")
    reviewers = set()
    records = []
    for name, expected in truth["labelSources"].items():
        path = directory / safe_relative(name)
        reviewer = json.loads(path.read_text())
        if bytes_digest(path) != expected or reviewer["corpusSha256"] != truth["corpusSha256"]:
            raise ValueError("Independent label evidence changed")
        if reviewer["reviewer"] in reviewers:
            raise ValueError("Independent reviewer identities must be distinct")
        reviewers.add(reviewer["reviewer"])
        validate_labels(data, label_map(reviewer["labels"]))
        records.append(reviewer)
    labels = label_map(truth["labels"])
    validate_labels(data, labels)
    if 'annotationVersion' in data:
        if data['annotationVersion'] != 1:
            raise ValueError('Unsupported annotation version')
        from taxonomy import validate_freeze
        validate_freeze(directory, data, truth, records)
    return data, labels


def validate_labels(data, labels):
    if set(labels) != {u["id"] for u in data["units"]} or any(x not in LABELS for x in labels.values()):
        raise ValueError("Missing, extra or invalid truth labels")


def score(data, labels, findings):
    validate_labels(data, labels)
    family_by_rule = {r: f for f, rules in data["families"].items() for r in rules}
    files = {f["path"]: f for f in data["files"]}
    groups, observed, details = {}, set(), []
    for split in sorted({f["split"] for f in data["files"]}):
        for family in data["families"]:
            groups[f"{split}/{family}"] = dict(tp=0, fp=0, fn=0, tn=0,
                positive=0, negative=0, ambiguous=0, unsupported=0, excluded_predictions=0)
    for unit in data["units"]:
        group = groups[files[unit["path"]]["split"] + "/" + family_by_rule[unit["ruleId"]]]
        group[labels[unit["id"]]] += 1
    outside = 0
    for finding in sorted(findings, key=canonical):
        location = finding["location"]
        if location["file"] not in files:
            raise ValueError("Prediction outside materialized corpus")
        family = family_by_rule.get(finding["rule_id"])
        if family is None:
            outside += 1
            continue
        group = groups[files[location["file"]]["split"] + "/" + family]
        matches = []
        for unit in data["units"]:
            if unit["path"] != location["file"] or unit["ruleId"] != finding["rule_id"]:
                continue
            if location["line"] == 0:
                matches_unit = ("dependencyName" in unit and unit["dependencyName"] ==
                    finding.get("evidence", {}).get("metadata", {}).get("dep_name"))
            else:
                matches_unit = (unit["line"] == location["line"] and
                    unit["startColumn"] <= location["column"] < unit["endColumn"])
            if matches_unit:
                matches.append(unit)
        if len(matches) > 1:
            raise ValueError("Prediction matches ambiguous truth intervals")
        unit = matches[0] if matches else None
        label = labels[unit["id"]] if unit else None
        if label in {"ambiguous", "unsupported"}:
            disposition = "excluded_prediction"
            group["excluded_predictions"] += 1
        elif label == "positive" and unit["id"] not in observed:
            disposition = "tp"
            group["tp"] += 1
        else:
            disposition = "fp"
            group["fp"] += 1
        if unit:
            observed.add(unit["id"])
        details.append(dict(unit=unit["id"] if unit else None, ruleId=finding["rule_id"],
                            location=location, disposition=disposition))
    for unit in data["units"]:
        label = labels[unit["id"]]
        if unit["id"] not in observed and label in {"positive", "negative"}:
            disposition = "fn" if label == "positive" else "tn"
            groups[files[unit["path"]]["split"] + "/" + family_by_rule[unit["ruleId"]]][disposition] += 1
            details.append(dict(unit=unit["id"], ruleId=unit["ruleId"], disposition=disposition))
    for m in groups.values():
        m["eligible"] = m["positive"] + m["negative"]
        m["precision_denominator"] = m["tp"] + m["fp"]
        m["recall_denominator"] = m["tp"] + m["fn"]
        m["precision"] = m["tp"] / m["precision_denominator"] if m["precision_denominator"] else None
        m["recall"] = m["tp"] / m["recall_denominator"] if m["recall_denominator"] else None
    return dict(groups=groups, out_of_scope_predictions=outside, details=details)


def normalized_report(report):
    value = copy.deepcopy(report)
    value.pop("generated_at", None)
    for finding in value.get("findings", []):
        evidence = finding.get("evidence", {})
        metadata = evidence.get("metadata", {})
        if (finding.get("rule_id") in {"CERT_EXPIRED", "CERT_NOT_YET_VALID"}
                and evidence.get("type") == "certificate"
                and metadata.get("detector") == "certificate_detector"):
            timestamp = metadata.pop("assessment_time_unix", None)
            if timestamp is not None and "snippet_preview" in evidence:
                evidence["snippet_preview"] = re.sub(
                    r"assessment_time_unix=" + re.escape(str(timestamp)) + r"(?=,|$)",
                    "assessment_time_unix=<VOLATILE>", evidence["snippet_preview"])
    return value


def order_digest(report):
    return digest(normalized_report(report))


def semantic_digest(report):
    value = normalized_report(report)
    for field in ["findings", "cbom", "dependency_sbom"]:
        if field in value:
            value[field].sort(key=canonical)
    return digest(value)


def check_coverage(diagnostics, expected_files):
    c = diagnostics["coverage"]
    fields = ["analyzed_files", "partial_files", "analysis_failed_files", "unsupported_files",
              "not_applicable_files", "unknown_files"]
    if c["selected_files"] != c["read_ok_files"] + c["read_failed_files"]:
        raise ValueError("Selected/read coverage does not reconcile")
    if c["read_ok_files"] != sum(c[f] for f in fields):
        raise ValueError("File terminal states do not reconcile")
    if c["selected_files"] != expected_files or c["read_ok_files"] != expected_files:
        raise ValueError("Corpus files were skipped or not read")
    return c


def label_map(records):
    result = {x["id"]: x["label"] for x in records}
    if len(result) != len(records):
        raise ValueError("Duplicate label identity")
    return result
