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
