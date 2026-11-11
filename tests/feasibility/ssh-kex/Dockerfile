FROM rust:1.93.1-bookworm@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31
RUN apt-get update && apt-get install -y --no-install-recommends python3 openssh-server && rm -rf /var/lib/apt/lists/* /etc/ssh/ssh_host_* && mkdir -p /run/sshd
# Official 10.2p1 tarball, SHA256 from https://www.openssh.org/txt/release-10.2
COPY openssh-10.2p1.tar.gz /tmp/openssh.tar.gz
RUN echo 'ccc42c0419937959263fa1dbd16dafc18c56b984c03562d2937ce56a60f798b2  /tmp/openssh.tar.gz' | sha256sum -c - && cd /tmp && tar xzf openssh.tar.gz && cd openssh-10.2p1 && ./configure --prefix=/opt/openssh --with-privsep-path=/run/sshd --with-privsep-user=sshd > /tmp/openssh-config.log && make -j2 sshd sshd-session sshd-auth ssh-keygen > /tmp/openssh-build.log && mkdir -p /opt/openssh/sbin /opt/openssh/bin /opt/openssh/libexec && cp sshd /opt/openssh/sbin/ && cp ssh-keygen /opt/openssh/bin/ && cp sshd-session sshd-auth /opt/openssh/libexec/ && rm -rf /tmp/openssh-10.2p1 /tmp/openssh.tar.gz
WORKDIR /work
COPY study/ tests/feasibility/ssh-kex/
RUN cargo build --manifest-path tests/feasibility/ssh-kex/Cargo.toml --locked --target-dir target/task-039-build > /tmp/study-build.log 2>&1
CMD ["python3", "tests/feasibility/ssh-kex/run-isolated.py", "--build-dir", "target/task-039-build", "--out-dir", "target/task-039-results"]
