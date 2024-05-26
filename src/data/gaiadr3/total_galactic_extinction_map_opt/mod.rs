// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the total_galactic_extinction_map_opt table.

use crate::traits::{Column, Table};

/// The total_galactic_extinction_map_opt table.
#[allow(non_camel_case_types)]
pub struct total_galactic_extinction_map_opt;

impl Table for total_galactic_extinction_map_opt {
    fn string(&self) -> String {
        "total_galactic_extinction_map_opt".to_string()
    }
}

/// The columns in the total_galactic_extinction_map_opt table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    healpix_id,
    a0,
    a0_uncertainty,
    num_tracers_used,
    status,
    optimum_hpx_level,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::healpix_id.to_string());
    col_strings.push(Col::a0.to_string());
    col_strings.push(Col::a0_uncertainty.to_string());
    col_strings.push(Col::num_tracers_used.to_string());
    col_strings.push(Col::status.to_string());
    col_strings.push(Col::optimum_hpx_level.to_string());
    map.insert(total_galactic_extinction_map_opt.string(), col_strings);
}
