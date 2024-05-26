// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the phot_variable_time_series_gfov table.

use crate::traits::{Column, Table};

/// The phot_variable_time_series_gfov table.
#[allow(non_camel_case_types)]
pub struct phot_variable_time_series_gfov;

impl Table for phot_variable_time_series_gfov {
    fn string(&self) -> String {
        "phot_variable_time_series_gfov".to_string()
    }
}

/// The columns in the phot_variable_time_series_gfov table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    observation_time,
    g_flux,
    g_flux_error,
    g_magnitude,
    rejected_by_variability_processing,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::observation_time.to_string());
    col_strings.push(Col::g_flux.to_string());
    col_strings.push(Col::g_flux_error.to_string());
    col_strings.push(Col::g_magnitude.to_string());
    col_strings.push(Col::rejected_by_variability_processing.to_string());
    map.insert(phot_variable_time_series_gfov.string(), col_strings);
}
