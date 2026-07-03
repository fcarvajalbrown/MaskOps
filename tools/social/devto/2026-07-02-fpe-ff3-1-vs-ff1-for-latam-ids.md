# Format-preserving encryption for PII in Polars: FF3-1 vs FF1 for RUT, CPF, and DNI

> Published 2026-07-02 to dev.to. https://dev.to/fcarvajalbrown/format-preserving-encryption-for-pii-in-polars-ff3-1-vs-ff1-for-rut-cpf-and-dni-70j
> Tags: polars, rust, python, privacy
> Cover: covers/fpe-ff3-ff1-domain-minimum.png
> Description: FF3-1 vs FF1 format-preserving encryption for RUT, CPF, and DNI in Polars: reversible PII masking, the domain minimum, and GDPR pseudonymization.

---

You need to hand a dataset of Chilean RUTs to an outside analytics team. They will join it against other tables by identifier, run the cohort analysis, and hand back a model. They do not need to know, and should never learn, who any of these people are. Asterisk the RUT column and the join dies on contact: `**********-K` matches every other asterisked RUT in the file. Not almost every one. Every one. You need the same input to reappear as the same output, shaped like a real, check-digit-valid identifier the rest of your schema still recognizes, and eight weeks later, when a fraud investigator needs the original RUT back for one row, you need to be able to give it to them.

Irreversible masking cannot do any of this. Hashing gets you consistency but not the format, and never the value back. What you need is format-preserving encryption: run a digit string through a cipher and get out another digit string, same length, same shape, that decrypts to the original under the key you hold. Nothing else.

## What FPE actually does

MaskOps exposes this as `mask_pii_fpe`. It masks digit-based PII, cards, phones, RUT, CPF, Argentine DNI, in place, and gives back something the same length and shape:

```python
import maskops
import secrets

key = secrets.token_bytes(32)     # AES-256, client holds this
tweak = secrets.token_bytes(7)    # per-column/per-dataset context

df.with_columns(maskops.mask_pii_fpe("rut_column", key, tweak))
```

`76.354.771-K` becomes some other RUT-shaped, check-digit-valid string of the same length, under this key and tweak. Run it back through with the same key and tweak and it decrypts. Non-digit PII, IBAN, VAT, email, IP, EU national IDs, gets none of this. It always asterisks. There is no clean digit domain to encrypt into, so MaskOps does not pretend there is.

The key never touches MaskOps' output. The client generates it, holds it, and passes it in at call time, and because MaskOps makes no network call and keeps no storage layer, there is nowhere for that key to leak to even if someone wanted it to. That separation is not a nice-to-have sitting on top of the design. It is the design. GDPR Article 4(5) defines pseudonymization as data that cannot be re-attributed to a person "without the use of additional information," provided that information "is kept separately." The key is the additional information. Keep it separate and the column is pseudonymized. Store it next to the data, and it is just encryption wearing a compliance label.

Say it plainly, because it gets glossed over in every vendor deck that sells this as a compliance checkbox: FPE output is still personal data. It is reversible by construction, so anyone holding the key can undo it, and that makes it pseudonymization under GDPR, not anonymization. Is that a weaker claim than "anonymized"? Yes. It is also the true one. Anonymous data falls outside GDPR's scope entirely. Pseudonymized data does not. If you need the stronger claim, use asterisk masking instead, and give up the join.

## Two ciphers, one decision NIST is still making

`mask_pii_fpe` takes a `mode` argument: `"ff3"`, the default, or `"ff1"`. Both are Feistel-network constructions over AES-256. Both are reversible. Both preserve length and format. The difference between them is not cryptographic strength today. It is which one NIST still stands behind tomorrow.

Give FF3-1 its due first. NIST SP 800-38G approved it in 2016 alongside FF1. A later draft narrowed it to FF3-1 by shrinking the tweak from 64 bits to 56 and raising the minimum domain. It is what MaskOps has supported the longest, and it is not broken today. Then, in the second public draft of SP 800-38G Revision 1, published in February 2025, NIST proposed removing FF3 and FF3-1 outright, citing a 2021 cryptanalytic result by Beyne. That draft has not been finalized as of this writing, so FF3-1 has not been formally withdrawn. But the direction is not a rumor. FF1 is the mode NIST intends to keep, and FF3-1 is the mode NIST intends to retire.

What does that attack actually require? Not a break of AES, and not key recovery. It is a distinguishing and message-recovery attack against the FF3-1 Feistel structure, and it needs a large number of known or chosen plaintext and ciphertext pairs, all under one fixed key and tweak, over one small domain. Shrink the domain or grow the volume under a single tweak, and the attack gets cheaper. Encrypt a handful of RUTs under a dedicated tweak, and you are nowhere near its reach. Encrypt millions of rows of a six-digit field under one tweak forever, and you are exactly the case NIST is warning about.

MaskOps' position is the one a security team doing diligence wants stated out loud, not discovered in a footnote: FF3-1 is not broken. It is scheduled, and FF1 is where anything new should land. `rekey_pii_fpe` exists to move a column from one to the other, decrypting under the old key and mode and re-encrypting under the new one in a single pass, so the plaintext never has to sit still as a materialized column while you switch.

## Why the domain minimum is not a formality

Both FF1 and FF3-1 inherit a minimum-domain rule from SP 800-38G: radix raised to the minimum length must be at least 1,000,000. In base 10, why six digits and not five? Because below that floor, the space of possible values is small enough that FPE stops being a meaningfully strong cipher, and small domains are exactly what makes the Beyne attack cheap. MaskOps enforces this in the Rust core, not in a comment somebody will forget to update:

```rust
pub(crate) const MIN_LEN: usize = 6;
pub(crate) const MAX_LEN: usize = 30;
```

Feed it a five-digit value and it refuses, for both modes, rather than handing back weak ciphertext and letting you find out the hard way.

Here is where the LatAm identifier set earns its keep as a worked example, because the three families do not hit that floor the same way. A Chilean RUT is a seven-or-eight-digit body plus one check digit (`76.354.771-K`, where `K` is the check digit), and MaskOps encrypts only the body, leaving the check digit in cleartext so the output still validates as a RUT. An Argentine DNI (`12.345.678`) carries no separate check digit, so the whole seven-or-eight-digit run gets encrypted. A Brazilian CPF (`529.982.247-25`) is eleven digits encrypted whole. All three clear the floor comfortably. A four-digit PIN or a short internal sequence number would not, and that is exactly the case `MIN_LEN` exists to refuse, not to work around.

## What this buys you

`mask_pii_fpe` answers one narrow, common question: how to anonymize PII in Python pipelines without breaking joins across tables or downstream format validation, and without losing the ability to recover the original value with a key you control. Call it PII masking in Polars, not a script bolted on from outside: the cipher runs inside the same expression engine already scanning the dataframe. If you have looked at Presidio for this job, know what it is and is not. Presidio does named-entity recognition over free text. It does not run natively as a Polars expression, and it does not ship check-digit-validated RUT, CPF, or CNPJ detection at all. MaskOps is a Presidio alternative for structured columns with schema-defined PII: RUT / CPF detection with real Módulo 11 validation, running as a native Rust expression with no network call anywhere in the path.

None of this replaces a lawyer or a data protection officer telling you your legal basis for processing personal data. It replaces the part of the compliance story that was always a data-engineering problem to begin with: getting a reversible, honestly-labeled pseudonymization pass onto a column without shipping the plaintext anywhere. FF3-1 is not broken. It is scheduled. Encrypt what is new under FF1, and decide once, not twice.

MaskOps is open source, MPL-2.0, on [PyPI](https://pypi.org/project/maskops/) and [GitHub](https://github.com/fcarvajalbrown/MaskOps).
