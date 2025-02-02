// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the dr1_neighbourhood table.

use crate::traits::{Column, Table};

/// Users wishing to look up the DR2 record for an astrophysical source
/// identified in DR1 must NOT simply extract the record from DR2 having the
/// same source identifier.
///
/// As described in the detailed description of attribute designation in
/// GaiaSource it is not guaranteed that the same astronomical source will
/// always have the same source identifier in different Data Releases. Hence
/// the only safe way to compare source records between different Data
/// Releases in general is to check the records of proximal source(s) in the
/// same small part of the sky. This table provides the means to do this via
/// a precomputed crossmatch of such sources, taking into account the proper
/// motions available at DR2.
///
/// Within the neighbourhood of a given DR2 source there may be none, one or
/// (rarely) several possible counterparts in DR1 indicated by rows in this
/// table. This occasional source confusion was introduced during the DR1
/// processing which used an earlier version of the software for matching of
/// transit observations to unique astrophysical sources. The subsequent
/// merging, splitting and deletion of identifiers introduced at DR1 during
/// the DR2 processing means there is no guaranteed one–to–one
/// correspondence in source identifiers between the releases.
///
/// For more details of the procedure used to create this crossmatch, see
/// Section [chap:xmdr1] in the online documentation.
#[allow(non_camel_case_types)]
pub struct dr1_neighbourhood;

impl Table for dr1_neighbourhood {
    fn string(&self) -> String {
        "dr1_neighbourhood".to_string()
    }
}

/// The columns in the dr1_neighbourhood table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Source identifier in Gaia DR1
    dr1_source_id,
    /// Source identifier in Gaia DR2
    dr2_source_id,
    /// Angular distance between the two sources
    angular_distance,
    /// G band magnitude difference between the sources
    magnitude_difference,
    /// Rank metric indicating relative probability of a good match
    rank,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the dr1_neighbourhood table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::dr1_source_id.to_string());
    col_strings.push(Col::dr2_source_id.to_string());
    col_strings.push(Col::angular_distance.to_string());
    col_strings.push(Col::magnitude_difference.to_string());
    col_strings.push(Col::rank.to_string());
    map.insert(dr1_neighbourhood.string(), col_strings);
}
