"""End-to-end tests for the maskops CLI (maskops run ...)."""
import subprocess
import sys
import textwrap
from pathlib import Path

import polars as pl
import pytest


def _run_cli(*args):
    """Invoke maskops._cli as a subprocess; returns (returncode, stdout, stderr)."""
    result = subprocess.run(
        [sys.executable, "-m", "maskops._cli", *args],
        capture_output=True,
        text=True,
    )
    return result.returncode, result.stdout, result.stderr


def _write_config(tmp_path: Path, content: str) -> Path:
    cfg = tmp_path / "config.toml"
    cfg.write_text(textwrap.dedent(content))
    return cfg


def _write_parquet(tmp_path: Path, df: pl.DataFrame) -> Path:
    p = tmp_path / "input.parquet"
    df.write_parquet(p)
    return p


class TestCliRun:
    def test_run_asterisk_mode(self, tmp_path):
        df = pl.DataFrame({"notes": ["email me at user@example.com", "nothing here"]})
        inp = _write_parquet(tmp_path, df)
        out = tmp_path / "output.parquet"
        cfg = _write_config(tmp_path, """
            [[columns]]
            name = "notes"
            mode = "asterisk"
        """)
        rc, stdout, stderr = _run_cli("run", str(cfg), str(inp), str(out))
        assert rc == 0, stderr
        result = pl.read_parquet(out)
        assert "user@example.com" not in result["notes"][0]   # email was masked
        assert result["notes"][1] == "nothing here"            # clean row unchanged

    def test_run_consistent_mode_is_deterministic(self, tmp_path):
        df = pl.DataFrame({"ref": ["user@example.com", "user@example.com"]})
        inp = _write_parquet(tmp_path, df)
        out = tmp_path / "output.parquet"
        cfg = _write_config(tmp_path, """
            [[columns]]
            name = "ref"
            mode = "consistent"
            salt = "test-salt"
        """)
        rc, _, stderr = _run_cli("run", str(cfg), str(inp), str(out))
        assert rc == 0, stderr
        result = pl.read_parquet(out)
        assert result["ref"][0] == result["ref"][1]       # same input → same output
        assert result["ref"][0] != "user@example.com"     # value was masked

    def test_run_pattern_filter_leaves_other_pii_intact(self, tmp_path):
        df = pl.DataFrame({"col": ["user@example.com and 4111111111111111"]})
        inp = _write_parquet(tmp_path, df)
        out = tmp_path / "output.parquet"
        cfg = _write_config(tmp_path, """
            [[columns]]
            name = "col"
            patterns = ["email"]
        """)
        rc, _, stderr = _run_cli("run", str(cfg), str(inp), str(out))
        assert rc == 0, stderr
        result = pl.read_parquet(out)
        assert "user@example.com" not in result["col"][0]  # email was masked
        assert "4111111111111111" in result["col"][0]       # credit card left alone

    def test_run_multiple_columns(self, tmp_path):
        df = pl.DataFrame({
            "email_col": ["user@example.com"],
            "notes": ["call 555-123-4567"],
        })
        inp = _write_parquet(tmp_path, df)
        out = tmp_path / "output.parquet"
        cfg = _write_config(tmp_path, """
            [[columns]]
            name = "email_col"

            [[columns]]
            name = "notes"
        """)
        rc, _, stderr = _run_cli("run", str(cfg), str(inp), str(out))
        assert rc == 0, stderr
        result = pl.read_parquet(out)
        assert "user@example.com" not in result["email_col"][0]

    def test_run_empty_config_passes_through(self, tmp_path):
        df = pl.DataFrame({"col": ["user@example.com"]})
        inp = _write_parquet(tmp_path, df)
        out = tmp_path / "output.parquet"
        cfg = _write_config(tmp_path, "")   # no columns declared
        rc, _, stderr = _run_cli("run", str(cfg), str(inp), str(out))
        assert rc == 0, stderr
        result = pl.read_parquet(out)
        assert result["col"][0] == "user@example.com"   # unchanged

    def test_error_config_not_found(self, tmp_path):
        inp = _write_parquet(tmp_path, pl.DataFrame({"x": ["a"]}))
        out = tmp_path / "output.parquet"
        rc, _, stderr = _run_cli("run", "nonexistent.toml", str(inp), str(out))
        assert rc != 0
        assert "config not found" in stderr

    def test_error_input_not_found(self, tmp_path):
        cfg = _write_config(tmp_path, "")
        out = tmp_path / "output.parquet"
        rc, _, stderr = _run_cli("run", str(cfg), "nonexistent.parquet", str(out))
        assert rc != 0
        assert "input file not found" in stderr

    def test_error_column_not_in_dataframe(self, tmp_path):
        df = pl.DataFrame({"a": ["foo"]})
        inp = _write_parquet(tmp_path, df)
        out = tmp_path / "output.parquet"
        cfg = _write_config(tmp_path, """
            [[columns]]
            name = "nonexistent_col"
        """)
        rc, _, stderr = _run_cli("run", str(cfg), str(inp), str(out))
        assert rc != 0
        assert "error:" in stderr
        assert "nonexistent_col" in stderr

    def test_error_consistent_without_salt(self, tmp_path):
        df = pl.DataFrame({"col": ["user@example.com"]})
        inp = _write_parquet(tmp_path, df)
        out = tmp_path / "output.parquet"
        cfg = _write_config(tmp_path, """
            [[columns]]
            name = "col"
            mode = "consistent"
        """)
        rc, _, stderr = _run_cli("run", str(cfg), str(inp), str(out))
        assert rc != 0
        assert "error:" in stderr

    def test_error_unknown_mode_no_traceback(self, tmp_path):
        df = pl.DataFrame({"col": ["user@example.com"]})
        inp = _write_parquet(tmp_path, df)
        out = tmp_path / "output.parquet"
        cfg = _write_config(tmp_path, """
            [[columns]]
            name = "col"
            mode = "badmode"
        """)
        rc, _, stderr = _run_cli("run", str(cfg), str(inp), str(out))
        assert rc != 0
        assert "Traceback" not in stderr    # no Python traceback leaked to user
        assert "error:" in stderr

    def test_help_output(self):
        rc, stdout, _ = _run_cli("--help")
        assert rc == 0
        assert "maskops" in stdout

    def test_run_help_output(self):
        rc, stdout, _ = _run_cli("run", "--help")
        assert rc == 0
        assert "config" in stdout


class TestCliRunYAML:
    def test_run_yaml_policy_asterisk(self, tmp_path):
        config = tmp_path / "policy.yaml"
        config.write_text(
            "columns:\n"
            "  notes:\n"
            "    patterns: [email]\n"
            "    mode: asterisk\n"
        )
        df = pl.DataFrame({"notes": ["contact: user@example.com"]})
        input_path = _write_parquet(tmp_path, df)
        output_path = tmp_path / "out.parquet"
        rc, out, err = _run_cli("run", str(config), str(input_path), str(output_path))
        assert rc == 0, err
        result = pl.read_parquet(output_path)
        assert "user@example.com" not in result["notes"][0]

    def test_run_yaml_policy_env_var(self, tmp_path, monkeypatch):
        monkeypatch.setenv("CLI_SALT", "test-salt")
        config = tmp_path / "policy.yaml"
        config.write_text(
            "columns:\n"
            "  col:\n"
            "    mode: consistent\n"
            "    salt: ${CLI_SALT}\n"
        )
        df = pl.DataFrame({"col": ["4111111111111111"]})
        input_path = _write_parquet(tmp_path, df)
        output_path = tmp_path / "out.parquet"
        rc, out, err = _run_cli("run", str(config), str(input_path), str(output_path))
        assert rc == 0, err
        result = pl.read_parquet(output_path)
        assert result["col"][0] != "4111111111111111"

    def test_run_toml_dict_format(self, tmp_path):
        config = tmp_path / "policy.toml"
        config.write_text(
            "[columns.notes]\n"
            'patterns = ["email"]\n'
            'mode = "asterisk"\n'
        )
        df = pl.DataFrame({"notes": ["user@example.com"]})
        input_path = _write_parquet(tmp_path, df)
        output_path = tmp_path / "out.parquet"
        rc, out, err = _run_cli("run", str(config), str(input_path), str(output_path))
        assert rc == 0, err
        result = pl.read_parquet(output_path)
        assert "user@example.com" not in result["notes"][0]
