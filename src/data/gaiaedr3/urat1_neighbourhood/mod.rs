// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the urat1_neighbourhood table.

use crate::traits::{Column, Table};

/// <p>URAT-1 Neighbourhood table includes all good neighbours for each matched Gaia object.
/// A good neighbour for a given Gaia source is a nearby object in the external catalogue whose
/// position is compatible (within position errors) with the Gaia target. <br/>
/// The cross-match algorithm is not symmetric and searches Gaia sources counterparts in URAT-1.
/// The cross-match algorithm is positional and exploits the full 5
/// parameters covariance matrix of Gaia astrometric solution when available and the
/// external catalogue positions and position errors. In addition it takes into account the
/// external catalogue environment using the local density.<br/>
/// <br/>
/// Please note that the cross-match algorithm is a trade-off between multiple requirements, in
/// particular between completeness and correctness. It is thus not limited to a simple cone search.<br/>
/// <br/>
/// Reference papers:<br/>
/// </p>
/// <p>DR1-DPACP-17<br/>
/// </p>
/// <p>DR2-DPACP-41<br/>
/// </p>
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
    /// Unique Gaia source identifier
    source_id,
    /// Original External Catalogue source identifier
    original_ext_source_id,
    /// Angular Distance between the two sources
    angular_distance,
    /// Score of neighbours
    score,
    /// Cross-match algorithm flag
    xm_flag,
    /// External Catalogue source identifier
    urat1_oid,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the urat1_neighbourhood table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::original_ext_source_id.to_string());
    col_strings.push(Col::angular_distance.to_string());
    col_strings.push(Col::score.to_string());
    col_strings.push(Col::xm_flag.to_string());
    col_strings.push(Col::urat1_oid.to_string());
    map.insert(urat1_neighbourhood.string(), col_strings);
}
