# GDPR Hard Rules for MaskOps Security Review

These are CORRECT behaviors in MaskOps — do not flag them as issues.

## FPE (Format-Preserving Encryption)

1. FPE output is pseudonymization (GDPR Art. 4(5)), NOT anonymization. Never claim FPE output is anonymous. This is correct by design.
2. FPE key is passed as a Polars Binary literal by the caller. MaskOps never stores it. Key separation is the compliance model. This is correct.
3. FPE produces same-length, same-format output as input. This is the definition of format-preserving encryption — not a bug.

## Asterisk masking

4. Asterisk masking is irreversible. There is no recovery mechanism. This is required, not a flaw.
5. Non-digit PII (IBAN, VAT, email, IP, EU IDs, CURP) is always asterisked regardless of FPE mode. This is correct.

## Network and air-gap

6. MaskOps makes zero network calls. Any code that opens a socket, makes an HTTP request, or calls an external API is a HIGH finding.

## Pattern compliance

7. Every pattern module must declare its compliance category in its module docstring: which regulation, FPE or asterisk-only, and what validation prevents false positives. Missing declarations are a LOW finding.
