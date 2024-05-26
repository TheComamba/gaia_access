// This code is generated by generate_code.py, do not modify it manually.
use crate::traits::{Column, Table};

#[allow(non_camel_case_types)]
pub struct tmass_original_valid;

impl Table for tmass_original_valid {
    fn string(&self) -> String {
        "tmass_original_valid".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    ph_qual,
    tmass_oid,
    designation,
    ra,
    dec,
    err_maj,
    err_min,
    err_ang,
    j_m,
    j_msigcom,
    h_m,
    h_msigcom,
    ks_m,
    ks_msigcom,
    ext_key,
    j_date,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::ph_qual.to_string());
    col_strings.push(Col::tmass_oid.to_string());
    col_strings.push(Col::designation.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::err_maj.to_string());
    col_strings.push(Col::err_min.to_string());
    col_strings.push(Col::err_ang.to_string());
    col_strings.push(Col::j_m.to_string());
    col_strings.push(Col::j_msigcom.to_string());
    col_strings.push(Col::h_m.to_string());
    col_strings.push(Col::h_msigcom.to_string());
    col_strings.push(Col::ks_m.to_string());
    col_strings.push(Col::ks_msigcom.to_string());
    col_strings.push(Col::ext_key.to_string());
    col_strings.push(Col::j_date.to_string());
    map.insert(tmass_original_valid.string(), col_strings);
}
