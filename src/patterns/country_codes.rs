//! Country code lookup table for phone number identification.
//!
//! Maps E.164 country code prefixes to ISO 3166-1 alpha-2 country names.
//! Longer prefixes take priority over shorter ones (e.g. +56 before +5).
//!
//! To add a new country: add a new entry to COUNTRY_CODES in prefix-length order.

/// Represents a matched country code.
#[derive(Debug, PartialEq)]
pub struct CountryCode {
    pub prefix: &'static str,
    pub country: &'static str,
    pub iso: &'static str,
}

/// E.164 country code table, ordered longest-prefix-first to ensure correct matching.
/// Source: ITU-T E.164 assigned country codes.
pub static COUNTRY_CODES: &[CountryCode] = &[
    // 3-digit prefixes first
    CountryCode { prefix: "+354", country: "Iceland",         iso: "IS" },
    CountryCode { prefix: "+353", country: "Ireland",         iso: "IE" },
    CountryCode { prefix: "+352", country: "Luxembourg",      iso: "LU" },
    CountryCode { prefix: "+351", country: "Portugal",        iso: "PT" },
    CountryCode { prefix: "+358", country: "Finland",         iso: "FI" },
    CountryCode { prefix: "+359", country: "Bulgaria",        iso: "BG" },
    CountryCode { prefix: "+370", country: "Lithuania",       iso: "LT" },
    CountryCode { prefix: "+371", country: "Latvia",          iso: "LV" },
    CountryCode { prefix: "+372", country: "Estonia",         iso: "EE" },
    CountryCode { prefix: "+385", country: "Croatia",         iso: "HR" },
    CountryCode { prefix: "+386", country: "Slovenia",        iso: "SI" },
    CountryCode { prefix: "+420", country: "Czech Republic",  iso: "CZ" },
    CountryCode { prefix: "+421", country: "Slovakia",        iso: "SK" },
    CountryCode { prefix: "+423", country: "Liechtenstein",   iso: "LI" },
    CountryCode { prefix: "+356", country: "Malta",           iso: "MT" },
    CountryCode { prefix: "+357", country: "Cyprus",          iso: "CY" },
    CountryCode { prefix: "+380", country: "Ukraine",         iso: "UA" },
    CountryCode { prefix: "+381", country: "Serbia",          iso: "RS" },
    CountryCode { prefix: "+595", country: "Paraguay",        iso: "PY" },
    CountryCode { prefix: "+598", country: "Uruguay",         iso: "UY" },
    CountryCode { prefix: "+591", country: "Bolivia",         iso: "BO" },
    CountryCode { prefix: "+593", country: "Ecuador",         iso: "EC" },
    CountryCode { prefix: "+502", country: "Guatemala",       iso: "GT" },
    CountryCode { prefix: "+503", country: "El Salvador",     iso: "SV" },
    CountryCode { prefix: "+504", country: "Honduras",        iso: "HN" },
    CountryCode { prefix: "+505", country: "Nicaragua",       iso: "NI" },
    CountryCode { prefix: "+506", country: "Costa Rica",      iso: "CR" },
    CountryCode { prefix: "+507", country: "Panama",          iso: "PA" },
    // 2-digit prefixes
    CountryCode { prefix: "+56",  country: "Chile",           iso: "CL" },
    CountryCode { prefix: "+55",  country: "Brazil",          iso: "BR" },
    CountryCode { prefix: "+54",  country: "Argentina",       iso: "AR" },
    CountryCode { prefix: "+51",  country: "Peru",            iso: "PE" },
    CountryCode { prefix: "+57",  country: "Colombia",        iso: "CO" },
    CountryCode { prefix: "+58",  country: "Venezuela",       iso: "VE" },
    CountryCode { prefix: "+52",  country: "Mexico",          iso: "MX" },
    CountryCode { prefix: "+44",  country: "United Kingdom",  iso: "GB" },
    CountryCode { prefix: "+49",  country: "Germany",         iso: "DE" },
    CountryCode { prefix: "+33",  country: "France",          iso: "FR" },
    CountryCode { prefix: "+34",  country: "Spain",           iso: "ES" },
    CountryCode { prefix: "+39",  country: "Italy",           iso: "IT" },
    CountryCode { prefix: "+31",  country: "Netherlands",     iso: "NL" },
    CountryCode { prefix: "+32",  country: "Belgium",         iso: "BE" },
    CountryCode { prefix: "+30",  country: "Greece",          iso: "GR" },
    CountryCode { prefix: "+36",  country: "Hungary",         iso: "HU" },
    CountryCode { prefix: "+48",  country: "Poland",          iso: "PL" },
    CountryCode { prefix: "+40",  country: "Romania",         iso: "RO" },
    CountryCode { prefix: "+45",  country: "Denmark",         iso: "DK" },
    CountryCode { prefix: "+46",  country: "Sweden",          iso: "SE" },
    CountryCode { prefix: "+47",  country: "Norway",          iso: "NO" },
    CountryCode { prefix: "+41",  country: "Switzerland",     iso: "CH" },
    CountryCode { prefix: "+43",  country: "Austria",         iso: "AT" },
    // 1-digit prefixes last
    CountryCode { prefix: "+1",   country: "USA/Canada",      iso: "US" },
    CountryCode { prefix: "+7",   country: "Russia",          iso: "RU" },
];

/// Returns the CountryCode for a given phone string, or None if unrecognized.
/// Matches longest prefix first.
pub fn identify_country(phone: &str) -> Option<&'static CountryCode> {
    COUNTRY_CODES.iter().find(|cc| phone.starts_with(cc.prefix))
}