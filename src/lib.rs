

mod patterns;

use pyo3::prelude::*;
use pyo3::types::PyModuleMethods;
use pyo3_polars::derive::polars_expr;
use pyo3_polars::export::polars_core::prelude::*;
use patterns::{mask_all, mask_all_fpe, contains_any_pii,
               mask_all_selected, mask_all_selected_fpe, contains_any_selected,
               mask_all_consistent, mask_all_selected_consistent,
               extract_all, ExtractResult, mask_all_audit};
use patterns::{Ff3Cipher, Ff1Cipher, FpeCipher, KEY_LEN, TWEAK_LEN};
use patterns::ConsistentHasher;
pub use patterns::fpe::Ff3Cipher as MaskopsFpe;

fn read_key_tweak<'a>(
    key_series: &'a Series,
    tweak_series: &'a Series,
    label: &str,
) -> PolarsResult<([u8; KEY_LEN], [u8; TWEAK_LEN])> {
    let key_bytes = key_series.binary()?
        .get(0)
        .ok_or_else(|| PolarsError::ComputeError(format!("{}: missing key", label).into()))?;
    let tweak_bytes = tweak_series.binary()?
        .get(0)
        .ok_or_else(|| PolarsError::ComputeError(format!("{}: missing tweak", label).into()))?;
    if key_bytes.len() != KEY_LEN {
        return Err(PolarsError::ComputeError(
            format!("{}: key must be {} bytes, got {}", label, KEY_LEN, key_bytes.len()).into()
        ));
    }
    if tweak_bytes.len() != TWEAK_LEN {
        return Err(PolarsError::ComputeError(
            format!("{}: tweak must be {} bytes, got {}", label, TWEAK_LEN, tweak_bytes.len()).into()
        ));
    }
    let key: [u8; KEY_LEN] = key_bytes.try_into().unwrap();
    let tweak: [u8; TWEAK_LEN] = tweak_bytes.try_into().unwrap();
    Ok((key, tweak))
}

fn build_cipher(key: [u8; KEY_LEN], tweak: [u8; TWEAK_LEN], mode: &str) -> PolarsResult<FpeCipher> {
    match mode {
        "ff3" | "" => Ok(FpeCipher::Ff3(Ff3Cipher::new(&key, &tweak))),
        "ff1"      => Ok(FpeCipher::Ff1(Ff1Cipher::new(&key, &tweak))),
        other => Err(PolarsError::ComputeError(
            format!("mask_pii_fpe: unknown mode '{}', expected 'ff3' or 'ff1'", other).into()
        )),
    }
}

#[polars_expr(output_type=String)]
fn mask_pii(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: StringChunked = if inputs.len() > 1 {
        let pat_str = inputs[1].str()?.get(0).unwrap_or("");
        let patterns: Vec<&str> = pat_str.split(',').filter(|s| !s.is_empty()).collect();
        ca.apply(|opt| opt.map(|s| std::borrow::Cow::Owned(mask_all_selected(s, &patterns))))
    } else {
        ca.apply(|opt| opt.map(|s| std::borrow::Cow::Owned(mask_all(s))))
    };
    Ok(out.into_series())
}

#[polars_expr(output_type=Boolean)]
fn contains_pii(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let out: BooleanChunked = if inputs.len() > 1 {
        let pat_str = inputs[1].str()?.get(0).unwrap_or("");
        let patterns: Vec<&str> = pat_str.split(',').filter(|s| !s.is_empty()).collect();
        ca.apply_nonnull_values_generic(DataType::Boolean, |s| contains_any_selected(s, &patterns))
    } else {
        ca.apply_nonnull_values_generic(DataType::Boolean, |s| contains_any_pii(s))
    };
    Ok(out.into_series())
}

#[polars_expr(output_type=String)]
fn mask_pii_fpe(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;

    let key_series   = inputs[1].cast(&DataType::Binary)?;
    let tweak_series = inputs[2].cast(&DataType::Binary)?;
    let (key, tweak) = read_key_tweak(&key_series, &tweak_series, "mask_pii_fpe")?;

    let mode = if inputs.len() > 3 { inputs[3].str()?.get(0).unwrap_or("ff3") } else { "ff3" };
    let cipher = build_cipher(key, tweak, mode)?;

    let out: StringChunked = if inputs.len() > 4 {
        let pat_str = inputs[4].str()?.get(0).unwrap_or("");
        let patterns: Vec<&str> = pat_str.split(',').filter(|s| !s.is_empty()).collect();
        ca.apply(|opt| opt.map(|s| std::borrow::Cow::Owned(mask_all_selected_fpe(s, &patterns, &cipher))))
    } else {
        ca.apply(|opt| opt.map(|s| std::borrow::Cow::Owned(mask_all_fpe(s, &cipher))))
    };
    Ok(out.into_series())
}

#[polars_expr(output_type=String)]
fn mask_pii_fpe_rekey(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;

    let old_key_series   = inputs[1].cast(&DataType::Binary)?;
    let old_tweak_series = inputs[2].cast(&DataType::Binary)?;
    let new_key_series   = inputs[3].cast(&DataType::Binary)?;
    let new_tweak_series = inputs[4].cast(&DataType::Binary)?;
    let (old_key, old_tweak) = read_key_tweak(&old_key_series, &old_tweak_series, "mask_pii_fpe_rekey")?;
    let (new_key, new_tweak) = read_key_tweak(&new_key_series, &new_tweak_series, "mask_pii_fpe_rekey")?;

    let mode = if inputs.len() > 5 { inputs[5].str()?.get(0).unwrap_or("ff3") } else { "ff3" };
    let old_cipher = build_cipher(old_key, old_tweak, mode)?;
    let new_cipher = build_cipher(new_key, new_tweak, mode)?;
    let cipher = FpeCipher::Rekey(Box::new(old_cipher), Box::new(new_cipher));

    let out: StringChunked = ca.apply(|opt| opt.map(|s| match cipher.encrypt(s) {
        Ok(rotated) => std::borrow::Cow::Owned(rotated),
        Err(_) => std::borrow::Cow::Owned(s.to_string()),
    }));
    Ok(out.into_series())
}

#[polars_expr(output_type=String)]
fn mask_pii_consistent(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let salt = inputs[1].str()?
        .get(0)
        .ok_or_else(|| PolarsError::ComputeError("mask_pii_consistent: missing salt".into()))?;
    let hasher = ConsistentHasher::new(salt);
    let out: StringChunked = if inputs.len() > 2 {
        let pat_str = inputs[2].str()?.get(0).unwrap_or("");
        let patterns: Vec<&str> = pat_str.split(',').filter(|s| !s.is_empty()).collect();
        ca.apply(|opt| opt.map(|s| std::borrow::Cow::Owned(mask_all_selected_consistent(s, &patterns, &hasher))))
    } else {
        ca.apply(|opt| opt.map(|s| std::borrow::Cow::Owned(mask_all_consistent(s, &hasher))))
    };
    Ok(out.into_series())
}

fn extract_pii_output(_: &[Field]) -> PolarsResult<Field> {
    let fields = vec![
        Field::new("email".into(),          DataType::String),
        Field::new("phone".into(),          DataType::String),
        Field::new("ip".into(),             DataType::String),
        Field::new("iban".into(),           DataType::String),
        Field::new("vat".into(),            DataType::String),
        Field::new("dni".into(),            DataType::String),
        Field::new("nie".into(),            DataType::String),
        Field::new("nin".into(),            DataType::String),
        Field::new("personalausweis".into(), DataType::String),
        Field::new("nir".into(),            DataType::String),
        Field::new("codice_fiscale".into(), DataType::String),
        Field::new("pesel".into(),          DataType::String),
        Field::new("bsn".into(),            DataType::String),
        Field::new("personnummer".into(),   DataType::String),
        Field::new("credit_card".into(),    DataType::String),
        Field::new("ssn".into(),            DataType::String),
        Field::new("us_passport".into(),    DataType::String),
        Field::new("rut".into(),            DataType::String),
        Field::new("cpf".into(),            DataType::String),
        Field::new("cnpj".into(),           DataType::String),
        Field::new("curp".into(),           DataType::String),
        Field::new("arg_dni".into(),        DataType::String),
        Field::new("co_cc".into(),          DataType::String),
        Field::new("co_nit".into(),         DataType::String),
        Field::new("ec_cedula".into(),      DataType::String),
        Field::new("pe_dni".into(),         DataType::String),
        Field::new("uy_ci".into(),          DataType::String),
        Field::new("npi".into(),            DataType::String),
        Field::new("mbi".into(),            DataType::String),
        Field::new("nhs".into(),            DataType::String),
        Field::new("sin".into(),            DataType::String),
        Field::new("tfn".into(),            DataType::String),
        Field::new("my_number".into(),      DataType::String),
        Field::new("rrn".into(),            DataType::String),
    ];
    Ok(Field::new("extract_pii".into(), DataType::Struct(fields)))
}

#[polars_expr(output_type_func=extract_pii_output)]
fn extract_pii(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let len = ca.len();

    let mut f_email         = Vec::with_capacity(len);
    let mut f_phone         = Vec::with_capacity(len);
    let mut f_ip            = Vec::with_capacity(len);
    let mut f_iban          = Vec::with_capacity(len);
    let mut f_vat           = Vec::with_capacity(len);
    let mut f_dni           = Vec::with_capacity(len);
    let mut f_nie           = Vec::with_capacity(len);
    let mut f_nin           = Vec::with_capacity(len);
    let mut f_personalausweis = Vec::with_capacity(len);
    let mut f_nir           = Vec::with_capacity(len);
    let mut f_codice_fiscale = Vec::with_capacity(len);
    let mut f_pesel         = Vec::with_capacity(len);
    let mut f_bsn           = Vec::with_capacity(len);
    let mut f_personnummer  = Vec::with_capacity(len);
    let mut f_credit_card   = Vec::with_capacity(len);
    let mut f_ssn           = Vec::with_capacity(len);
    let mut f_us_passport   = Vec::with_capacity(len);
    let mut f_rut           = Vec::with_capacity(len);
    let mut f_cpf           = Vec::with_capacity(len);
    let mut f_cnpj          = Vec::with_capacity(len);
    let mut f_curp          = Vec::with_capacity(len);
    let mut f_arg_dni       = Vec::with_capacity(len);
    let mut f_co_cc         = Vec::with_capacity(len);
    let mut f_co_nit        = Vec::with_capacity(len);
    let mut f_ec_cedula     = Vec::with_capacity(len);
    let mut f_pe_dni        = Vec::with_capacity(len);
    let mut f_uy_ci         = Vec::with_capacity(len);
    let mut f_npi           = Vec::with_capacity(len);
    let mut f_mbi           = Vec::with_capacity(len);
    let mut f_nhs           = Vec::with_capacity(len);
    let mut f_sin           = Vec::with_capacity(len);
    let mut f_tfn           = Vec::with_capacity(len);
    let mut f_my_number     = Vec::with_capacity(len);
    let mut f_rrn           = Vec::with_capacity(len);

    for opt in ca.into_iter() {
        let r: ExtractResult = match opt {
            Some(s) => extract_all(s),
            None => ExtractResult {
                email: None, phone: None, ip: None, iban: None, vat: None,
                dni: None, nie: None, nin: None, personalausweis: None,
                nir: None, codice_fiscale: None, pesel: None, bsn: None,
                personnummer: None, credit_card: None, ssn: None,
                us_passport: None, rut: None, cpf: None, cnpj: None, curp: None,
                arg_dni: None, co_cc: None, co_nit: None, ec_cedula: None,
                pe_dni: None, uy_ci: None, npi: None, mbi: None, nhs: None,
                sin: None, tfn: None, my_number: None, rrn: None,
            },
        };
        f_email.push(r.email);
        f_phone.push(r.phone);
        f_ip.push(r.ip);
        f_iban.push(r.iban);
        f_vat.push(r.vat);
        f_dni.push(r.dni);
        f_nie.push(r.nie);
        f_nin.push(r.nin);
        f_personalausweis.push(r.personalausweis);
        f_nir.push(r.nir);
        f_codice_fiscale.push(r.codice_fiscale);
        f_pesel.push(r.pesel);
        f_bsn.push(r.bsn);
        f_personnummer.push(r.personnummer);
        f_credit_card.push(r.credit_card);
        f_ssn.push(r.ssn);
        f_us_passport.push(r.us_passport);
        f_rut.push(r.rut);
        f_cpf.push(r.cpf);
        f_cnpj.push(r.cnpj);
        f_curp.push(r.curp);
        f_arg_dni.push(r.arg_dni);
        f_co_cc.push(r.co_cc);
        f_co_nit.push(r.co_nit);
        f_ec_cedula.push(r.ec_cedula);
        f_pe_dni.push(r.pe_dni);
        f_uy_ci.push(r.uy_ci);
        f_npi.push(r.npi);
        f_mbi.push(r.mbi);
        f_nhs.push(r.nhs);
        f_sin.push(r.sin);
        f_tfn.push(r.tfn);
        f_my_number.push(r.my_number);
        f_rrn.push(r.rrn);
    }

    let series: Vec<Series> = vec![
        StringChunked::from_iter_options("email".into(),          f_email.into_iter()).into_series(),
        StringChunked::from_iter_options("phone".into(),          f_phone.into_iter()).into_series(),
        StringChunked::from_iter_options("ip".into(),             f_ip.into_iter()).into_series(),
        StringChunked::from_iter_options("iban".into(),           f_iban.into_iter()).into_series(),
        StringChunked::from_iter_options("vat".into(),            f_vat.into_iter()).into_series(),
        StringChunked::from_iter_options("dni".into(),            f_dni.into_iter()).into_series(),
        StringChunked::from_iter_options("nie".into(),            f_nie.into_iter()).into_series(),
        StringChunked::from_iter_options("nin".into(),            f_nin.into_iter()).into_series(),
        StringChunked::from_iter_options("personalausweis".into(), f_personalausweis.into_iter()).into_series(),
        StringChunked::from_iter_options("nir".into(),            f_nir.into_iter()).into_series(),
        StringChunked::from_iter_options("codice_fiscale".into(), f_codice_fiscale.into_iter()).into_series(),
        StringChunked::from_iter_options("pesel".into(),          f_pesel.into_iter()).into_series(),
        StringChunked::from_iter_options("bsn".into(),            f_bsn.into_iter()).into_series(),
        StringChunked::from_iter_options("personnummer".into(),   f_personnummer.into_iter()).into_series(),
        StringChunked::from_iter_options("credit_card".into(),    f_credit_card.into_iter()).into_series(),
        StringChunked::from_iter_options("ssn".into(),            f_ssn.into_iter()).into_series(),
        StringChunked::from_iter_options("us_passport".into(),    f_us_passport.into_iter()).into_series(),
        StringChunked::from_iter_options("rut".into(),            f_rut.into_iter()).into_series(),
        StringChunked::from_iter_options("cpf".into(),            f_cpf.into_iter()).into_series(),
        StringChunked::from_iter_options("cnpj".into(),           f_cnpj.into_iter()).into_series(),
        StringChunked::from_iter_options("curp".into(),           f_curp.into_iter()).into_series(),
        StringChunked::from_iter_options("arg_dni".into(),        f_arg_dni.into_iter()).into_series(),
        StringChunked::from_iter_options("co_cc".into(),          f_co_cc.into_iter()).into_series(),
        StringChunked::from_iter_options("co_nit".into(),         f_co_nit.into_iter()).into_series(),
        StringChunked::from_iter_options("ec_cedula".into(),      f_ec_cedula.into_iter()).into_series(),
        StringChunked::from_iter_options("pe_dni".into(),         f_pe_dni.into_iter()).into_series(),
        StringChunked::from_iter_options("uy_ci".into(),          f_uy_ci.into_iter()).into_series(),
        StringChunked::from_iter_options("npi".into(),            f_npi.into_iter()).into_series(),
        StringChunked::from_iter_options("mbi".into(),            f_mbi.into_iter()).into_series(),
        StringChunked::from_iter_options("nhs".into(),            f_nhs.into_iter()).into_series(),
        StringChunked::from_iter_options("sin".into(),            f_sin.into_iter()).into_series(),
        StringChunked::from_iter_options("tfn".into(),            f_tfn.into_iter()).into_series(),
        StringChunked::from_iter_options("my_number".into(),      f_my_number.into_iter()).into_series(),
        StringChunked::from_iter_options("rrn".into(),            f_rrn.into_iter()).into_series(),
    ];

    Ok(StructChunked::from_series("extract_pii".into(), len, series.iter())?.into_series())
}

fn mask_pii_audit_output(_: &[Field]) -> PolarsResult<Field> {
    let count_fields = vec![
        Field::new("email".into(),           DataType::UInt32),
        Field::new("phone".into(),           DataType::UInt32),
        Field::new("ip".into(),              DataType::UInt32),
        Field::new("iban".into(),            DataType::UInt32),
        Field::new("vat".into(),             DataType::UInt32),
        Field::new("dni".into(),             DataType::UInt32),
        Field::new("nie".into(),             DataType::UInt32),
        Field::new("nin".into(),             DataType::UInt32),
        Field::new("personalausweis".into(), DataType::UInt32),
        Field::new("nir".into(),             DataType::UInt32),
        Field::new("codice_fiscale".into(),  DataType::UInt32),
        Field::new("pesel".into(),           DataType::UInt32),
        Field::new("bsn".into(),             DataType::UInt32),
        Field::new("personnummer".into(),    DataType::UInt32),
        Field::new("credit_card".into(),     DataType::UInt32),
        Field::new("ssn".into(),             DataType::UInt32),
        Field::new("us_passport".into(),     DataType::UInt32),
        Field::new("rut".into(),             DataType::UInt32),
        Field::new("cpf".into(),             DataType::UInt32),
        Field::new("cnpj".into(),            DataType::UInt32),
        Field::new("curp".into(),            DataType::UInt32),
        Field::new("arg_dni".into(),         DataType::UInt32),
        Field::new("co_cc".into(),           DataType::UInt32),
        Field::new("co_nit".into(),          DataType::UInt32),
        Field::new("ec_cedula".into(),       DataType::UInt32),
        Field::new("pe_dni".into(),          DataType::UInt32),
        Field::new("uy_ci".into(),           DataType::UInt32),
        Field::new("npi".into(),             DataType::UInt32),
        Field::new("mbi".into(),             DataType::UInt32),
        Field::new("nhs".into(),             DataType::UInt32),
        Field::new("sin".into(),             DataType::UInt32),
        Field::new("tfn".into(),             DataType::UInt32),
        Field::new("my_number".into(),       DataType::UInt32),
        Field::new("rrn".into(),             DataType::UInt32),
    ];
    let fields = vec![
        Field::new("masked".into(), DataType::String),
        Field::new("counts".into(), DataType::Struct(count_fields)),
    ];
    Ok(Field::new("mask_pii_audit".into(), DataType::Struct(fields)))
}

#[polars_expr(output_type_func=mask_pii_audit_output)]
fn mask_pii_audit(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].str()?;
    let len = ca.len();

    let mut masked: Vec<Option<String>> = Vec::with_capacity(len);
    let mut c_email           = Vec::with_capacity(len);
    let mut c_phone           = Vec::with_capacity(len);
    let mut c_ip              = Vec::with_capacity(len);
    let mut c_iban            = Vec::with_capacity(len);
    let mut c_vat             = Vec::with_capacity(len);
    let mut c_dni             = Vec::with_capacity(len);
    let mut c_nie             = Vec::with_capacity(len);
    let mut c_nin             = Vec::with_capacity(len);
    let mut c_personalausweis = Vec::with_capacity(len);
    let mut c_nir             = Vec::with_capacity(len);
    let mut c_codice_fiscale  = Vec::with_capacity(len);
    let mut c_pesel           = Vec::with_capacity(len);
    let mut c_bsn             = Vec::with_capacity(len);
    let mut c_personnummer    = Vec::with_capacity(len);
    let mut c_credit_card     = Vec::with_capacity(len);
    let mut c_ssn             = Vec::with_capacity(len);
    let mut c_us_passport     = Vec::with_capacity(len);
    let mut c_rut             = Vec::with_capacity(len);
    let mut c_cpf             = Vec::with_capacity(len);
    let mut c_cnpj            = Vec::with_capacity(len);
    let mut c_curp            = Vec::with_capacity(len);
    let mut c_arg_dni         = Vec::with_capacity(len);
    let mut c_co_cc           = Vec::with_capacity(len);
    let mut c_co_nit          = Vec::with_capacity(len);
    let mut c_ec_cedula       = Vec::with_capacity(len);
    let mut c_pe_dni          = Vec::with_capacity(len);
    let mut c_uy_ci           = Vec::with_capacity(len);
    let mut c_npi             = Vec::with_capacity(len);
    let mut c_mbi             = Vec::with_capacity(len);
    let mut c_nhs             = Vec::with_capacity(len);
    let mut c_sin             = Vec::with_capacity(len);
    let mut c_tfn             = Vec::with_capacity(len);
    let mut c_my_number       = Vec::with_capacity(len);
    let mut c_rrn             = Vec::with_capacity(len);

    for opt in ca.into_iter() {
        match opt {
            Some(s) => {
                let (m, c) = mask_all_audit(s);
                masked.push(Some(m));
                c_email.push(c.email);
                c_phone.push(c.phone);
                c_ip.push(c.ip);
                c_iban.push(c.iban);
                c_vat.push(c.vat);
                c_dni.push(c.dni);
                c_nie.push(c.nie);
                c_nin.push(c.nin);
                c_personalausweis.push(c.personalausweis);
                c_nir.push(c.nir);
                c_codice_fiscale.push(c.codice_fiscale);
                c_pesel.push(c.pesel);
                c_bsn.push(c.bsn);
                c_personnummer.push(c.personnummer);
                c_credit_card.push(c.credit_card);
                c_ssn.push(c.ssn);
                c_us_passport.push(c.us_passport);
                c_rut.push(c.rut);
                c_cpf.push(c.cpf);
                c_cnpj.push(c.cnpj);
                c_curp.push(c.curp);
                c_arg_dni.push(c.arg_dni);
                c_co_cc.push(c.co_cc);
                c_co_nit.push(c.co_nit);
                c_ec_cedula.push(c.ec_cedula);
                c_pe_dni.push(c.pe_dni);
                c_uy_ci.push(c.uy_ci);
                c_npi.push(c.npi);
                c_mbi.push(c.mbi);
                c_nhs.push(c.nhs);
                c_sin.push(c.sin);
                c_tfn.push(c.tfn);
                c_my_number.push(c.my_number);
                c_rrn.push(c.rrn);
            }
            None => {
                masked.push(None);
                c_email.push(0);
                c_phone.push(0);
                c_ip.push(0);
                c_iban.push(0);
                c_vat.push(0);
                c_dni.push(0);
                c_nie.push(0);
                c_nin.push(0);
                c_personalausweis.push(0);
                c_nir.push(0);
                c_codice_fiscale.push(0);
                c_pesel.push(0);
                c_bsn.push(0);
                c_personnummer.push(0);
                c_credit_card.push(0);
                c_ssn.push(0);
                c_us_passport.push(0);
                c_rut.push(0);
                c_cpf.push(0);
                c_cnpj.push(0);
                c_curp.push(0);
                c_arg_dni.push(0);
                c_co_cc.push(0);
                c_co_nit.push(0);
                c_ec_cedula.push(0);
                c_pe_dni.push(0);
                c_uy_ci.push(0);
                c_npi.push(0);
                c_mbi.push(0);
                c_nhs.push(0);
                c_sin.push(0);
                c_tfn.push(0);
                c_my_number.push(0);
                c_rrn.push(0);
            }
        }
    }

    let count_series: Vec<Series> = vec![
        UInt32Chunked::from_vec("email".into(),           c_email).into_series(),
        UInt32Chunked::from_vec("phone".into(),           c_phone).into_series(),
        UInt32Chunked::from_vec("ip".into(),              c_ip).into_series(),
        UInt32Chunked::from_vec("iban".into(),            c_iban).into_series(),
        UInt32Chunked::from_vec("vat".into(),             c_vat).into_series(),
        UInt32Chunked::from_vec("dni".into(),             c_dni).into_series(),
        UInt32Chunked::from_vec("nie".into(),             c_nie).into_series(),
        UInt32Chunked::from_vec("nin".into(),             c_nin).into_series(),
        UInt32Chunked::from_vec("personalausweis".into(), c_personalausweis).into_series(),
        UInt32Chunked::from_vec("nir".into(),             c_nir).into_series(),
        UInt32Chunked::from_vec("codice_fiscale".into(),  c_codice_fiscale).into_series(),
        UInt32Chunked::from_vec("pesel".into(),           c_pesel).into_series(),
        UInt32Chunked::from_vec("bsn".into(),             c_bsn).into_series(),
        UInt32Chunked::from_vec("personnummer".into(),    c_personnummer).into_series(),
        UInt32Chunked::from_vec("credit_card".into(),     c_credit_card).into_series(),
        UInt32Chunked::from_vec("ssn".into(),             c_ssn).into_series(),
        UInt32Chunked::from_vec("us_passport".into(),     c_us_passport).into_series(),
        UInt32Chunked::from_vec("rut".into(),             c_rut).into_series(),
        UInt32Chunked::from_vec("cpf".into(),             c_cpf).into_series(),
        UInt32Chunked::from_vec("cnpj".into(),            c_cnpj).into_series(),
        UInt32Chunked::from_vec("curp".into(),            c_curp).into_series(),
        UInt32Chunked::from_vec("arg_dni".into(),         c_arg_dni).into_series(),
        UInt32Chunked::from_vec("co_cc".into(),           c_co_cc).into_series(),
        UInt32Chunked::from_vec("co_nit".into(),          c_co_nit).into_series(),
        UInt32Chunked::from_vec("ec_cedula".into(),       c_ec_cedula).into_series(),
        UInt32Chunked::from_vec("pe_dni".into(),          c_pe_dni).into_series(),
        UInt32Chunked::from_vec("uy_ci".into(),           c_uy_ci).into_series(),
        UInt32Chunked::from_vec("npi".into(),             c_npi).into_series(),
        UInt32Chunked::from_vec("mbi".into(),             c_mbi).into_series(),
        UInt32Chunked::from_vec("nhs".into(),             c_nhs).into_series(),
        UInt32Chunked::from_vec("sin".into(),             c_sin).into_series(),
        UInt32Chunked::from_vec("tfn".into(),             c_tfn).into_series(),
        UInt32Chunked::from_vec("my_number".into(),       c_my_number).into_series(),
        UInt32Chunked::from_vec("rrn".into(),             c_rrn).into_series(),
    ];

    let counts = StructChunked::from_series("counts".into(), len, count_series.iter())?.into_series();
    let masked_series = StringChunked::from_iter_options("masked".into(), masked.into_iter()).into_series();

    let out_series = vec![masked_series, counts];
    Ok(StructChunked::from_series("mask_pii_audit".into(), len, out_series.iter())?.into_series())
}

#[pymodule]
fn _maskops(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", "1.7.0")?;
    Ok(())
}