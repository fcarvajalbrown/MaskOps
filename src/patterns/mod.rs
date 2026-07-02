pub mod eu;
pub mod latam;
pub mod contact;
pub mod financial;
pub mod us;
pub mod healthcare;
pub mod apac;
pub mod mea;
pub mod country_codes;
pub mod fpe;
pub mod fpe_ff1;
pub mod consistent;

use crate::patterns::eu::iban::{mask_iban, contains_iban, extract_iban};
use crate::patterns::eu::vat::{mask_vat, contains_vat, extract_vat};
use crate::patterns::eu::european_id::{
    mask_dni, contains_dni, extract_dni,
    mask_nie, contains_nie, extract_nie,
    mask_nin, contains_nin, extract_nin,
    mask_personalausweis, contains_personalausweis, extract_personalausweis,
};
use crate::patterns::eu::nir::{mask_nir, contains_nir, extract_nir};
use crate::patterns::eu::codice_fiscale::{mask_cf, contains_cf, extract_cf};
use crate::patterns::eu::pesel::{mask_pesel, contains_pesel, extract_pesel, mask_pesel_fpe, mask_pesel_consistent};
use crate::patterns::eu::bsn::{mask_bsn, contains_bsn, extract_bsn, mask_bsn_fpe, mask_bsn_consistent};
use crate::patterns::eu::personnummer::{
    mask_personnummer, contains_personnummer, extract_personnummer,
    mask_personnummer_fpe, mask_personnummer_consistent,
};
use crate::patterns::contact::email::{mask_email, contains_email, extract_email};
use crate::patterns::contact::phone::{mask_phone, contains_phone, extract_phone, mask_phone_fpe};
use crate::patterns::contact::ip::{mask_ip, contains_ip, extract_ip};
use crate::patterns::financial::credit_card::{mask_card, contains_card, extract_card, mask_card_fpe};
use crate::patterns::us::{mask_ssn, contains_ssn, extract_ssn, mask_ssn_fpe, mask_us_passport, contains_us_passport, extract_us_passport};
use crate::patterns::latam::latam_id::{
    mask_rut, contains_rut, extract_rut,
    mask_cpf, contains_cpf, extract_cpf,
    mask_curp, contains_curp, extract_curp,
    mask_rut_fpe, mask_cpf_fpe,
    mask_arg_dni, contains_arg_dni, extract_arg_dni, mask_arg_dni_fpe,
    mask_co_cc, contains_co_cc, extract_co_cc, mask_co_cc_fpe,
    mask_co_nit, contains_co_nit, extract_co_nit, mask_co_nit_fpe,
};
use crate::patterns::latam::{
    contains_ec_cedula, mask_ec_cedula, extract_ec_cedula, mask_ec_cedula_fpe,
    contains_pe_dni, mask_pe_dni, extract_pe_dni, mask_pe_dni_fpe,
    contains_pe_dni_bare, mask_pe_dni_bare, extract_pe_dni_bare,
    mask_pe_dni_bare_fpe, mask_pe_dni_bare_consistent,
    contains_uy_ci, mask_uy_ci, extract_uy_ci, mask_uy_ci_fpe, mask_uy_ci_consistent,
    contains_cnpj, mask_cnpj, extract_cnpj, mask_cnpj_fpe, mask_cnpj_consistent,
};
use crate::patterns::healthcare::{
    contains_npi, mask_npi, extract_npi, mask_npi_fpe,
    contains_mbi, mask_mbi, extract_mbi,
    contains_nhs, mask_nhs, extract_nhs, mask_nhs_fpe,
};
use crate::patterns::apac::{
    contains_sin, mask_sin, extract_sin, mask_sin_fpe, mask_sin_consistent,
    contains_tfn, mask_tfn, extract_tfn, mask_tfn_fpe, mask_tfn_consistent,
    contains_my_number, mask_my_number, extract_my_number, mask_my_number_fpe, mask_my_number_consistent,
    contains_rrn, mask_rrn, extract_rrn, mask_rrn_fpe, mask_rrn_consistent,
};
use crate::patterns::mea::{
    contains_za_id, mask_za_id, extract_za_id, mask_za_id_fpe, mask_za_id_consistent,
    contains_il_id, mask_il_id, extract_il_id, mask_il_id_fpe, mask_il_id_consistent,
};
pub use crate::patterns::fpe::{Ff3Cipher, FpeCipher, KEY_LEN, TWEAK_LEN};
pub use crate::patterns::fpe_ff1::Ff1Cipher;
pub use crate::patterns::consistent::ConsistentHasher;
use crate::patterns::contact::phone::mask_phone_consistent;
use crate::patterns::financial::credit_card::mask_card_consistent;
use crate::patterns::latam::latam_id::{
    mask_rut_consistent, mask_cpf_consistent,
    mask_arg_dni_consistent, mask_co_cc_consistent, mask_co_nit_consistent,
};
use crate::patterns::latam::{mask_ec_cedula_consistent, mask_pe_dni_consistent};
use crate::patterns::us::mask_ssn_consistent;
use crate::patterns::healthcare::{mask_npi_consistent, mask_nhs_consistent};

use crate::patterns::eu::iban::mask_iban_counted;
use crate::patterns::eu::vat::mask_vat_counted;
use crate::patterns::eu::nir::mask_nir_counted;
use crate::patterns::eu::codice_fiscale::mask_cf_counted;
use crate::patterns::eu::pesel::mask_pesel_counted;
use crate::patterns::eu::bsn::mask_bsn_counted;
use crate::patterns::eu::personnummer::mask_personnummer_counted;
use crate::patterns::eu::european_id::{
    mask_dni_counted, mask_nie_counted, mask_nin_counted, mask_personalausweis_counted,
};
use crate::patterns::contact::email::mask_email_counted;
use crate::patterns::contact::ip::mask_ip_counted;
use crate::patterns::contact::phone::mask_phone_counted;
use crate::patterns::financial::credit_card::mask_card_counted;
use crate::patterns::us::ssn::mask_ssn_counted;
use crate::patterns::us::passport::mask_us_passport_counted;
use crate::patterns::healthcare::mbi::mask_mbi_counted;
use crate::patterns::healthcare::npi::mask_npi_counted;
use crate::patterns::healthcare::nhs::mask_nhs_counted;
use crate::patterns::latam::latam_id::{
    mask_rut_counted, mask_cpf_counted, mask_curp_counted,
    mask_arg_dni_counted, mask_co_cc_counted, mask_co_nit_counted,
};
use crate::patterns::latam::ecuador::mask_ec_cedula_counted;
use crate::patterns::latam::peru::{mask_pe_dni_counted, mask_pe_dni_bare_counted};
use crate::patterns::latam::uruguay::mask_uy_ci_counted;
use crate::patterns::latam::brazil_cnpj::mask_cnpj_counted;
use crate::patterns::apac::canada_sin::mask_sin_counted;
use crate::patterns::apac::australia_tfn::mask_tfn_counted;
use crate::patterns::apac::japan_my_number::mask_my_number_counted;
use crate::patterns::apac::korea_rrn::mask_rrn_counted;
use crate::patterns::mea::south_africa::mask_za_id_counted;
use crate::patterns::mea::israel::mask_il_id_counted;

pub fn replace_counted<F>(re: &regex::Regex, s: &str, render: F) -> (String, u32)
where
    F: Fn(&regex::Captures) -> Option<String>,
{
    let count = std::cell::Cell::new(0u32);
    let out = re
        .replace_all(s, |caps: &regex::Captures| match render(caps) {
            Some(masked) => {
                count.set(count.get() + 1);
                masked
            }
            None => caps[0].to_string(),
        })
        .into_owned();
    (out, count.get())
}

#[inline]
pub fn has_pii_candidate(value: &str) -> bool {
    value.bytes().any(|b| b.is_ascii_digit() || b == b'@')
}

#[inline]
pub fn has_letter_only_ipv6_candidate(value: &str) -> bool {
    value.bytes().filter(|b| *b == b':').count() >= 2
}

pub fn mask_non_digit(value: &str) -> String {
    let s = mask_iban(value);
    let s = mask_vat(&s);
    let s = mask_email(&s);
    let s = mask_ip(&s);
    let s = mask_curp(&s);
    let s = mask_dni(&s);
    let s = mask_nie(&s);
    let s = mask_nin(&s);
    let s = mask_personalausweis(&s);
    let s = mask_us_passport(&s);
    let s = mask_mbi(&s);
    let s = mask_nir(&s);
    let s = mask_cf(&s);
    s
}

pub fn mask_digit(value: &str) -> String {
    let s = mask_phone(value);
    let s = mask_rut(&s);
    let s = mask_cpf(&s);
    let s = mask_cnpj(&s);
    let s = mask_card(&s);
    let s = mask_ssn(&s);
    let s = mask_arg_dni(&s);
    let s = mask_co_cc(&s);
    let s = mask_co_nit(&s);
    let s = mask_personnummer(&s);
    let s = mask_ec_cedula(&s);
    let s = mask_pe_dni(&s);
    let s = mask_npi(&s);
    let s = mask_nhs(&s);
    let s = mask_uy_ci(&s);
    let s = mask_sin(&s);
    let s = mask_tfn(&s);
    let s = mask_pesel(&s);
    let s = mask_bsn(&s);
    let s = mask_my_number(&s);
    let s = mask_rrn(&s);
    let s = mask_za_id(&s);
    let s = mask_il_id(&s);
    s
}

pub fn mask_digit_fpe(value: &str, cipher: &FpeCipher) -> String {
    let s = mask_phone_fpe(value, cipher);
    let s = mask_rut_fpe(&s, cipher);
    let s = mask_cpf_fpe(&s, cipher);
    let s = mask_cnpj_fpe(&s, cipher);
    let s = mask_card_fpe(&s, cipher);
    let s = mask_ssn_fpe(&s, cipher);
    let s = mask_arg_dni_fpe(&s, cipher);
    let s = mask_co_cc_fpe(&s, cipher);
    let s = mask_co_nit_fpe(&s, cipher);
    let s = mask_personnummer_fpe(&s, cipher);
    let s = mask_ec_cedula_fpe(&s, cipher);
    let s = mask_pe_dni_fpe(&s, cipher);
    let s = mask_npi_fpe(&s, cipher);
    let s = mask_nhs_fpe(&s, cipher);
    let s = mask_uy_ci_fpe(&s, cipher);
    let s = mask_sin_fpe(&s, cipher);
    let s = mask_tfn_fpe(&s, cipher);
    let s = mask_pesel_fpe(&s, cipher);
    let s = mask_bsn_fpe(&s, cipher);
    let s = mask_my_number_fpe(&s, cipher);
    let s = mask_rrn_fpe(&s, cipher);
    let s = mask_za_id_fpe(&s, cipher);
    let s = mask_il_id_fpe(&s, cipher);
    s
}

pub fn mask_all(value: &str) -> String {
    if !has_pii_candidate(value) {
        if has_letter_only_ipv6_candidate(value) {
            return mask_ip(value);
        }
        return value.to_string();
    }
    let s = mask_non_digit(value);
    mask_digit(&s)
}

pub fn mask_all_fpe(value: &str, cipher: &FpeCipher) -> String {
    if !has_pii_candidate(value) {
        if has_letter_only_ipv6_candidate(value) {
            return mask_ip(value);
        }
        return value.to_string();
    }
    let s = mask_non_digit(value);
    mask_digit_fpe(&s, cipher)
}

pub fn mask_digit_consistent(value: &str, hasher: &ConsistentHasher) -> String {
    let s = mask_phone_consistent(value, hasher);
    let s = mask_rut_consistent(&s, hasher);
    let s = mask_cpf_consistent(&s, hasher);
    let s = mask_cnpj_consistent(&s, hasher);
    let s = mask_card_consistent(&s, hasher);
    let s = mask_ssn_consistent(&s, hasher);
    let s = mask_arg_dni_consistent(&s, hasher);
    let s = mask_co_cc_consistent(&s, hasher);
    let s = mask_co_nit_consistent(&s, hasher);
    let s = mask_personnummer_consistent(&s, hasher);
    let s = mask_ec_cedula_consistent(&s, hasher);
    let s = mask_pe_dni_consistent(&s, hasher);
    let s = mask_npi_consistent(&s, hasher);
    let s = mask_nhs_consistent(&s, hasher);
    let s = mask_uy_ci_consistent(&s, hasher);
    let s = mask_sin_consistent(&s, hasher);
    let s = mask_tfn_consistent(&s, hasher);
    let s = mask_pesel_consistent(&s, hasher);
    let s = mask_bsn_consistent(&s, hasher);
    let s = mask_my_number_consistent(&s, hasher);
    let s = mask_rrn_consistent(&s, hasher);
    let s = mask_za_id_consistent(&s, hasher);
    let s = mask_il_id_consistent(&s, hasher);
    s
}

pub fn mask_all_consistent(value: &str, hasher: &ConsistentHasher) -> String {
    if !has_pii_candidate(value) {
        if has_letter_only_ipv6_candidate(value) {
            return mask_ip(value);
        }
        return value.to_string();
    }
    let s = mask_non_digit(value);
    mask_digit_consistent(&s, hasher)
}

pub fn mask_all_selected(value: &str, patterns: &[&str]) -> String {
    if !has_pii_candidate(value) {
        if patterns.contains(&"ip") && has_letter_only_ipv6_candidate(value) {
            return mask_ip(value);
        }
        return value.to_string();
    }
    let mut s = value.to_string();
    for pat in patterns {
        s = match *pat {
            "email"           => mask_email(&s),
            "phone"           => mask_phone(&s),
            "ip"              => mask_ip(&s),
            "iban"            => mask_iban(&s),
            "vat"             => mask_vat(&s),
            "dni"             => mask_dni(&s),
            "nie"             => mask_nie(&s),
            "nin"             => mask_nin(&s),
            "personalausweis" => mask_personalausweis(&s),
            "us_passport"     => mask_us_passport(&s),
            "curp"            => mask_curp(&s),
            "rut"             => mask_rut(&s),
            "cpf"             => mask_cpf(&s),
            "cnpj"            => mask_cnpj(&s),
            "ssn"             => mask_ssn(&s),
            "arg_dni"         => mask_arg_dni(&s),
            "co_cc"           => mask_co_cc(&s),
            "co_nit"          => mask_co_nit(&s),
            "ec_cedula"       => mask_ec_cedula(&s),
            "credit_card"     => mask_card(&s),
            "npi"             => mask_npi(&s),
            "mbi"             => mask_mbi(&s),
            "nhs"             => mask_nhs(&s),
            "pe_dni"          => mask_pe_dni_bare(&s),
            "nir"             => mask_nir(&s),
            "codice_fiscale"  => mask_cf(&s),
            "uy_ci"           => mask_uy_ci(&s),
            "sin"             => mask_sin(&s),
            "tfn"             => mask_tfn(&s),
            "pesel"           => mask_pesel(&s),
            "bsn"             => mask_bsn(&s),
            "personnummer"    => mask_personnummer(&s),
            "my_number"       => mask_my_number(&s),
            "rrn"             => mask_rrn(&s),
            "za_id"           => mask_za_id(&s),
            "il_id"           => mask_il_id(&s),
            _                 => s,
        };
    }
    s
}

pub fn mask_all_selected_fpe(value: &str, patterns: &[&str], cipher: &FpeCipher) -> String {
    if !has_pii_candidate(value) {
        if patterns.contains(&"ip") && has_letter_only_ipv6_candidate(value) {
            return mask_ip(value);
        }
        return value.to_string();
    }
    let mut s = value.to_string();
    for pat in patterns {
        s = match *pat {
            "email"           => mask_email(&s),
            "phone"           => mask_phone_fpe(&s, cipher),
            "ip"              => mask_ip(&s),
            "iban"            => mask_iban(&s),
            "vat"             => mask_vat(&s),
            "dni"             => mask_dni(&s),
            "nie"             => mask_nie(&s),
            "nin"             => mask_nin(&s),
            "personalausweis" => mask_personalausweis(&s),
            "us_passport"     => mask_us_passport(&s),
            "curp"            => mask_curp(&s),
            "rut"             => mask_rut_fpe(&s, cipher),
            "cpf"             => mask_cpf_fpe(&s, cipher),
            "cnpj"            => mask_cnpj_fpe(&s, cipher),
            "ssn"             => mask_ssn_fpe(&s, cipher),
            "arg_dni"         => mask_arg_dni_fpe(&s, cipher),
            "co_cc"           => mask_co_cc_fpe(&s, cipher),
            "co_nit"          => mask_co_nit_fpe(&s, cipher),
            "ec_cedula"       => mask_ec_cedula_fpe(&s, cipher),
            "credit_card"     => mask_card_fpe(&s, cipher),
            "npi"             => mask_npi_fpe(&s, cipher),
            "mbi"             => mask_mbi(&s),  
            "nhs"             => mask_nhs_fpe(&s, cipher),
            "pe_dni"          => mask_pe_dni_bare_fpe(&s, cipher),
            "nir"             => mask_nir(&s),       
            "codice_fiscale"  => mask_cf(&s),        
            "uy_ci"           => mask_uy_ci_fpe(&s, cipher),
            "sin"             => mask_sin_fpe(&s, cipher),
            "tfn"             => mask_tfn_fpe(&s, cipher),
            "pesel"           => mask_pesel_fpe(&s, cipher),
            "bsn"             => mask_bsn_fpe(&s, cipher),
            "personnummer"    => mask_personnummer_fpe(&s, cipher),
            "my_number"       => mask_my_number_fpe(&s, cipher),
            "rrn"             => mask_rrn_fpe(&s, cipher),
            "za_id"           => mask_za_id_fpe(&s, cipher),
            "il_id"           => mask_il_id_fpe(&s, cipher),
            _                 => s,
        };
    }
    s
}

pub fn mask_all_selected_consistent(value: &str, patterns: &[&str], hasher: &ConsistentHasher) -> String {
    if !has_pii_candidate(value) {
        if patterns.contains(&"ip") && has_letter_only_ipv6_candidate(value) {
            return mask_ip(value);
        }
        return value.to_string();
    }
    let mut s = value.to_string();
    for pat in patterns {
        s = match *pat {
            "email"           => mask_email(&s),
            "phone"           => mask_phone_consistent(&s, hasher),
            "ip"              => mask_ip(&s),
            "iban"            => mask_iban(&s),
            "vat"             => mask_vat(&s),
            "dni"             => mask_dni(&s),
            "nie"             => mask_nie(&s),
            "nin"             => mask_nin(&s),
            "personalausweis" => mask_personalausweis(&s),
            "us_passport"     => mask_us_passport(&s),
            "curp"            => mask_curp(&s),
            "rut"             => mask_rut_consistent(&s, hasher),
            "cpf"             => mask_cpf_consistent(&s, hasher),
            "cnpj"            => mask_cnpj_consistent(&s, hasher),
            "ssn"             => mask_ssn_consistent(&s, hasher),
            "arg_dni"         => mask_arg_dni_consistent(&s, hasher),
            "co_cc"           => mask_co_cc_consistent(&s, hasher),
            "co_nit"          => mask_co_nit_consistent(&s, hasher),
            "ec_cedula"       => mask_ec_cedula_consistent(&s, hasher),
            "credit_card"     => mask_card_consistent(&s, hasher),
            "npi"             => mask_npi_consistent(&s, hasher),
            "mbi"             => mask_mbi(&s),
            "nhs"             => mask_nhs_consistent(&s, hasher),
            "pe_dni"          => mask_pe_dni_bare_consistent(&s, hasher),
            "nir"             => mask_nir(&s),       
            "codice_fiscale"  => mask_cf(&s),        
            "uy_ci"           => mask_uy_ci_consistent(&s, hasher),
            "sin"             => mask_sin_consistent(&s, hasher),
            "tfn"             => mask_tfn_consistent(&s, hasher),
            "pesel"           => mask_pesel_consistent(&s, hasher),
            "bsn"             => mask_bsn_consistent(&s, hasher),
            "personnummer"    => mask_personnummer_consistent(&s, hasher),
            "my_number"       => mask_my_number_consistent(&s, hasher),
            "rrn"             => mask_rrn_consistent(&s, hasher),
            "za_id"           => mask_za_id_consistent(&s, hasher),
            "il_id"           => mask_il_id_consistent(&s, hasher),
            _                 => s,
        };
    }
    s
}

pub fn contains_any_selected(value: &str, patterns: &[&str]) -> bool {
    if !has_pii_candidate(value) {
        return patterns.contains(&"ip")
            && has_letter_only_ipv6_candidate(value)
            && contains_ip(value);
    }
    patterns.iter().any(|pat| match *pat {
        "email"           => contains_email(value),
        "phone"           => contains_phone(value),
        "ip"              => contains_ip(value),
        "iban"            => contains_iban(value),
        "vat"             => contains_vat(value),
        "dni"             => contains_dni(value),
        "nie"             => contains_nie(value),
        "nin"             => contains_nin(value),
        "personalausweis" => contains_personalausweis(value),
        "us_passport"     => contains_us_passport(value),
        "curp"            => contains_curp(value),
        "rut"             => contains_rut(value),
        "cpf"             => contains_cpf(value),
        "cnpj"            => contains_cnpj(value),
        "ssn"             => contains_ssn(value),
        "arg_dni"         => contains_arg_dni(value),
        "co_cc"           => contains_co_cc(value),
        "co_nit"          => contains_co_nit(value),
        "ec_cedula"       => contains_ec_cedula(value),
        "credit_card"     => contains_card(value),
        "npi"             => contains_npi(value),
        "mbi"             => contains_mbi(value),
        "nhs"             => contains_nhs(value),
        "pe_dni"          => contains_pe_dni_bare(value),
        "nir"             => contains_nir(value),
        "codice_fiscale"  => contains_cf(value),
        "uy_ci"           => contains_uy_ci(value),
        "sin"             => contains_sin(value),
        "tfn"             => contains_tfn(value),
        "pesel"           => contains_pesel(value),
        "bsn"             => contains_bsn(value),
        "personnummer"    => contains_personnummer(value),
        "my_number"       => contains_my_number(value),
        "rrn"             => contains_rrn(value),
        "za_id"           => contains_za_id(value),
        "il_id"           => contains_il_id(value),
        _                 => false,
    })
}

pub fn contains_any_pii(value: &str) -> bool {
    if !has_pii_candidate(value) {
        return has_letter_only_ipv6_candidate(value) && contains_ip(value);
    }
    contains_iban(value)
        || contains_vat(value)
        || contains_email(value)
        || contains_phone(value)
        || contains_ip(value)
        || contains_rut(value)
        || contains_cpf(value)
        || contains_cnpj(value)
        || contains_curp(value)
        || contains_card(value)
        || contains_dni(value)
        || contains_nie(value)
        || contains_nin(value)
        || contains_personalausweis(value)
        || contains_ssn(value)
        || contains_us_passport(value)
        || contains_arg_dni(value)
        || contains_co_cc(value)
        || contains_co_nit(value)
        || contains_ec_cedula(value)
        || contains_pe_dni(value)
        || contains_npi(value)
        || contains_mbi(value)
        || contains_nhs(value)
        || contains_nir(value)
        || contains_cf(value)
        || contains_uy_ci(value)
        || contains_sin(value)
        || contains_tfn(value)
        || contains_pesel(value)
        || contains_bsn(value)
        || contains_personnummer(value)
        || contains_my_number(value)
        || contains_rrn(value)
        || contains_za_id(value)
        || contains_il_id(value)
}

#[derive(Default)]
pub struct ExtractResult {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub ip: Option<String>,
    pub iban: Option<String>,
    pub vat: Option<String>,
    pub dni: Option<String>,
    pub nie: Option<String>,
    pub nin: Option<String>,
    pub personalausweis: Option<String>,
    pub nir: Option<String>,
    pub codice_fiscale: Option<String>,
    pub pesel: Option<String>,
    pub bsn: Option<String>,
    pub personnummer: Option<String>,
    pub credit_card: Option<String>,
    pub ssn: Option<String>,
    pub us_passport: Option<String>,
    pub rut: Option<String>,
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub curp: Option<String>,
    pub arg_dni: Option<String>,
    pub co_cc: Option<String>,
    pub co_nit: Option<String>,
    pub ec_cedula: Option<String>,
    pub pe_dni: Option<String>,
    pub uy_ci: Option<String>,
    pub npi: Option<String>,
    pub mbi: Option<String>,
    pub nhs: Option<String>,
    pub sin: Option<String>,
    pub tfn: Option<String>,
    pub my_number: Option<String>,
    pub rrn: Option<String>,
    pub za_id: Option<String>,
    pub il_id: Option<String>,
}

pub fn extract_all(value: &str) -> ExtractResult {
    if !has_pii_candidate(value) {
        if has_letter_only_ipv6_candidate(value) {
            return ExtractResult { ip: extract_ip(value), ..ExtractResult::default() };
        }
        return ExtractResult::default();
    }
    ExtractResult {
        email:          extract_email(value),
        phone:          extract_phone(value),
        ip:             extract_ip(value),
        iban:           extract_iban(value),
        vat:            extract_vat(value),
        dni:            extract_dni(value),
        nie:            extract_nie(value),
        nin:            extract_nin(value),
        personalausweis: extract_personalausweis(value),
        nir:            extract_nir(value),
        codice_fiscale: extract_cf(value),
        pesel:          extract_pesel(value),
        bsn:            extract_bsn(value),
        personnummer:   extract_personnummer(value),
        credit_card:    extract_card(value),
        ssn:            extract_ssn(value),
        us_passport:    extract_us_passport(value),
        rut:            extract_rut(value),
        cpf:            extract_cpf(value),
        cnpj:           extract_cnpj(value),
        curp:           extract_curp(value),
        arg_dni:        extract_arg_dni(value),
        co_cc:          extract_co_cc(value),
        co_nit:         extract_co_nit(value),
        ec_cedula:      extract_ec_cedula(value),
        pe_dni:         extract_pe_dni(value),
        uy_ci:          extract_uy_ci(value),
        npi:            extract_npi(value),
        mbi:            extract_mbi(value),
        nhs:            extract_nhs(value),
        sin:            extract_sin(value),
        tfn:            extract_tfn(value),
        my_number:      extract_my_number(value),
        rrn:            extract_rrn(value),
        za_id:          extract_za_id(value),
        il_id:          extract_il_id(value),
    }
}

#[derive(Default)]
pub struct AuditCounts {
    pub email: u32,
    pub phone: u32,
    pub ip: u32,
    pub iban: u32,
    pub vat: u32,
    pub dni: u32,
    pub nie: u32,
    pub nin: u32,
    pub personalausweis: u32,
    pub nir: u32,
    pub codice_fiscale: u32,
    pub pesel: u32,
    pub bsn: u32,
    pub personnummer: u32,
    pub credit_card: u32,
    pub ssn: u32,
    pub us_passport: u32,
    pub rut: u32,
    pub cpf: u32,
    pub cnpj: u32,
    pub curp: u32,
    pub arg_dni: u32,
    pub co_cc: u32,
    pub co_nit: u32,
    pub ec_cedula: u32,
    pub pe_dni: u32,
    pub uy_ci: u32,
    pub npi: u32,
    pub mbi: u32,
    pub nhs: u32,
    pub sin: u32,
    pub tfn: u32,
    pub my_number: u32,
    pub rrn: u32,
    pub za_id: u32,
    pub il_id: u32,
}

pub fn mask_all_audit(value: &str) -> (String, AuditCounts) {
    if !has_pii_candidate(value) {
        if has_letter_only_ipv6_candidate(value) {
            let (s, n) = mask_ip_counted(value);
            return (s, AuditCounts { ip: n, ..AuditCounts::default() });
        }
        return (value.to_string(), AuditCounts::default());
    }
    let mut c = AuditCounts::default();

    let (s, n) = mask_iban_counted(value);         c.iban = n;
    let (s, n) = mask_vat_counted(&s);             c.vat = n;
    let (s, n) = mask_email_counted(&s);           c.email = n;
    let (s, n) = mask_ip_counted(&s);              c.ip = n;
    let (s, n) = mask_curp_counted(&s);            c.curp = n;
    let (s, n) = mask_dni_counted(&s);             c.dni = n;
    let (s, n) = mask_nie_counted(&s);             c.nie = n;
    let (s, n) = mask_nin_counted(&s);             c.nin = n;
    let (s, n) = mask_personalausweis_counted(&s); c.personalausweis = n;
    let (s, n) = mask_us_passport_counted(&s);     c.us_passport = n;
    let (s, n) = mask_mbi_counted(&s);             c.mbi = n;
    let (s, n) = mask_nir_counted(&s);             c.nir = n;
    let (s, n) = mask_cf_counted(&s);              c.codice_fiscale = n;

    let (s, n) = mask_phone_counted(&s);           c.phone = n;
    let (s, n) = mask_rut_counted(&s);             c.rut = n;
    let (s, n) = mask_cpf_counted(&s);             c.cpf = n;
    let (s, n) = mask_cnpj_counted(&s);            c.cnpj = n;
    let (s, n) = mask_card_counted(&s);            c.credit_card = n;
    let (s, n) = mask_ssn_counted(&s);             c.ssn = n;
    let (s, n) = mask_arg_dni_counted(&s);         c.arg_dni = n;
    let (s, n) = mask_co_cc_counted(&s);           c.co_cc = n;
    let (s, n) = mask_co_nit_counted(&s);          c.co_nit = n;
    let (s, n) = mask_personnummer_counted(&s);    c.personnummer = n;
    let (s, n) = mask_ec_cedula_counted(&s);       c.ec_cedula = n;
    let (s, n) = mask_pe_dni_counted(&s);          c.pe_dni = n;
    let (s, n) = mask_npi_counted(&s);             c.npi = n;
    let (s, n) = mask_nhs_counted(&s);             c.nhs = n;
    let (s, n) = mask_uy_ci_counted(&s);           c.uy_ci = n;
    let (s, n) = mask_sin_counted(&s);             c.sin = n;
    let (s, n) = mask_tfn_counted(&s);             c.tfn = n;
    let (s, n) = mask_pesel_counted(&s);           c.pesel = n;
    let (s, n) = mask_bsn_counted(&s);             c.bsn = n;
    let (s, n) = mask_my_number_counted(&s);       c.my_number = n;
    let (s, n) = mask_rrn_counted(&s);             c.rrn = n;
    let (s, n) = mask_za_id_counted(&s);           c.za_id = n;
    let (s, n) = mask_il_id_counted(&s);           c.il_id = n;

    (s, c)
}

pub fn mask_all_audit_selected(value: &str, patterns: &[&str]) -> (String, AuditCounts) {
    if !has_pii_candidate(value) {
        if patterns.contains(&"ip") && has_letter_only_ipv6_candidate(value) {
            let (s, n) = mask_ip_counted(value);
            return (s, AuditCounts { ip: n, ..AuditCounts::default() });
        }
        return (value.to_string(), AuditCounts::default());
    }
    let sel = |name: &str| patterns.contains(&name);
    let mut c = AuditCounts::default();
    let mut s = value.to_string();

    if sel("iban")            { let (ns, n) = mask_iban_counted(&s);            s = ns; c.iban = n; }
    if sel("vat")             { let (ns, n) = mask_vat_counted(&s);             s = ns; c.vat = n; }
    if sel("email")           { let (ns, n) = mask_email_counted(&s);           s = ns; c.email = n; }
    if sel("ip")              { let (ns, n) = mask_ip_counted(&s);              s = ns; c.ip = n; }
    if sel("curp")            { let (ns, n) = mask_curp_counted(&s);            s = ns; c.curp = n; }
    if sel("dni")             { let (ns, n) = mask_dni_counted(&s);             s = ns; c.dni = n; }
    if sel("nie")             { let (ns, n) = mask_nie_counted(&s);             s = ns; c.nie = n; }
    if sel("nin")             { let (ns, n) = mask_nin_counted(&s);             s = ns; c.nin = n; }
    if sel("personalausweis") { let (ns, n) = mask_personalausweis_counted(&s); s = ns; c.personalausweis = n; }
    if sel("us_passport")     { let (ns, n) = mask_us_passport_counted(&s);     s = ns; c.us_passport = n; }
    if sel("mbi")             { let (ns, n) = mask_mbi_counted(&s);             s = ns; c.mbi = n; }
    if sel("nir")             { let (ns, n) = mask_nir_counted(&s);             s = ns; c.nir = n; }
    if sel("codice_fiscale")  { let (ns, n) = mask_cf_counted(&s);              s = ns; c.codice_fiscale = n; }

    if sel("phone")           { let (ns, n) = mask_phone_counted(&s);           s = ns; c.phone = n; }
    if sel("rut")             { let (ns, n) = mask_rut_counted(&s);             s = ns; c.rut = n; }
    if sel("cpf")             { let (ns, n) = mask_cpf_counted(&s);             s = ns; c.cpf = n; }
    if sel("cnpj")            { let (ns, n) = mask_cnpj_counted(&s);            s = ns; c.cnpj = n; }
    if sel("credit_card")     { let (ns, n) = mask_card_counted(&s);            s = ns; c.credit_card = n; }
    if sel("ssn")             { let (ns, n) = mask_ssn_counted(&s);             s = ns; c.ssn = n; }
    if sel("arg_dni")         { let (ns, n) = mask_arg_dni_counted(&s);         s = ns; c.arg_dni = n; }
    if sel("co_cc")           { let (ns, n) = mask_co_cc_counted(&s);           s = ns; c.co_cc = n; }
    if sel("co_nit")          { let (ns, n) = mask_co_nit_counted(&s);          s = ns; c.co_nit = n; }
    if sel("personnummer")    { let (ns, n) = mask_personnummer_counted(&s);    s = ns; c.personnummer = n; }
    if sel("ec_cedula")       { let (ns, n) = mask_ec_cedula_counted(&s);       s = ns; c.ec_cedula = n; }
    if sel("pe_dni")          { let (ns, n) = mask_pe_dni_bare_counted(&s);     s = ns; c.pe_dni = n; }
    if sel("npi")             { let (ns, n) = mask_npi_counted(&s);             s = ns; c.npi = n; }
    if sel("nhs")             { let (ns, n) = mask_nhs_counted(&s);             s = ns; c.nhs = n; }
    if sel("uy_ci")           { let (ns, n) = mask_uy_ci_counted(&s);           s = ns; c.uy_ci = n; }
    if sel("sin")             { let (ns, n) = mask_sin_counted(&s);             s = ns; c.sin = n; }
    if sel("tfn")             { let (ns, n) = mask_tfn_counted(&s);             s = ns; c.tfn = n; }
    if sel("pesel")           { let (ns, n) = mask_pesel_counted(&s);           s = ns; c.pesel = n; }
    if sel("bsn")             { let (ns, n) = mask_bsn_counted(&s);             s = ns; c.bsn = n; }
    if sel("my_number")       { let (ns, n) = mask_my_number_counted(&s);       s = ns; c.my_number = n; }
    if sel("rrn")             { let (ns, n) = mask_rrn_counted(&s);             s = ns; c.rrn = n; }
    if sel("za_id")           { let (ns, n) = mask_za_id_counted(&s);           s = ns; c.za_id = n; }
    if sel("il_id")           { let (ns, n) = mask_il_id_counted(&s);           s = ns; c.il_id = n; }

    (s, c)
}

pub fn extract_all_selected(value: &str, patterns: &[&str]) -> ExtractResult {
    if !has_pii_candidate(value) {
        if patterns.contains(&"ip") && has_letter_only_ipv6_candidate(value) {
            return ExtractResult { ip: extract_ip(value), ..ExtractResult::default() };
        }
        return ExtractResult::default();
    }
    let sel = |name: &str| patterns.contains(&name);
    let pick = |name: &str, f: &dyn Fn(&str) -> Option<String>| if sel(name) { f(value) } else { None };
    ExtractResult {
        email:           pick("email", &extract_email),
        phone:           pick("phone", &extract_phone),
        ip:              pick("ip", &extract_ip),
        iban:            pick("iban", &extract_iban),
        vat:             pick("vat", &extract_vat),
        dni:             pick("dni", &extract_dni),
        nie:             pick("nie", &extract_nie),
        nin:             pick("nin", &extract_nin),
        personalausweis: pick("personalausweis", &extract_personalausweis),
        nir:             pick("nir", &extract_nir),
        codice_fiscale:  pick("codice_fiscale", &extract_cf),
        pesel:           pick("pesel", &extract_pesel),
        bsn:             pick("bsn", &extract_bsn),
        personnummer:    pick("personnummer", &extract_personnummer),
        credit_card:     pick("credit_card", &extract_card),
        ssn:             pick("ssn", &extract_ssn),
        us_passport:     pick("us_passport", &extract_us_passport),
        rut:             pick("rut", &extract_rut),
        cpf:             pick("cpf", &extract_cpf),
        cnpj:            pick("cnpj", &extract_cnpj),
        curp:            pick("curp", &extract_curp),
        arg_dni:         pick("arg_dni", &extract_arg_dni),
        co_cc:           pick("co_cc", &extract_co_cc),
        co_nit:          pick("co_nit", &extract_co_nit),
        ec_cedula:       pick("ec_cedula", &extract_ec_cedula),
        pe_dni:          pick("pe_dni", &extract_pe_dni_bare),
        uy_ci:           pick("uy_ci", &extract_uy_ci),
        npi:             pick("npi", &extract_npi),
        mbi:             pick("mbi", &extract_mbi),
        nhs:             pick("nhs", &extract_nhs),
        sin:             pick("sin", &extract_sin),
        tfn:             pick("tfn", &extract_tfn),
        my_number:       pick("my_number", &extract_my_number),
        rrn:             pick("rrn", &extract_rrn),
        za_id:           pick("za_id", &extract_za_id),
        il_id:           pick("il_id", &extract_il_id),
    }
}