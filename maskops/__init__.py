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


def mask_pii(expr: IntoExpr) -> pl.Expr:
    """
    Mask all detected PII in a String column.

    Replaces matched PII (IBAN, VAT, …) with asterisks while preserving
    non-sensitive characters around the match.

    Parameters
    ----------
    expr : IntoExpr
        A Polars column name (str) or expression resolving to a String series.

    Returns
    -------
    pl.Expr
        A new expression with PII replaced by `*` characters.

    Examples
    --------
    >>> df.with_columns(maskops.mask_pii("iban_col"))
    """
    return register_plugin_function(
        plugin_path=_LIB,
        function_name="mask_pii",
        args=[pl.col(expr) if isinstance(expr, str) else expr],
        is_elementwise=True,
    )


def contains_pii(expr: IntoExpr) -> pl.Expr:
    """
    Detect whether a String column contains any known PII pattern.

    Returns a Boolean column — ``True`` where PII was found.

    Parameters
    ----------
    expr : IntoExpr
        A Polars column name (str) or expression resolving to a String series.

    Returns
    -------
    pl.Expr
        A Boolean expression.

    Examples
    --------
    >>> df.filter(maskops.contains_pii("notes"))
    """
    return register_plugin_function(
        plugin_path=_LIB,
        function_name="contains_pii",
        args=[pl.col(expr) if isinstance(expr, str) else expr],
        is_elementwise=True,
    )