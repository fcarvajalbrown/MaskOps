# MaskOps Paid Pilot Agreement (template)

> One-page pilot agreement. Fill the bracketed fields per deal. This is a commercial template, not legal advice, have a lawyer review before first use in Chile/Brazil. Purpose: convert evaluation into a paying relationship with a single, pre-agreed success test.

---

**Parties:** [Felipe Carvajal Brown Software] ("Provider") and [Client legal name] ("Client").
**Effective date:** [date]   **Pilot term:** [60 / 90] days from effective date.

## 1. Purpose

Client evaluates MaskOps against one written success criterion (Section 3) on Client's own data, in Client's own environment. MaskOps runs fully air-gapped, makes no network calls, and Client retains custody of all data and any FPE keys at all times. Provider never receives Client data or keys.

## 2. Scope

- Provider grants Client a time-limited license to MaskOps for the pilot term, for use by up to [N] users on [environment / dataset].
- Provider supplies onboarding (install + first masked run), a worked manifest / RAT example, and support with a [24h] response window for the term.
- Out of scope: bespoke pattern development beyond [the listed PII families], production SLA, anything not written here.

## 3. Success criterion (single, binary)

The pilot succeeds if, on Client's own data:

> **[mask 1,000,000 rows in under 3 seconds AND produce a manifest / RAT export that passes Client's auditor review on Client's RUT/CPF/CNPJ columns].**

One criterion, measured once, at the end of the term. Both parties sign off pass/fail in writing.

## 4. Fee and conversion

- **Pilot fee:** [USD 200-600 / equivalent CLP / BRL], invoiced at start. The pilot is paid, not free, by design.
- **Credit on conversion:** if Client signs an annual retainer within [30] days of a passing pilot, **100% of the pilot fee is credited** to the first invoice.
- **Money-back:** if the success criterion is **not** met, Provider refunds the pilot fee in full. Client's only risk is time.

## 5. Data, keys, and security

- Client data never leaves Client's perimeter. Provider has no access to it.
- FPE keys are generated and held by Client. Provider never sees, stores, or transmits any key. (FPE output is pseudonymized, not anonymized.)
- MaskOps makes no outbound network calls during the pilot.

## 6. Term, IP, confidentiality

- The license terminates at the end of the pilot term unless an annual retainer is signed. On non-conversion, Client ceases use and Provider's IP reverts fully to Provider.
- Each party keeps the other's non-public information confidential.
- [Optional design-partner clause: in exchange for a [40-50%] rate lock for [12-24] months, Client agrees to a named or anonymized case study and biweekly roadmap feedback.]

## 7. Signatures

Provider: ________________________   Date: __________
Client: __________________________   Date: __________

---

### Running notes (delete before sending)

- Keep it to one page, procurement reads short documents faster.
- Set the success criterion *with* the technical evaluator so it is theirs, not yours, pre-agreed criteria make pilots ~3x more likely to close.
- Do not judge conversion before ~90 days, dev-tool evaluation cycles run 90-180 days.
