from typing import Literal, TypeAlias, Union

import polars as pl

from maskops._keys import (
    derive_key as derive_key,
    derive_tweak as derive_tweak,
    validate_key as validate_key,
    validate_tweak as validate_tweak,
)
from maskops._manifest import (
    masking_manifest as masking_manifest,
    write_manifest as write_manifest,
)
from maskops._policy import (
    Policy as Policy,
    load_policy as load_policy,
)

IntoExpr: TypeAlias = Union[pl.Expr, str]
MaskMode: TypeAlias = Literal["asterisk", "consistent"]
FpeMode: TypeAlias = Literal["ff3", "ff1"]

__all__: list[str]

def mask_pii(
    expr: IntoExpr,
    patterns: list[str] | None = ...,
    mode: MaskMode = ...,
    salt: str | None = ...,
) -> pl.Expr: ...
def contains_pii(
    expr: IntoExpr,
    patterns: list[str] | None = ...,
) -> pl.Expr: ...
def mask_pii_fpe(
    expr: IntoExpr,
    key: bytes,
    tweak: bytes,
    patterns: list[str] | None = ...,
    mode: FpeMode = ...,
) -> pl.Expr: ...
def rekey_pii_fpe(
    expr: IntoExpr,
    old_key: bytes,
    old_tweak: bytes,
    new_key: bytes,
    new_tweak: bytes,
    mode: FpeMode = ...,
    pattern: str | None = ...,
) -> pl.Expr: ...
def extract_pii(
    expr: IntoExpr,
    patterns: list[str] | None = ...,
) -> pl.Expr: ...
def mask_pii_audit(
    expr: IntoExpr,
    patterns: list[str] | None = ...,
) -> pl.Expr: ...
