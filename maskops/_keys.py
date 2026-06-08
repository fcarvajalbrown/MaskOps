"""maskops FPE key management — validation and deterministic key/tweak derivation.

Compliance category: GDPR Art. 32 (security of processing). All functions are pure and
offline — no network, no entropy source beyond the caller-supplied master secret — so key
material derivation stays inside the client's air-gapped trust boundary. Derivation is
HKDF-SHA256 / HMAC-SHA256 only; MaskOps never persists or transmits the resulting keys.
"""
from __future__ import annotations
import hashlib
import hmac

KEY_LEN = 32
TWEAK_LEN = 7


def validate_key(key: bytes) -> bytes:
    """Validate an FPE key: 32 raw bytes, not a single repeated byte (weak-key guard)."""
    if not isinstance(key, (bytes, bytearray)):
        raise TypeError(f"key must be bytes, got {type(key).__name__}")
    if len(key) != KEY_LEN:
        raise ValueError(f"key must be {KEY_LEN} bytes, got {len(key)}")
    if len(set(key)) == 1:
        raise ValueError("key is a single repeated byte — refusing weak key")
    return bytes(key)


def validate_tweak(tweak: bytes) -> bytes:
    """Validate an FPE tweak: exactly 7 raw bytes."""
    if not isinstance(tweak, (bytes, bytearray)):
        raise TypeError(f"tweak must be bytes, got {type(tweak).__name__}")
    if len(tweak) != TWEAK_LEN:
        raise ValueError(f"tweak must be {TWEAK_LEN} bytes, got {len(tweak)}")
    return bytes(tweak)


def _hkdf(master: bytes, info: bytes, length: int) -> bytes:
    prk = hmac.new(b"maskops-hkdf-salt", master, hashlib.sha256).digest()
    out = b""
    block = b""
    counter = 1
    while len(out) < length:
        block = hmac.new(prk, block + info + bytes([counter]), hashlib.sha256).digest()
        out += block
        counter += 1
    return out[:length]


def derive_key(master: bytes, context: str) -> bytes:
    """Derive a 32-byte FPE key from a master secret and a context label via HKDF-SHA256.

    Deterministic per (master, context): the same pair always yields the same key, so a
    per-tenant or per-dataset key can be regenerated from one stored master secret instead
    of storing every derived key. Different contexts yield independent keys.
    """
    if not isinstance(master, (bytes, bytearray)):
        raise TypeError(f"master must be bytes, got {type(master).__name__}")
    return _hkdf(bytes(master), b"key:" + context.encode("utf-8"), KEY_LEN)


def derive_tweak(master: bytes, context: str) -> bytes:
    """Derive a 7-byte FPE tweak from a master secret and a context label via HMAC-SHA256.

    Use the same master secret as ``derive_key`` with a context that identifies the
    data domain (tenant, table, column). Deterministic and independent of the key.
    """
    if not isinstance(master, (bytes, bytearray)):
        raise TypeError(f"master must be bytes, got {type(master).__name__}")
    mac = hmac.new(bytes(master), b"tweak:" + context.encode("utf-8"), hashlib.sha256)
    return mac.digest()[:TWEAK_LEN]
