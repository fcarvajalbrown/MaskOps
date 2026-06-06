

mod patterns;

use pyo3::prelude::*;
use pyo3::types::PyModuleMethods;
use pyo3_polars::derive::polars_expr;
use pyo3_polars::export::polars_core::prelude::*;
use patterns::{mask_all, mask_all_fpe, contains_any_pii,
               mask_all_selected, mask_all_selected_fpe, contains_any_selected,
               mask_all_consistent, mask_all_selected_consistent,
               extract_all, ExtractResult};
use patterns::{Ff3Cipher, KEY_LEN, TWEAK_LEN};
use patterns::ConsistentHasher;
pub use patterns::fpe::Ff3Cipher as MaskopsFpe;

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

    let key_bytes = key_series.binary()?
        .get(0)
        .ok_or_else(|| PolarsError::ComputeError("mask_pii_fpe: missing key".into()))?;
    let tweak_bytes = tweak_series.binary()?
        .get(0)
        .ok_or_else(|| PolarsError::ComputeError("mask_pii_fpe: missing tweak".into()))?;

    if key_bytes.len() != KEY_LEN {
        return Err(PolarsError::ComputeError(
            format!("mask_pii_fpe: key must be {} bytes, got {}", KEY_LEN, key_bytes.len()).into()
        ));
    }
    if tweak_bytes.len() != TWEAK_LEN {
        return Err(PolarsError::ComputeError(
            format!("mask_pii_fpe: tweak must be {} bytes, got {}", TWEAK_LEN, tweak_bytes.len()).into()
        ));
    }

    let key:   [u8; KEY_LEN]   = key_bytes.try_into().unwrap();
    let tweak: [u8; TWEAK_LEN] = tweak_bytes.try_into().unwrap();
    let cipher = Ff3Cipher::new(&key, &tweak);

    let out: StringChunked = if inputs.len() > 3 {
        let pat_str = inputs[3].str()?.get(0).unwrap_or("");
        let patterns: Vec<&str> = pat_str.split(',').filter(|s| !s.is_empty()).collect();
        ca.apply(|opt| opt.map(|s| std::borrow::Cow::Owned(mask_all_selected_fpe(s, &patterns, &cipher))))
    } else {
        ca.apply(|opt| opt.map(|s| std::borrow::Cow::Owned(mask_all_fpe(s, &cipher))))
    };
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

    for opt in ca.into_iter() {
        let r: ExtractResult = match opt {
            Some(s) => extract_all(s),
            None => ExtractResult {
                email: None, phone: None, ip: None, iban: None, vat: None,
                dni: None, nie: None, nin: None, personalausweis: None,
                nir: None, codice_fiscale: None, pesel: None, bsn: None,
                personnummer: None, credit_card: None, ssn: None,
                us_passport: None, rut: None, cpf: None, curp: None,
                arg_dni: None, co_cc: None, co_nit: None, ec_cedula: None,
                pe_dni: None, uy_ci: None, npi: None, mbi: None, nhs: None,
                sin: None, tfn: None,
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
    ];

    Ok(StructChunked::from_series("extract_pii".into(), len, series.iter())?.into_series())
}

#[pymodule]
fn _maskops(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", "0.1.5")?;
    Ok(())
}