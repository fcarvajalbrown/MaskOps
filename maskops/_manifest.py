"""maskops masking manifest / RAT export — per-column PII inventory for data-processing registers."""
from __future__ import annotations
import json
from datetime import datetime, timezone
from pathlib import Path
from typing import Union

import polars as pl
import maskops

PathLike = Union[str, Path]

PII_FAMILIES = [
    "email", "phone", "ip", "iban", "vat", "dni", "nie", "nin",
    "personalausweis", "nir", "codice_fiscale", "pesel", "bsn",
    "personnummer", "credit_card", "ssn", "us_passport", "rut", "cpf",
    "cnpj", "curp", "arg_dni", "co_cc", "co_nit", "ec_cedula", "pe_dni",
    "uy_ci", "npi", "mbi", "nhs", "sin", "tfn", "my_number", "rrn",
]

REGULATION = {
    "email": "General personal data (cross-jurisdiction)",
    "phone": "General personal data (cross-jurisdiction)",
    "ip": "General personal data (cross-jurisdiction)",
    "iban": "GDPR / financial (EU)",
    "vat": "GDPR (EU)",
    "credit_card": "PCI-DSS",
    "dni": "GDPR — Spain (LOPDGDD)",
    "nie": "GDPR — Spain (LOPDGDD)",
    "nin": "UK GDPR / DPA 2018",
    "personalausweis": "GDPR — Germany (BDSG)",
    "nir": "GDPR — France (CNIL)",
    "codice_fiscale": "GDPR — Italy",
    "pesel": "GDPR — Poland",
    "bsn": "GDPR — Netherlands",
    "personnummer": "GDPR — Sweden",
    "ssn": "US — GLBA / state privacy laws",
    "us_passport": "US — Privacy Act",
    "npi": "US — HIPAA",
    "mbi": "US — HIPAA",
    "nhs": "UK GDPR / DPA 2018 (NHS)",
    "rut": "Chile — Ley 21.719",
    "cpf": "Brazil — LGPD",
    "cnpj": "Brazil — legal-entity confidentiality (LGPD-adjacent)",
    "curp": "Mexico — LFPDPPP",
    "arg_dni": "Argentina — Ley 25.326",
    "co_cc": "Colombia — Ley 1581",
    "co_nit": "Colombia — Ley 1581",
    "ec_cedula": "Ecuador — LOPDP",
    "pe_dni": "Peru — Ley 29733",
    "uy_ci": "Uruguay — Ley 18.331",
    "sin": "Canada — PIPEDA",
    "tfn": "Australia — Privacy Act 1988",
    "my_number": "Japan — APPI",
    "rrn": "South Korea — PIPA",
}

REVERSIBLE_FAMILIES = {
    "phone", "credit_card", "ssn", "rut", "cpf", "cnpj", "arg_dni",
    "co_cc", "co_nit", "ec_cedula", "pe_dni", "uy_ci", "npi", "nhs",
    "sin", "tfn", "pesel", "bsn", "personnummer", "my_number", "rrn",
}

_MANIFEST_SCHEMA = {
    "column": pl.String,
    "pii_family": pl.String,
    "match_count": pl.UInt32,
    "regulation": pl.String,
    "mask_mode": pl.String,
}

def _effective_mode(family: str, mode: str) -> str:
    if mode == "asterisk":
        return "asterisk"
    if family in REVERSIBLE_FAMILIES:
        return mode
    return "asterisk"

def masking_manifest(
    df: pl.DataFrame,
    columns: list = None,
    mode: str = "asterisk",
) -> pl.DataFrame:
    """
    Build a masking manifest (RAT / data-processing register) for *df*.

    Scans the requested string columns, counts every PII family detected per
    column, and returns one row per ``(column, pii_family)`` with the match
    count, the governing regulation, and the effective mask mode. This feeds a
    Ley 21.719 / GDPR Article 30 record of processing activities and gives
    auditors per-column evidence of what was masked and why.

    Parameters
    ----------
    df : pl.DataFrame
        The source DataFrame (before or after masking — detection is mode-independent).
    columns : list[str] | None
        Columns to inventory. Defaults to every String column in *df*.
    mode : str
        Masking strategy the manifest documents: ``"asterisk"`` (default),
        ``"consistent"``, or ``"fpe"``. Non-digit families are always asterisked,
        so their ``mask_mode`` stays ``"asterisk"`` regardless of this value.

    Returns
    -------
    pl.DataFrame
        Columns: ``column``, ``pii_family``, ``match_count``, ``regulation``, ``mask_mode``.
        Empty if no PII is found.

    Examples
    --------
    >>> manifest = maskops.masking_manifest(df)
    >>> manifest = maskops.masking_manifest(df, columns=["notes"], mode="fpe")
    >>> maskops.write_manifest(manifest, "rat.json", source="customers.parquet")
    """
    if mode not in ("asterisk", "consistent", "fpe"):
        raise ValueError(f"masking_manifest: unknown mode '{mode}' — use 'asterisk', 'consistent', or 'fpe'")

    if columns is None:
        columns = [name for name, dtype in df.schema.items() if dtype == pl.String]

    rows = []
    for col in columns:
        if col not in df.columns:
            raise ValueError(f"masking_manifest: column '{col}' not found in DataFrame")
        counts = (
            df.select(maskops.mask_pii_audit(col).alias("_audit"))
            .unnest("_audit")
            .unnest("counts")
            .select([pl.col(fam).sum().alias(fam) for fam in PII_FAMILIES])
            .row(0, named=True)
        )
        for fam in PII_FAMILIES:
            total = int(counts[fam] or 0)
            if total > 0:
                rows.append({
                    "column": col,
                    "pii_family": fam,
                    "match_count": total,
                    "regulation": REGULATION[fam],
                    "mask_mode": _effective_mode(fam, mode),
                })

    return pl.DataFrame(rows, schema=_MANIFEST_SCHEMA)

def write_manifest(
    manifest: pl.DataFrame,
    path: PathLike,
    source: str = None,
) -> None:
    """
    Write a masking manifest to a JSON RAT document.

    Wraps the manifest rows in a compliance header (tool version, generation
    timestamp, optional data source, record count) so the file stands alone as
    auditor evidence.

    Parameters
    ----------
    manifest : pl.DataFrame
        A DataFrame produced by :func:`masking_manifest`.
    path : str or Path
        Destination ``.json`` path.
    source : str | None
        Optional identifier for the masked dataset (e.g. a file name or table).
    """
    try:
        from importlib.metadata import version
        maskops_version = version("maskops")
    except Exception:
        maskops_version = "unknown"

    document = {
        "maskops_version": maskops_version,
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "source": source,
        "record_count": manifest.height,
        "records": manifest.to_dicts(),
    }
    Path(path).write_text(json.dumps(document, indent=2, ensure_ascii=False), encoding="utf-8")
