import re

import polars as pl
import pytest

import maskops
from maskops._manifest import PII_FAMILIES, REGULATION, REVERSIBLE_FAMILIES

_PROBE = pl.DataFrame({"col": ["probe"]})


def _rust_pattern_names():
    with pytest.raises(pl.exceptions.ComputeError) as excinfo:
        _PROBE.with_columns(maskops.mask_pii("col", patterns=["__no_such_pattern__"]))
    message = str(excinfo.value)
    _, _, listed = message.partition("valid patterns: ")
    assert listed, f"pattern error message no longer advertises the valid list: {message}"
    return [name.strip() for name in listed.strip().rstrip(".").split(",")]


def _extract_pii_fields():
    out = _PROBE.select(maskops.extract_pii("col").alias("pii")).unnest("pii")
    return list(out.columns)


def _audit_count_fields():
    out = _PROBE.select(maskops.mask_pii_audit("col").alias("audit")).unnest("audit").unnest("counts")
    return [name for name in out.columns if name != "masked"]


class TestPatternRegistrySync:
    def test_extract_fields_match_pattern_names(self):
        assert sorted(_extract_pii_fields()) == sorted(_rust_pattern_names())

    def test_audit_counts_match_pattern_names(self):
        assert sorted(_audit_count_fields()) == sorted(_rust_pattern_names())

    def test_extract_and_audit_expose_the_same_families(self):
        assert _extract_pii_fields() == _audit_count_fields()

    def test_manifest_families_match_the_engine(self):
        assert sorted(PII_FAMILIES) == sorted(_rust_pattern_names())

    def test_every_family_has_a_regulation(self):
        assert sorted(REGULATION) == sorted(PII_FAMILIES)

    def test_reversible_families_are_known_families(self):
        assert REVERSIBLE_FAMILIES <= set(PII_FAMILIES)

    def test_reversible_families_survive_fpe_as_digits(self):
        unknown = REVERSIBLE_FAMILIES - set(_rust_pattern_names())
        assert not unknown

    def test_documented_pattern_names_match_the_engine(self):
        section = maskops.mask_pii.__doc__.split("Valid names:")[1].split("Unknown names")[0]
        documented = set(re.findall(r"``([a-z_]+)``", section))
        assert documented == set(_rust_pattern_names())
