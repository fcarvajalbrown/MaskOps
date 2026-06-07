pub mod eu;
pub mod latam;
pub mod contact;
pub mod financial;
pub mod us;
pub mod healthcare;
pub mod apac;
pub mod country_codes;
pub mod fpe;
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
    contains_uy_ci, mask_uy_ci, extract_uy_ci, mask_uy_ci_fpe, mask_uy_ci_consistent,
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
pub use crate::patterns::fpe::{Ff3Cipher, KEY_LEN, TWEAK_LEN};
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
    s
}

pub fn mask_digit_fpe(value: &str, cipher: &Ff3Cipher) -> String {
    let s = mask_phone_fpe(value, cipher);
    let s = mask_rut_fpe(&s, cipher);
    let s = mask_cpf_fpe(&s, cipher);
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
    s
}

pub fn mask_all(value: &str) -> String {
    let s = mask_non_digit(value);
    mask_digit(&s)
}

pub fn mask_all_fpe(value: &str, cipher: &Ff3Cipher) -> String {
    let s = mask_non_digit(value);
    mask_digit_fpe(&s, cipher)
}

pub fn mask_digit_consistent(value: &str, hasher: &ConsistentHasher) -> String {
    let s = mask_phone_consistent(value, hasher);
    let s = mask_rut_consistent(&s, hasher);
    let s = mask_cpf_consistent(&s, hasher);
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
    s
}

pub fn mask_all_consistent(value: &str, hasher: &ConsistentHasher) -> String {
    let s = mask_non_digit(value);
    mask_digit_consistent(&s, hasher)
}

pub fn mask_all_selected(value: &str, patterns: &[&str]) -> String {
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
            "ssn"             => mask_ssn(&s),
            "arg_dni"         => mask_arg_dni(&s),
            "co_cc"           => mask_co_cc(&s),
            "co_nit"          => mask_co_nit(&s),
            "ec_cedula"       => mask_ec_cedula(&s),
            "credit_card"     => mask_card(&s),
            "npi"             => mask_npi(&s),
            "mbi"             => mask_mbi(&s),
            "nhs"             => mask_nhs(&s),
            "pe_dni"          => mask_pe_dni(&s),
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
            _                 => s,
        };
    }
    s
}

pub fn mask_all_selected_fpe(value: &str, patterns: &[&str], cipher: &Ff3Cipher) -> String {
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
            "ssn"             => mask_ssn_fpe(&s, cipher),
            "arg_dni"         => mask_arg_dni_fpe(&s, cipher),
            "co_cc"           => mask_co_cc_fpe(&s, cipher),
            "co_nit"          => mask_co_nit_fpe(&s, cipher),
            "ec_cedula"       => mask_ec_cedula_fpe(&s, cipher),
            "credit_card"     => mask_card_fpe(&s, cipher),
            "npi"             => mask_npi_fpe(&s, cipher),
            "mbi"             => mask_mbi(&s),  
            "nhs"             => mask_nhs_fpe(&s, cipher),
            "pe_dni"          => mask_pe_dni_fpe(&s, cipher),
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
            _                 => s,
        };
    }
    s
}

pub fn mask_all_selected_consistent(value: &str, patterns: &[&str], hasher: &ConsistentHasher) -> String {
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
            "ssn"             => mask_ssn_consistent(&s, hasher),
            "arg_dni"         => mask_arg_dni_consistent(&s, hasher),
            "co_cc"           => mask_co_cc_consistent(&s, hasher),
            "co_nit"          => mask_co_nit_consistent(&s, hasher),
            "ec_cedula"       => mask_ec_cedula_consistent(&s, hasher),
            "credit_card"     => mask_card_consistent(&s, hasher),
            "npi"             => mask_npi_consistent(&s, hasher),
            "mbi"             => mask_mbi(&s),
            "nhs"             => mask_nhs_consistent(&s, hasher),
            "pe_dni"          => mask_pe_dni_consistent(&s, hasher),
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
            _                 => s,
        };
    }
    s
}

pub fn contains_any_selected(value: &str, patterns: &[&str]) -> bool {
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
        "ssn"             => contains_ssn(value),
        "arg_dni"         => contains_arg_dni(value),
        "co_cc"           => contains_co_cc(value),
        "co_nit"          => contains_co_nit(value),
        "ec_cedula"       => contains_ec_cedula(value),
        "credit_card"     => contains_card(value),
        "npi"             => contains_npi(value),
        "mbi"             => contains_mbi(value),
        "nhs"             => contains_nhs(value),
        "pe_dni"          => contains_pe_dni(value),
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
        _                 => false,
    })
}

pub fn contains_any_pii(value: &str) -> bool {
    contains_iban(value)
        || contains_vat(value)
        || contains_email(value)
        || contains_phone(value)
        || contains_ip(value)
        || contains_rut(value)
        || contains_cpf(value)
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
}

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
}

pub fn extract_all(value: &str) -> ExtractResult {
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
    }
}