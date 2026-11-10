# Remove advisory policy from ordinary HTML

The user requests understandable investigation results, not more internal evaluation
fields. Remove the optional advisory-policy block entirely from HTML, including
its heading, aggregate decisions/counts and technical profile/reason section.
Keep the explicitly requested policy JSON and full Markdown record unchanged;
retain assessment/inventory mismatch validation. No new CLI/storage/rule/network
contract is introduced. This follow-up supersedes the earlier summary choice.

1. Make the existing optional-policy regression require identical HTML with and
   without assessment for the same document, in both languages, while Markdown
   retains policy facts and machine files remain identical. Confirm RED.
2. Remove HTML-only rendering; update old HTML assumptions and both report guides/
   READMEs. Do not change Markdown evaluation or actual risk findings.
3. Run workspace tests/fmt/clippy, Node and quality Python; obtain independent
   source review. Rebuild/regenerate the originally viewed samples and record
   actual sizes/assertions. Actual browser acceptance remains unavailable, not passed.
4. Explain why030/031 were prerequisites for optional live-TLS032, while034/035
   serve interoperability/repeated comparisons, not basic HTML consumption.
5. Commit the local correction and its evidence; no publication.
