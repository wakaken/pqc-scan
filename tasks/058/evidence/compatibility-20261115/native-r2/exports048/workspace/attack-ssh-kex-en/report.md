# SSH hybrid key exchange verification

An explicit prior public host-key pin and the server signature are required. This profile tests hybrid ML-KEM768+X25519 with classical Ed25519 host authentication, without user authentication or login. A failure does not establish global server non-support.

Strings resembling secrets were omitted. Original values cannot be recovered from this output. Observation completion describes collection; it does not remove this export loss.


## Overview

- Diagnostic code: address_denied
- Inspection outcome: Requested observation could not complete (failed)
- Connected peer: null
- Requested host: localhost
- Requested port: 443
- Received socket bytes: 0
- Sent socket bytes: 0

|Field path|Field|Value|
|---|---|---|
|/attempt_id|Attempt within this invocation|1|
|/details/credential|credential|\[masked-sensitive-content\]|
|/details/untrusted|untrusted|&lt;/script&gt;&lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;&lt;img src=x onerror="document.title='EXECUTED048'"&gt;&lt;svg onload="document.title='EXECUTED048'"&gt; \[click\](javascript:document.title='EXECUTED048') \| \` \`\`\` " &amp; 日😀  &lt;img src=x onerror="document.title='EXECUTED048'"&gt; &lt;script&gt;document.title='EXECUTED048'&lt;/script&gt;|
|/diagnostics/0|Diagnostic code|address_denied|
|/generated_at|Generated at|2026-11-15T07:16:05.937219+00:00|
|/limits/dns_timeout_ns|DNS timeout (ns)|5000000000|
|/limits/max_dns_answers|Maximum DNS answers|8|
|/limits/max_received_bytes|Receive limit (bytes)|524288|
|/limits/max_sent_bytes|Send limit (bytes)|65536|
|/limits/overall_timeout_ns|Overall timeout (ns)|15000000000|
|/limits/protocol_timeout_ns|Protocol timeout (ns)|5000000000|
|/limits/tcp_timeout_ns|TCP timeout (ns)|3000000000|
|/outcome|Inspection outcome|Requested observation could not complete (failed)|
|/protocol|Observation method|ssh_kex|
|/redaction_applied|Secret-like text omitted|true|
|/scanner_version|Scanner version|0.2.0|
|/schema_version|Schema version|1|
|/target/connected_peer|Connected peer|null|
|/target/host|Requested host|localhost|
|/target/pinned_address|Pinned destination|null|
|/target/port|Requested port|443|
|/target/resolved_addresses|Checked DNS answers|null|
|/traffic/received_bytes|Received socket bytes|0|
|/traffic/sent_bytes|Sent socket bytes|0|
