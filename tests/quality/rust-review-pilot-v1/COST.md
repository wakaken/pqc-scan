# Codex app usage budget

The active method uses the app's existing signed-in account and configured
model. Maximum 24 new chats, sequential, without retries, payment, purchases,
usage resets, API-key configuration, direct API calls or Codex CLI use. The user
authorized this method and experiment start on 2026-11-17.

The earlier USD 2.16-5.28 API-equivalent estimates and USD 8 reserve are historical
reference scenarios, not a charge or the budget for this app run. Preserve the
[original note](../../../docs/records/rust-review-app-20261117/historical-preparation/COST.md.txt).

| Item | Accounting treatment |
|---|---|
| Trial count | At most 24 independent app chats; no replacement attempts. |
| Account allowance | Read before launch and after each six completions. Shared with other account work. |
| Additional purchases/resets | None authorized. Do not change billing or authentication. |
| Per-trial token/credit/money usage | Record only if exposed; otherwise Unknown, not zero. |
| Model/effort/speed | App defaults; exposed metadata only. Do not assume the prior API scenario's model. |
| Time | Five minutes per trial is a monitoring budget, not a verified hard cap. Human scoring is separate. |

A read-only pre-update snapshot showed ordinary usage allowed and 53 percent
consumed in a reported 10,080-minute account window. This does not predict that
24 reviews fit. Do not retain account or reset-credit identifiers in artifacts.
Stop new launches on an unavailable allowance, payment/reset requirement or
remaining reported allowance of 10 percent. No API fallback.

Official [authentication](https://learn.chatgpt.com/docs/auth) and
[pricing](https://learn.chatgpt.com/docs/pricing) distinguish subscription usage
from API-key billing. API dollars do not predict included subscription tasks.
Actual incremental money remains Unknown without billing evidence. Usage deltas
can include this parent chat, preparation reviewers and unrelated work; do not
attribute them solely to the experiment. No new monetary budget is requested.
