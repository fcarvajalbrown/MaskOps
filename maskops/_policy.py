"""maskops policy file loader — YAML/TOML config → Policy → apply to DataFrame."""
from __future__ import annotations
import os
import re
import sys
from pathlib import Path
from typing import Union

import polars as pl
import maskops

PathLike = Union[str, Path]

_ENV_RE = re.compile(r"\$\{([^}]+)\}")

def _interpolate(value: str, column: str) -> str:
    """Replace ${VAR_NAME} tokens with environment variable values."""
    def _sub(m: re.Match) -> str:
        name = m.group(1)
        val = os.environ.get(name)
        if val is None:
            raise KeyError(
                f"policy: column '{column}' references undefined env var '${{{name}}}'"
            )
        return val
    return _ENV_RE.sub(_sub, value)

class Policy:
    """A set of per-column masking rules loaded from a policy file."""

    def __init__(self, columns: dict) -> None:
        self._rules: dict[str, dict] = {}
        for col_name, rule in columns.items():
            mode = rule.get("mode", "asterisk")
            if mode not in ("asterisk", "consistent"):
                raise ValueError(
                    f"policy: column '{col_name}' unknown mode '{mode}' — "
                    "use 'asterisk' or 'consistent'"
                )
            if mode == "consistent":
                salt = rule.get("salt")
                if not salt:
                    raise ValueError(
                        f"policy: column '{col_name}' mode='consistent' requires a 'salt'"
                    )
            self._rules[col_name] = {
                "mode": mode,
                "patterns": rule.get("patterns") or None,
                "salt": rule.get("salt"),
            }

    def apply(self, df: pl.DataFrame) -> pl.DataFrame:
        """Apply all masking rules to *df* and return the modified DataFrame."""
        for col_name, rule in self._rules.items():
            if col_name not in df.columns:
                raise ValueError(
                    f"policy: column '{col_name}' not found in DataFrame"
                )
            mode = rule["mode"]
            patterns = rule["patterns"]
            if mode == "consistent":
                expr = maskops.mask_pii(col_name, patterns=patterns, mode="consistent", salt=rule["salt"])
            else:
                expr = maskops.mask_pii(col_name, patterns=patterns)
            df = df.with_columns(expr)
        return df

    @classmethod
    def from_dict(cls, data: dict) -> "Policy":
        """Build a Policy from a parsed config dict (``{"columns": {...}}``)."""
        raw_columns = data.get("columns", {})
        if not isinstance(raw_columns, dict):
            raise ValueError(
                "policy: 'columns' must be a mapping (got list — "
                "use the TOML [[columns]] array format only with the CLI)"
            )
        resolved: dict = {}
        for col_name, rule in raw_columns.items():
            resolved_rule = {}
            for k, v in rule.items():
                resolved_rule[k] = _interpolate(v, col_name) if isinstance(v, str) else v
            resolved[col_name] = resolved_rule
        return cls(resolved)

def load_policy(path: PathLike) -> Policy:
    """
    Load a YAML or TOML policy file and return a :class:`Policy`.

    Parameters
    ----------
    path : str or Path
        Path to a ``.yaml``/``.yml`` or ``.toml`` policy file.

    Raises
    ------
    FileNotFoundError
        If the file does not exist.
    ValueError
        If the format is invalid or a required field is missing.
    KeyError
        If an ``${ENV_VAR}`` reference cannot be resolved.
    """
    path = Path(path)
    if not path.exists():
        raise FileNotFoundError(f"policy file not found: {path}")

    suffix = path.suffix.lower()
    if suffix in (".yaml", ".yml"):
        import yaml
        with path.open("r", encoding="utf-8") as f:
            data = yaml.safe_load(f) or {}
        return Policy.from_dict(data)

    if suffix == ".toml":
        return _load_toml_policy(path)

    raise ValueError(f"unsupported policy file extension '{suffix}' — use .yaml, .yml, or .toml")

def _load_toml_policy(path: Path) -> Policy:
    if sys.version_info >= (3, 11):
        import tomllib
    else:
        try:
            import tomllib
        except ImportError:
            import tomli as tomllib  

    with path.open("rb") as f:
        data = tomllib.load(f)

    
    if isinstance(data.get("columns"), dict):
        return Policy.from_dict(data)

    
    cols_list = data.get("columns", [])
    if not isinstance(cols_list, list):
        raise ValueError("policy: 'columns' must be a dict or array of tables")
    cols_dict: dict = {}
    for entry in cols_list:
        name = entry.get("name")
        if not name:
            raise ValueError(f"policy: array-format entry missing 'name': {entry}")
        cols_dict[name] = {k: v for k, v in entry.items() if k != "name"}
    return Policy.from_dict({"columns": cols_dict})
