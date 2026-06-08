# FF3-1, NIST SP 800-38G, and MaskOps FPE — Status & Risk Position

> Defensibility note for security and compliance due diligence. Written to be raised *before* a prospect's security team raises it.
> Last reviewed: 2026-06-08.

---

## What MaskOps implements

`mask_pii_fpe` uses FF3-1 format-preserving encryption over AES-256:

- **Key:** 32 bytes (AES-256), held by the client. MaskOps never stores or transmits it.
- **Tweak:** 7 bytes (56-bit) — this is the FF3-1 construction. The original FF3 used a 64-bit tweak; FF3-1 narrowed it to 56 bits.
- **Radix:** 10 (decimal digits).
- **Domain:** 2–30 digits per value.
- **Structure:** 8-round Feistel with the FF3-1 tweak split (`src/patterns/fpe.rs`).

FF3-1 is what produces the reversible, format-preserving pseudonymization claimed under GDPR Art. 4(5).

---

## NIST status (as of June 2026)

- **SP 800-38G (2016)** approved two FPE modes: **FF1** and **FF3**.
- **SP 800-38G Rev.1, first public draft** narrowed FF3 to **FF3-1** by reducing the tweak from 64 to 56 bits and raising the minimum domain size.
- **SP 800-38G Rev.1, second public draft (3 February 2025)** proposes removing **FF3 and FF3-1 entirely**, citing the Beyne (2021) cryptanalysis. The public comment period closed 4 April 2025.
- **This is still a draft — not final.** As of this review, FF3-1 has not been formally withdrawn in a published final document.
- **FF1 is the surviving NIST-approved FPE mode.**

The practical reading: NIST intends to deprecate FF3-1. A security team doing diligence in 2026 will know this. The correct posture is to acknowledge it directly, not to lead with "NIST-approved."

---

## The Beyne (2021) attack — what it actually requires

The attack is a distinguishing / message-recovery attack against the FF3-1 Feistel structure. It is **not** a break of AES and **not** a key-recovery attack. It requires:

- A **large** number of known or chosen plaintext/ciphertext pairs,
- All produced under a **single key + single tweak**,
- Over a **single small domain** (the smaller the domain, the cheaper the attack).

In other words, the exposure scales with how much data you encrypt under one fixed tweak in one format. It is a practical concern for high-volume, fixed-tweak, small-domain deployments — and a limited concern for bounded ID columns encrypted with appropriate tweak separation. It is not zero. Do not describe it as theoretical.

---

## MaskOps risk position and mitigations

1. **Key separation is mandatory and enforced by design.** The client holds the 32-byte key; MaskOps never sees it. Without the key, FPE output is not reversible. This is the primary control.

2. **Tweak / domain separation reduces the attack surface.** The Beyne attack needs volume under *one* tweak. Using a **distinct tweak per column and per dataset** shrinks the ciphertext available under any single tweak+domain. Today this is the operator's responsibility — the API takes a caller-supplied tweak literal, and a single tweak applied across an entire column is the worst case for this attack. Document your tweak strategy as part of your control set.

3. **Asterisk masking has no FPE exposure.** Where reversibility is not required, `mask_pii` (irreversible asterisk masking) sidesteps the question entirely. Recommend it whenever recovery is not a genuine requirement.

4. **FPE is pseudonymization, never anonymization.** Per [the GDPR reference](gdpr-reference.md), FF3-1 output remains personal data under GDPR. The cryptographic strength of the mode does not change that legal classification.

---

## Forward path

- **FF1 support** (the NIST-surviving FPE mode) is the clean long-term answer and is being evaluated for the roadmap. It would let clients choose the mode that survives the SP 800-38G Rev.1 finalization.
- **FPE key-management helpers (roadmap v1.8)** — rotation, tweak derivation, validation — will make per-context tweak separation a first-class, less error-prone feature rather than an operator convention.

---

## The honest one-line summary for a security team

> MaskOps FPE uses FF3-1, which NIST proposes to deprecate in the (not-yet-final) SP 800-38G Rev.1 second draft. The Beyne (2021) weakness requires large ciphertext volumes under a single key+tweak over a small domain; we mitigate with mandatory client-held key separation and recommend per-column/per-dataset tweak separation. For irreversible needs we recommend asterisk masking. FF1 support is on the roadmap as the NIST-surviving alternative.

---

## Sources

- [SP 800-38G Rev.1 (second public draft) — NIST CSRC](https://csrc.nist.gov/pubs/sp/800/38/g/r1/2pd)
- [SP 800-38G (2016, original) — NIST CSRC](https://csrc.nist.gov/pubs/sp/800/38/g/final)
- [Block Cipher Techniques — NIST CSRC news](https://csrc.nist.gov/Projects/block-cipher-techniques/news)
- [Format-preserving encryption — Wikipedia (attack history)](https://en.wikipedia.org/wiki/Format-preserving_encryption)
