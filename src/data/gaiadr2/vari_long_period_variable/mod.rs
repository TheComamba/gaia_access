// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the vari_long_period_variable table.

use crate::traits::{Column, Table};

/// The vari_long_period_variable table.
#[allow(non_camel_case_types)]
pub struct vari_long_period_variable;

impl Table for vari_long_period_variable {
    fn string(&self) -> String {
        "vari_long_period_variable".to_string()
    }
}

/// The columns in the vari_long_period_variable table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    abs_mag_bol,
    abs_mag_bol_error,
    rsg_flag,
    bolometric_corr,
    bolometric_corr_error,
    frequency,
    frequency_error,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::abs_mag_bol.to_string());
    col_strings.push(Col::abs_mag_bol_error.to_string());
    col_strings.push(Col::rsg_flag.to_string());
    col_strings.push(Col::bolometric_corr.to_string());
    col_strings.push(Col::bolometric_corr_error.to_string());
    col_strings.push(Col::frequency.to_string());
    col_strings.push(Col::frequency_error.to_string());
    map.insert(vari_long_period_variable.string(), col_strings);
}
