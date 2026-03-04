#!/usr/bin/env bash
set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SCAN_BIN="${SCAN_BIN:-$WORKSPACE_ROOT/target/debug/pqc-scan}"
RULES_DIR="${RULES_DIR:-$WORKSPACE_ROOT/rules}"
TARGET_BASE="${TARGET_BASE:-/tmp/pqc-oss-fixtures}"
REPORT_BASE="${REPORT_BASE:-/tmp/pqc-oss-reports}"

repos=(
  "java-realworld|https://github.com/gothinkster/spring-boot-realworld-example-app.git"
  "python-product|https://github.com/apache/airflow.git"
  "go-realworld|https://github.com/gothinkster/golang-gin-realworld-example-app.git"
  "js-realworld|https://github.com/gothinkster/node-express-realworld-example-app.git"
  "ts-realworld|https://github.com/lujakob/nestjs-realworld-example-app.git"
  "ruby-realworld|https://github.com/gothinkster/rails-realworld-example-app.git"
  "rust-realworld|https://github.com/launchbadge/realworld-axum-sqlx.git"
)

mkdir -p "$TARGET_BASE" "$REPORT_BASE"

if [[ ! -x "$SCAN_BIN" ]]; then
  echo "pqc-scan binary not found: $SCAN_BIN"
  echo "Run: cargo build --bin pqc-scan"
  exit 1
fi

for entry in "${repos[@]}"; do
  name="${entry%%|*}"
  repo="${entry##*|}"
  dst="$TARGET_BASE/$name"
  out="$REPORT_BASE/$name"

  if [[ ! -d "$dst/.git" ]]; then
    echo "[clone] $repo"
    git clone --depth 1 "$repo" "$dst" >/dev/null
  else
    echo "[skip] already cloned: $name"
  fi

  echo "[scan] $name"
  "$SCAN_BIN" scan "$dst" --format json --out-dir "$out" --rules-dir "$RULES_DIR" >/dev/null

  summary="$(jq -r '.summary | "findings=\(.total_findings) scanned=\(.scanned_files) skipped=\(.skipped_files)"' "$out/report.json")"
  echo "[result] $name $summary"
  echo "[report] $out/report.json"
  echo

done
