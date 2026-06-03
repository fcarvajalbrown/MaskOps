# GDPR Reference for MaskOps

> MaskOps-specific reference. Covers only the articles and recitals directly relevant to PII masking, pseudonymization, and air-gapped data processing.
> Last reviewed: 2026-06-03. Source: [gdpr-info.eu](https://gdpr-info.eu)

---

## The core distinction MaskOps is built on

| Concept | GDPR status | MaskOps implementation |
|---------|-------------|----------------------|
| **Anonymous data** | Outside GDPR scope (Recital 26) | Not achievable with structured PII — don't claim this |
| **Pseudonymous data** | Inside GDPR scope, reduced risk | FPE output (`mask_pii_fpe`) — key stored separately |
| **Personal data** | Full GDPR obligations | Raw PII before masking |

**The key rule:** FPE output is pseudonymous — still personal data, still under GDPR, but with significantly reduced risk and lighter compliance burden. Asterisk output is irreversible but not legally anonymous unless re-identification is truly impossible (high bar). Never overclaim either mode.

---

## Recital 26 — Anonymous vs. pseudonymous data

> Data protection principles do **not** apply to truly anonymous information — data rendered anonymous such that the subject is not or no longer identifiable.
>
> Pseudonymized data that **could** be linked to individuals through additional information (e.g. an FPE key) qualifies as **personal data** and remains inside GDPR scope.

**MaskOps implication:** FPE-masked RUTs, CPFs, and credit card numbers are pseudonymous — the key makes re-identification possible. Key separation (client holds the key, MaskOps never sees it) is what limits risk, not what removes GDPR applicability.

---

## Article 4 — Key definitions

**Personal data (Art. 4(1)):** Any information relating to an identified or identifiable natural person. A RUT, CPF, CURP, IBAN, or email address is personal data.

**Pseudonymisation (Art. 4(5)):**
> Processing personal data in such a manner that the personal data can no longer be attributed to a specific data subject without the use of additional information, provided that such additional information is kept separately and is subject to technical and organisational measures to ensure that the personal data are not attributed to an identified or identifiable natural person.

This is the exact legal definition MaskOps FPE mode satisfies — provided the key is kept separately.

**Processing (Art. 4(2)):** Any operation performed on personal data, including collection, storage, use, and erasure. Running `mask_pii` on a DataFrame column is processing under GDPR.

---

## Article 5 — Principles (what clients must comply with)

| Principle | What it means for MaskOps clients |
|-----------|----------------------------------|
| **Lawfulness, fairness, transparency** | Must have a legal basis for processing PII before masking it |
| **Purpose limitation** | Masked data used only for the stated purpose (analytics, ML, etc.) |
| **Data minimisation** | Only collect and process what is necessary — MaskOps helps by masking excess fields |
| **Accuracy** | Not directly affected by masking |
| **Storage limitation** | Masked data should not be stored longer than needed |
| **Integrity and confidentiality** | MaskOps directly addresses this — encryption and pseudonymization at the pipeline level |

---

## Article 25 — Data protection by design and by default

> Controllers must implement suitable technical and organisational measures — including pseudonymisation — to fulfil data-protection principles by design and by default.

**MaskOps position:** MaskOps is a technical implementation of Art. 25. Clients can cite MaskOps integration as a data-protection-by-design measure in their compliance documentation and auditor reports.

The auditor artefact (one-pager describing what is masked, how, and under which legal basis) produced under the retainer model directly supports Art. 25 compliance evidence.

---

## Article 32 — Security of processing

> Controllers and processors must implement security measures including:
> - **Pseudonymisation and encryption of personal data**
> - Confidentiality, integrity, availability and resilience of processing systems
> - Ability to restore data access after incidents
> - Regular testing and evaluation of security measures

**MaskOps directly satisfies the pseudonymisation and encryption requirement** of Art. 32(1)(a). This is the strongest legal hook for sales: integrating MaskOps is a documented step toward Art. 32 compliance.

---

## What MaskOps does NOT cover

| GDPR obligation | Who covers it |
|----------------|--------------|
| Lawful basis for processing (Art. 6) | Client / their legal team |
| Data subject rights (access, erasure, portability — Art. 15–20) | Client's systems |
| Breach notification (Art. 33–34) | Client's incident response process |
| Data Protection Officer (Art. 37) | Client |
| International transfer safeguards (Art. 46) | Client / legal team |
| Consent management | Client |

MaskOps is a technical tool, not a compliance programme. Always communicate this clearly to clients.

---

## Relevant special category note (Article 9)

Health data, biometric data, genetic data, and racial/ethnic origin are **special category data** under Art. 9 — higher protection requirements, explicit consent or specific legal basis required.

MaskOps patterns that touch special category data:
- `healthcare/` module (NPI, Medicare, NHS) — health identifiers
- `eu/european_id.rs` — national IDs often linked to ethnicity in some jurisdictions
- `contact/email.rs` — emails can link to health accounts

When a client processes special category data, note that masking reduces risk but does not change the lawful basis requirement.

---

## FPE key management — the compliance-critical detail

The entire pseudonymization argument rests on key separation. The key must:

1. Never be stored in the same database, file, or system as the masked data
2. Be accessible only to authorized personnel for specific, documented purposes (fraud investigation, regulatory audit, customer service lookup)
3. Be rotatable — if a key is compromised, the pseudonymization is broken for that dataset
4. Be backed up securely — if lost, pseudonymized data becomes permanently inaccessible

MaskOps v1.8.0 (FPE key management helpers) directly addresses points 3 and 4.

---

## Sources

- [GDPR full text — EUR-Lex](https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX%3A32016R0679)
- [Recital 26 — gdpr-info.eu](https://gdpr-info.eu/recitals/no-26/)
- [Article 4 — gdpr-info.eu](https://gdpr-info.eu/art-4-gdpr/)
- [Article 5 — gdpr-info.eu](https://gdpr-info.eu/art-5-gdpr/)
- [Article 25 — gdpr-info.eu](https://gdpr-info.eu/art-25-gdpr/)
- [Article 32 — gdpr-info.eu](https://gdpr-info.eu/art-32-gdpr/)
