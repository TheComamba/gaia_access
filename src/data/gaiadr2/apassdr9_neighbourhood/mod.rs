// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the apassdr9_neighbourhood table.

use crate::traits::{Column, Table};

/// APASS DR9 Neighbourhood table includes all good neighbours for each
/// matched Gaia object. A good neighbour for a given Gaia object is a
/// nearby object in the external catalogue whose position is compatible
/// (within position errors) with the target.
#[allow(non_camel_case_types)]
pub struct apassdr9_neighbourhood;

impl Table for apassdr9_neighbourhood {
    fn string(&self) -> String {
        "apassdr9_neighbourhood".to_string()
    }
}

/// The columns in the apassdr9_neighbourhood table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Unique Gaia source identifier
    source_id,
    /// Original External Catalogue source identifier
    original_ext_source_id,
    /// Angular Distance between the two sources
    angular_distance,
    /// Score of neighbours
    score,
    /// Number of Gaia astrometric params used
    gaia_astrometric_params,
    /// External Catalogue source identifier
    apassdr9_oid,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the apassdr9_neighbourhood table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::original_ext_source_id.to_string());
    col_strings.push(Col::angular_distance.to_string());
    col_strings.push(Col::score.to_string());
    col_strings.push(Col::gaia_astrometric_params.to_string());
    col_strings.push(Col::apassdr9_oid.to_string());
    map.insert(apassdr9_neighbourhood.string(), col_strings);
}
