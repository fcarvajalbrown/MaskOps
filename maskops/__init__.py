from __future__ import annotations
from pathlib import Path
from typing import TYPE_CHECKING

import polars as pl
from polars.plugins import register_plugin_function
from typing import Union
import polars as pl
IntoExpr = Union[pl.Expr, str]

if TYPE_CHECKING:
    pass

from maskops._keys import validate_key, validate_tweak, derive_key, derive_tweak

_LIB = Path(__file__).parent

def mask_pii(
    expr: IntoExpr,
    patterns: list = None,
    mode: str = "asterisk",
    salt: str = None,
) -> pl.Expr:
    """
    Mask detected PII in a String column.

    Parameters
    ----------
    expr : IntoExpr
        A Polars column name (str) or expression resolving to a String series.
    patterns : list[str] | None
        Optional list of pattern names to apply. Valid names:
        ``email``, ``phone``, ``ip``, ``iban``, ``vat``, ``dni``, ``nie``,
        ``nin``, ``personalausweis``, ``us_passport``, ``curp``, ``rut``,
        ``cpf``, ``cnpj``, ``ssn``, ``arg_dni``, ``co_cc``, ``co_nit``, ``ec_cedula``,
        ``credit_card``, ``npi``, ``mbi``, ``nhs``, ``pe_dni``.
    mode : str
        Masking mode. ``"asterisk"`` (default): irreversible redaction.
        ``"consistent"``: deterministic HMAC-SHA256 pseudonymization — requires ``salt``.
        The same value always produces the same masked output for a given salt, so applying
        ``mode="consistent"`` with the same salt across multiple columns preserves referential
        integrity: a customer ID appearing in ``customer_id``, ``reference_id``, and free-text
        notes will mask to the same value everywhere.
    salt : str | None
        Required when ``mode="consistent"``. Secret salt for HMAC-SHA256.
        Must be kept separate from the data (same GDPR key-separation rule as FPE).

    Examples
    --------
    >>> df.with_columns(maskops.mask_pii("col"))
    >>> df.with_columns(maskops.mask_pii("col", patterns=["email", "ssn"]))
    >>> df.with_columns(maskops.mask_pii("col", mode="consistent", salt="my-secret"))
    >>> df.with_columns(
    ...     maskops.mask_pii("customer_id", mode="consistent", salt="my-secret"),
    ...     maskops.mask_pii("reference_id", mode="consistent", salt="my-secret"),
    ... )
    """
    if mode == "consistent":
        if salt is None:
            raise ValueError("mask_pii: mode='consistent' requires a salt")
        col_expr = pl.col(expr) if isinstance(expr, str) else expr
        args = [col_expr, pl.lit(salt)]
        if patterns is not None:
            args.append(pl.lit(",".join(patterns)))
        return register_plugin_function(
            plugin_path=_LIB,
            function_name="mask_pii_consistent",
            args=args,
            is_elementwise=True,
        )
    
    args = [pl.col(expr) if isinstance(expr, str) else expr]
    if patterns is not None:
        args.append(pl.lit(",".join(patterns)))
    return register_plugin_function(
        plugin_path=_LIB,
        function_name="mask_pii",
        args=args,
        is_elementwise=True,
    )

def contains_pii(expr: IntoExpr, patterns: list = None) -> pl.Expr:
    """
    Detect whether a String column contains any known PII pattern.

    Returns a Boolean column — ``True`` where PII was found.

    Parameters
    ----------
    expr : IntoExpr
        A Polars column name (str) or expression resolving to a String series.
    patterns : list[str] | None
        Optional list of pattern names to check. Same valid names as
        ``mask_pii``. Omit to check all patterns.

    Examples
    --------
    >>> df.filter(maskops.contains_pii("notes"))
    >>> df.filter(maskops.contains_pii("notes", patterns=["email", "ssn"]))
    """
    args = [pl.col(expr) if isinstance(expr, str) else expr]
    if patterns is not None:
        args.append(pl.lit(",".join(patterns)))
    return register_plugin_function(
        plugin_path=_LIB,
        function_name="contains_pii",
        args=args,
        is_elementwise=True,
    )

def mask_pii_fpe(
    expr: IntoExpr,
    key: bytes,
    tweak: bytes,
    patterns: list = None,
    mode: str = "ff3",
) -> pl.Expr:
    """
    Mask digit-based PII (cards, phones, RUT, CPF) using format-preserving encryption.
    Non-digit PII (IBAN, VAT, email, IP, EU IDs) is still asterisked.

    The output preserves the original format and length — all digits in, all digits out.
    Reversible using the same key, tweak, and mode via the Rust API.

    Parameters
    ----------
    expr : IntoExpr
        A Polars column name (str) or expression resolving to a String series.
    key : bytes
        32-byte AES-256 key. Must be kept separate from the data (GDPR requirement).
    tweak : bytes
        7-byte context identifier (e.g. tenant ID or dataset identifier).
    patterns : list[str] | None
        Optional list of pattern names to apply. Omit to apply all.
    mode : str
        FPE algorithm. ``"ff3"`` (default): FF3-1. ``"ff1"``: NIST SP 800-38G FF1,
        the construction NIST retains after the FF3 cryptanalysis. Both are reversible;
        the mode used to mask must be used to unmask.

    Returns
    -------
    pl.Expr
        A new expression with digit PII pseudonymised via the chosen FPE mode.

    Examples
    --------
    >>> key   = secrets.token_bytes(32)
    >>> tweak = secrets.token_bytes(7)
    >>> df.with_columns(maskops.mask_pii_fpe("col", key, tweak))
    >>> df.with_columns(maskops.mask_pii_fpe("col", key, tweak, mode="ff1"))
    """
    validate_key(key)
    validate_tweak(tweak)
    args = [
        pl.col(expr) if isinstance(expr, str) else expr,
        pl.lit(key, dtype=pl.Binary),
        pl.lit(tweak, dtype=pl.Binary),
        pl.lit(mode),
    ]
    if patterns is not None:
        args.append(pl.lit(",".join(patterns)))
    return register_plugin_function(
        plugin_path=_LIB,
        function_name="mask_pii_fpe",
        args=args,
        is_elementwise=True,
    )

def rekey_pii_fpe(
    expr: IntoExpr,
    old_key: bytes,
    old_tweak: bytes,
    new_key: bytes,
    new_tweak: bytes,
    mode: str = "ff3",
) -> pl.Expr:
    """
    Rotate the FPE key on a column of FPE tokens without exposing plaintext.

    Operates cell-by-cell on a dedicated identifier column whose values are single
    FPE ciphertexts (e.g. the output of ``mask_pii_fpe`` over a bare ``card_number``
    column). Each digit cell is decrypted with the old key/tweak and re-encrypted with
    the new key/tweak in one pass; the plaintext exists only transiently inside the Rust
    kernel, never as a column. Cells that are not all-digit FPE tokens pass through
    unchanged. The result is identical to masking the original plaintext under the new
    key/tweak — so rotation is exact, not approximate.

    Note: because MaskOps FPE does not preserve issuer prefixes or check digits, tokens
    embedded in free text cannot be re-detected after masking. Rotate the dedicated
    identifier column, not free-text columns.

    Parameters
    ----------
    expr : IntoExpr
        A column whose cells are FPE tokens produced under ``old_key``/``old_tweak``.
    old_key, old_tweak : bytes
        The key/tweak the column is currently encrypted under.
    new_key, new_tweak : bytes
        The key/tweak to re-encrypt under.
    mode : str
        FPE algorithm, ``"ff3"`` or ``"ff1"``. Must match the mode used to mask.

    Examples
    --------
    >>> df.with_columns(maskops.rekey_pii_fpe("card_number", k1, t1, k2, t2))
    """
    for k in (old_key, new_key):
        validate_key(k)
    for t in (old_tweak, new_tweak):
        validate_tweak(t)
    args = [
        pl.col(expr) if isinstance(expr, str) else expr,
        pl.lit(old_key, dtype=pl.Binary),
        pl.lit(old_tweak, dtype=pl.Binary),
        pl.lit(new_key, dtype=pl.Binary),
        pl.lit(new_tweak, dtype=pl.Binary),
        pl.lit(mode),
    ]
    return register_plugin_function(
        plugin_path=_LIB,
        function_name="mask_pii_fpe_rekey",
        args=args,
        is_elementwise=True,
    )

def extract_pii(expr: IntoExpr) -> pl.Expr:
    return register_plugin_function(
        plugin_path=_LIB,
        function_name="extract_pii",
        args=[pl.col(expr) if isinstance(expr, str) else expr],
        is_elementwise=True,
    )

def mask_pii_audit(expr: IntoExpr) -> pl.Expr:
    return register_plugin_function(
        plugin_path=_LIB,
        function_name="mask_pii_audit",
        args=[pl.col(expr) if isinstance(expr, str) else expr],
        is_elementwise=True,
    )


from maskops._policy import load_policy
from maskops._manifest import masking_manifest, write_manifest

__all__ = [
    "mask_pii", "contains_pii", "mask_pii_fpe", "rekey_pii_fpe",
    "extract_pii", "mask_pii_audit",
    "load_policy", "masking_manifest", "write_manifest",
    "validate_key", "validate_tweak", "derive_key", "derive_tweak",
]