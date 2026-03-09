"""
tests/generate_fixtures.py

Generates fake but structurally valid PII test data for multiple locales.

Covers: IBAN, EU VAT, email, phone, IPv4/IPv6, RUT, CPF, CURP, and credit cards.

Run once to produce:
  tests/fixtures/eu_pii_sample.csv
  tests/fixtures/phone_sample.csv
  tests/fixtures/latam_pii_sample.csv
  tests/fixtures/card_pii_sample.csv

Usage:
    pip install faker
    python tests/generate_fixtures.py
"""

from faker import Faker
import polars as pl
from pathlib import Path
import random

# ---------------------------------------------------------------------------
# Locales
# ---------------------------------------------------------------------------

EU_LOCALES = ["de_DE", "fr_FR", "es_ES", "it_IT", "nl_NL", "pl_PL", "pt_PT", "sv_SE"]

# Faker locale mapped to each country code in country_codes.rs
PHONE_LOCALES = {
    "+354": "is_IS",  # Iceland — fallback to en_US if unavailable
    "+353": "en_IE",
    "+352": "fr_LU",
    "+351": "pt_PT",
    "+358": "fi_FI",
    "+370": "lt_LT",
    "+371": "lv_LV",
    "+372": "et_EE",
    "+385": "hr_HR",
    "+386": "sl_SI",
    "+420": "cs_CZ",
    "+421": "sk_SK",
    "+56":  "es_CL",
    "+55":  "pt_BR",
    "+54":  "es_AR",
    "+51":  "es_PE",
    "+57":  "es_CO",
    "+52":  "es_MX",
    "+44":  "en_GB",
    "+49":  "de_DE",
    "+33":  "fr_FR",
    "+34":  "es_ES",
    "+39":  "it_IT",
    "+31":  "nl_NL",
    "+32":  "nl_BE",
    "+48":  "pl_PL",
    "+46":  "sv_SE",
    "+47":  "no_NO",
    "+41":  "de_CH",
    "+43":  "de_AT",
    "+1":   "en_US",
}

fakes_eu = {locale: Faker(locale) for locale in EU_LOCALES}
Faker.seed(42)
random.seed(42)

# ---------------------------------------------------------------------------
# EU PII fixture (IBAN + VAT + email embedded in text)
# ---------------------------------------------------------------------------

def generate_eu_row(fake: Faker) -> dict:
    """Generate one realistic row mixing EU PII into natural-language strings."""
    iban = fake.iban()
    name = fake.name()
    company = fake.company()
    email = fake.email()
    phone = fake.phone_number()

    return {
        "locale": fake.locale(),
        "iban_clean": iban,
        "email_clean": email,
        "phone_clean": phone,
        "notes": random.choice([
            f"Transfer to {name}: {iban}",
            f"Invoice from {company}, bank ref {iban}",
            f"Contact {email} for payment details",
            f"Call {phone} to confirm transfer {iban}",
            f"No financial data here, just a note from {name}",
        ]),
        "free_text": random.choice([
            fake.sentence(),
            f"Account {iban} was flagged",
            f"Reach out to {email}",
            f"Phone: {phone}",
            fake.paragraph(),
        ]),
    }

eu_rows = []
for locale, fake in fakes_eu.items():
    for _ in range(200):
        eu_rows.append(generate_eu_row(fake))

df_eu = pl.DataFrame(eu_rows)

# ---------------------------------------------------------------------------
# Phone fixture — one row per country code, E.164 format
# ---------------------------------------------------------------------------

phone_rows = []
for prefix, locale in PHONE_LOCALES.items():
    try:
        fake = Faker(locale)
    except Exception:
        fake = Faker("en_US")
    for _ in range(50):  # 50 rows per country
        raw = fake.phone_number()
        # Normalize to E.164-ish by prepending prefix if not present
        normalized = raw if raw.startswith("+") else f"{prefix}{raw.lstrip('0')}"
        phone_rows.append({
            "prefix": prefix,
            "locale": locale,
            "phone_raw": raw,
            "phone_e164": normalized,
            "sentence": f"Call us at {normalized} for support",
        })

df_phone = pl.DataFrame(phone_rows)

# ---------------------------------------------------------------------------
# LatAm ID fixture — RUT (Chile), CPF (Brazil), CURP (Mexico)
# ---------------------------------------------------------------------------

def generate_rut() -> str:
    """Generate a valid Chilean RUT with correct Módulo 11 check digit."""
    body = random.randint(1_000_000, 25_000_000)
    digits = [int(d) for d in reversed(str(body))]
    factors = [2, 3, 4, 5, 6, 7]
    total = sum(d * factors[i % 6] for i, d in enumerate(digits))
    remainder = 11 - (total % 11)
    dv = "0" if remainder == 11 else "K" if remainder == 10 else str(remainder)
    return f"{body:,}".replace(",", ".") + f"-{dv}"


def generate_cpf() -> str:
    """Generate a valid Brazilian CPF with correct Módulo 11 check digits."""
    d = [random.randint(0, 9) for _ in range(9)]
    # Reject all-same
    while len(set(d)) == 1:
        d = [random.randint(0, 9) for _ in range(9)]

    r1 = (sum(v * (10 - i) for i, v in enumerate(d)) * 10) % 11
    d1 = 0 if r1 == 10 else r1
    d.append(d1)

    r2 = (sum(v * (11 - i) for i, v in enumerate(d)) * 10) % 11
    d2 = 0 if r2 == 10 else r2
    d.append(d2)

    return f"{''.join(map(str, d[:3]))}.{''.join(map(str, d[3:6]))}.{''.join(map(str, d[6:9]))}-{''.join(map(str, d[9:]))}"


# CURP uses a fixed valid sample pool — generative synthesis is out of scope
CURP_SAMPLES = [
    "BADD110313HCMLNS09",
    "GODE561231MDFRRL06",
    "HEGE560427MVZRRL06",
    "LOOA631201HVZPNS08",
    "MOCA530428HVZRGL04",
]

LATAM_FAKES = {
    "es_CL": Faker("es_CL"),
    "pt_BR": Faker("pt_BR"),
    "es_MX": Faker("es_MX"),
}

latam_rows = []
for locale, fake in LATAM_FAKES.items():
    for _ in range(200):
        rut = generate_rut()
        cpf = generate_cpf()
        curp = random.choice(CURP_SAMPLES)
        latam_rows.append({
            "locale": locale,
            "rut_clean": rut,
            "cpf_clean": cpf,
            "curp_clean": curp,
            "notes": random.choice([
                f"Cliente RUT {rut} registrado",
                f"CPF do cliente: {cpf} confirmado",
                f"CURP: {curp} registrado",
                f"Datos: {rut} / {cpf}",
                fake.sentence(),
            ]),
        })

df_latam = pl.DataFrame(latam_rows)

# ---------------------------------------------------------------------------
# Credit card fixture — Visa, Mastercard, Amex, Discover, Maestro
# ---------------------------------------------------------------------------

def generate_luhn(partial: list) -> str:
    """Complete a partial card number with a valid Luhn check digit."""
    total = 0
    for i, d in enumerate(reversed(partial)):
        n = d * 2 if i % 2 == 0 else d
        total += n - 9 if n > 9 else n
    check = (10 - (total % 10)) % 10
    return "".join(map(str, partial)) + str(check)


def generate_visa() -> str:
    """Generate a valid 16-digit Visa number."""
    partial = [4] + [random.randint(0, 9) for _ in range(14)]
    return generate_luhn(partial)


def generate_mastercard() -> str:
    """Generate a valid 16-digit Mastercard number (51-55 range)."""
    partial = [5, random.randint(1, 5)] + [random.randint(0, 9) for _ in range(13)]
    return generate_luhn(partial)


def generate_amex() -> str:
    """Generate a valid 15-digit Amex number (34 or 37 prefix)."""
    partial = [3, random.choice([4, 7])] + [random.randint(0, 9) for _ in range(12)]
    return generate_luhn(partial)


def generate_discover() -> str:
    """Generate a valid 16-digit Discover number (6011 prefix)."""
    partial = [6, 0, 1, 1] + [random.randint(0, 9) for _ in range(11)]
    return generate_luhn(partial)


def generate_maestro() -> str:
    """Generate a valid 16-digit Maestro number (6304 prefix)."""
    partial = [6, 3, 0, 4] + [random.randint(0, 9) for _ in range(11)]
    return generate_luhn(partial)


CARD_GENERATORS = {
    "visa":       generate_visa,
    "mastercard": generate_mastercard,
    "amex":       generate_amex,
    "discover":   generate_discover,
    "maestro":    generate_maestro,
}

fake_en = Faker("en_US")

card_rows = []
for scheme, gen in CARD_GENERATORS.items():
    for _ in range(200):
        card = gen()
        card_rows.append({
            "scheme": scheme,
            "card_clean": card,
            "notes": random.choice([
                f"Payment charged to card {card}",
                f"Card number: {card} approved",
                f"Refund issued to {card}",
                fake_en.sentence(),
            ]),
        })

df_cards = pl.DataFrame(card_rows)

# ---------------------------------------------------------------------------
# Write fixtures
# ---------------------------------------------------------------------------

out = Path(__file__).parent / "fixtures"
out.mkdir(exist_ok=True)

df_eu.write_csv(out / "eu_pii_sample.csv")
# polars write_csv doesn't take encoding — the file is already UTF-8, just the reader was wrong
df_phone.write_csv(out / "phone_sample.csv")
df_latam.write_csv(out / "latam_pii_sample.csv")
df_cards.write_csv(out / "card_pii_sample.csv")
print(f"Generated {len(df_cards)} card rows -> tests/fixtures/card_pii_sample.csv")

print(f"Generated {len(df_eu)} EU rows -> tests/fixtures/eu_pii_sample.csv")
print(f"Generated {len(df_phone)} phone rows -> tests/fixtures/phone_sample.csv")
print(df_eu.head(3))
print(df_phone.head(3))
print(f"Generated {len(df_latam)} LatAm rows -> tests/fixtures/latam_pii_sample.csv")