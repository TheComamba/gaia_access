// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the ucac4_original_valid table.

use crate::traits::{Column, Table};

/// The ucac4_original_valid table.
#[allow(non_camel_case_types)]
pub struct ucac4_original_valid;

impl Table for ucac4_original_valid {
    fn string(&self) -> String {
        "ucac4_original_valid".to_string()
    }
}

/// The columns in the ucac4_original_valid table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    objt,
    cdf,
    ucac4_oid,
    ucac4_identifier,
    ra,
    dec,
    ra_error_mean_epoch,
    dec_error_mean_epoch,
    pmra,
    pmde,
    pmra_error,
    pmde_error,
    mean_epoch_ra,
    mean_epoch_dec,
    magm,
    maga,
    sigmag,
    n_epochs,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::objt.to_string());
    col_strings.push(Col::cdf.to_string());
    col_strings.push(Col::ucac4_oid.to_string());
    col_strings.push(Col::ucac4_identifier.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::ra_error_mean_epoch.to_string());
    col_strings.push(Col::dec_error_mean_epoch.to_string());
    col_strings.push(Col::pmra.to_string());
    col_strings.push(Col::pmde.to_string());
    col_strings.push(Col::pmra_error.to_string());
    col_strings.push(Col::pmde_error.to_string());
    col_strings.push(Col::mean_epoch_ra.to_string());
    col_strings.push(Col::mean_epoch_dec.to_string());
    col_strings.push(Col::magm.to_string());
    col_strings.push(Col::maga.to_string());
    col_strings.push(Col::sigmag.to_string());
    col_strings.push(Col::n_epochs.to_string());
    map.insert(ucac4_original_valid.string(), col_strings);
}
