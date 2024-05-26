// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the total_galactic_extinction_map table.

use crate::traits::{Column, Table};

/// This table provides the Total Galactic Extinction (TGE) map for extinction parameters $A_0$ describing the effective total Galactic extinction and related uncertainties at four separate HEALPix levels, namely levels 6 to 9. For further details see Section \ref{ssec:cu8par_apsis_tge} of the online documentation.
///
///
#[allow(non_camel_case_types)]
pub struct total_galactic_extinction_map;

impl Table for total_galactic_extinction_map {
    fn string(&self) -> String {
        "total_galactic_extinction_map".to_string()
    }
}

/// The columns in the total_galactic_extinction_map table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Solution Identifier
    solution_id,
    /// HEALPix identification
    healpix_id,
    /// HEALPix level used
    healpix_level,
    /// Mean $A_0$ extinction parameter
    a0,
    /// Uncertainty for the mean $A_0$
    a0_uncertainty,
    /// Minimum $A_0$ value used for the HEALPix of interest
    a0_min,
    /// Maximum $A_0$ value used for the HEALPix of interest
    a0_max,
    /// Number of tracers used
    num_tracers_used,
    /// Flag to indicate whether a given HEALPix level is the optimum (True) or not (False)
    optimum_hpx_flag,
    /// Exit status for TGE
    status,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::healpix_id.to_string());
    col_strings.push(Col::healpix_level.to_string());
    col_strings.push(Col::a0.to_string());
    col_strings.push(Col::a0_uncertainty.to_string());
    col_strings.push(Col::a0_min.to_string());
    col_strings.push(Col::a0_max.to_string());
    col_strings.push(Col::num_tracers_used.to_string());
    col_strings.push(Col::optimum_hpx_flag.to_string());
    col_strings.push(Col::status.to_string());
    map.insert(total_galactic_extinction_map.string(), col_strings);
}
