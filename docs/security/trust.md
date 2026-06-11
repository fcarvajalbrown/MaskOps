# MaskOps Security & Trust

This page is the single reference for a vendor security review of MaskOps. It exists to answer the standard questionnaire up front, so an evaluation does not stall for six to twelve weeks. Everything here is verifiable against this repository.

## Architecture is the security model

MaskOps is a native Polars plugin (compiled Rust, loaded in-process). It is not a SaaS and has no server component.

- **Zero network calls.** MaskOps makes no outbound connection at any time — no telemetry, no license check, no model download at runtime. It runs identically in an isolated VPC, on-premise, or on a disconnected host. Data residency and cross-border-transfer questions are answered by the fact that no transfer occurs.
- **No subprocessors.** Nothing leaves the customer perimeter, so there is no subprocessor to assess and no third party in the data path.
- **Client-held keys.** Reversible masking uses FF3-1 AES-256 format-preserving encryption (FF1 also available). The customer generates and holds the 32-byte key and 7-byte tweak and passes them in at call time. MaskOps never stores, logs, or transmits a key, and never co-locates a key with masked data. (FF3-1/FF1 output is pseudonymized, not anonymous — GDPR Art. 4(5).)
- **Irreversible mode is irreversible.** Asterisk masking destroys the original; there is no recovery path, escrow, or backdoor.

## Software Bill of Materials (SBOM)

Published, machine-readable SBOMs in CycloneDX format:

- [`maskops.cdx.json`](./maskops.cdx.json) — the Rust crate and its full transitive dependency tree (the compiled `cdylib` supply chain), every component carrying a PURL and a SHA hash.
- [`maskops-python.cdx.json`](./maskops-python.cdx.json) — the declared Python runtime dependencies (`polars`, `pyyaml`, `tomli` on Python < 3.11).

**Verify it yourself.** The SBOMs are reproducible from source:

```bash
cargo install cargo-cyclonedx
pip install cyclonedx-bom
bash tools/sbom/generate.sh
```

## Cryptography

- Format-preserving encryption: **FF3-1** and **FF1**, both AES-256, NIST SP 800-38G family.
- Key separation is enforced by design: the engine never persists or transmits key material; key custody is entirely the customer's.
- Deterministic (consistent) masking is available without a key for referential integrity, using a keyless hash — documented as pseudonymization, not anonymization.

## Data protection and compliance posture

- **GDPR / data-protection model:** see [`docs/gdpr/gdpr-reference.md`](../gdpr/gdpr-reference.md). Hard rules the codebase never breaks: FPE is pseudonymization (never claimed anonymous); the key is never stored with masked data; asterisk masking has no recovery path; no network calls, ever.
- **Ley 21.719 (Chile):** MaskOps maps to both techniques the law names — anonimización (asterisk, irreversible) and seudonimización (FPE, reversible with the client-held key) — and exports a per-column masking manifest / RAT for the data-processing register.
- **Honest gaps.** MaskOps is **not** SOC 2 or ISO 27001 certified today; both are planned as enterprise-tier signals. We do not claim absolute compliance — MaskOps is a technical measure, and the legal basis for processing remains the customer's. An independent penetration test is scoped (see below). In the interim, the published SBOM, the air-gapped design, and open source code stand in for parts of a third-party attestation.

## Vulnerability disclosure

Coordinated disclosure policy and contact: [`SECURITY.md`](../../SECURITY.md).

## Build and distribution provenance

- Built with `maturin`; wheels published to PyPI via a tag-triggered GitHub Actions workflow.
- Source is public; the compiled artifact is reproducible from the pinned `Cargo.lock` and the SBOM above.

---

*MaskOps v2.0.0. This page describes technical and organizational measures; it is not a legal opinion or a compliance guarantee.*
