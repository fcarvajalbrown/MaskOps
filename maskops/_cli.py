"""maskops CLI — maskops run <config> <input.parquet> <output.parquet>"""
from __future__ import annotations
import argparse
import sys
from pathlib import Path

import polars as pl
from maskops._policy import load_policy

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
        policy = load_policy(config_path)
        df = pl.read_parquet(input_path)
        df = policy.apply(df)
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

    run_p = subparsers.add_parser("run", help="mask a Parquet file using a YAML or TOML policy")
    run_p.add_argument("config", help="path to YAML or TOML policy file")
    run_p.add_argument("input", help="input Parquet file path")
    run_p.add_argument("output", help="output Parquet file path")
    run_p.set_defaults(func=cmd_run)

    args = parser.parse_args()
    args.func(args)

if __name__ == "__main__":
    main()
