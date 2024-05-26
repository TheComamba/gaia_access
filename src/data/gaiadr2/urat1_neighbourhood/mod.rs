// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the urat1_neighbourhood table.

use crate::traits::{Column, Table};

/// The urat1_neighbourhood table.
#[allow(non_camel_case_types)]
pub struct urat1_neighbourhood;

impl Table for urat1_neighbourhood {
    fn string(&self) -> String {
        "urat1_neighbourhood".to_string()
    }
}

/// The columns in the urat1_neighbourhood table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    original_ext_source_id,
    angular_distance,
    score,
    gaia_astrometric_params,
    urat1_oid,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::original_ext_source_id.to_string());
    col_strings.push(Col::angular_distance.to_string());
    col_strings.push(Col::score.to_string());
    col_strings.push(Col::gaia_astrometric_params.to_string());
    col_strings.push(Col::urat1_oid.to_string());
    map.insert(urat1_neighbourhood.string(), col_strings);
}
