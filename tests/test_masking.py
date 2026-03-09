"""
Tests for maskops PII masking expressions.

Run after `maturin develop`:
    pytest tests/ -v
"""

import re
import csv
from pathlib import Path

import polars as pl
import pytest
import maskops


# ---------------------------------------------------------------------------
# IBAN
# ---------------------------------------------------------------------------

class TestMaskIBAN:
    def test_german_iban_masked(self):
        df = pl.DataFrame({"col": ["DE89370400440532013000"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.startswith("DE89")
        assert "*" in result
        assert "370400440532013000" not in result

    def test_iban_embedded_in_text(self):
        df = pl.DataFrame({"col": ["Payment ref DE89370400440532013000 confirmed"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "DE89" in result
        assert "370400440532013000" not in result
        assert "Payment ref" in result
        assert "confirmed" in result

    def test_no_iban_untouched(self):
        original = "No sensitive data here"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_null_passthrough(self):
        df = pl.DataFrame({"col": [None]}, schema={"col": pl.String})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result is None


# ---------------------------------------------------------------------------
# VAT
# ---------------------------------------------------------------------------

class TestMaskVAT:
    def test_german_vat_masked(self):
        df = pl.DataFrame({"col": ["DE123456789"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.startswith("DE")
        assert "123456789" not in result

    def test_french_vat_masked(self):
        df = pl.DataFrame({"col": ["FR12345678901"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.startswith("FR")
        assert "12345678901" not in result


# ---------------------------------------------------------------------------
# contains_pii
# ---------------------------------------------------------------------------

class TestContainsPII:
    def test_detects_iban(self):
        df = pl.DataFrame({"col": ["DE89370400440532013000", "clean text"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]

    def test_detects_vat(self):
        df = pl.DataFrame({"col": ["VAT: DE123456789", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]

    def test_null_is_false(self):
        df = pl.DataFrame({"col": [None]}, schema={"col": pl.String})
        result = df.with_columns(maskops.contains_pii("col"))["col"][0]
        assert result is None or result is False

    def test_detects_email(self):
        df = pl.DataFrame({"col": ["contact: user@example.com", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]

    def test_detects_phone(self):
        df = pl.DataFrame({"col": ["+14155552671", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]


# ---------------------------------------------------------------------------
# Email
# ---------------------------------------------------------------------------

class TestMaskEmail:
    def test_local_part_masked(self):
        df = pl.DataFrame({"col": ["user@example.com"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "@example.com" in result
        assert "user" not in result
        assert "*" in result

    def test_domain_preserved(self):
        df = pl.DataFrame({"col": ["john.doe@company.org"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.endswith("@company.org")

    def test_email_in_sentence(self):
        df = pl.DataFrame({"col": ["Send results to john@example.com please"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "john" not in result
        assert "@example.com" in result
        assert "Send results to" in result

    def test_multiple_emails(self):
        df = pl.DataFrame({"col": ["a@x.com and b@y.com"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "a@" not in result
        assert "b@" not in result
        assert "@x.com" in result
        assert "@y.com" in result


# ---------------------------------------------------------------------------
# Phone
# ---------------------------------------------------------------------------

class TestMaskPhone:
    def test_us_phone_masked(self):
        df = pl.DataFrame({"col": ["+14155552671"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.startswith("+1")
        assert "4155552671" not in result
        assert "*" in result

    def test_country_prefix_preserved(self):
        df = pl.DataFrame({"col": ["+447911123456"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.startswith("+44")

    def test_phone_in_sentence(self):
        df = pl.DataFrame({"col": ["Call me at +14155552671 tomorrow"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "4155552671" not in result
        assert "Call me at" in result
        assert "tomorrow" in result

    def test_non_phone_untouched(self):
        original = "ref #12345"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original


# ---------------------------------------------------------------------------
# IP Address
# ---------------------------------------------------------------------------

class TestMaskIP:
    def test_ipv4_host_masked(self):
        df = pl.DataFrame({"col": ["192.168.1.100"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.startswith("192.168")
        assert "1.100" not in result
        assert "*" in result

    def test_ipv4_in_sentence(self):
        df = pl.DataFrame({"col": ["Server at 10.0.0.1 is down"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "10.0" in result
        assert "0.1" not in result
        assert "Server at" in result
        assert "is down" in result

    def test_ipv6_masked(self):
        df = pl.DataFrame({"col": ["2001:db8:0:0:1:2:3:4"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "2001:db8" in result
        assert "1:2:3:4" not in result
        assert "*" in result

    def test_non_ip_untouched(self):
        original = "version 1.2.3"
        df = pl.DataFrame({"col": ["version 1.2.3"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_contains_pii_detects_ip(self):
        df = pl.DataFrame({"col": ["192.168.1.1", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]


# ---------------------------------------------------------------------------
# RUT (Chile)
# ---------------------------------------------------------------------------

class TestMaskRUT:
    def test_rut_body_masked(self):
        df = pl.DataFrame({"col": ["76.354.771-K"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.endswith("-K")
        assert "76.354.771" not in result
        assert "*" in result

    def test_rut_check_digit_numeric(self):
        df = pl.DataFrame({"col": ["12.531.909-2"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.endswith("-2")
        assert "12.531.909" not in result
        assert "*" in result

    def test_rut_without_dots(self):
        df = pl.DataFrame({"col": ["76354771-K"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.endswith("-K")
        assert "76354771" not in result

    def test_rut_in_sentence(self):
        df = pl.DataFrame({"col": ["Cliente RUT 76.354.771-K registrado"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "76.354.771" not in result
        assert "Cliente RUT" in result
        assert "registrado" in result

    def test_invalid_rut_untouched(self):
        original = "RUT 12.345.678-0"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_contains_pii_detects_rut(self):
        df = pl.DataFrame({"col": ["76.354.771-K", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]


# ---------------------------------------------------------------------------
# CPF (Brazil)
# ---------------------------------------------------------------------------

class TestMaskCPF:
    def test_cpf_body_masked(self):
        df = pl.DataFrame({"col": ["529.982.247-25"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.endswith("-25")
        assert "529.982.247" not in result
        assert "*" in result

    def test_cpf_without_dots(self):
        df = pl.DataFrame({"col": ["52998224725"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "52998224725" not in result
        assert "*" in result

    def test_cpf_in_sentence(self):
        df = pl.DataFrame({"col": ["CPF do cliente: 529.982.247-25 confirmado"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "529.982.247" not in result
        assert "CPF do cliente:" in result
        assert "confirmado" in result

    def test_invalid_cpf_untouched(self):
        original = "CPF 111.111.111-11"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_contains_pii_detects_cpf(self):
        df = pl.DataFrame({"col": ["529.982.247-25", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]


# ---------------------------------------------------------------------------
# CURP (Mexico)
# ---------------------------------------------------------------------------

class TestMaskCURP:
    def test_curp_fully_masked(self):
        df = pl.DataFrame({"col": ["BADD110313HCMLNS09"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == "******************"

    def test_curp_in_sentence(self):
        df = pl.DataFrame({"col": ["CURP: BADD110313HCMLNS09 registrado"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "BADD110313HCMLNS09" not in result
        assert "CURP:" in result
        assert "registrado" in result

    def test_invalid_curp_untouched(self):
        original = "ref ABC123"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_contains_pii_detects_curp(self):
        df = pl.DataFrame({"col": ["BADD110313HCMLNS09", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]


# ---------------------------------------------------------------------------
# LatAm ID fixture-based tests
# ---------------------------------------------------------------------------

LATAM_FIXTURE = Path(__file__).parent / "fixtures" / "latam_pii_sample.csv"

@pytest.mark.skipif(not LATAM_FIXTURE.exists(), reason="Run generate_fixtures.py first")
class TestLatamFixtures:
    def _load(self):
        with open(LATAM_FIXTURE, encoding="utf-8") as f:
            return list(csv.DictReader(f))

    def test_rut_masked_in_notes(self):
        rows = self._load()
        rut_rows = [r for r in rows if r["rut_clean"] in r["notes"]]
        assert len(rut_rows) > 0, "No RUT-in-notes rows found"
        for row in rut_rows:
            df = pl.DataFrame({"col": [row["notes"]]})
            result = df.with_columns(maskops.mask_pii("col"))["col"][0]
            body = row["rut_clean"].rsplit("-", 1)[0]
            assert body not in result
            assert "*" in result

    def test_cpf_masked_in_notes(self):
        rows = self._load()
        cpf_rows = [r for r in rows if r["cpf_clean"] in r["notes"]]
        assert len(cpf_rows) > 0, "No CPF-in-notes rows found"
        for row in cpf_rows:
            df = pl.DataFrame({"col": [row["notes"]]})
            result = df.with_columns(maskops.mask_pii("col"))["col"][0]
            body = row["cpf_clean"].rsplit("-", 1)[0]
            assert body not in result
            assert "*" in result

    def test_curp_masked_in_notes(self):
        rows = self._load()
        curp_rows = [r for r in rows if r["curp_clean"] in r["notes"]]
        assert len(curp_rows) > 0, "No CURP-in-notes rows found"
        for row in curp_rows:
            df = pl.DataFrame({"col": [row["notes"]]})
            result = df.with_columns(maskops.mask_pii("col"))["col"][0]
            assert row["curp_clean"] not in result
            assert "*" in result

    def test_contains_pii_on_rut_clean(self):
        rows = self._load()[:20]
        df = pl.DataFrame({"col": [r["rut_clean"] for r in rows]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert all(result)

    def test_contains_pii_on_cpf_clean(self):
        rows = self._load()[:20]
        df = pl.DataFrame({"col": [r["cpf_clean"] for r in rows]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert all(result)


# ---------------------------------------------------------------------------
# EU PII fixture-based tests
# ---------------------------------------------------------------------------

EU_FIXTURE = Path(__file__).parent / "fixtures" / "eu_pii_sample.csv"

@pytest.mark.skipif(not EU_FIXTURE.exists(), reason="Run generate_fixtures.py first")
class TestEUFixtures:
    def _load(self):
        with open(EU_FIXTURE, encoding="utf-8") as f:
            return list(csv.DictReader(f))

    def test_iban_masked_in_notes(self):
        rows = self._load()
        iban_rows = [r for r in rows if r["iban_clean"] in r["notes"]]
        assert len(iban_rows) > 0, "No IBAN-in-notes rows found"
        for row in iban_rows:
            df = pl.DataFrame({"col": [row["notes"]]})
            result = df.with_columns(maskops.mask_pii("col"))["col"][0]
            assert row["iban_clean"][4:] not in result
            assert "*" in result

    def test_iban_masked_in_free_text(self):
        rows = self._load()
        iban_rows = [r for r in rows if r["iban_clean"] in r["free_text"]]
        assert len(iban_rows) > 0, "No IBAN-in-free_text rows found"
        for row in iban_rows:
            df = pl.DataFrame({"col": [row["free_text"]]})
            result = df.with_columns(maskops.mask_pii("col"))["col"][0]
            assert row["iban_clean"][4:] not in result

    def test_email_masked_in_notes(self):
        rows = self._load()
        email_rows = [r for r in rows if r["email_clean"] in r["notes"]]
        assert len(email_rows) > 0, "No email-in-notes rows found"
        for row in email_rows:
            df = pl.DataFrame({"col": [row["notes"]]})
            result = df.with_columns(maskops.mask_pii("col"))["col"][0]
            local = row["email_clean"].split("@")[0]
            assert local not in result
            assert "*" in result

    def test_email_masked_in_free_text(self):
        rows = self._load()
        email_rows = [r for r in rows if r["email_clean"] in r["free_text"]]
        assert len(email_rows) > 0, "No email-in-free_text rows found"
        for row in email_rows:
            df = pl.DataFrame({"col": [row["free_text"]]})
            result = df.with_columns(maskops.mask_pii("col"))["col"][0]
            local = row["email_clean"].split("@")[0]
            assert local not in result

    def test_contains_pii_on_iban_clean(self):
        rows = self._load()[:20]
        df = pl.DataFrame({"col": [r["iban_clean"] for r in rows]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert all(result)

    def test_contains_pii_on_email_clean(self):
        rows = self._load()[:20]
        df = pl.DataFrame({"col": [r["email_clean"] for r in rows]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert all(result)


# ---------------------------------------------------------------------------
# Phone fixture-based tests
# ---------------------------------------------------------------------------

PHONE_FIXTURE = Path(__file__).parent / "fixtures" / "phone_sample.csv"

@pytest.mark.skipif(not PHONE_FIXTURE.exists(), reason="Run generate_fixtures.py first")
class TestPhoneFixtures:
    def _load(self):
        with open(PHONE_FIXTURE, encoding="utf-8") as f:
            return list(csv.DictReader(f))

    def test_phone_masked_in_sentence(self):
        rows = self._load()
        # Only clean E.164: + followed by digits only, length 10-15 chars total
        e164_rows = [
            r for r in rows
            if re.fullmatch(r'\+\d{7,14}', r["phone_e164"])
            and r["phone_e164"] in r["sentence"]
        ]
        assert len(e164_rows) > 0, "No clean E.164-in-sentence rows found"
        masked_count = sum(
            1 for row in e164_rows
            if "*" in pl.DataFrame({"col": [row["sentence"]]})
            .with_columns(maskops.mask_pii("col"))["col"][0]
        )
        assert masked_count / len(e164_rows) >= 0.7

    def test_contains_pii_on_e164(self):
        rows = self._load()
        e164_rows = [
            r for r in rows
            if re.fullmatch(r'\+\d{7,14}', r["phone_e164"])
        ][:30]
        detected = []
        for row in e164_rows:
            df = pl.DataFrame({"col": [row["phone_e164"]]})
            detected.append(df.with_columns(maskops.contains_pii("col"))["col"][0])
        assert sum(detected) / len(detected) >= 0.7, "Less than 70% of E.164 phones detected"

    def test_prefix_preserved_after_masking(self):
        rows = self._load()
        e164_rows = [
            r for r in rows
            if re.fullmatch(r'\+\d{7,14}', r["phone_e164"])
        ][:30]
        for row in e164_rows:
            df = pl.DataFrame({"col": [row["phone_e164"]]})
            result = df.with_columns(maskops.mask_pii("col"))["col"][0]
            if "*" in result:
                assert result.startswith(row["prefix"])