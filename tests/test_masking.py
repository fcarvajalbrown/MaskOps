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

class TestMaskCNPJ:
    def test_cnpj_body_masked(self):
        df = pl.DataFrame({"col": ["11.222.333/0001-81"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result.endswith("-81")
        assert "11.222.333" not in result
        assert "*" in result

    def test_cnpj_without_separators(self):
        df = pl.DataFrame({"col": ["11222333000181"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "11222333000181" not in result
        assert "*" in result

    def test_cnpj_in_sentence(self):
        df = pl.DataFrame({"col": ["CNPJ da empresa: 11.222.333/0001-81 ativo"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "11.222.333/0001-81" not in result
        assert "CNPJ da empresa:" in result
        assert "ativo" in result

    def test_invalid_cnpj_not_extracted(self):
        df = pl.DataFrame({"col": ["11.222.333/0001-99"]})
        result = df.with_columns(maskops.extract_pii("col")).unnest("col")["cnpj"][0]
        assert result is None

    def test_contains_pii_detects_cnpj(self):
        df = pl.DataFrame({"col": ["11.222.333/0001-81", "nothing"]})
        result = df.with_columns(maskops.contains_pii("col"))["col"].to_list()
        assert result == [True, False]

    def test_cnpj_extracted(self):
        df = pl.DataFrame({"col": ["empresa 11.222.333/0001-81"]})
        result = df.with_columns(maskops.extract_pii("col")).unnest("col")["cnpj"][0]
        assert result == "11.222.333/0001-81"

    def test_cnpj_audit_count(self):
        df = pl.DataFrame({"col": ["11.222.333/0001-81 e 11222333000181"]})
        audit = df.with_columns(maskops.mask_pii_audit("col")).unnest("col").unnest("counts")
        assert audit["cnpj"][0] == 2

    def test_cnpj_consistent_deterministic(self):
        df = pl.DataFrame({"col": ["11.222.333/0001-81", "11222333000181"]})
        out = df.with_columns(maskops.mask_pii("col", mode="consistent", salt="s"))["col"].to_list()
        assert out[0] == out[1]

    def test_invalid_cnpj_prefix_not_masked_as_co_cc(self):
        original = "empresa 11.222.333/0001-99 fim"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

    def test_non_cnpj_slash_still_masks_arg_dni(self):
        df = pl.DataFrame({"col": ["expediente 11.222.333/2020"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "11.222.333" not in result
        assert "*" in result

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

PHONE_FIXTURE = Path(__file__).parent / "fixtures" / "phone_sample.csv"

@pytest.mark.skipif(not PHONE_FIXTURE.exists(), reason="Run generate_fixtures.py first")
class TestPhoneFixtures:
    def _load(self):
        with open(PHONE_FIXTURE, encoding="utf-8") as f:
            return list(csv.DictReader(f))

    def test_phone_masked_in_sentence(self):
        rows = self._load()
        
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
        original = "4111111111111112"  
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

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
        
        original = "1111111111111111"
        df = pl.DataFrame({"col": [original]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result == original

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
        original = "12345678A"  
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

                

KEY  = bytes(range(32))
TWEAK = bytes([1, 2, 3, 4, 5, 6, 7])

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

    def test_cnpj_fpe_preserves_length(self):
        df = pl.DataFrame({"col": ["11222333000181"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert len(result) == 14
        assert result.isdigit()
        assert result != "11222333000181"

    def test_different_tweaks_differ(self):
        tweak2 = b"\x01" * 7
        df = pl.DataFrame({"col": ["4111111111111111"]})
        r1 = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        r2 = df.with_columns(maskops.mask_pii_fpe("col", KEY, tweak2))["col"][0]
        assert r1 != r2

    def test_non_digit_pii_still_asterisked(self):
        df = pl.DataFrame({"col": ["email: john@example.com card: 4111111111111111"]})
        result = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert "*" in result           
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
        assert flags[-1] is False  
        assert any(flags[:-1])     

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
        assert len(result) == len(self.PII_ROWS) - 1  
        for row in result["text"].to_list():
            assert "123-45-6789" not in row
            assert "john@example.com" not in row

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

class TestMaskCONIT:
    def test_valid_nit_masked(self):
        
        
        
        df = pl.DataFrame({"col": ["NIT: 900123456-8"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "900123456" not in result
        assert result.endswith("-8")
        assert "*" in result

    def test_invalid_nit_check_untouched(self):
        original = "900123456-0"  
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

class TestPatternSelection:
    """mask_pii / contains_pii / mask_pii_fpe with patterns= argument."""

    def test_mask_only_email(self):
        df = pl.DataFrame({"col": ["email: john@example.com and SSN: 123-45-6789"]})
        result = df.with_columns(maskops.mask_pii("col", patterns=["email"]))["col"][0]
        assert "john@example.com" not in result
        assert "123-45-6789" in result  

    def test_mask_only_ssn(self):
        df = pl.DataFrame({"col": ["email: john@example.com and SSN: 123-45-6789"]})
        result = df.with_columns(maskops.mask_pii("col", patterns=["ssn"]))["col"][0]
        assert "john@example.com" in result  
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
        assert result == original  

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
        assert "+56912345678" in result  

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

class TestMaskNPI:
    
    def test_valid_npi_masked(self):
        df = pl.DataFrame({"col": ["NPI: 1234567893"]})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "1234567893" not in result
        assert "**********" in result

    def test_invalid_npi_untouched(self):
        original = "1234567890"  
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

class TestMaskMBI:
    
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

class TestMaskNHS:
    
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

class TestMaskPiiConsistent:
    """Deterministic hash-based pseudonymization via mode='consistent'."""

    SALT = "test-salt-v08"

    def _mask(self, value, **kwargs):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(
            maskops.mask_pii("col", mode="consistent", salt=self.SALT, **kwargs)
        )["col"][0]

    

    def test_same_input_same_output(self):
        out1 = self._mask("123-45-6789")
        out2 = self._mask("123-45-6789")
        assert out1 == out2

    def test_cross_column_same_value_same_output(self):
        df = pl.DataFrame({"a": ["123-45-6789"], "b": ["123-45-6789"]})
        result = df.with_columns(
            maskops.mask_pii("a", mode="consistent", salt=self.SALT),
            maskops.mask_pii("b", mode="consistent", salt=self.SALT),
        )
        assert result["a"][0] == result["b"][0]

    def test_cross_column_different_values_different_output(self):
        df = pl.DataFrame({"a": ["123-45-6789"], "b": ["987-65-4321"]})
        result = df.with_columns(
            maskops.mask_pii("a", mode="consistent", salt=self.SALT),
            maskops.mask_pii("b", mode="consistent", salt=self.SALT),
        )
        assert result["a"][0] != result["b"][0]

    def test_different_input_different_output(self):
        out1 = self._mask("123-45-6789")
        out2 = self._mask("987-65-4321")
        assert out1 != out2

    def test_different_salt_different_output(self):
        df = pl.DataFrame({"col": ["123-45-6789"]})
        out1 = df.with_columns(maskops.mask_pii("col", mode="consistent", salt="salt-a"))["col"][0]
        out2 = df.with_columns(maskops.mask_pii("col", mode="consistent", salt="salt-b"))["col"][0]
        assert out1 != out2

    

    def test_ssn_not_original(self):
        out = self._mask("123-45-6789")
        assert "123456789" not in out.replace("-", "")
        assert out != "123-45-6789"

    def test_credit_card_not_original(self):
        out = self._mask("4111111111111111")
        assert out != "4111111111111111"
        assert len(out) == 16
        assert out.isdigit()

    

    def test_email_asterisked(self):
        out = self._mask("send to user@example.com please")
        assert "user@example.com" not in out
        assert "*" in out

    def test_iban_asterisked(self):
        out = self._mask("DE89370400440532013000")
        assert "370400440532013000" not in out
        assert "*" in out

    

    def test_ssn_format_preserved(self):
        out = self._mask("123-45-6789")
        assert re.fullmatch(r"\d{3}-\d{2}-\d{4}", out), f"SSN format broken: {out}"

    def test_credit_card_16_digits(self):
        out = self._mask("4111111111111111")
        assert len(out) == 16
        assert out.isdigit()

    

    def test_patterns_filter_only_masks_selected(self):
        text = "SSN 123-45-6789 card 4111111111111111"
        df = pl.DataFrame({"col": [text]})
        out = df.with_columns(
            maskops.mask_pii("col", mode="consistent", salt=self.SALT, patterns=["ssn"])
        )["col"][0]
        assert "123-45-6789" not in out
        assert "4111111111111111" in out

    

    def test_null_passthrough(self):
        df = pl.DataFrame({"col": [None]}, schema={"col": pl.String})
        result = df.with_columns(
            maskops.mask_pii("col", mode="consistent", salt=self.SALT)
        )["col"][0]
        assert result is None

class TestMaskNIR:
    """French NIR / INSEE social security number."""

    VALID = "185037505600181"    
    INVALID = "185037505600100"  

    def _mask(self, value):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.mask_pii("col"))["col"][0]

    def _contains(self, value):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.contains_pii("col"))["col"][0]

    def test_detects_valid_nir(self):
        assert self._contains(self.VALID)

    def test_does_not_detect_invalid_key(self):
        assert not self._contains(self.INVALID)

    def test_masks_valid_nir(self):
        out = self._mask(self.VALID)
        assert self.VALID not in out
        assert "*" in out

    def test_full_redaction_length(self):
        out = self._mask(self.VALID)
        assert len(out) == 15
        assert all(c == "*" for c in out)

    def test_invalid_key_not_masked(self):
        out = self._mask(self.INVALID)
        assert out == self.INVALID

    def test_in_sentence(self):
        text = f"NIR du patient : {self.VALID} date de naissance 1985-03-01"
        out = self._mask(text)
        assert self.VALID not in out
        assert "*" * 15 in out

    def test_patterns_filter(self):
        df = pl.DataFrame({"col": [self.VALID]})
        out = df.with_columns(maskops.mask_pii("col", patterns=["nir"]))["col"][0]
        assert self.VALID not in out
        assert "*" in out

    def test_null_passthrough(self):
        df = pl.DataFrame({"col": [None]}, schema={"col": pl.String})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result is None

class TestMaskCodiceFiscale:
    """Italian Codice Fiscale."""

    VALID = "RSSMRA80A01H501U"    
    INVALID = "RSSMRA80A01H501X"  

    def _mask(self, value):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.mask_pii("col"))["col"][0]

    def _contains(self, value):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.contains_pii("col"))["col"][0]

    def test_detects_valid_cf(self):
        assert self._contains(self.VALID)

    def test_does_not_detect_invalid_check(self):
        assert not self._contains(self.INVALID)

    def test_full_redaction(self):
        out = self._mask(self.VALID)
        assert out == "*" * 16

    def test_invalid_not_masked(self):
        out = self._mask(self.INVALID)
        assert out == self.INVALID

    def test_in_sentence(self):
        text = f"Il codice fiscale è {self.VALID} e la data di nascita è 01/01/1980."
        out = self._mask(text)
        assert self.VALID not in out
        assert "*" * 16 in out

    def test_patterns_filter(self):
        df = pl.DataFrame({"col": [self.VALID]})
        out = df.with_columns(maskops.mask_pii("col", patterns=["codice_fiscale"]))["col"][0]
        assert out == "*" * 16

    def test_null_passthrough(self):
        df = pl.DataFrame({"col": [None]}, schema={"col": pl.String})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result is None

class TestMaskUruguayCedula:
    """Uruguayan cédula de identidad."""

    VALID = "1.111.111-1"    
    INVALID = "1.111.111-9"  

    def _mask(self, value, **kwargs):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.mask_pii("col", **kwargs))["col"][0]

    def _contains(self, value):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.contains_pii("col"))["col"][0]

    def test_detects_valid_ci(self):
        assert self._contains(self.VALID)

    def test_does_not_detect_invalid_check(self):
        assert not self._contains(self.INVALID)

    def test_asterisk_mask(self):
        out = self._mask(self.VALID)
        assert self.VALID not in out
        assert "*" in out
        assert len(out) == len(self.VALID)

    def test_invalid_not_masked(self):
        out = self._mask(self.INVALID)
        assert out == self.INVALID

    def test_in_sentence(self):
        text = f"Cédula: {self.VALID}, expedida por el Registro Civil"
        out = self._mask(text)
        assert self.VALID not in out

    def test_fpe_preserves_format(self):
        key = bytes(range(32))
        tweak = b"\x00" * 7
        df = pl.DataFrame({"col": [self.VALID]})
        out = df.with_columns(
            maskops.mask_pii_fpe("col", key, tweak)
        )["col"][0]
        assert out != self.VALID
        assert re.fullmatch(r"\d\.\d{3}\.\d{3}-\d", out), f"format broken: {out}"

    def test_consistent_deterministic(self):
        out1 = self._mask(self.VALID, mode="consistent", salt="test-salt")
        out2 = self._mask(self.VALID, mode="consistent", salt="test-salt")
        assert out1 == out2

    def test_consistent_not_original(self):
        out = self._mask(self.VALID, mode="consistent", salt="test-salt")
        assert out != self.VALID

    def test_patterns_filter(self):
        df = pl.DataFrame({"col": [self.VALID]})
        out = df.with_columns(maskops.mask_pii("col", patterns=["uy_ci"]))["col"][0]
        assert self.VALID not in out
        assert "*" in out

    def test_null_passthrough(self):
        df = pl.DataFrame({"col": [None]}, schema={"col": pl.String})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result is None

class TestMaskCanadaSIN:
    """Canadian Social Insurance Number."""

    VALID_FORMATTED = "130-692-544"
    VALID_COMPACT   = "130692544"
    INVALID         = "130-692-543"  

    def _mask(self, value, **kwargs):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.mask_pii("col", **kwargs))["col"][0]

    def _contains(self, value):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.contains_pii("col"))["col"][0]

    def test_detects_formatted(self):
        assert self._contains(self.VALID_FORMATTED)

    def test_detects_compact(self):
        assert self._contains(self.VALID_COMPACT)

    def test_does_not_detect_invalid_luhn(self):
        assert not self._contains(self.INVALID)

    def test_asterisk_formatted(self):
        out = self._mask(self.VALID_FORMATTED)
        assert self.VALID_FORMATTED not in out
        assert "*" in out
        assert len(out) == len(self.VALID_FORMATTED)

    def test_asterisk_compact(self):
        out = self._mask(self.VALID_COMPACT)
        assert out == "*" * 9

    def test_invalid_not_masked(self):
        out = self._mask(self.INVALID)
        assert out == self.INVALID

    def test_in_sentence(self):
        text = f"SIN: {self.VALID_FORMATTED} — do not share"
        out = self._mask(text)
        assert self.VALID_FORMATTED not in out

    def test_fpe_preserves_digit_count(self):
        key = bytes(range(32))
        tweak = b"\x00" * 7
        df = pl.DataFrame({"col": [self.VALID_COMPACT]})
        out = df.with_columns(
            maskops.mask_pii_fpe("col", key, tweak)
        )["col"][0]
        assert len(out) == 9
        assert out.isdigit()

    def test_consistent_deterministic(self):
        out1 = self._mask(self.VALID_COMPACT, mode="consistent", salt="test-salt")
        out2 = self._mask(self.VALID_COMPACT, mode="consistent", salt="test-salt")
        assert out1 == out2

    def test_consistent_not_original(self):
        out = self._mask(self.VALID_COMPACT, mode="consistent", salt="test-salt")
        assert out != self.VALID_COMPACT

    def test_patterns_filter(self):
        df = pl.DataFrame({"col": [self.VALID_COMPACT]})
        out = df.with_columns(maskops.mask_pii("col", patterns=["sin"]))["col"][0]
        assert self.VALID_COMPACT not in out

    def test_null_passthrough(self):
        df = pl.DataFrame({"col": [None]}, schema={"col": pl.String})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result is None

class TestMaskAustraliaTFN:
    """Australian Tax File Number."""

    VALID_SPACED  = "123 456 782"
    VALID_COMPACT = "123456782"
    INVALID       = "123456789"   

    def _mask(self, value, **kwargs):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.mask_pii("col", **kwargs))["col"][0]

    def _contains(self, value):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.contains_pii("col"))["col"][0]

    def test_detects_spaced(self):
        assert self._contains(self.VALID_SPACED)

    def test_detects_compact(self):
        assert self._contains(self.VALID_COMPACT)

    def test_does_not_detect_invalid(self):
        assert not self._contains(self.INVALID)

    def test_asterisk_spaced(self):
        out = self._mask(self.VALID_SPACED)
        assert self.VALID_SPACED not in out
        assert "*" in out

    def test_asterisk_compact(self):
        out = self._mask(self.VALID_COMPACT)
        assert out == "*" * 9

    def test_invalid_not_masked(self):
        out = self._mask(self.INVALID)
        assert out == self.INVALID

    def test_in_sentence(self):
        text = f"TFN: {self.VALID_SPACED} — confidential ATO record"
        out = self._mask(text)
        assert self.VALID_SPACED not in out

    def test_fpe_preserves_digit_count(self):
        key = bytes(range(32))
        tweak = b"\x00" * 7
        df = pl.DataFrame({"col": [self.VALID_COMPACT]})
        out = df.with_columns(
            maskops.mask_pii_fpe("col", key, tweak)
        )["col"][0]
        assert len(out) == 9
        assert out.isdigit()

    def test_consistent_deterministic(self):
        out1 = self._mask(self.VALID_COMPACT, mode="consistent", salt="test-salt")
        out2 = self._mask(self.VALID_COMPACT, mode="consistent", salt="test-salt")
        assert out1 == out2

    def test_consistent_not_original(self):
        out = self._mask(self.VALID_COMPACT, mode="consistent", salt="test-salt")
        assert out != self.VALID_COMPACT

    def test_patterns_filter(self):
        df = pl.DataFrame({"col": [self.VALID_COMPACT]})
        out = df.with_columns(maskops.mask_pii("col", patterns=["tfn"]))["col"][0]
        assert self.VALID_COMPACT not in out

    def test_null_passthrough(self):
        df = pl.DataFrame({"col": [None]}, schema={"col": pl.String})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result is None

class TestMaskPESEL:
    """Polish PESEL (11-digit national ID)."""

    VALID   = "91010112346"   
    INVALID = "91010112340"   

    def _mask(self, value, **kwargs):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.mask_pii("col", **kwargs))["col"][0]

    def _contains(self, value):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.contains_pii("col"))["col"][0]

    def test_detects_valid_pesel(self):
        assert self._contains(self.VALID)

    def test_does_not_detect_invalid_check(self):
        assert not self._contains(self.INVALID)

    def test_asterisk_mask(self):
        out = self._mask(self.VALID)
        assert out == "*" * 11

    def test_invalid_not_masked(self):
        assert self._mask(self.INVALID) == self.INVALID

    def test_in_sentence(self):
        text = f"PESEL: {self.VALID}, proszę nie udostępniać"
        out = self._mask(text)
        assert self.VALID not in out
        assert "*" * 11 in out

    def test_fpe_preserves_length(self):
        key = bytes(range(32))
        tweak = b"\x00" * 7
        df = pl.DataFrame({"col": [self.VALID]})
        out = df.with_columns(maskops.mask_pii_fpe("col", key, tweak))["col"][0]
        assert out != self.VALID
        assert re.fullmatch(r"\d{11}", out), f"format broken: {out}"

    def test_consistent_deterministic(self):
        out1 = self._mask(self.VALID, mode="consistent", salt="s")
        out2 = self._mask(self.VALID, mode="consistent", salt="s")
        assert out1 == out2

    def test_consistent_not_original(self):
        assert self._mask(self.VALID, mode="consistent", salt="s") != self.VALID

    def test_patterns_filter(self):
        df = pl.DataFrame({"col": [self.VALID]})
        out = df.with_columns(maskops.mask_pii("col", patterns=["pesel"]))["col"][0]
        assert out == "*" * 11

    def test_null_passthrough(self):
        df = pl.DataFrame({"col": [None]}, schema={"col": pl.String})
        assert df.with_columns(maskops.mask_pii("col"))["col"][0] is None

class TestMaskBSN:
    """Dutch BSN (Burgerservicenummer, 9 digits, 11-proof)."""

    VALID   = "123456782"   
    INVALID = "123456789"   

    def _mask(self, value, **kwargs):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.mask_pii("col", **kwargs))["col"][0]

    def _contains(self, value):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.contains_pii("col"))["col"][0]

    def test_detects_valid_bsn(self):
        assert self._contains(self.VALID)

    def test_does_not_detect_invalid_check(self):
        assert not self._contains(self.INVALID)

    def test_asterisk_mask(self):
        out = self._mask(self.VALID)
        assert out == "*" * 9

    def test_invalid_not_masked(self):
        assert self._mask(self.INVALID) == self.INVALID

    def test_in_sentence(self):
        text = f"BSN: {self.VALID} (burger)"
        out = self._mask(text)
        assert self.VALID not in out
        assert "*" * 9 in out

    def test_fpe_preserves_length(self):
        key = bytes(range(32))
        tweak = b"\x00" * 7
        df = pl.DataFrame({"col": [self.VALID]})
        out = df.with_columns(maskops.mask_pii_fpe("col", key, tweak))["col"][0]
        assert out != self.VALID
        assert re.fullmatch(r"\d{9}", out), f"format broken: {out}"

    def test_consistent_deterministic(self):
        out1 = self._mask(self.VALID, mode="consistent", salt="s")
        out2 = self._mask(self.VALID, mode="consistent", salt="s")
        assert out1 == out2

    def test_consistent_not_original(self):
        assert self._mask(self.VALID, mode="consistent", salt="s") != self.VALID

    def test_patterns_filter(self):
        df = pl.DataFrame({"col": [self.VALID]})
        out = df.with_columns(maskops.mask_pii("col", patterns=["bsn"]))["col"][0]
        assert out == "*" * 9

    def test_null_passthrough(self):
        df = pl.DataFrame({"col": [None]}, schema={"col": pl.String})
        assert df.with_columns(maskops.mask_pii("col"))["col"][0] is None

class TestMaskPersonnummer:
    """Swedish personnummer (YYMMDD-NNNN format, Luhn on 10 digits)."""

    VALID_SHORT = "811228-9874"    
    VALID_LONG  = "19811228-9874"  
    INVALID     = "811228-9873"    

    def _mask(self, value, **kwargs):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.mask_pii("col", **kwargs))["col"][0]

    def _contains(self, value):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.contains_pii("col"))["col"][0]

    def test_detects_valid_short(self):
        assert self._contains(self.VALID_SHORT)

    def test_detects_valid_long(self):
        assert self._contains(self.VALID_LONG)

    def test_does_not_detect_invalid(self):
        assert not self._contains(self.INVALID)

    def test_asterisk_mask_short(self):
        out = self._mask(self.VALID_SHORT)
        assert self.VALID_SHORT not in out
        assert re.fullmatch(r"\*{6}-\*{4}", out), f"format broken: {out}"

    def test_asterisk_mask_long(self):
        out = self._mask(self.VALID_LONG)
        assert self.VALID_LONG not in out
        assert re.fullmatch(r"\*{8}-\*{4}", out), f"format broken: {out}"

    def test_invalid_not_masked(self):
        assert self._mask(self.INVALID) == self.INVALID

    def test_in_sentence(self):
        text = f"Personnummer: {self.VALID_SHORT}, konfidentiellt"
        out = self._mask(text)
        assert self.VALID_SHORT not in out

    def test_fpe_preserves_format_short(self):
        key = bytes(range(32))
        tweak = b"\x00" * 7
        df = pl.DataFrame({"col": [self.VALID_SHORT]})
        out = df.with_columns(maskops.mask_pii_fpe("col", key, tweak))["col"][0]
        assert out != self.VALID_SHORT
        assert re.fullmatch(r"\d{6}-\d{4}", out), f"format broken: {out}"

    def test_consistent_deterministic(self):
        out1 = self._mask(self.VALID_SHORT, mode="consistent", salt="s")
        out2 = self._mask(self.VALID_SHORT, mode="consistent", salt="s")
        assert out1 == out2

    def test_consistent_not_original(self):
        assert self._mask(self.VALID_SHORT, mode="consistent", salt="s") != self.VALID_SHORT

    def test_patterns_filter(self):
        df = pl.DataFrame({"col": [self.VALID_SHORT]})
        out = df.with_columns(maskops.mask_pii("col", patterns=["personnummer"]))["col"][0]
        assert self.VALID_SHORT not in out
        assert "-" in out

    def test_null_passthrough(self):
        df = pl.DataFrame({"col": [None]}, schema={"col": pl.String})
        assert df.with_columns(maskops.mask_pii("col"))["col"][0] is None


class TestExtractPII:
    def _extract(self, text):
        df = pl.DataFrame({"col": [text]})
        return df.with_columns(maskops.extract_pii("col").alias("pii"))["pii"][0]

    def test_email_detected(self):
        r = self._extract("contact user@example.com for info")
        assert r["email"] == "user@example.com"
        assert r["phone"] is None

    def test_credit_card_detected(self):
        r = self._extract("card 4532015112830366 used")
        assert r["credit_card"] == "4532015112830366"
        assert r["email"] is None

    def test_ssn_detected(self):
        r = self._extract("SSN is 321-45-6789")
        assert r["ssn"] == "321-45-6789"

    def test_iban_detected(self):
        r = self._extract("pay to DE89370400440532013000 please")
        assert r["iban"] == "DE89370400440532013000"

    def test_ip_detected(self):
        r = self._extract("server at 192.168.1.1 is down")
        assert r["ip"] == "192.168.1.1"

    def test_phone_detected(self):
        r = self._extract("call +1 800 555 1234 now")
        assert r["phone"] is not None
        assert "+1" in r["phone"]

    def test_null_input_all_none(self):
        df = pl.DataFrame({"col": [None]}, schema={"col": pl.String})
        r = df.with_columns(maskops.extract_pii("col").alias("pii"))["pii"][0]
        assert r["email"] is None
        assert r["credit_card"] is None
        assert r["ssn"] is None

    def test_no_pii_all_none(self):
        r = self._extract("nothing sensitive here")
        for field in ["email", "phone", "ip", "iban", "credit_card", "ssn", "rut", "cpf"]:
            assert r[field] is None, f"expected {field} to be None"

    def test_returns_struct_column(self):
        df = pl.DataFrame({"col": ["user@example.com"]})
        result = df.with_columns(maskops.extract_pii("col").alias("pii"))
        assert isinstance(result["pii"].dtype, pl.Struct)

    def test_struct_has_36_fields(self):
        df = pl.DataFrame({"col": ["user@example.com"]})
        result = df.with_columns(maskops.extract_pii("col").alias("pii"))
        assert len(result["pii"].dtype.fields) == 36

    def test_multiple_rows(self):
        df = pl.DataFrame({"col": ["user@example.com", "4532015112830366", "nothing"]})
        result = df.with_columns(maskops.extract_pii("col").alias("pii"))
        assert result["pii"][0]["email"] == "user@example.com"
        assert result["pii"][1]["credit_card"] == "4532015112830366"
        assert result["pii"][2]["email"] is None


class TestMaskJapanMyNumber:
    """Japanese My Number (個人番号) — 12-digit national ID."""

    VALID_COMPACT = "123456789123"
    VALID_SPACED  = "1234 5678 9123"
    INVALID       = "123456789124"

    def _mask(self, value, **kwargs):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.mask_pii("col", **kwargs))["col"][0]

    def _contains(self, value):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.contains_pii("col"))["col"][0]

    def test_detects_compact(self):
        assert self._contains(self.VALID_COMPACT)

    def test_detects_spaced(self):
        assert self._contains(self.VALID_SPACED)

    def test_does_not_detect_invalid(self):
        assert not self._contains(self.INVALID)

    def test_asterisk_compact(self):
        out = self._mask(self.VALID_COMPACT)
        assert out == "*" * 12

    def test_asterisk_spaced(self):
        out = self._mask(self.VALID_SPACED)
        assert self.VALID_SPACED not in out
        assert "*" in out
        assert len(out) == len(self.VALID_SPACED)

    def test_invalid_not_masked(self):
        out = self._mask(self.INVALID)
        assert out == self.INVALID

    def test_in_sentence(self):
        text = f"個人番号: {self.VALID_COMPACT} — 要保護"
        out = self._mask(text)
        assert self.VALID_COMPACT not in out

    def test_fpe_preserves_digit_count(self):
        key = bytes(range(32))
        tweak = b"\x00" * 7
        df = pl.DataFrame({"col": [self.VALID_COMPACT]})
        out = df.with_columns(maskops.mask_pii_fpe("col", key, tweak))["col"][0]
        assert len(out) == 12
        assert out.isdigit()

    def test_consistent_deterministic(self):
        out1 = self._mask(self.VALID_COMPACT, mode="consistent", salt="test-salt")
        out2 = self._mask(self.VALID_COMPACT, mode="consistent", salt="test-salt")
        assert out1 == out2

    def test_consistent_not_original(self):
        out = self._mask(self.VALID_COMPACT, mode="consistent", salt="test-salt")
        assert out != self.VALID_COMPACT

    def test_patterns_filter(self):
        df = pl.DataFrame({"col": [self.VALID_COMPACT]})
        out = df.with_columns(maskops.mask_pii("col", patterns=["my_number"]))["col"][0]
        assert self.VALID_COMPACT not in out

    def test_extract_pii(self):
        df = pl.DataFrame({"col": [self.VALID_COMPACT]})
        result = df.with_columns(maskops.extract_pii("col").alias("pii"))
        assert result["pii"][0]["my_number"] == self.VALID_COMPACT

    def test_null_passthrough(self):
        df = pl.DataFrame({"col": [None]}, schema={"col": pl.String})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result is None


class TestMaskKoreaRRN:
    """South Korean Resident Registration Number (주민등록번호)."""

    VALID_FMT     = "900101-1234568"
    VALID_COMPACT = "9001011234568"
    INVALID       = "900101-1234567"

    def _mask(self, value, **kwargs):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.mask_pii("col", **kwargs))["col"][0]

    def _contains(self, value):
        df = pl.DataFrame({"col": [value]})
        return df.with_columns(maskops.contains_pii("col"))["col"][0]

    def test_detects_formatted(self):
        assert self._contains(self.VALID_FMT)

    def test_detects_compact(self):
        assert self._contains(self.VALID_COMPACT)

    def test_does_not_detect_invalid(self):
        assert not self._contains(self.INVALID)

    def test_asterisk_formatted(self):
        out = self._mask(self.VALID_FMT)
        assert self.VALID_FMT not in out
        assert "*" in out
        assert len(out) == len(self.VALID_FMT)

    def test_asterisk_compact(self):
        out = self._mask(self.VALID_COMPACT)
        assert out == "*" * 13

    def test_invalid_not_masked(self):
        out = self._mask(self.INVALID)
        assert out == self.INVALID

    def test_in_sentence(self):
        text = f"주민번호: {self.VALID_FMT} — 개인정보"
        out = self._mask(text)
        assert self.VALID_FMT not in out

    def test_fpe_formatted_preserves_separator(self):
        key = bytes(range(32))
        tweak = b"\x00" * 7
        df = pl.DataFrame({"col": [self.VALID_FMT]})
        out = df.with_columns(maskops.mask_pii_fpe("col", key, tweak))["col"][0]
        assert "-" in out
        assert len(out) == len(self.VALID_FMT)

    def test_fpe_compact_preserves_digit_count(self):
        key = bytes(range(32))
        tweak = b"\x00" * 7
        df = pl.DataFrame({"col": [self.VALID_COMPACT]})
        out = df.with_columns(maskops.mask_pii_fpe("col", key, tweak))["col"][0]
        assert len(out) == 13
        assert out.isdigit()

    def test_consistent_deterministic(self):
        out1 = self._mask(self.VALID_COMPACT, mode="consistent", salt="test-salt")
        out2 = self._mask(self.VALID_COMPACT, mode="consistent", salt="test-salt")
        assert out1 == out2

    def test_consistent_not_original(self):
        out = self._mask(self.VALID_COMPACT, mode="consistent", salt="test-salt")
        assert out != self.VALID_COMPACT

    def test_patterns_filter(self):
        df = pl.DataFrame({"col": [self.VALID_FMT]})
        out = df.with_columns(maskops.mask_pii("col", patterns=["rrn"]))["col"][0]
        assert self.VALID_FMT not in out

    def test_extract_pii(self):
        df = pl.DataFrame({"col": [self.VALID_FMT]})
        result = df.with_columns(maskops.extract_pii("col").alias("pii"))
        assert result["pii"][0]["rrn"] == self.VALID_FMT

    def test_null_passthrough(self):
        df = pl.DataFrame({"col": [None]}, schema={"col": pl.String})
        result = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert result is None


class TestMaskPIIAudit:
    def _audit(self, text):
        df = pl.DataFrame({"col": [text]})
        return df.with_columns(maskops.mask_pii_audit("col").alias("a"))["a"][0]

    def test_masked_matches_mask_pii(self):
        text = "email a@b.com and card 4111 1111 1111 1111"
        df = pl.DataFrame({"col": [text]})
        out = df.with_columns(
            maskops.mask_pii("col").alias("plain"),
            maskops.mask_pii_audit("col").alias("a"),
        )
        assert out["a"][0]["masked"] == out["plain"][0]

    def test_single_email_count(self):
        r = self._audit("contact user@example.com please")
        assert r["counts"]["email"] == 1
        assert r["counts"]["credit_card"] == 0

    def test_multiple_email_count(self):
        r = self._audit("mail x@y.com and z@w.org")
        assert r["counts"]["email"] == 2

    def test_credit_card_count(self):
        r = self._audit("card 4532015112830366 used")
        assert r["counts"]["credit_card"] == 1

    def test_mixed_families(self):
        r = self._audit("SSN 321-45-6789 mail a@b.com ip 192.168.1.1")
        assert r["counts"]["ssn"] == 1
        assert r["counts"]["email"] == 1
        assert r["counts"]["ip"] == 1

    def test_invalid_not_counted(self):
        r = self._audit("card 4111 1111 1111 1112 is invalid")
        assert r["counts"]["credit_card"] == 0

    def test_no_pii_all_zero(self):
        r = self._audit("nothing sensitive here")
        for fam in ["email", "phone", "ip", "credit_card", "ssn", "iban"]:
            assert r["counts"][fam] == 0

    def test_null_input(self):
        df = pl.DataFrame({"col": [None]}, schema={"col": pl.String})
        r = df.with_columns(maskops.mask_pii_audit("col").alias("a"))["a"][0]
        assert r["masked"] is None
        assert r["counts"]["email"] == 0

    def test_returns_nested_struct(self):
        df = pl.DataFrame({"col": ["user@example.com"]})
        dtype = df.with_columns(maskops.mask_pii_audit("col").alias("a"))["a"].dtype
        assert isinstance(dtype, pl.Struct)
        names = [f.name for f in dtype.fields]
        assert names == ["masked", "counts"]
        counts_field = next(f for f in dtype.fields if f.name == "counts")
        assert isinstance(counts_field.dtype, pl.Struct)
        assert len(counts_field.dtype.fields) == 36

    def test_counts_are_uint32(self):
        df = pl.DataFrame({"col": ["user@example.com"]})
        counts = df.with_columns(
            maskops.mask_pii_audit("col").alias("a")
        ).select(pl.col("a").struct.field("counts").struct.field("email"))
        assert counts.to_series().dtype == pl.UInt32

    def test_multiple_rows(self):
        df = pl.DataFrame({"col": ["a@b.com", "x@y.com z@w.org", "nothing"]})
        out = df.with_columns(maskops.mask_pii_audit("col").alias("a"))
        assert out["a"][0]["counts"]["email"] == 1
        assert out["a"][1]["counts"]["email"] == 2
        assert out["a"][2]["counts"]["email"] == 0


KEY2  = bytes(range(32, 64))
TWEAK2 = bytes([7, 6, 5, 4, 3, 2, 1])


class TestFpeFf1Mode:
    def test_ff1_preserves_length_and_digits(self):
        df = pl.DataFrame({"col": ["4111111111111111"]})
        out = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK, mode="ff1"))["col"][0]
        assert len(out) == 16
        assert out.isdigit()

    def test_ff1_differs_from_ff3(self):
        df = pl.DataFrame({"col": ["4111111111111111"]})
        ff3 = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK, mode="ff3"))["col"][0]
        ff1 = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK, mode="ff1"))["col"][0]
        assert ff1 != ff3

    def test_default_mode_is_ff3(self):
        df = pl.DataFrame({"col": ["4111111111111111"]})
        default = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        ff3 = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK, mode="ff3"))["col"][0]
        assert default == ff3

    def test_ff1_differs_by_tweak(self):
        df = pl.DataFrame({"col": ["4111111111111111"]})
        a = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK, mode="ff1"))["col"][0]
        b = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK2, mode="ff1"))["col"][0]
        assert a != b

    def test_unknown_mode_raises(self):
        df = pl.DataFrame({"col": ["4111111111111111"]})
        with pytest.raises(Exception):
            df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK, mode="ff9"))


class TestFpeRekey:
    def _mask(self, val, key, tweak, mode):
        df = pl.DataFrame({"col": [val]})
        return df.with_columns(maskops.mask_pii_fpe("col", key, tweak, mode=mode))["col"][0]

    def _rekey(self, token, ok, ot, nk, nt, mode):
        df = pl.DataFrame({"col": [token]})
        return df.with_columns(maskops.rekey_pii_fpe("col", ok, ot, nk, nt, mode=mode))["col"][0]

    @pytest.mark.parametrize("mode", ["ff3", "ff1"])
    def test_rekey_equals_direct_mask(self, mode):
        token = self._mask("4111111111111111", KEY, TWEAK, mode)
        rotated = self._rekey(token, KEY, TWEAK, KEY2, TWEAK2, mode)
        direct = self._mask("4111111111111111", KEY2, TWEAK2, mode)
        assert rotated == direct

    @pytest.mark.parametrize("mode", ["ff3", "ff1"])
    def test_rekey_is_reversible_full_circle(self, mode):
        token = self._mask("4111111111111111", KEY, TWEAK, mode)
        rotated = self._rekey(token, KEY, TWEAK, KEY2, TWEAK2, mode)
        back = self._rekey(rotated, KEY2, TWEAK2, KEY, TWEAK, mode)
        assert back == token

    def test_non_token_passes_through(self):
        out = self._rekey("hello world", KEY, TWEAK, KEY2, TWEAK2, "ff3")
        assert out == "hello world"

    def test_rekey_preserves_length(self):
        token = self._mask("4111111111111111", KEY, TWEAK, "ff3")
        rotated = self._rekey(token, KEY, TWEAK, KEY2, TWEAK2, "ff3")
        assert len(rotated) == 16 and rotated.isdigit()


class TestKeyManagement:
    def test_validate_key_accepts_good(self):
        assert maskops.validate_key(bytes(range(32))) == bytes(range(32))

    def test_validate_key_rejects_length(self):
        with pytest.raises(ValueError):
            maskops.validate_key(bytes(16))

    def test_validate_key_rejects_weak_repeated(self):
        with pytest.raises(ValueError):
            maskops.validate_key(b"\x00" * 32)

    def test_validate_tweak_rejects_length(self):
        with pytest.raises(ValueError):
            maskops.validate_tweak(bytes(4))

    def test_derive_key_is_deterministic(self):
        a = maskops.derive_key(b"master-secret", "tenant-A")
        b = maskops.derive_key(b"master-secret", "tenant-A")
        assert a == b and len(a) == 32

    def test_derive_key_context_separation(self):
        a = maskops.derive_key(b"master-secret", "tenant-A")
        b = maskops.derive_key(b"master-secret", "tenant-B")
        assert a != b

    def test_derive_tweak_is_deterministic_and_sized(self):
        a = maskops.derive_tweak(b"master-secret", "tenant-A")
        b = maskops.derive_tweak(b"master-secret", "tenant-A")
        assert a == b and len(a) == 7

    def test_derived_key_passes_validation(self):
        k = maskops.derive_key(b"master-secret", "tenant-A")
        assert maskops.validate_key(k) == k

    def test_derived_key_usable_for_fpe(self):
        k = maskops.derive_key(b"master-secret", "tenant-A")
        t = maskops.derive_tweak(b"master-secret", "tenant-A")
        df = pl.DataFrame({"col": ["4111111111111111"]})
        out = df.with_columns(maskops.mask_pii_fpe("col", k, t))["col"][0]
        assert out.isdigit() and len(out) == 16


ZA_ID = "8507158001087"
ZA_ID2 = "9001015009086"
IL_ID = "012345674"


class TestMaskZaId:
    def test_masks_valid(self):
        df = pl.DataFrame({"col": [f"ID {ZA_ID} on file"]})
        out = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert ZA_ID not in out
        assert "*" * 13 in out

    def test_rejects_bad_date(self):
        bad = "9013015009081"
        df = pl.DataFrame({"col": [f"x {bad} y"]})
        out = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert bad in out

    def test_rejects_wrong_length(self):
        df = pl.DataFrame({"col": ["850715800108"]})
        out = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert "850715800108" in out

    def test_contains(self):
        df = pl.DataFrame({"col": [f"ID {ZA_ID}"]})
        assert df.with_columns(maskops.contains_pii("col", patterns=["za_id"]).alias("h"))["h"][0]

    def test_extract(self):
        df = pl.DataFrame({"col": [f"ID {ZA_ID}"]})
        ex = df.with_columns(maskops.extract_pii("col").alias("e"))
        assert ex.select(pl.col("e").struct.field("za_id"))["za_id"][0] == ZA_ID

    def test_audit_count(self):
        df = pl.DataFrame({"col": [f"{ZA_ID} and {ZA_ID2}"]})
        a = df.with_columns(maskops.mask_pii_audit("col").alias("a"))
        assert a.select(pl.col("a").struct.field("counts").struct.field("za_id"))["za_id"][0] == 1

    def test_fpe_preserves_digits(self):
        df = pl.DataFrame({"col": [ZA_ID]})
        out = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert out.isdigit() and len(out) == 13 and "*" not in out

    def test_consistent_is_deterministic(self):
        df = pl.DataFrame({"col": [ZA_ID]})
        a = df.with_columns(maskops.mask_pii("col", mode="consistent", salt="s"))["col"][0]
        b = df.with_columns(maskops.mask_pii("col", mode="consistent", salt="s"))["col"][0]
        assert a == b and ZA_ID not in a


class TestMaskIlId:
    def test_masks_valid(self):
        df = pl.DataFrame({"col": [f"teudat {IL_ID} zehut"]})
        out = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert IL_ID not in out
        assert "*" * 9 in out

    def test_rejects_bad_checksum(self):
        bad = "012345670"
        df = pl.DataFrame({"col": [f"x {bad} y"]})
        out = df.with_columns(maskops.mask_pii("col"))["col"][0]
        assert bad in out

    def test_contains(self):
        df = pl.DataFrame({"col": [f"id {IL_ID}"]})
        assert df.with_columns(maskops.contains_pii("col", patterns=["il_id"]).alias("h"))["h"][0]

    def test_extract(self):
        df = pl.DataFrame({"col": [f"id {IL_ID}"]})
        ex = df.with_columns(maskops.extract_pii("col").alias("e"))
        assert ex.select(pl.col("e").struct.field("il_id"))["il_id"][0] == IL_ID

    def test_audit_count(self):
        df = pl.DataFrame({"col": [f"id {IL_ID}"]})
        a = df.with_columns(maskops.mask_pii_audit("col").alias("a"))
        assert a.select(pl.col("a").struct.field("counts").struct.field("il_id"))["il_id"][0] == 1

    def test_fpe_preserves_digits(self):
        df = pl.DataFrame({"col": [IL_ID]})
        out = df.with_columns(maskops.mask_pii_fpe("col", KEY, TWEAK))["col"][0]
        assert out.isdigit() and len(out) == 9 and "*" not in out


class TestUnifiedPatterns:
    SRC = "mail a@b.com card 4111111111111111 ssn 123-45-6789"

    def _audit(self, patterns=None):
        df = pl.DataFrame({"col": [self.SRC]})
        return df.with_columns(maskops.mask_pii_audit("col", patterns=patterns).alias("a"))

    def test_audit_patterns_masks_only_selected(self):
        a = self._audit(patterns=["email"])
        masked = a.select(pl.col("a").struct.field("masked"))["masked"][0]
        assert "a@b.com" not in masked
        assert "4111111111111111" in masked
        assert "123-45-6789" in masked

    def test_audit_patterns_counts_only_selected(self):
        a = self._audit(patterns=["email"])
        counts = a.select(pl.col("a").struct.field("counts")).unnest("counts")
        assert counts["email"][0] == 1
        assert counts["ssn"][0] == 0
        assert counts["credit_card"][0] == 0

    def test_audit_no_patterns_masks_all(self):
        a = self._audit()
        masked = a.select(pl.col("a").struct.field("masked"))["masked"][0]
        assert "a@b.com" not in masked
        assert "4111111111111111" not in masked
        assert "123-45-6789" not in masked

    def test_extract_patterns_nulls_non_selected(self):
        df = pl.DataFrame({"col": [self.SRC]})
        ex = df.with_columns(maskops.extract_pii("col", patterns=["email"]).alias("e"))
        assert ex.select(pl.col("e").struct.field("email"))["email"][0] == "a@b.com"
        assert ex.select(pl.col("e").struct.field("ssn"))["ssn"][0] is None
        assert ex.select(pl.col("e").struct.field("credit_card"))["credit_card"][0] is None

    def test_extract_no_patterns_populates_all(self):
        df = pl.DataFrame({"col": [self.SRC]})
        ex = df.with_columns(maskops.extract_pii("col").alias("e"))
        assert ex.select(pl.col("e").struct.field("email"))["email"][0] == "a@b.com"
        assert ex.select(pl.col("e").struct.field("ssn"))["ssn"][0] == "123-45-6789"

    def test_audit_multi_pattern_selection(self):
        a = self._audit(patterns=["email", "ssn"])
        masked = a.select(pl.col("a").struct.field("masked"))["masked"][0]
        assert "a@b.com" not in masked and "123-45-6789" not in masked
        assert "4111111111111111" in masked