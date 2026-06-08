"""Tests for maskops.masking_manifest / write_manifest (RAT export)."""
import json

import polars as pl
import pytest
import maskops

@pytest.fixture
def df():
    return pl.DataFrame({
        "notes": [
            "cliente RUT 12.345.678-5 email a@b.com",
            "empresa 11.222.333/0001-81 CPF 529.982.247-25",
            "nada aqui",
        ],
        "card": ["4111111111111111", "x", "y"],
        "amount": [10, 20, 30],
    })

class TestMaskingManifest:
    def test_returns_dataframe_with_schema(self, df):
        m = maskops.masking_manifest(df)
        assert m.columns == ["column", "pii_family", "match_count", "regulation", "mask_mode"]

    def test_only_string_columns_scanned(self, df):
        m = maskops.masking_manifest(df)
        assert "amount" not in m["column"].to_list()

    def test_counts_aggregate_across_rows(self, df):
        m = maskops.masking_manifest(df)
        rut = m.filter((pl.col("column") == "notes") & (pl.col("pii_family") == "rut"))
        assert rut["match_count"][0] == 1

    def test_regulation_mapping(self, df):
        m = maskops.masking_manifest(df)
        cnpj = m.filter(pl.col("pii_family") == "cnpj")
        assert "legal-entity" in cnpj["regulation"][0]
        rut = m.filter(pl.col("pii_family") == "rut")
        assert "Ley 21.719" in rut["regulation"][0]

    def test_fpe_mode_only_digit_families(self, df):
        m = maskops.masking_manifest(df, mode="fpe")
        email = m.filter(pl.col("pii_family") == "email")
        assert email["mask_mode"][0] == "asterisk"
        cpf = m.filter(pl.col("pii_family") == "cpf")
        assert cpf["mask_mode"][0] == "fpe"

    def test_explicit_columns(self, df):
        m = maskops.masking_manifest(df, columns=["card"])
        assert set(m["column"].to_list()) == {"card"}
        assert m["pii_family"].to_list() == ["credit_card"]

    def test_unknown_column_raises(self, df):
        with pytest.raises(ValueError):
            maskops.masking_manifest(df, columns=["missing"])

    def test_unknown_mode_raises(self, df):
        with pytest.raises(ValueError):
            maskops.masking_manifest(df, mode="bogus")

    def test_empty_when_no_pii(self):
        m = maskops.masking_manifest(pl.DataFrame({"x": ["nothing here"]}))
        assert m.height == 0
        assert m.columns == ["column", "pii_family", "match_count", "regulation", "mask_mode"]

class TestWriteManifest:
    def test_json_document_structure(self, df, tmp_path):
        m = maskops.masking_manifest(df)
        out = tmp_path / "rat.json"
        maskops.write_manifest(m, out, source="demo.parquet")
        doc = json.loads(out.read_text(encoding="utf-8"))
        assert set(doc) == {"maskops_version", "generated_at", "source", "record_count", "records"}
        assert doc["source"] == "demo.parquet"
        assert doc["record_count"] == m.height
        assert len(doc["records"]) == m.height

    def test_records_round_trip(self, df, tmp_path):
        m = maskops.masking_manifest(df)
        out = tmp_path / "rat.json"
        maskops.write_manifest(m, out)
        doc = json.loads(out.read_text(encoding="utf-8"))
        assert doc["records"] == m.to_dicts()
        assert doc["source"] is None
