// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the gaiaedr3_spurious table.

use crate::traits::{Column, Table};

/// The gaiaedr3_spurious table.
#[allow(non_camel_case_types)]
pub struct gaiaedr3_spurious;

impl Table for gaiaedr3_spurious {
    fn string(&self) -> String {
        "gaiaedr3_spurious".to_string()
    }
}

/// The columns in the gaiaedr3_spurious table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    fidelity_v2,
    theta_arcsec_worst_source,
    norm_dg,
    dist_nearest_neighbor_at_least_m2_brighter,
    dist_nearest_neighbor_at_least_0_brighter,
    dist_nearest_neighbor_at_least_2_brighter,
    dist_nearest_neighbor_at_least_4_brighter,
    dist_nearest_neighbor_at_least_6_brighter,
    dist_nearest_neighbor_at_least_10_brighter,
    fidelity_v1,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::fidelity_v2.to_string());
    col_strings.push(Col::theta_arcsec_worst_source.to_string());
    col_strings.push(Col::norm_dg.to_string());
    col_strings.push(Col::dist_nearest_neighbor_at_least_m2_brighter.to_string());
    col_strings.push(Col::dist_nearest_neighbor_at_least_0_brighter.to_string());
    col_strings.push(Col::dist_nearest_neighbor_at_least_2_brighter.to_string());
    col_strings.push(Col::dist_nearest_neighbor_at_least_4_brighter.to_string());
    col_strings.push(Col::dist_nearest_neighbor_at_least_6_brighter.to_string());
    col_strings.push(Col::dist_nearest_neighbor_at_least_10_brighter.to_string());
    col_strings.push(Col::fidelity_v1.to_string());
    map.insert(gaiaedr3_spurious.string(), col_strings);
}
