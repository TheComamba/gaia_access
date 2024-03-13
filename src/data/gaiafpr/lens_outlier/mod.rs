// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct lens_outlier;

impl Table for lens_outlier {
    fn string(&self) -> String {
        "lens_outlier".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    outlier_id,
    ra_obs,
    dec_obs,
    g_flux_obs,
    g_flux_obs_error,
    g_mag_obs,
    epoch_obs,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::outlier_id.to_string());
    col_strings.push(Col::ra_obs.to_string());
    col_strings.push(Col::dec_obs.to_string());
    col_strings.push(Col::g_flux_obs.to_string());
    col_strings.push(Col::g_flux_obs_error.to_string());
    col_strings.push(Col::g_mag_obs.to_string());
    col_strings.push(Col::epoch_obs.to_string());
    map.insert(lens_outlier.string(), col_strings);
}