#!/bin/bash

set -e

WORKDIR=$(pwd)
SCAN_TOOL="pqc-scan"
RULES_DIR="./pqc-scan/rules"

echo "Starting PQC Scan Benchmark"
echo "Working directory: $WORKDIR"

REPOS=(
"https://github.com/keycloak/keycloak"
"https://github.com/django/django"
"https://github.com/tiangolo/fastapi"
"https://github.com/apache/airflow"
"https://github.com/rails/rails"
"https://github.com/discourse/discourse"
"https://github.com/kubernetes/kubernetes"
"https://github.com/prometheus/prometheus"
"https://github.com/nodejs/node"
"https://github.com/vercel/next.js"
"https://github.com/n8n-io/n8n"
"https://github.com/BurntSushi/ripgrep"
"https://github.com/tokio-rs/tokio"
"https://github.com/vectordotdev/vector"
)

mkdir -p results

for REPO in "${REPOS[@]}"; do

    NAME=$(basename "$REPO")

    echo "================================="
    echo "Processing: $NAME"
    echo "================================="

    if [ ! -d "$NAME" ]; then
        echo "Cloning $NAME..."
        git clone --depth 1 "$REPO"
    else
        echo "$NAME already exists, pulling latest..."
        cd "$NAME"
        git pull
        cd ..
    fi

    OUTDIR="./results/pqc-report-$NAME"

    echo "Running pqc-scan for $NAME..."

    $SCAN_TOOL scan "./$NAME" \
        --format all \
        --out-dir "$OUTDIR" \
        --rules-dir "$RULES_DIR"

    echo "Finished scan for $NAME"
    echo ""

done

echo "================================="
echo "All scans completed"
echo "Results saved in ./results"
echo "================================="