# MaskOps Pricing Tiers (canonical reference)

> This is the single source of truth for what MaskOps charges and how it is presented. The website pricing section and every quote should mirror it. Derived from `docs/pricing-research-chile-brazil-2026.md`. Numbers are hypotheses under active price discovery (see `wtp-survey.md`), not fixed facts.

## Principle

No directly-comparable priced PII-masking product exists in Chile or Brazil, so the price is validated by selling, not by research. Public pricing is therefore a **band with a "from" anchor**, enterprise is **quote-only** (which matches the market, BigID/Immuta/Skyflow are all quote-only), and every serious conversation is **anchored to fine-and-breach value in USD**, never to a monthly number.

## The ladder

| Tier | Public presentation | Internal target | Gate / what's included |
|---|---|---|---|
| **Open source** | Free | $0 | Core masking (`mask_pii`, `contains_pii`, `mask_pii_fpe`), asterisk + FPE, all PII families. The funnel, not a tier to monetize. |
| **Retainer (early adopter)** | **from USD 200/mo** | $200-400/mo, rate-locked for the first cohort | Support retainer, defined response window, security patches, pattern updates within 2 weeks of law changes, private channel. |
| **Team / SLA** | **from USD 600/mo** | $600-1,000/mo | Above + audit manifest / RAT export, FPE key-management guidance, faster SLA, migration package. |
| **Enterprise / License** | **Request pricing** | $25K-60K/yr or $2-5K/mo, custom | Air-gapped license, guaranteed response, compliance reporting, on-prem deployment support, IP-license / OEM option for fintechs. |

"from" on the two retainer tiers signals there is a floor without locking the ceiling. The enterprise tier never shows a number.

## Value anchors (lead with these, not the monthly price)

- **Chile Ley 21.719:** fine ceiling ~CLP 1.4B / ~USD 1.5M, in force 1 Dec 2026 (the near-term trigger).
- **Brazil LGPD:** up to 2% of revenue, capped R$50M per infraction.
- **Brazil finance-sector breach:** avg R$8.92M (IBM 2025). The hardest ROI anchor for the target buyer.
- **ROI line:** the early-adopter retainer is ~0.17% of one average Brazil breach.

## Localization

- Post a **separate in-region list ~40-55% below global** for Chile (CLP / UF) and Brazil (BRL), gated against US/EU purchase to prevent price leakage. Do **not** ad-hoc discount.
- **Never PPP-discount enterprise or IP deals**, anchor those to fine value in USD.

## The pilot (not free)

The public entry point is a **paid, metric-gated pilot**, not a free trial (free enterprise trials convert <10%, paid pilots 40-80%). See `paid-pilot-agreement.md`. Charge the retainer rate for a fixed 60-90 day term, 100% credited to the annual on conversion, with a money-back window. A token fee roughly doubles conversion versus free.

## Guardrails

- Never call FPE output anonymous, never claim absolute compliance (project compliance rules).
- Do not use Open Finance as a 2026 urgency lever, NCG 514 slipped to July 2027 (NCG 569). Ley 21.719 is the live deadline.
