# MaskOps Chile GTM Design

**Date:** 2026-06-03
**Model:** Side income — 2–5 clients, USD 1–3K/month recurring
**Approach:** Free pilot → paid retainer via direct LinkedIn outreach
**First market:** Chilean fintechs (Fast tier first)

---

## Context

- Author is Chilean, software/data engineering background
- MaskOps is open-core: MPL-2.0 library + proprietary enterprise product (dual-license with CLA)
- Chile's new data protection law passed Congress August 2024, pending constitutional review → regulatory urgency is real and near-term
- FPE (format-preserving encryption) mode is the key differentiator: reversible pseudonymization, GDPR Art. 4(5) compliant, key stays with client
- See `docs/gdpr/gdpr-reference.md` for the full compliance model
- See `docs/regulatory/latam-privacy-laws.md` for country-by-country regulatory reference

---

## Phase 0 — Free Pilot (now)

### Goal
Land 1 named reference client. No revenue yet.

### Scope of the free pilot (hard boundaries)

**Included:**
- MaskOps integrated into one existing Polars pipeline (one data source)
- Masking rules configured for their PII types (RUT, email, phone, credit card)
- One set of pytest tests covering their patterns
- One-page compliance summary for their legal team
- 3 weeks max, 2 check-in calls

**Not included:**
- GUI, CLI, or infrastructure work
- Multiple pipelines
- Ongoing support after delivery
- Any changes to their codebase beyond the integration point

**Ask in return (stated before starting):**
- 2-sentence LinkedIn testimonial
- Permission to use company name as reference client
- 30-minute debrief call at the end

### Conversion at debrief call
Offer the standard retainer: *"For USD 200/month I keep this updated as the Chilean law evolves, add new patterns when needed, and you get priority support."*

---

## Target List — Chilean Fintechs

### Tier classification

| Tier | Characteristics | Decision speed |
|------|----------------|---------------|
| **Fast** | Pure private, founder/CTO-led | 1–3 weeks |
| **Medium** | Corporate parent behind them | 4–10 weeks |
| **Slow** | State-adjacent, CORFO-backed, BancoEstado products | 3–12 months |

### Fast tier (target first)

| Company | Why they care | Decision maker |
|---------|--------------|----------------|
| Fintual | CMF-registered, investment data for thousands of users | Head of Engineering / CTO |
| Destacame | Credit scoring, extremely sensitive financial + identity data | CTO / Head of Data |
| Fingo | Open banking, account + transaction data | CTO |
| Kushki | Payment processor, PCI-DSS + Chilean regulation | Head of Security / CTO |

### Medium tier (target after 1 reference client)

| Company | Why they care | Decision maker |
|---------|--------------|----------------|
| Tenpo (Walmart Chile backing) | Digital wallet, 3M+ users, CMF-regulated | Head of Data |
| Mach (BCI) | Banking-grade compliance, large PII volume | Data Engineering Lead |
| Mercado Pago CL | Massive PII pipeline, upcoming law enforcement | Data/Compliance Engineering |

### Slow tier (skip until v1.0 + 2 reference clients)
BancoEstado digital products, CORFO-backed fintechs, any state-adjacent entity.

---

## Outreach Pipeline

### Stack
- **Prospect list:** Apollo.io free tier (50/month) or manual LinkedIn search → `prospects.csv`
- **Message generation:** Python script + Claude API (Haiku model, ~USD 0.001/prospect)
- **Sending:** Manual copy-paste from `outreach.csv` → LinkedIn (never automate sending — ToS violation)

### prospects.csv schema
```
name, company, role, linkedin_url, tier, recent_post_or_note
```

### Generation script
```python
import anthropic, csv, json

client = anthropic.Anthropic()

SYSTEM = """You are Felipe, a Chilean software engineer who built MaskOps —
a GDPR-compliant, air-gapped PII masking library for Polars, powered by Rust.
You're reaching out to fintechs about a free 3-week pilot integration.
Chile's new data protection law (passed Aug 2024, pending enactment) creates
urgency. Write in English unless the prospect's profile suggests Spanish.
Be direct, human, and brief. Never use marketing language."""

def generate_messages(prospect: dict) -> dict:
    tier = prospect["tier"]
    ask = {
        "Fast":   "offer a 15-min call and mention the free pilot directly",
        "Medium": "ask to connect their data team, don't pitch yet",
    }[tier]

    prompt = f"""
Prospect: {prospect['name']}, {prospect['role']} at {prospect['company']}
Tier: {tier}
Recent activity: {prospect.get('recent_post_or_note', 'none')}

Generate:
1. CONNECTION NOTE (max 300 chars): {ask}
2. FOLLOWUP 1 (sent 2-4 days after acceptance): value + soft ask
3. FOLLOWUP 2 (sent 5-7 days if no reply to followup 1):
   drop the Presidio benchmark angle (4,000x faster, 1M rows in 3 seconds)
   + free pilot offer

Return as JSON: {{"connection_note": "", "followup_1": "", "followup_2": ""}}
"""
    response = client.messages.create(
        model="claude-haiku-4-5-20251001",
        max_tokens=600,
        system=SYSTEM,
        messages=[{"role": "user", "content": prompt}]
    )
    return json.loads(response.content[0].text)

with open("prospects.csv") as f, open("outreach.csv", "w") as out:
    reader = csv.DictReader(f)
    writer = csv.DictWriter(out, fieldnames=[
        "name","company","role","linkedin_url","tier",
        "connection_note","followup_1","followup_2"
    ])
    writer.writeheader()
    for row in reader:
        if row["tier"] == "Slow":
            continue
        msgs = generate_messages(row)
        writer.writerow({**row, **msgs})
        print(f"✓ {row['name']} @ {row['company']}")
```

### Outreach sequence by tier

| Day | Fast tier | Medium tier |
|-----|-----------|-------------|
| 0 | Send connection note (direct: free pilot) | Send connection note (curiosity angle) |
| 2–4 | Followup 1: direct call ask | Followup 1: compliance angle, ask for data team intro |
| 7 | Followup 2: Presidio benchmark + pilot offer | Followup 2: soft pilot offer |
| 10 | — | Followup 3 (medium only): value drop, no ask |

Target: 10–15 prospects/week, 20 minutes of manual sending.

---

## Retainer Offering

### Pricing tiers

| Tier | Price | When to use |
|------|-------|-------------|
| Launch (first 3 clients) | USD 200/month | Remove all friction, close fast |
| Standard | USD 400/month | After 1 reference client |
| Dual-jurisdiction (GDPR + Chile) | USD 700/month | Multinational subsidiaries, auditor requirements |

Qualify dual-jurisdiction on the call: *"Are you a subsidiary of a multinational, or do you process data from EU citizens?"*

### What the retainer client gets

| Deliverable | Detail |
|-------------|--------|
| Enterprise license | Right to use proprietary CLI + policy engine as it ships |
| Pattern updates | New masking patterns within 2 weeks of Chilean law changes |
| Regulation monitoring | Monthly email: Chilean law changes and whether they're affected |
| Privacy policy alignment | Config matches PII types committed to in their published privacy policy |
| GDPR layer | If processing EU citizen data: EU patterns configured (IBAN, VAT, EU national IDs) |
| Auditor-ready documentation | Formal masking spec for CMF, SOC 2, or ISO 27001 auditors |
| Priority support | 24h response on bugs, false positives, pipeline issues |
| Version upgrades | Integration stays compatible with each new MaskOps release |

### What is NOT included
- Custom development outside MaskOps scope
- GDPR/legal advice (software engineer, not a lawyer — always state this)
- Unlimited calls

### The FPE decryption pitch
MaskOps FPE mode is the strongest sales hook:

> *"Your data team sees RUTs. Your analytics models train on RUTs. But at no point is a real RUT sitting in a log file or a data warehouse — and if a regulator asks for a specific record, you can produce it in 10 seconds. The key stays with you, air-gapped. Nobody else can decrypt it."*

This is GDPR Art. 4(5) pseudonymization. Lighter regulatory burden than storing raw PII. Their legal team will value the distinction. See `docs/gdpr/gdpr-reference.md` for the full argument.

---

## Chile → LATAM Expansion Sequence

| Phase | Trigger | Action |
|-------|---------|--------|
| **Phase 0** | Now | Free pilot to 2 Fast-tier Chilean fintechs |
| **Phase 1** | 1 reference client + testimonial | Switch to paid (USD 200/month), target 4 more Chilean fintechs |
| **Phase 2** | 3 paying Chilean clients (USD 600/month) | Begin Argentina outreach. **Argentine DNI (v0.5.0) must be shipped first.** |
| **Phase 3** | 5 paying clients total (USD 1K+/month) | Colombia and Peru. Colombian CC/NIT and Peruvian DNI must be shipped. |
| **Phase 4** | v1.0 ships (CLI + policy files live) | Approach Medium-tier and state-adjacent companies with a full product demo |

**Rule:** Never pitch a country whose ID format is not yet implemented. GTM and roadmap expand in lockstep.

---

## Key Decisions Recorded

- **Side income model chosen** — 2–5 clients, USD 1–3K/month. No company formation required at this stage.
- **Direct outreach chosen** over content marketing — faster to first client.
- **Fast-tier fintechs first** — short decision cycles, CTO/founder approval without procurement committee.
- **Free pilot as first step** — removes trust barrier with zero track record.
- **LLM pipeline is semi-automated** — generates drafts, human sends. LinkedIn auto-send violates ToS.
- **FPE decryption is the primary hook** — not compliance-as-a-burden but competitive advantage.
- **GDPR overclaiming is explicitly forbidden** — FPE = pseudonymization, not anonymization.
- **Slow-tier entities deferred** — until 2+ reference clients exist.
