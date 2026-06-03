"""maskops CLI — maskops run <config.toml> <input.parquet> <output.parquet>"""
from __future__ import annotations
import argparse
import sys
from pathlib import Path

if sys.version_info >= (3, 11):
    import tomllib
else:
    try:
        import tomllib
    except ImportError:
        import tomli as tomllib  # type: ignore[no-redef]

import polars as pl
import maskops


def _load_config(path: Path) -> list[dict]:
    with path.open("rb") as f:
        data = tomllib.load(f)
    columns = data.get("columns", [])
    if not isinstance(columns, list):
        raise ValueError("config: 'columns' must be an array of tables ([[columns]])")
    return columns


def _apply_column(df: pl.DataFrame, col_cfg: dict) -> pl.DataFrame:
    name = col_cfg.get("name")
    if not name:
        raise ValueError(f"config: column entry missing required 'name' field: {col_cfg}")
    if name not in df.columns:
        raise ValueError(f"config: column '{name}' not found in input file")
    mode = col_cfg.get("mode", "asterisk")
    patterns = col_cfg.get("patterns") or None
    if mode == "consistent":
        salt = col_cfg.get("salt")
        if not salt:
            raise ValueError(
                f"config: column '{name}' mode='consistent' requires a 'salt' value"
            )
        expr = maskops.mask_pii(name, patterns=patterns, mode="consistent", salt=salt)
    elif mode == "asterisk":
        expr = maskops.mask_pii(name, patterns=patterns)
    else:
        raise ValueError(
            f"config: column '{name}' unknown mode '{mode}' — use 'asterisk' or 'consistent'"
        )
    return df.with_columns(expr)


def cmd_run(args: argparse.Namespace) -> None:
    config_path = Path(args.config)
    input_path = Path(args.input)
    output_path = Path(args.output)

    if not config_path.exists():
        print(f"error: config not found: {config_path}", file=sys.stderr)
        sys.exit(1)
    if not input_path.exists():
        print(f"error: input file not found: {input_path}", file=sys.stderr)
        sys.exit(1)

    try:
        columns = _load_config(config_path)
        df = pl.read_parquet(input_path)
        for col_cfg in columns:
            df = _apply_column(df, col_cfg)
        df.write_parquet(output_path)
        print(f"wrote {len(df)} rows → {output_path}")
    except Exception as exc:
        print(f"error: {exc}", file=sys.stderr)
        sys.exit(1)


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="maskops",
        description="High-speed PII masking for Parquet files.",
    )
    subparsers = parser.add_subparsers(dest="command", metavar="command")
    subparsers.required = True

    run_p = subparsers.add_parser("run", help="mask a Parquet file using a TOML config")
    run_p.add_argument("config", help="path to TOML config file")
    run_p.add_argument("input", help="input Parquet file path")
    run_p.add_argument("output", help="output Parquet file path")
    run_p.set_defaults(func=cmd_run)

    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
