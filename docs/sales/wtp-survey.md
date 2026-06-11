# MaskOps Willingness-to-Pay Survey

> Purpose: discover the real price for MaskOps when no local comparable exists. Run this with 5-10 target buyers (fintech compliance/DPO, data-platform leads) before fixing any public number. It is a Van Westendorp Price Sensitivity Meter adapted for a regulated-data tool, plus three qualifying questions.

## How to run it

- **Who:** the economic buyer (Compliance/DPO lead) and one technical evaluator per account. Source them through FinteChile / ABFintechs and warm intros, not cold lists.
- **When:** during or right after the pilot's "aha" moment (they have masked their own RUT/CPF data and seen the manifest). A price asked after value is felt is worth ten asked before.
- **How:** a 10-minute call, not a form. Read the four price questions verbatim, in order, and let them answer in their own currency (CLP / BRL / USD). Record the raw number, do not round in the moment.
- **Frame first, once:** "MaskOps masks PII inside your Polars/pandas pipeline, fully air-gapped, with reversible FPE and an auditor-ready RAT export. It removes a seven-figure fine and breach exposure. I want your honest read on price, there are no wrong answers."

## The four price questions (Van Westendorp)

Ask all four, in this order, about an annual retainer (then repeat for an enterprise/air-gapped license if they qualify):

1. **Too expensive:** "At what yearly price would MaskOps be so expensive you would not consider it?"
2. **Too cheap:** "At what yearly price would it be so cheap you would doubt it could be enterprise-grade or properly supported?"
3. **Getting expensive:** "At what yearly price does it start to feel expensive, something you would have to think hard about but might still buy?"
4. **Bargain:** "At what yearly price does it feel like a clear bargain, an easy yes?"

### Reading the results

- Plot all four across respondents. The **Optimal Price Point** is where "too cheap" and "too expensive" curves cross, the **Range of Acceptable Pricing** is between the "getting expensive" and "bargain" crossings.
- If most answers cluster well above today's $200/mo (~$2.4K/yr), that is the white-space signal, raise. If the "too cheap" answers sit near $2.4K/yr, the current price is actively *hurting* credibility.
- Treat the enterprise/license number separately, it has no desk-research anchor and these answers are your only data.

## Three qualifying questions (ask after the price block)

5. **Trigger:** "What would have to be true for you to buy this in the next 6 months?" (Listen for: Ley 21.719 deadline, an auditor request, a breach scare, an Open Finance milestone.)
6. **Budget owner & process:** "Whose budget does this come from, and what does it take to approve a tool at that price?" (Maps the 9-13 person buying committee and the security-review gate.)
7. **Build-vs-buy:** "If you didn't buy this, what would you do instead, and how much would that cost you?" (Surfaces the free-Presidio / build-it objection and the real alternative cost.)

## What to do with the data

- After 5-7 calls you will have a defensible band. Set the public "from" number at the lower edge of the acceptable range, not at the optimal point, leave room to negotiate up per deal.
- Keep enterprise quote-only until at least 3 closed deals give you observed (not stated) willingness to pay.
- Log every raw answer with date, country, role, and trigger in a simple sheet, the pattern across countries (Chile UF-denominated vs Brazil BRL) matters as much as any single number.
