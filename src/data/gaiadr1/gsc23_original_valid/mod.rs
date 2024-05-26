// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the gsc23_original_valid table.

use crate::traits::{Column, Table};

/// The gsc23_original_valid table.
#[allow(non_camel_case_types)]
pub struct gsc23_original_valid;

impl Table for gsc23_original_valid {
    fn string(&self) -> String {
        "gsc23_original_valid".to_string()
    }
}

/// The columns in the gsc23_original_valid table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    gsc23_oid,
    gsc23_identifier,
    ra,
    dec,
    position_epoch,
    ra_error,
    dec_error,
    fpg_mag,
    fpg_mag_error,
    fpg_mag_code,
    jpg_mag,
    jpg_mag_error,
    jpg_mag_code,
    v_mag,
    v_mag_error,
    v_mag_code,
    npg_mag,
    npg_mag_error,
    npg_mag_code,
    b_mag,
    b_mag_error,
    b_mag_code,
    classification,
    status,
    mult_flag,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::gsc23_oid.to_string());
    col_strings.push(Col::gsc23_identifier.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::position_epoch.to_string());
    col_strings.push(Col::ra_error.to_string());
    col_strings.push(Col::dec_error.to_string());
    col_strings.push(Col::fpg_mag.to_string());
    col_strings.push(Col::fpg_mag_error.to_string());
    col_strings.push(Col::fpg_mag_code.to_string());
    col_strings.push(Col::jpg_mag.to_string());
    col_strings.push(Col::jpg_mag_error.to_string());
    col_strings.push(Col::jpg_mag_code.to_string());
    col_strings.push(Col::v_mag.to_string());
    col_strings.push(Col::v_mag_error.to_string());
    col_strings.push(Col::v_mag_code.to_string());
    col_strings.push(Col::npg_mag.to_string());
    col_strings.push(Col::npg_mag_error.to_string());
    col_strings.push(Col::npg_mag_code.to_string());
    col_strings.push(Col::b_mag.to_string());
    col_strings.push(Col::b_mag_error.to_string());
    col_strings.push(Col::b_mag_code.to_string());
    col_strings.push(Col::classification.to_string());
    col_strings.push(Col::status.to_string());
    col_strings.push(Col::mult_flag.to_string());
    map.insert(gsc23_original_valid.string(), col_strings);
}
