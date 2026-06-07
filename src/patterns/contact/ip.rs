

use once_cell::sync::Lazy;
use regex::Regex;

static IPV4_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})\b").unwrap()
});

fn valid_ipv4(o1: &str, o2: &str, o3: &str, o4: &str) -> bool {
    [o1, o2, o3, o4].iter().all(|o| o.parse::<u32>().map(|n| n <= 255).unwrap_or(false))
}

static IPV6_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b([0-9a-fA-F]{0,4}:[0-9a-fA-F]{0,4}:[0-9a-fA-F]{0,4}:[0-9a-fA-F]{0,4}):[0-9a-fA-F]{0,4}:[0-9a-fA-F]{0,4}:[0-9a-fA-F]{0,4}:[0-9a-fA-F]{0,4}\b").unwrap()
});

pub fn contains_ip(s: &str) -> bool {
    IPV4_RE.captures_iter(s).any(|c| valid_ipv4(&c[1], &c[2], &c[3], &c[4]))
        || IPV6_RE.is_match(s)
}

pub fn extract_ip(s: &str) -> Option<String> {
    IPV4_RE.captures_iter(s)
        .find(|c| valid_ipv4(&c[1], &c[2], &c[3], &c[4]))
        .map(|c| c[0].to_string())
        .or_else(|| IPV6_RE.find(s).map(|m| m.as_str().to_string()))
}

pub fn mask_ip_counted(s: &str) -> (String, u32) {
    let (s, n4) = crate::patterns::replace_counted(&IPV4_RE, s, |caps: &regex::Captures| {
        if valid_ipv4(&caps[1], &caps[2], &caps[3], &caps[4]) {
            Some(format!("{}.{}.*.*", &caps[1], &caps[2]))
        } else {
            None
        }
    });
    let (s, n6) = crate::patterns::replace_counted(&IPV6_RE, &s, |caps: &regex::Captures| {
        Some(format!("{}:****:****:****:****", &caps[1]))
    });
    (s, n4 + n6)
}

pub fn mask_ip(s: &str) -> String {
    mask_ip_counted(s).0
}