FROM rust:1.93.1-bookworm
RUN apt-get update && apt-get install -y --no-install-recommends openssh-server python3 && rm -rf /var/lib/apt/lists/* /etc/ssh/ssh_host_* && mkdir -p /run/sshd
WORKDIR /work
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY crates/ ./crates/
RUN cargo test --locked -p pqc_scan_probe_ssh --features runtime --test openssh --no-run --message-format=json > /tmp/artifacts.jsonl && python3 -c 'import json,pathlib; rows=[json.loads(x) for x in pathlib.Path("/tmp/artifacts.jsonl").read_text().splitlines()]; p=next(x["executable"] for x in rows if x.get("executable") and x["target"]["name"]=="openssh"); pathlib.Path("/test-binary").write_bytes(pathlib.Path(p).read_bytes()); pathlib.Path("/test-binary").chmod(0o755)'
ENV PQC_SCAN_TEST_SSH_HYBRID=sntrup761x25519-sha512@openssh.com
CMD ["/test-binary", "--ignored", "--nocapture", "--test-threads=1"]
