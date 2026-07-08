from __future__ import annotations

import secrets

import polars as pl

import maskops

df: pl.DataFrame = pl.DataFrame({"notes": ["contact a@b.com"]})

masked: pl.Expr = maskops.mask_pii("notes")
masked_selected: pl.Expr = maskops.mask_pii("notes", patterns=["email", "ssn"])
masked_consistent: pl.Expr = maskops.mask_pii("notes", mode="consistent", salt="s")
found: pl.Expr = maskops.contains_pii("notes", patterns=["email"])

key: bytes = secrets.token_bytes(32)
tweak: bytes = secrets.token_bytes(7)
fpe: pl.Expr = maskops.mask_pii_fpe("notes", key, tweak)
fpe_ff1: pl.Expr = maskops.mask_pii_fpe("notes", key, tweak, mode="ff1")

new_key: bytes = secrets.token_bytes(32)
new_tweak: bytes = secrets.token_bytes(7)
rotated: pl.Expr = maskops.rekey_pii_fpe("notes", key, tweak, new_key, new_tweak, pattern="rut")

extracted: pl.Expr = maskops.extract_pii("notes")
audited: pl.Expr = maskops.mask_pii_audit("notes")

out: pl.DataFrame = df.with_columns(masked, found, fpe, extracted, audited)

manifest: pl.DataFrame = maskops.masking_manifest(df)
maskops.write_manifest(manifest, "rat.json", source="customers.parquet")

validated_key: bytes = maskops.validate_key(key)
validated_tweak: bytes = maskops.validate_tweak(tweak)
derived_key: bytes = maskops.derive_key(b"master-secret", "tenant-a")
derived_tweak: bytes = maskops.derive_tweak(b"master-secret", "tenant-a")

policy: maskops.Policy = maskops.load_policy("policy.yaml")
applied: pl.DataFrame = policy.apply(df)
