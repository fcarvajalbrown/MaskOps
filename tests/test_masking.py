"""
Tests for maskops PII masking expressions.

Run after `maturin develop`:
    pytest tests/ -v
"""

import re
import csv
import tempfile
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

# ---------------------------------------------------------------------------
# Credit card (unit tests)
# ---------------------------------------------------------------------------

class TestMaskCard:
    def test_visa_masked(self):
        df = pl.DataFrame({"col": ["4111111111111111"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.startswith("411111")
        assert result.endswith("1111")
        assert result == "411111******1111"

    def test_mastercard_masked(self):
        df = pl.DataFrame({"col": ["5500005555555559"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.startswith("550000")
        assert result.endswith("5559")

    def test_amex_masked(self):
        df = pl.DataFrame({"col": ["371449635398431"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.startswith("371449")
        assert result.endswith("8431")
        assert result == "371449*****8431"

    def test_discover_masked(self):
        df = pl.DataFrame({"col": ["6011111111111117"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.startswith("601111")
        assert result.endswith("1117")

    def test_card_in_sentence(self):
        df = pl.DataFrame({"col": ["Charged to card 4111111111111111 approved"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "4111111111111111" not in result
        assert "Charged to card" in result
        assert "approved" in result

    def test_contains_pii_detects_card(self):
        df = pl.DataFrame({"col": ["4111111111111111", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]

    def test_invalid_card_untouched(self):
        original = "4111111111111112"  # valid format, fails Luhn
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

# ---------------------------------------------------------------------------
# Credit card fixture-based tests
# ---------------------------------------------------------------------------

CARD_FIXTURE = Path(__file__).parent / "fixtures" / "card_pii_sample.csv"

@pytest.mark.skipif(not CARD_FIXTURE.exists(), reason="Run generate_fixtures.py first")
class TestCardFixtures:
    def _load(self):
        with open(CARD_FIXTURE, encoding="utf-8") as f:
            return list(csv.DictReader(f))

    def test_bin_preserved_after_masking(self):
        rows = self._load()
        for row in rows:
            df = pl.DataFrame({"col": [row["card_clean"]]})
            result = df.with_columns(maskops.mask_pii("col"))["col"][0]
            assert result.startswith(row["card_clean"][:6]), \
                f"BIN not preserved for {row['scheme']}: {result}"

    def test_last4_preserved_after_masking(self):
        rows = self._load()
        for row in rows:
            df = pl.DataFrame({"col": [row["card_clean"]]})
            result = df.with_columns(maskops.mask_pii("col"))["col"][0]
            assert result.endswith(row["card_clean"][-4:]), \
                f"Last 4 not preserved for {row['scheme']}: {result}"

    def test_middle_is_masked(self):
        rows = self._load()
        for row in rows:
            df = pl.DataFrame({"col": [row["card_clean"]]})
            result = df.with_columns(maskops.mask_pii("col"))["col"][0]
            assert "*" in result, f"No masking applied for {row['scheme']}: {result}"
            middle = result[6:-4]
            assert all(c == "*" for c in middle), \
                f"Middle not fully masked for {row['scheme']}: {result}"

    def test_card_masked_in_notes(self):
        rows = self._load()
        card_rows = [r for r in rows if r["card_clean"] in r["notes"]]
        assert len(card_rows) > 0, "No card-in-notes rows found"
        for row in card_rows:
            df = pl.DataFrame({"col": [row["notes"]]})
            result = df.with_columns(maskops.mask_pii("col"))["col"][0]
            middle = row["card_clean"][6:-4]
            assert middle not in result
            assert "*" in result

    def test_contains_pii_detects_all_schemes(self):
        for scheme in ["visa", "mastercard", "amex", "discover", "maestro"]:
            rows = [r for r in self._load() if r["scheme"] == scheme][:5]
            df = pl.DataFrame({"col": [r["card_clean"] for r in rows]})
            result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
            assert all(result), f"contains_pii missed cards for scheme: {scheme}"

    def test_invalid_card_untouched(self):
        # All same digit — fails Luhn
        original = "1111111111111111"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

# ---------------------------------------------------------------------------
# European ID (unit tests)
# ---------------------------------------------------------------------------

class TestMaskEuropeanID:
    def test_dni_masked(self):
        df = pl.DataFrame({"col": ["12345678Z"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.endswith("Z")
        assert "12345678" not in result
        assert result == "********Z"

    def test_nie_masked(self):
        df = pl.DataFrame({"col": ["X1234567L"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.endswith("L")
        assert "X1234567" not in result
        assert "*" in result

    def test_nin_masked(self):
        df = pl.DataFrame({"col": ["AB 12 34 56 C"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.endswith("C")
        assert "12 34 56" not in result
        assert "*" in result

    def test_personalausweis_masked(self):
        df = pl.DataFrame({"col": ["T220001293"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == "**********"

    def test_dni_in_sentence(self):
        df = pl.DataFrame({"col": ["DNI del cliente: 12345678Z registrado"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "12345678" not in result
        assert "DNI del cliente:" in result
        assert "registrado" in result

    def test_invalid_dni_untouched(self):
        original = "12345678A"  # wrong check letter
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_contains_pii_detects_dni(self):
        df = pl.DataFrame({"col": ["12345678Z", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]

    def test_contains_pii_detects_nin(self):
        df = pl.DataFrame({"col": ["AB 12 34 56 C", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]

    def test_contains_pii_detects_personalausweis(self):
        df = pl.DataFrame({"col": ["T220001293", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]


# ---------------------------------------------------------------------------
# European ID fixture-based tests
# ---------------------------------------------------------------------------

EU_ID_FIXTURE = Path(__file__).parent / "fixtures" / "european_id_sample.csv"

@pytest.mark.skipif(not EU_ID_FIXTURE.exists(), reason="Run generate_fixtures.py first")
class TestEuropeanIDFixtures:
    def _load(self):
        with open(EU_ID_FIXTURE, encoding="utf-8") as f:
            return list(csv.DictReader(f))

    def test_id_masked_in_notes(self):
        rows = self._load()
        id_rows = [r for r in rows if r["id_clean"] in r["notes"]]
        assert len(id_rows) > 0, "No ID-in-notes rows found"
        for row in id_rows:
            df = pl.DataFrame({"col": [row["notes"]]})
            result = df.with_columns(maskops.mask_pii("col"))["col"][0]
            assert "*" in result, \
                f"No masking for {row['id_type']}: {row['id_clean']} → {result}"

    def test_contains_pii_on_dni(self):
        rows = [r for r in self._load() if r["id_type"] == "dni"][:20]
        df = pl.DataFrame({"col": [r["id_clean"] for r in rows]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert all(result), "contains_pii missed DNI values"

    def test_contains_pii_on_nie(self):
        rows = [r for r in self._load() if r["id_type"] == "nie"][:20]
        df = pl.DataFrame({"col": [r["id_clean"] for r in rows]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert all(result), "contains_pii missed NIE values"

    def test_contains_pii_on_nin(self):
        rows = [r for r in self._load() if r["id_type"] == "nin"][:20]
        df = pl.DataFrame({"col": [r["id_clean"] for r in rows]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert all(result), "contains_pii missed NIN values"

    def test_dni_check_letter_preserved(self):
        rows = [r for r in self._load() if r["id_type"] == "dni"][:20]
        for row in rows:
            df = pl.DataFrame({"col": [row["id_clean"]]})
            result = df.with_columns(maskops.mask_pii("col"))["col"][0]
            assert result.endswith(row["id_clean"][-1]), \
                f"Check letter not preserved for DNI: {result}"

    def test_personalausweis_fully_masked(self):
        rows = [r for r in self._load() if r["id_type"] == "personalausweis"][:20]
        for row in rows:
            df = pl.DataFrame({"col": [row["id_clean"]]})
            result = df.with_columns(maskops.mask_pii("col"))["col"][0]
            assert result == "*" * len(row["id_clean"]), \
                f"Personalausweis not fully masked: {result}"

                # ---------------------------------------------------------------------------
# FPE unit tests
# ---------------------------------------------------------------------------

KEY  = b"\x00" * 32  # 32-byte AES-256 key (test only — never use zero key in production)
TWEAK = b"\x00" * 7  # 7-byte tweak

class TestMaskPiiFpe:
    def test_card_fpe_preserves_length(self):
        df = pl.DataFrame({"col": ["4111111111111111"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert len(result) == 16

    def test_card_fpe_output_is_digits(self):
        df = pl.DataFrame({"col": ["4111111111111111"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert result.isdigit()

    def test_card_fpe_differs_from_plaintext(self):
        df = pl.DataFrame({"col": ["4111111111111111"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert result != "4111111111111111"

    def test_card_fpe_differs_from_asterisk(self):
        df = pl.DataFrame({"col": ["4111111111111111"]})
        fpe    = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        ast    = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert fpe != ast
        assert "*" not in fpe

    def test_phone_fpe_preserves_prefix(self):
        df = pl.DataFrame({"col": ["+56912345678"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert result.startswith("+56")

    def test_phone_fpe_no_asterisks(self):
        df = pl.DataFrame({"col": ["+56912345678"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert "*" not in result

    def test_different_tweaks_differ(self):
        tweak2 = b"\x01" * 7
        df = pl.DataFrame({"col": ["4111111111111111"]})
        r1 = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        r2 = df.with_columns(maskops.mask_pii_fpe("col", KEY, tweak2))["col"][0]
        assert r1 != r2

    def test_non_digit_pii_still_asterisked(self):
        df = pl.DataFrame({"col": ["email: john@example.com card: 4111111111111111"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert "*" in result           # email asterisked
        assert "john" not in result
        assert "4111111111111111" not in result

    def test_wrong_key_length_raises(self):
        df = pl.DataFrame({"col": ["4111111111111111"]})
        with pytest.raises(Exception):
            df.with_columns(maskops.mask_pii_fpe("col", b"\x00" * 16, TWEAK))

    def test_wrong_tweak_length_raises(self):
        df = pl.DataFrame({"col": ["4111111111111111"]})
        with pytest.raises(Exception):
            df.with_columns(maskops.mask_pii_fpe("col", KEY, b"\x00" * 4))

    def test_rut_fpe_no_asterisks(self):
        df = pl.DataFrame({"col": ["12.345.678-9"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert "*" not in result

    def test_cpf_fpe_no_asterisks(self):
        df = pl.DataFrame({"col": ["529.982.247-25"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert "*" not in result

# ---------------------------------------------------------------------------
# FPE fixture-based tests
# ---------------------------------------------------------------------------

class TestFpeFixtures:
    def _load(self, fixture):
        with open(fixture, encoding="utf-8") as f:
            return list(csv.DictReader(f))

    def test_cards_fpe_all_digits(self):
        rows = self._load(CARD_FIXTURE)[:50]
        for row in rows:
            df = pl.DataFrame({"col": [row["card_clean"]]})
            result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
            assert result.isdigit(), f"Non-digit in FPE output: {result}"

    def test_cards_fpe_preserves_length(self):
        rows = self._load(CARD_FIXTURE)[:50]
        for row in rows:
            df = pl.DataFrame({"col": [row["card_clean"]]})
            result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
            assert len(result) == len(row["card_clean"]), \
                f"Length changed for {row['scheme']}: {result}"

    def test_cards_fpe_differs_from_plaintext(self):
        rows = self._load(CARD_FIXTURE)[:50]
        for row in rows:
            df = pl.DataFrame({"col": [row["card_clean"]]})
            result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
            assert result != row["card_clean"], \
                f"FPE did not change value for {row['scheme']}: {result}"

    def test_latam_fpe_no_asterisks(self):
        rows = self._load(LATAM_FIXTURE)[:50]
        for row in rows:
            for field in ["rut_clean", "cpf_clean"]:
                df = pl.DataFrame({"col": [row[field]]})
                result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
                assert "*" not in result, \
                    f"Asterisk found in FPE output for {field}: {result}"


# ---------------------------------------------------------------------------
# SSN
# ---------------------------------------------------------------------------

class TestMaskSSN:
    def test_valid_ssn_masked(self):
        df = pl.DataFrame({"col": ["123-45-6789"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == "***-**-****"

    def test_ssn_in_sentence(self):
        df = pl.DataFrame({"col": ["SSN: 123-45-6789 on file"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "123-45-6789" not in result
        assert "SSN:" in result
        assert "on file" in result

    def test_invalid_area_000_untouched(self):
        original = "000-45-6789"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_invalid_area_666_untouched(self):
        original = "666-45-6789"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_itin_area_untouched(self):
        original = "900-45-6789"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_invalid_group_00_untouched(self):
        original = "123-00-6789"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_invalid_serial_0000_untouched(self):
        original = "123-45-0000"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_woolworth_wallet_untouched(self):
        original = "078-05-1120"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_known_invalid_219_untouched(self):
        original = "219-09-9999"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_contains_pii_detects_ssn(self):
        df = pl.DataFrame({"col": ["123-45-6789", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]

    def test_ssn_fpe_preserves_format(self):
        df = pl.DataFrame({"col": ["123-45-6789"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        parts = result.split("-")
        assert len(parts) == 3
        assert len(parts[0]) == 3 and parts[0].isdigit()
        assert len(parts[1]) == 2 and parts[1].isdigit()
        assert len(parts[2]) == 4 and parts[2].isdigit()

    def test_ssn_fpe_differs_from_plaintext(self):
        df = pl.DataFrame({"col": ["123-45-6789"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert result != "123-45-6789"

    def test_ssn_fpe_no_asterisks(self):
        df = pl.DataFrame({"col": ["123-45-6789"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert "*" not in result


# ---------------------------------------------------------------------------
# US passport
# ---------------------------------------------------------------------------

class TestMaskUSPassport:
    def test_valid_passport_masked(self):
        df = pl.DataFrame({"col": ["A12345678"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == "*********"

    def test_passport_in_sentence(self):
        df = pl.DataFrame({"col": ["Passport: A12345678 issued 2020"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "A12345678" not in result
        assert "Passport:" in result
        assert "issued 2020" in result

    def test_contains_pii_detects_passport(self):
        df = pl.DataFrame({"col": ["A12345678", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]

    def test_passport_asterisked_in_fpe_mode(self):
        df = pl.DataFrame({"col": ["A12345678"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert result == "*********"


# ---------------------------------------------------------------------------
# Lazy scan pipeline (streaming)
# ---------------------------------------------------------------------------

class TestLazyScanPipeline:
    """Verifies mask_pii and contains_pii work through scan_parquet → sink_parquet."""

    PII_ROWS = [
        "SSN: 123-45-6789",
        "Passport: A12345678",
        "Email: john@example.com",
        "IBAN: DE89370400440532013000",
        "Card: 4111111111111111",
        "RUT: 76.354.771-K",
        "Clean row with no PII",
    ]

    def _write_parquet(self, tmp_path: Path) -> Path:
        p = tmp_path / "input.parquet"
        pl.DataFrame({"text": self.PII_ROWS}).write_parquet(p)
        return p

    def test_mask_pii_lazy_collect(self, tmp_path):
        src = self._write_parquet(tmp_path)
        result = (
            pl.scan_parquet(src)
            .with_columns(maskops.mask_pii("text"))
            .collect()
        )
        assert len(result) == len(self.PII_ROWS)
        assert "123-45-6789" not in result["text"].to_list()
        assert "john@example.com" not in result["text"].to_list()
        assert "Clean row with no PII" in result["text"].to_list()

    def test_mask_pii_sink_parquet(self, tmp_path):
        src = self._write_parquet(tmp_path)
        out = tmp_path / "output.parquet"
        (
            pl.scan_parquet(src)
            .with_columns(maskops.mask_pii("text"))
            .sink_parquet(out)
        )
        result = pl.read_parquet(out)
        assert len(result) == len(self.PII_ROWS)
        assert "123-45-6789" not in result["text"].to_list()
        assert "A12345678" not in result["text"].to_list()

    def test_contains_pii_lazy_collect(self, tmp_path):
        src = self._write_parquet(tmp_path)
        result = (
            pl.scan_parquet(src)
            .with_columns(maskops.contains_pii("text"))
            .collect()
        )
        flags = result["text"].to_list()
        assert flags[-1] is False  # clean row
        assert any(flags[:-1])     # at least one PII row detected

    def test_mask_pii_fpe_lazy_collect(self, tmp_path):
        src = self._write_parquet(tmp_path)
        result = (
            pl.scan_parquet(src)
            .with_columns(maskops.mask_pii_fpe("text", KEY, TWEAK))
            .collect()
        )
        assert len(result) == len(self.PII_ROWS)
        assert "4111111111111111" not in result["text"].to_list()
        assert "Clean row with no PII" in result["text"].to_list()

    def test_filter_then_mask_lazy(self, tmp_path):
        src = self._write_parquet(tmp_path)
        result = (
            pl.scan_parquet(src)
            .filter(maskops.contains_pii("text"))
            .with_columns(maskops.mask_pii("text"))
            .collect()
        )
        assert len(result) == len(self.PII_ROWS) - 1  # clean row filtered out
        for row in result["text"].to_list():
            assert "123-45-6789" not in row
            assert "john@example.com" not in row


# ---------------------------------------------------------------------------
# Argentine DNI
# ---------------------------------------------------------------------------

class TestMaskArgDNI:
    def test_dotted_8digit_masked(self):
        df = pl.DataFrame({"col": ["DNI: 12.345.678"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "12.345.678" not in result
        assert "*" in result
        assert "." in result

    def test_dotted_7digit_masked(self):
        df = pl.DataFrame({"col": ["DNI: 1.234.567"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "1.234.567" not in result
        assert "*" in result

    def test_contains_pii_detects_arg_dni(self):
        df = pl.DataFrame({"col": ["12.345.678", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]

    def test_arg_dni_fpe_no_asterisks(self):
        df = pl.DataFrame({"col": ["12.345.678"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert "*" not in result
        assert result != "12.345.678"


# ---------------------------------------------------------------------------
# Colombian CC
# ---------------------------------------------------------------------------

class TestMaskCOCC:
    def test_10digit_cc_masked(self):
        df = pl.DataFrame({"col": ["Cédula: 1.234.567.890"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "1.234.567.890" not in result
        assert "*" in result

    def test_contains_pii_detects_co_cc(self):
        df = pl.DataFrame({"col": ["1.234.567.890", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]

    def test_co_cc_fpe_no_asterisks(self):
        df = pl.DataFrame({"col": ["1.234.567.890"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert "*" not in result
        assert result != "1.234.567.890"


# ---------------------------------------------------------------------------
# Colombian NIT
# ---------------------------------------------------------------------------

class TestMaskCONIT:
    def test_valid_nit_masked(self):
        # NIT 900123456 with DIAN check digit
        # sum = 6*3+5*7+4*13+3*17+2*19+1*23+0*29+0*37+9*41 = 18+35+52+51+38+23+0+0+369 = 586
        # 586 % 11 = 3, check = 11-3 = 8
        df = pl.DataFrame({"col": ["NIT: 900123456-8"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "900123456" not in result
        assert result.endswith("-8")
        assert "*" in result

    def test_invalid_nit_check_untouched(self):
        original = "900123456-0"  # wrong check digit
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_contains_pii_detects_co_nit(self):
        df = pl.DataFrame({"col": ["900123456-8", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]

    def test_nit_fpe_preserves_check_digit(self):
        df = pl.DataFrame({"col": ["900123456-8"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert result.endswith("-8")
        assert "*" not in result
        assert result != "900123456-8"


# ---------------------------------------------------------------------------
# Pattern selection
# ---------------------------------------------------------------------------

class TestPatternSelection:
    """mask_pii / contains_pii / mask_pii_fpe with patterns= argument."""

    def test_mask_only_email(self):
        df = pl.DataFrame({"col": ["email: john@example.com and SSN: 123-45-6789"]})
        result = df.with_columns(maskops.mask_pii("col", patterns=["email"]))["col"][0]
        assert "john@example.com" not in result
        assert "123-45-6789" in result  # SSN untouched

    def test_mask_only_ssn(self):
        df = pl.DataFrame({"col": ["email: john@example.com and SSN: 123-45-6789"]})
        result = df.with_columns(maskops.mask_pii("col", patterns=["ssn"]))["col"][0]
        assert "john@example.com" in result  # email untouched
        assert "123-45-6789" not in result

    def test_mask_multiple_patterns(self):
        df = pl.DataFrame({"col": ["john@example.com card 4111111111111111"]})
        result = df.with_columns(maskops.mask_pii("col", patterns=["email", "credit_card"]))["col"][0]
        assert "john@example.com" not in result
        assert "4111111111111111" not in result

    def test_mask_no_patterns_masks_all(self):
        df = pl.DataFrame({"col": ["john@example.com and 123-45-6789"]})
        full = df.with_columns(maskops.mask_pii("col"))["col"][0]
        selected_all = df.with_columns(maskops.mask_pii("col", patterns=["email", "ssn"]))["col"][0]
        assert "john@example.com" not in full
        assert "123-45-6789" not in full
        assert "john@example.com" not in selected_all
        assert "123-45-6789" not in selected_all

    def test_unknown_pattern_ignored(self):
        original = "john@example.com"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col", patterns=["nonexistent_pattern"]))["col"][0]
        assert result == original  # nothing masked

    def test_contains_pii_pattern_filter(self):
        df = pl.DataFrame({"col": ["john@example.com", "123-45-6789", "nothing"]})
        email_only = df.with_columns(maskops.contains_pii("col", patterns=["email"]))["col"].to_list()
        ssn_only = df.with_columns(maskops.contains_pii("col", patterns=["ssn"]))["col"].to_list()
        assert email_only == [True, False, False]
        assert ssn_only == [False, True, False]

    def test_mask_pii_fpe_pattern_filter(self):
        df = pl.DataFrame({"col": ["phone +56912345678 card 4111111111111111"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK, patterns=["credit_card"]))["col"][0]
        assert "4111111111111111" not in result
        assert "+56912345678" in result  # phone untouched

    def test_pattern_npi(self):
        df = pl.DataFrame({"col": ["NPI: 1234567893 email: x@x.com"]})
        result = df.with_columns(maskops.mask_pii("col", patterns=["npi"]))["col"][0]
        assert "1234567893" not in result
        assert "x@x.com" in result

    def test_backward_compatible_no_patterns(self):
        df = pl.DataFrame({"col": ["john@example.com"]})
        with_none = df.with_columns(maskops.mask_pii("col"))["col"][0]
        with_explicit = df.with_columns(maskops.mask_pii("col", patterns=None))["col"][0]
        assert with_none == with_explicit
        assert "john@example.com" not in with_none


# ---------------------------------------------------------------------------
# Healthcare: NPI
# ---------------------------------------------------------------------------

class TestMaskNPI:
    # 1234567893 — valid NPI (Luhn check with HIPAA prefix 24)
    def test_valid_npi_masked(self):
        df = pl.DataFrame({"col": ["NPI: 1234567893"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "1234567893" not in result
        assert "**********" in result

    def test_invalid_npi_untouched(self):
        original = "1234567890"  # wrong check digit (would need 3 to be valid)
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_contains_pii_detects_npi(self):
        df = pl.DataFrame({"col": ["1234567893", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]

    def test_npi_fpe_no_asterisks(self):
        df = pl.DataFrame({"col": ["1234567893"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert "*" not in result
        assert result != "1234567893"


# ---------------------------------------------------------------------------
# Healthcare: MBI
# ---------------------------------------------------------------------------

class TestMaskMBI:
    # 1EG4TE5MK72 — well-known CMS example MBI
    def test_valid_mbi_masked(self):
        df = pl.DataFrame({"col": ["MBI: 1EG4TE5MK72"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "1EG4TE5MK72" not in result
        assert "***********" in result

    def test_contains_pii_detects_mbi(self):
        df = pl.DataFrame({"col": ["1EG4TE5MK72", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]

    def test_mbi_asterisked_in_fpe_mode(self):
        df = pl.DataFrame({"col": ["1EG4TE5MK72"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert result == "***********"


# ---------------------------------------------------------------------------
# Healthcare: NHS
# ---------------------------------------------------------------------------

class TestMaskNHS:
    # 9434765919 — valid NHS number
    def test_valid_nhs_masked(self):
        df = pl.DataFrame({"col": ["NHS: 943 476 5919"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "943" not in result
        assert "476" not in result
        assert "*" in result

    def test_compact_nhs_masked(self):
        df = pl.DataFrame({"col": ["9434765919"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "9434765919" not in result

    def test_invalid_nhs_untouched(self):
        # Use spaced format so NPI regex (requires 10 consecutive digits) won't fire.
        # 999 999 9990: check digit should be 9, not 0 → invalid NHS.
        original = "999 999 9990"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_contains_pii_detects_nhs(self):
        df = pl.DataFrame({"col": ["9434765919", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]

    def test_nhs_fpe_no_asterisks(self):
        df = pl.DataFrame({"col": ["9434765919"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert "*" not in result
        assert result != "9434765919"


# ---------------------------------------------------------------------------
# LatAm: Peruvian DNI
# ---------------------------------------------------------------------------

class TestMaskPeDNI:
    def test_pe_dni_masked(self):
        df = pl.DataFrame({"col": ["DNI: 12345678"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "12345678" not in result
        assert "********" in result

    def test_contains_pii_detects_pe_dni(self):
        df = pl.DataFrame({"col": ["12345678", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]

    def test_pe_dni_fpe_no_asterisks(self):
        df = pl.DataFrame({"col": ["12345678"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert "*" not in result
        assert result != "12345678"


# ---------------------------------------------------------------------------
# Consistent masking (v0.8)
# ---------------------------------------------------------------------------

class TestMaskPiiConsistent:
    """Deterministic hash-based pseudonymization via mode='consistent'."""

    SALT = "test-salt-v08"

    def _mask(self, value, **kwargs):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(
            maskops.mask_pii("col", mode="consistent", salt=self.SALT, **kwargs)
        )["col"][0]

    # --- Determinism ---

    def test_same_input_same_output(self):
        out1 = self._mask("123-45-6789")
        out2 = self._mask("123-45-6789")
        assert out1 == out2

    def test_different_input_different_output(self):
        out1 = self._mask("123-45-6789")
        out2 = self._mask("987-65-4321")
        assert out1 != out2

    def test_different_salt_different_output(self):
        df = pl.DataFrame({"col": ["123-45-6789"]})
        out1 = df.with_columns(maskops.mask_pii("col", mode="consistent", salt="salt-a"))["col"][0]
        out2 = df.with_columns(maskops.mask_pii("col", mode="consistent", salt="salt-b"))["col"][0]
        assert out1 != out2

    # --- Output is not the original ---

    def test_ssn_not_original(self):
        out = self._mask("123-45-6789")
        assert "123456789" not in out.replace("-", "")
        assert out != "123-45-6789"

    def test_credit_card_not_original(self):
        out = self._mask("4111111111111111")
        assert out != "4111111111111111"
        assert len(out) == 16
        assert out.isdigit()

    # --- Non-digit PII still asterisked ---

    def test_email_asterisked(self):
        out = self._mask("send to user@example.com please")
        assert "user@example.com" not in out
        assert "*" in out

    def test_iban_asterisked(self):
        out = self._mask("DE89370400440532013000")
        assert "370400440532013000" not in out
        assert "*" in out

    # --- Format of digit PII output ---

    def test_ssn_format_preserved(self):
        out = self._mask("123-45-6789")
        assert re.fullmatch(r"\d{3}-\d{2}-\d{4}", out), f"SSN format broken: {out}"

    def test_credit_card_16_digits(self):
        out = self._mask("4111111111111111")
        assert len(out) == 16
        assert out.isdigit()

    # --- patterns= filter respected ---

    def test_patterns_filter_only_masks_selected(self):
        text = "SSN 123-45-6789 card 4111111111111111"
        df = pl.DataFrame({"col": [text]})
        out = df.with_columns(
            maskops.mask_pii("col", mode="consistent", salt=self.SALT, patterns=["ssn"])
        )["col"][0]
        assert "123-45-6789" not in out
        assert "4111111111111111" in out

    # --- Null passthrough ---

    def test_null_passthrough(self):
        df = pl.DataFrame({"col": [None]}, schema={"col": pl.String})
        result = df.with_columns(
            maskops.mask_pii("col", mode="consistent", salt=self.SALT)
        )["col"][0]
        assert result is None