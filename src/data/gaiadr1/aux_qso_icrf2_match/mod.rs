// This code is generated by generate_code.py, do not modify it manually.
use crate::traits::{Column, Table};

#[allow(non_camel_case_types)]
pub struct aux_qso_icrf2_match;

impl Table for aux_qso_icrf2_match {
    fn string(&self) -> String {
        "aux_qso_icrf2_match".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    ref_epoch,
    ra,
    ra_error,
    dec,
    dec_error,
    ra_dec_corr,
    phot_g_mean_mag,
    astrometric_priors_used,
    icrf2_match,
    rot_flag,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::ref_epoch.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::ra_error.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::dec_error.to_string());
    col_strings.push(Col::ra_dec_corr.to_string());
    col_strings.push(Col::phot_g_mean_mag.to_string());
    col_strings.push(Col::astrometric_priors_used.to_string());
    col_strings.push(Col::icrf2_match.to_string());
    col_strings.push(Col::rot_flag.to_string());
    map.insert(aux_qso_icrf2_match.string(), col_strings);
}
