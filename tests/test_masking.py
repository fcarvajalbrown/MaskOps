"""
Tests for maskops PII masking expressions.

Run after `maturin develop`:
    pytest tests/ -v
"""

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
