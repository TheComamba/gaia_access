// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the aux_sso_orbit_residuals table.

use crate::traits::{Column, Table};

/// The aux_sso_orbit_residuals table.
#[allow(non_camel_case_types)]
pub struct aux_sso_orbit_residuals;

impl Table for aux_sso_orbit_residuals {
    fn string(&self) -> String {
        "aux_sso_orbit_residuals".to_string()
    }
}

/// The columns in the aux_sso_orbit_residuals table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    transit_id,
    observation_id,
    number_mp,
    epoch,
    residual_ra,
    residual_dec,
    residual_al,
    residual_ac,
    selected,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::transit_id.to_string());
    col_strings.push(Col::observation_id.to_string());
    col_strings.push(Col::number_mp.to_string());
    col_strings.push(Col::epoch.to_string());
    col_strings.push(Col::residual_ra.to_string());
    col_strings.push(Col::residual_dec.to_string());
    col_strings.push(Col::residual_al.to_string());
    col_strings.push(Col::residual_ac.to_string());
    col_strings.push(Col::selected.to_string());
    map.insert(aux_sso_orbit_residuals.string(), col_strings);
}
