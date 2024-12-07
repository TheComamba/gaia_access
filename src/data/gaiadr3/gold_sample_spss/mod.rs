// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the gold_sample_spss table.

use crate::traits::{Column, Table};

/// Recommended parameters for the Gaia Spectro-Photometric Standard stars \citep{2021MNRAS.503.3660P}, selected as described in \cite{DR3-DPACP-123}, from the available parameters produced by CU8 \citep{DR3-DPACP-157} and CU6 \citep{DR3-DPACP-159}.
#[allow(non_camel_case_types)]
pub struct gold_sample_spss;

impl Table for gold_sample_spss {
    fn string(&self) -> String {
        "gold_sample_spss".to_string()
    }
}

/// The columns in the gold_sample_spss table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Unique source identifier (unique within a particular Data Release)
    source_id,
    /// SPSS identifier
    spss_id,
    /// SPSS adopted name
    spss_name,
    /// Spectral type
    spectraltype,
    /// Binary flag
    bin_flag,
    /// Variability flag
    var_flag,
    /// Best effective temperature
    teff,
    /// Uncertainty of the best effective temperature
    teff_error,
    /// Best surface gravity
    logg,
    /// Uncertainty on the best surface gravity
    logg_error,
    /// Best iron abundance
    feh,
    /// Uncertainty on the best iron abundance
    feh_error,
    /// Best alpha enhancement
    alphafe,
    /// Uncertainty on the best alpha enhancement
    alphafe_error,
    /// Best radius
    radius,
    /// Uncertainty on the best radius
    radius_error,
    /// Best luminosity
    lum,
    /// Uncertainty on the best luminosity
    lum_error,
    /// Best mass
    mass,
    /// Uncertainty on the best mass
    mass_error,
    /// Best age
    age,
    /// Uncertainty on the best age
    age_error,
    /// Best extinction at 547.7 nm
    azero,
    /// Uncertainty on the best extinction at 547.7 nm
    azero_error,
    /// Extinction in the G band
    ag,
    /// Uncertainty on the extinction in the G band
    ag_error,
    /// Reddening in BP-RP
    ebpminrp,
    /// Uncertainty on the reddening in BP-RP
    ebpminrp_error,
    /// Best distance
    distancepc,
    /// Uncertainty on the best distance
    distancepc_error,
    /// Radial velocity
    radial_velocity,
    /// Uncertainty on the radial velocity
    radial_velocity_error,
    /// Best projected rotation
    vsini,
    /// Uncertainty on the best projected rotation
    vsin_error,
    /// Free text notes on this object
    notes,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the gold_sample_spss table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::spss_id.to_string());
    col_strings.push(Col::spss_name.to_string());
    col_strings.push(Col::spectraltype.to_string());
    col_strings.push(Col::bin_flag.to_string());
    col_strings.push(Col::var_flag.to_string());
    col_strings.push(Col::teff.to_string());
    col_strings.push(Col::teff_error.to_string());
    col_strings.push(Col::logg.to_string());
    col_strings.push(Col::logg_error.to_string());
    col_strings.push(Col::feh.to_string());
    col_strings.push(Col::feh_error.to_string());
    col_strings.push(Col::alphafe.to_string());
    col_strings.push(Col::alphafe_error.to_string());
    col_strings.push(Col::radius.to_string());
    col_strings.push(Col::radius_error.to_string());
    col_strings.push(Col::lum.to_string());
    col_strings.push(Col::lum_error.to_string());
    col_strings.push(Col::mass.to_string());
    col_strings.push(Col::mass_error.to_string());
    col_strings.push(Col::age.to_string());
    col_strings.push(Col::age_error.to_string());
    col_strings.push(Col::azero.to_string());
    col_strings.push(Col::azero_error.to_string());
    col_strings.push(Col::ag.to_string());
    col_strings.push(Col::ag_error.to_string());
    col_strings.push(Col::ebpminrp.to_string());
    col_strings.push(Col::ebpminrp_error.to_string());
    col_strings.push(Col::distancepc.to_string());
    col_strings.push(Col::distancepc_error.to_string());
    col_strings.push(Col::radial_velocity.to_string());
    col_strings.push(Col::radial_velocity_error.to_string());
    col_strings.push(Col::vsini.to_string());
    col_strings.push(Col::vsin_error.to_string());
    col_strings.push(Col::notes.to_string());
    map.insert(gold_sample_spss.string(), col_strings);
}
