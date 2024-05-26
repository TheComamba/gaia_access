// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the phot_variable_time_series_gfov_statistical_parameters table.

use crate::traits::{Column, Table};

/// The phot_variable_time_series_gfov_statistical_parameters table.
#[allow(non_camel_case_types)]
pub struct phot_variable_time_series_gfov_statistical_parameters;

impl Table for phot_variable_time_series_gfov_statistical_parameters {
    fn string(&self) -> String {
        "phot_variable_time_series_gfov_statistical_parameters".to_string()
    }
}

/// The columns in the phot_variable_time_series_gfov_statistical_parameters table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    num_observations_processed,
    mean_obs_time,
    minimum,
    maximum,
    mean,
    median,
    range,
    std_dev,
    skewness,
    kurtosis,
    time_duration,
    median_absolute_deviation,
    abbe,
    iqr,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::num_observations_processed.to_string());
    col_strings.push(Col::mean_obs_time.to_string());
    col_strings.push(Col::minimum.to_string());
    col_strings.push(Col::maximum.to_string());
    col_strings.push(Col::mean.to_string());
    col_strings.push(Col::median.to_string());
    col_strings.push(Col::range.to_string());
    col_strings.push(Col::std_dev.to_string());
    col_strings.push(Col::skewness.to_string());
    col_strings.push(Col::kurtosis.to_string());
    col_strings.push(Col::time_duration.to_string());
    col_strings.push(Col::median_absolute_deviation.to_string());
    col_strings.push(Col::abbe.to_string());
    col_strings.push(Col::iqr.to_string());
    map.insert(
        phot_variable_time_series_gfov_statistical_parameters.string(),
        col_strings,
    );
}
