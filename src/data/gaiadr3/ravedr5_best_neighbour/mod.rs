// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the ravedr5_best_neighbour table.

use crate::traits::{Column, Table};

/// <p>RAVE DR5 BestNeighbour table lists each matched external catalogue object with its best
/// neighbour in Gaia.
/// The cross-match algorithm is not symmetric and searches RAVE DR5 sources counterparts in Gaia.<br/>
/// The best neighbour is chosen among good neighbours as the one with the highest value of
/// the figure of merit, which evaluates the ratio between two opposite models/hypotheses:
/// the counterpart candidate is a match or it is found by chance.
/// Good neighbours are nearby objects in Gaia whose position is
/// compatible within position errors with the external catalogue target.<br/>
/// The cross-match algorithm is positional and exploits the full 5
/// parameters covariance matrix of Gaia astrometric solution when available and the
/// external catalogue positions and position errors. In addition it takes into account the
/// Gaia environment using the local density.<br/>
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
pub struct ravedr5_best_neighbour;

impl Table for ravedr5_best_neighbour {
    fn string(&self) -> String {
        "ravedr5_best_neighbour".to_string()
    }
}

/// The columns in the ravedr5_best_neighbour table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Unique Gaia source identifier
    source_id,
    /// Original External Catalogue source identifier
    original_ext_source_id,
    /// Angular Distance between the two sources
    angular_distance,
    /// Cross-match algorithm flag
    xm_flag,
    /// External Catalogue source identifier
    clean_ravedr5_oid,
    /// Number of neighbours in Gaia Catalogue
    number_of_neighbours,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::original_ext_source_id.to_string());
    col_strings.push(Col::angular_distance.to_string());
    col_strings.push(Col::xm_flag.to_string());
    col_strings.push(Col::clean_ravedr5_oid.to_string());
    col_strings.push(Col::number_of_neighbours.to_string());
    map.insert(ravedr5_best_neighbour.string(), col_strings);
}
