use once_cell::sync::Lazy;
use regex::Regex;
use std::net::Ipv6Addr;
use std::str::FromStr;

static IPV4_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})\b").unwrap()
});

fn valid_ipv4(o1: &str, o2: &str, o3: &str, o4: &str) -> bool {
    [o1, o2, o3, o4].iter().all(|o| o.parse::<u32>().map(|n| n <= 255).unwrap_or(false))
}

static IPV6_CANDIDATE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[0-9A-Fa-f:.]+").unwrap()
});

fn boundary_ok(s: &str, start: usize, end: usize) -> bool {
    let before = s[..start].chars().next_back();
    let after = s[end..].chars().next();
    !matches!(before, Some(c) if c.is_ascii_alphanumeric())
        && !matches!(after, Some(c) if c.is_ascii_alphanumeric())
}

fn parse_ipv6_run(run: &str) -> Option<(usize, Ipv6Addr)> {
    if run.bytes().filter(|b| *b == b':').count() < 2 {
        return None;
    }
    if !run.bytes().any(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    if let Ok(addr) = Ipv6Addr::from_str(run) {
        return Some((run.len(), addr));
    }
    let trimmed = run.trim_end_matches('.');
    if trimmed.len() < run.len() {
        if let Ok(addr) = Ipv6Addr::from_str(trimmed) {
            return Some((trimmed.len(), addr));
        }
    }
    None
}

fn find_ipv6_spans(s: &str) -> Vec<(usize, usize, Ipv6Addr)> {
    IPV6_CANDIDATE_RE
        .find_iter(s)
        .filter_map(|m| {
            let (len, addr) = parse_ipv6_run(m.as_str())?;
            let end = m.start() + len;
            if boundary_ok(s, m.start(), end) {
                Some((m.start(), end, addr))
            } else {
                None
            }
        })
        .collect()
}

fn mask_ipv6_addr(addr: &Ipv6Addr) -> String {
    let seg = addr.segments();
    format!("{:x}:{:x}:{:x}:{:x}:****:****:****:****", seg[0], seg[1], seg[2], seg[3])
}

pub fn contains_ip(s: &str) -> bool {
    IPV4_RE.captures_iter(s).any(|c| valid_ipv4(&c[1], &c[2], &c[3], &c[4]))
        || !find_ipv6_spans(s).is_empty()
}

pub fn extract_ip(s: &str) -> Option<String> {
    IPV4_RE.captures_iter(s)
        .find(|c| valid_ipv4(&c[1], &c[2], &c[3], &c[4]))
        .map(|c| c[0].to_string())
        .or_else(|| {
            find_ipv6_spans(s)
                .first()
                .map(|(start, end, _)| s[*start..*end].to_string())
        })
}

pub fn mask_ip_counted(s: &str) -> (String, u32) {
    let spans = find_ipv6_spans(s);
    let (s6, n6) = if spans.is_empty() {
        (s.to_string(), 0u32)
    } else {
        let mut out = String::with_capacity(s.len());
        let mut last = 0usize;
        for (start, end, addr) in &spans {
            out.push_str(&s[last..*start]);
            out.push_str(&mask_ipv6_addr(addr));
            last = *end;
        }
        out.push_str(&s[last..]);
        (out, spans.len() as u32)
    };
    let (s4, n4) = crate::patterns::replace_counted(&IPV4_RE, &s6, |caps: &regex::Captures| {
        if valid_ipv4(&caps[1], &caps[2], &caps[3], &caps[4]) {
            Some(format!("{}.{}.*.*", &caps[1], &caps[2]))
        } else {
            None
        }
    });
    (s4, n4 + n6)
}

pub fn mask_ip(s: &str) -> String {
    mask_ip_counted(s).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compressed_ipv6_masked() {
        let out = mask_ip("addr 2001:db8::1 down");
        assert_eq!(out, "addr 2001:db8:0:0:****:****:****:**** down");
    }

    #[test]
    fn test_link_local_compressed_masked() {
        assert!(!mask_ip("fe80::1").contains("fe80::1"));
    }

    #[test]
    fn test_full_form_ipv6_masked() {
        let out = mask_ip("2001:db8:0:0:1:2:3:4");
        assert_eq!(out, "2001:db8:0:0:****:****:****:****");
    }

    #[test]
    fn test_all_hex_letter_ipv6_detected() {
        assert!(contains_ip("fdab:beef:cafe:face:feed:deaf:beef:fade"));
    }

    #[test]
    fn test_time_not_matched() {
        assert!(!contains_ip("meeting at 12:30:45 today"));
    }

    #[test]
    fn test_mac_address_not_matched() {
        assert!(!contains_ip("mac 00:1A:2B:3C:4D:5E"));
    }

    #[test]
    fn test_rust_path_not_matched() {
        assert_eq!(mask_ip("std::env and Vec::new"), "std::env and Vec::new");
    }

    #[test]
    fn test_version_not_matched() {
        assert_eq!(mask_ip("version 1.2.3"), "version 1.2.3");
    }

    #[test]
    fn test_trailing_period_trimmed() {
        let out = mask_ip("ping fe80::1.");
        assert!(out.ends_with('.'));
        assert!(!out.contains("fe80::1"));
    }

    #[test]
    fn test_ipv4_mapped_masked() {
        assert!(!mask_ip("::ffff:192.168.0.1").contains("192.168.0.1"));
    }

    #[test]
    fn test_extract_compressed() {
        assert_eq!(extract_ip("host 2001:db8::1"), Some("2001:db8::1".to_string()));
    }
}
