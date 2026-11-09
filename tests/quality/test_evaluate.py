"""Sanity cases with hand-derived truth, not scanner-generated expectations."""
import copy
import json
import pathlib
import tempfile
import unittest

import evaluate


def corpus():
    return {"version": 1, "families": {"api": ["API"]},
            "files": [{"path": "a.js", "split": "holdout", "lineage": "a"}],
            "units": [{"id": "one", "path": "a.js", "ruleId": "API", "line": 1,
                       "startColumn": 1, "endColumn": 5},
                      {"id": "two", "path": "a.js", "ruleId": "API", "line": 1,
                       "startColumn": 7, "endColumn": 11}]}


def hit(column=1, rule="API", line=1):
    return {"rule_id": rule, "location": {"file": "a.js", "line": line, "column": column}}


class ScoringTests(unittest.TestCase):
    def score(self, findings, labels=None, data=None):
        return evaluate.score(data or corpus(), labels or {"one": "positive", "two": "positive"}, findings)

    def test_missing_second_same_line_occurrence_is_fn_and_duplicate_is_fp(self):
        report = self.score([hit(), hit()])
        m = report["groups"]["holdout/api"]
        self.assertEqual((m["tp"], m["fp"], m["fn"]), (1, 1, 1))
        self.assertEqual(m["precision"], 0.5)
        self.assertEqual(m["recall"], 0.5)

    def test_distinct_occurrences_match_one_to_one(self):
        m = self.score([hit(2), hit(8)])["groups"]["holdout/api"]
        self.assertEqual((m["tp"], m["fp"], m["fn"]), (2, 0, 0))

    def test_uncertain_units_and_predictions_are_outside_denominators(self):
        report = self.score([hit(), hit(8)], {"one": "ambiguous", "two": "unsupported"})
        m = report["groups"]["holdout/api"]
        self.assertEqual((m["ambiguous"], m["unsupported"], m["excluded_predictions"]), (1, 1, 2))
        self.assertIsNone(m["precision"])
        self.assertIsNone(m["recall"])

    def test_negative_and_unmatched_scoped_hits_are_false_positives(self):
        report = self.score([hit(), hit(50), hit(1, "OTHER")], {"one": "negative", "two": "negative"})
        m = report["groups"]["holdout/api"]
        self.assertEqual((m["fp"], m["tn"]), (2, 1))
        self.assertEqual(report["out_of_scope_predictions"], 1)
        self.assertEqual(m["precision"], 0)
        self.assertIsNone(m["recall"])

    def test_unknown_dependency_line_requires_exact_name_metadata(self):
        data = corpus()
        data["units"][0]["dependencyName"] = "library"
        f = hit(line=0, column=0)
        f["evidence"] = {"metadata": {"dep_name": "other"}}
        self.assertEqual(self.score([f], data=data)["groups"]["holdout/api"]["tp"], 0)
        f["evidence"]["metadata"]["dep_name"] = "library"
        self.assertEqual(self.score([f], data=data)["groups"]["holdout/api"]["tp"], 1)

    def test_score_is_stable_when_only_native_finding_order_changes(self):
        findings = [hit(3), hit(1), hit(8), hit(50)]
        self.assertEqual(self.score(findings), self.score(list(reversed(findings))))

    def test_missing_label_is_an_error_not_an_exclusion(self):
        with self.assertRaises(ValueError):
            self.score([], {"one": "positive"})

    def test_boundary_is_half_open_and_wrong_rule_is_not_tp(self):
        m = self.score([hit(5), hit(8, "OTHER")])["groups"]["holdout/api"]
        self.assertEqual((m["tp"], m["fp"], m["fn"]), (0, 1, 2))


class IntegrityTests(unittest.TestCase):
    def test_canonical_digest_ignores_only_timestamp_and_order_not_duplicates_or_ids(self):
        a = {"generated_at": "first", "findings": [hit(), hit(8)], "cbom": [], "dependency_sbom": [], "summary": {}}
        b = copy.deepcopy(a)
        b["generated_at"] = "second"
        b["findings"].reverse()
        self.assertEqual(evaluate.semantic_digest(a), evaluate.semantic_digest(b))
        self.assertNotEqual(evaluate.order_digest(a), evaluate.order_digest(b))
        b["findings"].append(hit())
        self.assertNotEqual(evaluate.semantic_digest(a), evaluate.semantic_digest(b))
        b = copy.deepcopy(a)
        b["findings"][0]["finding_id"] = "changed"
        self.assertNotEqual(evaluate.semantic_digest(a), evaluate.semantic_digest(b))

    def test_only_certificate_validity_assessment_clock_is_volatile(self):
        for certificate, expected_equal in [(True, True), (False, False)]:
            f = hit()
            f["rule_id"] = "CERT_EXPIRED" if certificate else "API"
            f["evidence"] = {"type": "certificate" if certificate else "regex_match",
                "metadata": {"assessment_time_unix": "10", "validity_status": "expired", "detector": "certificate_detector"},
                "snippet_preview": "validity=expired, assessment_time_unix=10, not_after_unix=5"}
            a = {"findings": [f]}
            b = copy.deepcopy(a)
            b["findings"][0]["evidence"]["metadata"]["assessment_time_unix"] = "11"
            b["findings"][0]["evidence"]["snippet_preview"] = "validity=expired, assessment_time_unix=11, not_after_unix=5"
            self.assertEqual(evaluate.semantic_digest(a) == evaluate.semantic_digest(b), expected_equal)
            b["findings"][0]["evidence"]["metadata"]["validity_status"] = "valid"
            self.assertNotEqual(evaluate.semantic_digest(a), evaluate.semantic_digest(b))

    def test_duplicate_label_cannot_be_silently_overwritten(self):
        records = [{"id": "one", "label": "positive"}, {"id": "one", "label": "negative"}]
        with self.assertRaises(ValueError):
            evaluate.label_map(records)

    def test_coverage_partitions_and_unknown_status_remain_visible(self):
        coverage = dict(selected_files=2, read_ok_files=2, read_failed_files=0,
                        analyzed_files=0, partial_files=0, analysis_failed_files=0,
                        unsupported_files=0, not_applicable_files=0, unknown_files=2,
                        status="unknown", not_enumerated_population=True)
        self.assertEqual(evaluate.check_coverage({"coverage": coverage}, 2)["status"], "unknown")
        coverage["analyzed_files"] = 1
        with self.assertRaises(ValueError):
            evaluate.check_coverage({"coverage": coverage}, 2)

    def test_safe_paths_reject_traversal_absolute_and_aliases(self):
        for name in ["../escape", "/tmp/escape", "a/../b", "a\\b", "", "./a"]:
            with self.assertRaises(ValueError, msg=name):
                evaluate.safe_relative(name)
        self.assertEqual(evaluate.safe_relative("holdout/a.js"), pathlib.Path("holdout/a.js"))

    def test_split_lineage_and_source_digests_are_enforced(self):
        repo = pathlib.Path(__file__).resolve().parents[2]
        data = json.loads((repo / "tests/quality/corpus.json").read_text())
        evaluate.validate_corpus(data, repo)
        changed = copy.deepcopy(data)
        changed["files"][0]["sha256"] = "0" * 64
        with self.assertRaises(ValueError):
            evaluate.validate_corpus(changed, repo)
        changed = copy.deepcopy(data)
        changed["files"][1]["split"] = "holdout"
        with self.assertRaises(ValueError):
            evaluate.validate_corpus(changed, repo)


if __name__ == "__main__":
    unittest.main()
