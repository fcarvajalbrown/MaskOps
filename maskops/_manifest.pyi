from pathlib import Path
from typing import Literal, TypeAlias, Union

import polars as pl

PathLike: TypeAlias = Union[str, Path]
ManifestMode: TypeAlias = Literal["asterisk", "consistent", "fpe"]

PII_FAMILIES: list[str]
REGULATION: dict[str, str]
REVERSIBLE_FAMILIES: set[str]

def masking_manifest(
    df: pl.DataFrame,
    columns: list[str] | None = ...,
    mode: ManifestMode = ...,
) -> pl.DataFrame: ...
def write_manifest(
    manifest: pl.DataFrame,
    path: PathLike,
    source: str | None = ...,
) -> None: ...
