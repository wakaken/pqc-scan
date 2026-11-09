# Static PQC readiness fixtures

These minimal source fragments are scanner inputs, not runnable applications or
cryptographic migration recipes. They intentionally omit imports, provider setup
and key material. The seven positive files map to PQC_JAVA_API, PQC_GO_API,
PQC_NODE_API, PQC_WEBCRYPTO_API, PQC_PYTHON_API, PQC_RUST_API and PQC_RUBY_API.
negative.js must not produce PQC API or draft-algorithm findings. overrides.conf
contains three explicit classical-only/disabled TLS configuration signals.

Scan only this directory with --format all to inspect informational PQC inventory
in JSON, HTML, Markdown, SARIF and CBOM. An algorithm reference is not proof that
the application executes it, that the backend supports it, or that TLS negotiated
it. Dependency presence is always quantum-uncertain, even at a capable version.

Reviewed official sources (2026-11-09):

- https://openjdk.org/jeps/496 and https://openjdk.org/jeps/497
- https://go.dev/doc/go1.24 and https://go.dev/doc/go1.27
- https://go.dev/doc/godebug
- https://nodejs.org/api/crypto.html and https://nodejs.org/api/webcrypto.html
- https://cryptography.io/en/50.0.2/changelog/
- https://docs.rs/rustls/latest/src/rustls/manual/defaults.rs.html
- https://ruby.github.io/openssl/History_md.html
- https://www.rfc-editor.org/rfc/rfc9881.html
- https://www.rfc-editor.org/rfc/rfc9909.html
