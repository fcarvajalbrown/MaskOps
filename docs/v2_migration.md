# MaskOps v2.0 Migration Guide

MaskOps 2.0 is the enterprise release. It unifies the three enterprise pillars —
configurable patterns, structured output, and audit — behind one consistent API, and
marks the point from which the public surface is stable for the whole 2.x line.

**2.0 is a drop-in upgrade.** Every 1.x call site keeps working unchanged. There is one
behavioral change to be aware of (weak-key rejection, below); everything else is additive.

## What changed

### One behavioral change: weak-key rejection

`mask_pii_fpe` now validates the key before use. A key that is not 32 bytes, or that is a
single byte repeated 32 times (e.g. `b"\x00" * 32`), is rejected with `ValueError`.

```python
maskops.mask_pii_fpe("card", b"\x00" * 32, tweak)   # 1.x: ran; 2.0: ValueError
maskops.mask_pii_fpe("card", secrets.token_bytes(32), tweak)   # correct
```

If a pipeline used a placeholder all-zero key, replace it with real key material. Use
`maskops.derive_key(master, context)` if you need a deterministic key from a master secret.

The tweak is also length-checked (must be 7 bytes). No other call changes behavior.

## What's new in the 2.x surface

These landed across 1.8 → 2.0 and are all additive.

### Unified `patterns=` (2.0)

`patterns=` now works on every detection expression, not just `mask_pii`. The structured
and audit outputs honour the same family selection:

```python
maskops.extract_pii("notes", patterns=["email", "iban"])      # only those fields populated
maskops.mask_pii_audit("notes", patterns=["ssn", "credit_card"])  # only those masked + counted
```

Omitting `patterns=` keeps the previous behaviour (all families).

### FF1 mode and key management (1.8)

```python
maskops.mask_pii_fpe("card", key, tweak, mode="ff1")   # NIST SP 800-38G FF1
maskops.rekey_pii_fpe("card", old_key, old_tweak, new_key, new_tweak)  # key rotation
maskops.derive_key(master, "tenant-A")     # HKDF-SHA256, offline
maskops.derive_tweak(master, "tenant-A")   # HMAC-SHA256, offline
maskops.validate_key(key); maskops.validate_tweak(tweak)
```

`mode="ff3"` stays the default, so existing FPE calls are unchanged.

### MEA identifiers (1.9)

South African ID (`za_id`, POPIA) and Israeli ID / Teudat Zehut (`il_id`, PPL) are now
detected, masked, extracted, audited, and selectable like every other family.

## Stable API for 2.x

The public surface below is stable for the 2.x line:

- Expressions: `mask_pii`, `contains_pii`, `mask_pii_fpe`, `rekey_pii_fpe`,
  `extract_pii`, `mask_pii_audit`
- Key management: `derive_key`, `derive_tweak`, `validate_key`, `validate_tweak`
- Manifest: `masking_manifest`, `write_manifest`, `load_policy`

`mask_pii(mode=...)` supports `"asterisk"`, `"consistent"`; `mask_pii_fpe(mode=...)`
supports `"ff3"`, `"ff1"`. New PII families may be added in minor releases — they only
add Struct fields to `extract_pii` / `mask_pii_audit`, never rename or remove existing ones.
