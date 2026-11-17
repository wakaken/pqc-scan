# Pilot cost estimate (not incurred experiment usage)

Pricing checked 2026-11-17. Proposed configuration: GPT-6.1 Sol, Standard speed,
short context, 24 single-response trials, no tools, retries or model grading.
The selected model/tier and price must be reconfirmed before execution. This is
not an estimate of the current preparation chat or its preparation reviewers.

[Official API pricing](https://developers.openai.com/api/docs/pricing) lists
USD 2.00 uncached input, USD 0.10 cached input and USD 10.00 output per million
tokens for this configuration. No cache saving or explicit cache-write service
is assumed. The [official Codex credit table](https://learn.chatgpt.com/docs/pricing)
lists 50 input, 2.5 cached-input and 250 output credits per million at Standard
speed. API dollars and plan credits are different accounting bases. Included
subscription allowance and purchased-credit dollar cost depend on the account;
the account's actual incremental charge is Unknown.

The twelve frozen packets are approximately 36.5-44.5 kB each. The per-file
UTF-8-byte/4 estimates are in `private/input-size.json`. This is not tokenizer
measurement, and does not count hidden client instructions, reasoning, repeated
inputs or automatic recovery. The identical A/B packet bodies make the supplement
the only authored input difference. The following scenarios reserve more input
than that rough estimate; actual usage must come from receipts.

| Planning scenario | Input per trial | Billable output per trial, including reasoning | 24-trial API equivalent | Standard credits |
|---|---:|---:|---:|---:|
| Working allowance | 15,000 | 6,000 | USD 2.16 | 54 |
| Higher usage allowance | 30,000 | 16,000 | USD 5.28 | 132 |

For example: 24 x (15,000 x 2 + 6,000 x 10) / 1,000,000 = USD 2.16.
Use (uncached_input x input_rate + cached_input x cached_rate + billable_output x
output_rate) / 1,000,000 for receipts. Do not double-count reasoning when it is
already included in the provider's billed output total. Optional caching changes
both the cost and the experiment configuration and must be recorded.

Proposed API budget reserve: USD 8 for the experiment, excluding taxes, currency
conversion, infrastructure, human work and preparation reviews. It is not an
approved charge or a guaranteed upper bound: the CLI has no verified hard token
or dollar cap here. An enforceable budget/supervisor and reservations are required
before execution. Fast mode, another model, multi-turn tool use, long contexts or
retries invalidate this estimate. Use separate plan-credit authorization for a
subscription run rather than treating USD 8 as a credit balance.

Time allowance: 24 x 5 minutes = 120 minutes maximum trial wall time after
supervisor enforcement. Plan another 60-120 minutes of human scoring/adjudication;
that is a planning allowance, not measured labor. Runtime provisioning is extra.
No experiment cost or speed improvement has been measured.
