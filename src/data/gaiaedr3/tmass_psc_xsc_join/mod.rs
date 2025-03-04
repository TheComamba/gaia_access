// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the tmass_psc_xsc_join table.

use crate::traits::{Column, Table};

/// Convenience table to be used to join 2MASS PSC+XSC catalogue with the
/// cross-match results. The table links the external catalogue original
/// sourceId (originalExtSourceId) to the corresponding the additional
/// numerical identifier (cleanTmassPscXscOid).
/// Both originalExtSourceId and cleanTmassPscXscOid are present in the
/// cross-match output tables (tmassPscXscBestNeighbour and
/// tmassPscXscNeighbourhood). However, in case there are suspected
/// duplicates in the external catalogue, different originalExtSourceId will
/// correspond to the same cleanTmassPscXscOid.
/// In the cross-match output table only the originalExtSourceId of the
/// source with the best astrometry among the suspected duplicates will be
/// listed.
/// In practice, users may use the originalExtSourceId in the original
/// catalogue to find the matching source with the best astrometry. Users
/// interested to find all matching suspected duplicates should instead use
/// the cleanTmassPscXscOid in the join with the cross-match result tables.
/// See Documentation, [chap:crossmatch], for more details on the duplicates
/// in the external catalogues and their treatment in the cross-match
/// computations.
#[allow(non_camel_case_types)]
pub struct tmass_psc_xsc_join;

impl Table for tmass_psc_xsc_join {
    fn string(&self) -> String {
        "tmass_psc_xsc_join".to_string()
    }
}

/// The columns in the tmass_psc_xsc_join table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Original 2MASS PSC source identifier
    original_psc_source_id,
    /// Original 2MASS XSC source identifier
    original_xsc_source_id,
    /// External Catalogue source identifier
    clean_tmass_psc_xsc_oid,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the tmass_psc_xsc_join table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::original_psc_source_id.to_string());
    col_strings.push(Col::original_xsc_source_id.to_string());
    col_strings.push(Col::clean_tmass_psc_xsc_oid.to_string());
    map.insert(tmass_psc_xsc_join.string(), col_strings);
}
