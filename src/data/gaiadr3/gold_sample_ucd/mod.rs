// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the gold_sample_ucd table.

use crate::traits::{Column, Table};

/// The gold_sample_ucd table.
#[allow(non_camel_case_types)]
pub struct gold_sample_ucd;

impl Table for gold_sample_ucd {
    fn string(&self) -> String {
        "gold_sample_ucd".to_string()
    }
}

/// The columns in the gold_sample_ucd table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    ftot_fobs,
    lum,
    lum_uncertainty,
    radius,
    radius_uncertainty,
    reduced_chi2,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::ftot_fobs.to_string());
    col_strings.push(Col::lum.to_string());
    col_strings.push(Col::lum_uncertainty.to_string());
    col_strings.push(Col::radius.to_string());
    col_strings.push(Col::radius_uncertainty.to_string());
    col_strings.push(Col::reduced_chi2.to_string());
    map.insert(gold_sample_ucd.string(), col_strings);
}
