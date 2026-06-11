import argparse
import random

import polars as pl
from faker import Faker

TEMPLATES = [
    "Customer {name} paid with card {cc} on {date}.",
    "Please email {email} or call {phone} regarding invoice {inv}.",
    "Wire transfer to IBAN {iban} was approved by {name}.",
    "Login attempt from {ip} on the account held by {name} in {city}.",
    "Mastercard {cc} refund processed for {email}.",
    "Server {ip} blocked an outbound request to {email}.",
    "Contact {name} at {phone} about the {city} branch onboarding.",
    "Account {iban} was flagged for review; notify {email}.",
    "No sensitive data in this short note about the {city} project.",
    "Reach {name} ({email}) or {phone} for the {city} supplier contract.",
    "Charge card {cc} and confirm the order with {name} by reply.",
    "Internal memo: {name} reviewed the {city} pipeline, nothing flagged.",
    "Refund of invoice {inv} issued to {email}; card ending in {cc}.",
    "The {city} office reported a login from {ip} at midnight.",
]


def build_pool(fake, size):
    pool = []
    for _ in range(size):
        fields = {
            "name": fake.name(),
            "email": fake.email(),
            "phone": fake.phone_number(),
            "cc": fake.credit_card_number(),
            "iban": fake.iban(),
            "ip": fake.ipv4(),
            "city": fake.city(),
            "date": fake.date(),
            "inv": fake.random_int(1000, 99999),
        }
        pool.append(random.choice(TEMPLATES).format(**fields))
    return pool


def main():
    parser = argparse.ArgumentParser(description="Generate realistic benchmark data")
    parser.add_argument("--rows", type=int, default=1_000_000)
    parser.add_argument("--pool", type=int, default=100_000)
    parser.add_argument("--seed", type=int, default=42)
    parser.add_argument("--out", default="target/bench_data.parquet")
    args = parser.parse_args()

    Faker.seed(args.seed)
    random.seed(args.seed)
    fake = Faker("en_US")

    print(f"Building unique pool of {args.pool:,} realistic records...")
    pool = build_pool(fake, args.pool)

    print(f"Sampling {args.rows:,} rows from the pool...")
    rows = [random.choice(pool) for _ in range(args.rows)]

    df = pl.DataFrame({"text": rows})
    df.write_parquet(args.out)
    uniq = df["text"].n_unique()
    print(f"Wrote {args.out}: {len(df):,} rows, {uniq:,} unique, {df.estimated_size('mb'):.1f} MB")


if __name__ == "__main__":
    main()
