"""
tests/generate_fixtures.py

Generates fake but structurally valid PII test data for multiple EU locales.
Run once to produce tests/fixtures/eu_pii_sample.csv

Usage:
    pip install faker
    python tests/generate_fixtures.py
"""

from faker import Faker
import polars as pl
from pathlib import Path
import random

# EU locales with good Faker PII support
LOCALES = ["de_DE", "fr_FR", "es_ES", "it_IT", "nl_NL", "pl_PL", "pt_PT", "sv_SE"]

fakes = {locale: Faker(locale) for locale in LOCALES}
Faker.seed(42)
random.seed(42)

def generate_row(fake: Faker) -> dict:
    """Generate one realistic row mixing PII into natural-language strings."""
    iban = fake.iban()
    name = fake.name()
    company = fake.company()

    return {
        "locale": fake.locale(),
        # Clean fields — single PII value
        "iban_clean": iban,
        # Embedded — PII inside a sentence (harder for regex)
        "notes": random.choice([
            f"Transfer to {name}: {iban}",
            f"Invoice from {company}, bank ref {iban}",
            f"Payment confirmed for account {iban}",
            f"No financial data here, just a note from {name}",
            f"Contact {fake.email()} for details",
        ]),
        # Mixed — may or may not contain PII
        "free_text": random.choice([
            fake.sentence(),
            f"Account {iban} was flagged",
            fake.paragraph(),
        ]),
    }

rows = []
for locale, fake in fakes.items():
    for _ in range(200):  # 200 rows per country = 1600 total
        rows.append(generate_row(fake))

df = pl.DataFrame(rows)

out = Path(__file__).parent / "fixtures"
out.mkdir(exist_ok=True)
df.write_csv(out / "eu_pii_sample.csv")

print(f"Generated {len(df)} rows -> tests/fixtures/eu_pii_sample.csv")
print(df.head(5))