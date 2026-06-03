//! IP address detection and masking.
//!
//! IPv4: masks the last two octets (host portion).
//! IPv6: masks the last 64 bits (interface identifier).
//!
//! Examples:
//!   192.168.1.100  →  192.168.*.*
//!   2001:db8::1    →  2001:db8::****:****:****:****

use once_cell::sync::Lazy;
use regex::Regex;

static IPV4_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{1,3}\.\d{1,3})\.\d{1,3}\.\d{1,3}\b").unwrap()
});

static IPV6_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([0-9a-fA-F]{0,4}:[0-9a-fA-F]{0,4}:[0-9a-fA-F]{0,4}:[0-9a-fA-F]{0,4}):[0-9a-fA-F]{0,4}:[0-9a-fA-F]{0,4}:[0-9a-fA-F]{0,4}:[0-9a-fA-F]{0,4}\b").unwrap()
});

/// Returns true if the input contains an IPv4 or IPv6 address.
pub fn contains_ip(s: &str) -> bool {
    IPV4_RE.is_match(s) || IPV6_RE.is_match(s)
}

/// Masks the host portion of any IP addresses found in the input.
pub fn mask_ip(s: &str) -> String {
    let s = IPV4_RE.replace_all(s, "$1.*.*");
    let s = IPV6_RE.replace_all(&s, "$1:****:****:****:****");
    s.into_owned()
}