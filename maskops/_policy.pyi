from pathlib import Path
from typing import TypeAlias, Union

import polars as pl

PathLike: TypeAlias = Union[str, Path]

class Policy:
    def __init__(self, columns: dict[str, dict[str, object]]) -> None: ...
    def apply(self, df: pl.DataFrame) -> pl.DataFrame: ...
    @classmethod
    def from_dict(cls, data: dict[str, object]) -> Policy: ...

def load_policy(path: PathLike) -> Policy: ...
