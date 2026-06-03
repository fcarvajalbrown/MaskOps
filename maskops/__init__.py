"""
maskops — High-speed PII masking as a Polars plugin.

Usage::

    import polars as pl
    import maskops

    df = pl.DataFrame({"payments": ["Ref: DE89370400440532013000", "nothing here"]})
    df.with_columns(maskops.mask_pii("payments"))
    # ┌──────────────────────────────┐
    # │ payments                     │
    # ╞══════════════════════════════╡
    # │ Ref: DE89******************  │
    # │ nothing here                 │
    # └──────────────────────────────┘
"""

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

# Path to the compiled Rust shared library inside this package
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
        ``cpf``, ``ssn``, ``arg_dni``, ``co_cc``, ``co_nit``, ``ec_cedula``,
        ``credit_card``, ``npi``, ``mbi``, ``nhs``, ``pe_dni``.
    mode : str
        Masking mode. ``"asterisk"`` (default): irreversible redaction.
        ``"consistent"``: deterministic HMAC-SHA256 pseudonymization — requires ``salt``.
    salt : str | None
        Required when ``mode="consistent"``. Secret salt for HMAC-SHA256.
        Must be kept separate from the data (same GDPR key-separation rule as FPE).

    Examples
    --------
    >>> df.with_columns(maskops.mask_pii("col"))
    >>> df.with_columns(maskops.mask_pii("col", patterns=["email", "ssn"]))
    >>> df.with_columns(maskops.mask_pii("col", mode="consistent", salt="my-secret"))
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
    # default: asterisk
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

def mask_pii_fpe(expr: IntoExpr, key: bytes, tweak: bytes, patterns: list = None) -> pl.Expr:
    """
    Mask digit-based PII (cards, phones, RUT, CPF) using FF3-1 format-preserving encryption.
    Non-digit PII (IBAN, VAT, email, IP, EU IDs) is still asterisked.

    The output preserves the original format and length — all digits in, all digits out.
    Reversible using the same key and tweak via the Rust API (Ff3Cipher::decrypt).

    Parameters
    ----------
    expr : IntoExpr
        A Polars column name (str) or expression resolving to a String series.
    key : bytes
        32-byte AES-256 key. Must be kept separate from the data (GDPR requirement).
    tweak : bytes
        7-byte context identifier (e.g. tenant ID or dataset identifier).

    Returns
    -------
    pl.Expr
        A new expression with digit PII pseudonymised via FF3-1.

    Examples
    --------
    >>> key   = secrets.token_bytes(32)
    >>> tweak = secrets.token_bytes(7)
    >>> df.with_columns(maskops.mask_pii_fpe("col", key, tweak))
    """
    args = [
        pl.col(expr) if isinstance(expr, str) else expr,
        pl.lit(key, dtype=pl.Binary),
        pl.lit(tweak, dtype=pl.Binary),
    ]
    if patterns is not None:
        args.append(pl.lit(",".join(patterns)))
    return register_plugin_function(
        plugin_path=_LIB,
        function_name="mask_pii_fpe",
        args=args,
        is_elementwise=True,
    )