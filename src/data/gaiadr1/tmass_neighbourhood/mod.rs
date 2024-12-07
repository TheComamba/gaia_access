// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the tmass_neighbourhood table.

use crate::traits::{Column, Table};

/// 2MASS implementation of BaseNeighbourhod
#[allow(non_camel_case_types)]
pub struct tmass_neighbourhood;

impl Table for tmass_neighbourhood {
    fn string(&self) -> String {
        "tmass_neighbourhood".to_string()
    }
}

/// The columns in the tmass_neighbourhood table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// The additional numeric unique source identifier of the External
    /// catalogue, increasing with Declination.
    tmass_oid,
    /// Unique identifier of the Gaia source, the attribute corresponds to
    /// GaiaSource.sourceId
    source_id,
    /// The unique source identifier in the original External catalogue.
    original_ext_source_id,
    /// Angular distance between a Gaia source and its nearest neighbour in the
    /// External Catalogue
    angular_distance,
    /// Score of a given neighbour.  
    /// For the first Gaia release the score will be a likelihood ratio based on
    /// geometric distance and local density of the external catalogue: the
    /// higher the score, the most probable the match is.
    score,
    /// This flag is set to 0 if Gaia proper motions were not available and were
    /// thus not used in the XMatch.  
    /// This flag is set to 1 if Gaia proper motions were available and were
    /// thus used in the XMatch (for the First Gaia release the TGASS
    /// sub-sample.
    proper_motion_flag,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the tmass_neighbourhood table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::tmass_oid.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::original_ext_source_id.to_string());
    col_strings.push(Col::angular_distance.to_string());
    col_strings.push(Col::score.to_string());
    col_strings.push(Col::proper_motion_flag.to_string());
    map.insert(tmass_neighbourhood.string(), col_strings);
}
