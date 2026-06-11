# MaskOps Pricing Research — Chile & Brazil (2026)

> Source: parallel web-search research (10 pricing searchers + synthesis), June 2026. Figures are 2025-2026 where available. Read the "Gaps / Low-Confidence Areas" section before acting on any number.

## 1. Recommended Price Ladder

| Tier | USD (global list) | Chile (CLP, ~PPP-adjusted) | Brazil (BRL, ~PPP-adjusted) | What it includes |
|---|---|---|---|---|
| **OSS / Free** | $0 | $0 | $0 | Core masking expressions (`mask_pii`, `contains_pii`), asterisk + FPE, all LATAM/EU/US/APAC families. MIT-style open core. |
| **Retainer / SMB** | **$200/mo** (~$2.4K/yr) | ~CLP 95K-110K/mo (≈2.5-2.8 UF) | ~R$1,000-1,100/mo | Support retainer, defined response window, ticket cap, security patches, private channel. (Current early-adopter price — keep.) |
| **Team / SLA** | **$600-1,000/mo** (~$7.2K-12K/yr) | ~CLP 350K-550K/mo | ~R$3,000-5,000/mo | Above + audit manifest / RAT export, FPE key-management guidance, faster SLA, migration package. |
| **Enterprise / License** | **$2,000-5,000/mo or $25K-60K/yr** custom | quote, PPP-floored | quote, PPP-floored | Air-gapped license, guaranteed response, compliance reporting, on-prem deployment support, IP-license/OEM option for fintechs. |

**Local positioning note:** PPP gaps are large (Chile ~60% below US GDP/cap PPP, Brazil ~74% [tradingeconomics.com/chile, /brazil]). Post a *separate in-region list* (don't ad-hoc discount) ~40-55% below global, gated against US/EU purchase to prevent price leakage [getmonetizely.com region-discount guide]. For enterprise/IP deals, anchor to compliance/fine value in USD — do **not** PPP-discount those.

**Pricing-model evolution:** add a **usage meter (rows/records masked)** as an option alongside the flat retainer — 70% of vendors move off pure per-seat by 2028; usage-priced firms grow ~29% faster [advisable.com, nxcode.io]. MaskOps' natural meter is rows processed.

**Open-core split (by buyer persona, not technical metric):** core masking stays free for developers; **audit manifest/RAT export + FPE key-management/compliance reporting are the paid gate** — these appeal to managers/auditors [opencoreventures.com].

## 2. Value-Based Pricing Rationale

The retainer is a rounding error against the downside it removes:

- **Chile Ley 21.719:** max fine 20,000 UTM ≈ **CLP 1.4B / ~USD 1.5M**, or 2-4% of Chilean revenue for repeat serious violations [anguitaosorio.cl, sii.cl/utm2026].
- **Brazil LGPD:** up to 2% of Brazilian revenue, **capped R$50M (~USD 9M) per infraction** [compliancehub.wiki]; pending PL 4530/23 would raise this to 20% / R$100M [hoganlovells.com].
- **Brazil breach cost (the strongest sales anchor):** 2025 average **R$7.19M**; **finance R$8.92M, healthcare R$11.43M** — exactly MaskOps' target verticals [IBM Cost of a Data Breach 2025, ibm.com/reports/data-breach].
- **ROI math:** $2.4K/yr retainer ≈ **0.17% of one average Brazil breach** and ~0.16% of either fine ceiling. Enterprise tier ($25K-60K/yr) still <1% of one finance-sector breach.

Brazil framing should lead with **breach-cost avoidance + BACEN/Open Finance obligations**, not ANPD fines (enforcement still modest — ~R$98M cumulative 2023-2025 [securiti.ai]). Chile framing leads with the **Dec 2026 fine deadline** as the urgency trigger.

## 3. Chile vs Brazil Differences

| | **Chile** | **Brazil** |
|---|---|---|
| **Buying trigger** | Ley 21.719 in force **1 Dec 2026** (SMEs get warning-only grace to Dec 2027); Open Finance NCG 514 **slipped to July 2027** via NCG 569 (1 Jun 2026) [carey.cl] | LGPD live; Open Finance Brasil mature (>100M clients, ~4B API calls/wk) [finsidersbrasil.com.br] |
| **Closest local comparable** | Legal/compliance **retainers 20-60 UF/mo** (~USD 830-2,490) [grupowolf.law]; SME tool CERTAIN "desde 1 UF/mo" (~USD 42) [certain.cl] | **DPOaaS R$2,500-6,500/mo**; SME LGPD SaaS R$228-285/mo, 48-mo lock-in (DPOnet) [dponet.com] |
| **MaskOps position** | Below local legal retainers; room to raise | **Below even cheapest DPOaaS (R$1,900)** — room to raise toward R$1,900-3,000 if bundled into adequacy stack |
| **Market size** | ~485-540 fintechs, USD 854M raised 2025 [fintechile.org] | ~1,706 fintech ops (~58.7% of LATAM) [ainvest.com] |
| **Currency** | UF-denominated; expect quote-gated vendors | BRL flat-rate + per-transaction (SERPRO R$0.08-0.18/tx; DINAMO consumption-billed) [serpro.gov.br, dinamonetworks.com] |
| **Pricing culture** | UF/quote, opaque | More published SME price cards, but technical masking still foreign/quote-only |
| **White space** | **No local priced PII-masking SaaS found** — unoccupied niche | SME tools are governance/consent paperwork, **not engine-level masking** — gap MaskOps fills |

## 4. Strongest Data Points

1. **MaskOps undercuts every commercial comparator by an order of magnitude.** 5 of 7 (BigID, Immuta, Privacera, Protegrity, Skyflow) are quote-only at $100K-500K+/yr; Skyflow effective min ~$195K/yr [vendr.com/skyflow]; only VGS exposes sub-$2K/mo (~$1K/mo starter) [verygoodsecurity.com/pricing]. MaskOps at ~$2.4K/yr sits below all — including self-hosted Presidio's ~€13.2K/yr operating cost [anonym.legal].
2. **The "replace a Presidio build" narrative is worth five figures/yr.** anonym.legal sells managed Presidio-alternative at €180/yr while pricing DIY self-host at €10K-20K yr-one TCO [anonym.legal] — direct support for an enterprise tier well above $200/mo, reinforced by MaskOps' ~4000x speed edge.
3. **Brazil finance breach = R$8.92M** [IBM 2025, ibm.com/reports/data-breach] — the single hardest ROI anchor for the target buyer.
4. **Chile fine ceiling CLP 1.4B / ~USD 1.5M, hard deadline 1 Dec 2026** [anguitaosorio.cl, sii.cl] — compresses enterprise procurement into 2H 2026.
5. **74% of orgs will pay for security/maintenance/license-compliance of OSS they use** [Tidelift] — validates the exact open-core value MaskOps monetizes; Confluent: 35% of $100K+/yr customers started free — the funnel produces large contracts.

## 5. Gaps / Low-Confidence Areas

- **No direct local priced comparable for a dedicated PII-masking tool** in either country — all MaskOps WTP is *inferred* from DPO retainers, governance SaaS, and global tooling. Treat $200/mo as plausible but unvalidated locally (confidence: low).
- **Enterprise/IP-license & OEM pricing for fintechs is entirely unanchored** — no LATAM data point exists for selling/licensing the IP itself; the $25K-60K/yr figure is derived from global SMB-floor-to-enterprise ratios (4-10x), not observed deals.
- **Open-core free-to-paid conversion 0.5-3%** [getmonetizely] means the retainer model needs a wide GitHub-install funnel; current top-of-funnel size is unknown.
- **OneTrust and most enterprise platforms are quote-only** — the $50K-300K+/yr ceiling is from third-party aggregators, not list prices.
- **PPP discount depth is a judgment call** — published LATAM SaaS guidance (30-55% off) conflicts somewhat with raw PPP ratios (60-74%); the 40-55% recommendation splits the difference.
- **NCG 514 timing was corrected mid-research** (July 2026 → July 2027 via NCG 569) — confirm before using Open Finance as a 2026 urgency lever; the wave now peaks 2026-2027.
- **PL 4530/23 (20% / R$100M)** is a *proposed* bill, not law — use only as forward-looking risk framing.
- No per-fintech compliance-cost figure published for NCG 514/569 obligations in Chile; willingness-to-pay there is qualitative only.
