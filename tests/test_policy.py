"""Tests for maskops Policy API."""
import os
import pytest
import polars as pl
import maskops
from maskops._policy import Policy

class TestPolicyApply:
    def test_asterisk_mode_masks_email(self):
        policy = Policy({"col": {"mode": "asterisk", "patterns": ["email"]}})
        df = pl.DataFrame({"col": ["contact: user@example.com"]})
        result = policy.apply(df)
        assert "user@example.com" not in result["col"][0]
        assert "***" in result["col"][0] or result["col"][0] == "contact: ***"

    def test_asterisk_mode_all_patterns_when_none(self):
        policy = Policy({"col": {"mode": "asterisk"}})
        df = pl.DataFrame({"col": ["user@example.com"]})
        result = policy.apply(df)
        assert "user@example.com" not in result["col"][0]

    def test_consistent_mode_is_deterministic(self):
        policy = Policy({"col": {"mode": "consistent", "salt": "test-salt"}})
        df = pl.DataFrame({"col": ["4111111111111111"]})
        r1 = policy.apply(df)
        r2 = policy.apply(df)
        assert r1["col"][0] == r2["col"][0]
        assert r1["col"][0] != "4111111111111111"

    def test_consistent_mode_without_salt_raises(self):
        with pytest.raises(ValueError, match="salt"):
            Policy({"col": {"mode": "consistent"}})

    def test_unknown_mode_raises(self):
        with pytest.raises(ValueError, match="mode"):
            Policy({"col": {"mode": "shred"}})

    def test_unknown_column_raises(self):
        policy = Policy({"missing_col": {"mode": "asterisk"}})
        df = pl.DataFrame({"col": ["user@example.com"]})
        with pytest.raises(ValueError, match="missing_col"):
            policy.apply(df)

    def test_multiple_columns_applied_in_order(self):
        policy = Policy({
            "a": {"mode": "asterisk", "patterns": ["email"]},
            "b": {"mode": "asterisk", "patterns": ["email"]},
        })
        df = pl.DataFrame({
            "a": ["user@example.com"],
            "b": ["other@example.com"],
        })
        result = policy.apply(df)
        assert "user@example.com" not in result["a"][0]
        assert "other@example.com" not in result["b"][0]

    def test_empty_columns_dict_returns_unchanged(self):
        policy = Policy({})
        df = pl.DataFrame({"col": ["user@example.com"]})
        result = policy.apply(df)
        assert result["col"][0] == "user@example.com"

class TestLoadPolicyYAML:
    def test_load_yaml_asterisk(self, tmp_path):
        p = tmp_path / "policy.yaml"
        p.write_text(
            "columns:\n"
            "  notes:\n"
            "    patterns: [email]\n"
            "    mode: asterisk\n"
        )
        policy = maskops.load_policy(p)
        df = pl.DataFrame({"notes": ["user@example.com"]})
        result = policy.apply(df)
        assert "user@example.com" not in result["notes"][0]

    def test_load_yml_extension(self, tmp_path):
        p = tmp_path / "policy.yml"
        p.write_text("columns:\n  col:\n    mode: asterisk\n")
        policy = maskops.load_policy(p)
        assert isinstance(policy, Policy)

    def test_load_yaml_consistent_with_literal_salt(self, tmp_path):
        p = tmp_path / "policy.yaml"
        p.write_text(
            "columns:\n"
            "  col:\n"
            "    mode: consistent\n"
            "    salt: mysecret\n"
        )
        policy = maskops.load_policy(p)
        df = pl.DataFrame({"col": ["4111111111111111"]})
        r1 = policy.apply(df)
        r2 = policy.apply(df)
        assert r1["col"][0] == r2["col"][0]

    def test_load_yaml_env_var_interpolation(self, tmp_path, monkeypatch):
        monkeypatch.setenv("MASK_SALT", "env-salt-value")
        p = tmp_path / "policy.yaml"
        p.write_text(
            "columns:\n"
            "  col:\n"
            "    mode: consistent\n"
            "    salt: ${MASK_SALT}\n"
        )
        policy = maskops.load_policy(p)
        df = pl.DataFrame({"col": ["4111111111111111"]})
        result = policy.apply(df)
        assert result["col"][0] != "4111111111111111"

    def test_load_yaml_missing_env_var_raises(self, tmp_path, monkeypatch):
        monkeypatch.delenv("UNDEFINED_VAR", raising=False)
        p = tmp_path / "policy.yaml"
        p.write_text(
            "columns:\n"
            "  col:\n"
            "    mode: consistent\n"
            "    salt: ${UNDEFINED_VAR}\n"
        )
        with pytest.raises(KeyError, match="UNDEFINED_VAR"):
            maskops.load_policy(p)

    def test_load_yaml_no_patterns_applies_all(self, tmp_path):
        p = tmp_path / "policy.yaml"
        p.write_text("columns:\n  col:\n    mode: asterisk\n")
        policy = maskops.load_policy(p)
        df = pl.DataFrame({"col": ["user@example.com"]})
        result = policy.apply(df)
        assert "user@example.com" not in result["col"][0]

    def test_load_yaml_file_not_found(self, tmp_path):
        with pytest.raises(FileNotFoundError):
            maskops.load_policy(tmp_path / "missing.yaml")

class TestLoadPolicyTOML:
    def test_load_toml_dict_format_asterisk(self, tmp_path):
        p = tmp_path / "policy.toml"
        p.write_text(
            "[columns.notes]\n"
            'patterns = ["email"]\n'
            'mode = "asterisk"\n'
        )
        policy = maskops.load_policy(p)
        df = pl.DataFrame({"notes": ["user@example.com"]})
        result = policy.apply(df)
        assert "user@example.com" not in result["notes"][0]

    def test_load_toml_dict_format_consistent(self, tmp_path):
        p = tmp_path / "policy.toml"
        p.write_text(
            "[columns.col]\n"
            'mode = "consistent"\n'
            'salt = "mysecret"\n'
        )
        policy = maskops.load_policy(p)
        df = pl.DataFrame({"col": ["4111111111111111"]})
        r1 = policy.apply(df)
        r2 = policy.apply(df)
        assert r1["col"][0] == r2["col"][0]

    def test_load_toml_env_var_interpolation(self, tmp_path, monkeypatch):
        monkeypatch.setenv("SALT_VAR", "env-salt")
        p = tmp_path / "policy.toml"
        p.write_text(
            "[columns.col]\n"
            'mode = "consistent"\n'
            'salt = "${SALT_VAR}"\n'
        )
        policy = maskops.load_policy(p)
        df = pl.DataFrame({"col": ["4111111111111111"]})
        result = policy.apply(df)
        assert result["col"][0] != "4111111111111111"

    def test_load_toml_array_format_backward_compat(self, tmp_path):
        p = tmp_path / "policy.toml"
        p.write_text(
            "[[columns]]\n"
            'name = "notes"\n'
            'patterns = ["email"]\n'
            'mode = "asterisk"\n'
        )
        policy = maskops.load_policy(p)
        df = pl.DataFrame({"notes": ["user@example.com"]})
        result = policy.apply(df)
        assert "user@example.com" not in result["notes"][0]

    def test_load_toml_unsupported_extension_raises(self, tmp_path):
        p = tmp_path / "policy.json"
        p.write_text("{}")
        with pytest.raises(ValueError, match="unsupported"):
            maskops.load_policy(p)
