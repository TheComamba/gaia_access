// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the sso_reflectance_spectrum table.

use crate::traits::{Column, Table};

/// This table contains the mean BP/RP reflectance spectra of asteroids computed as the ratio between the asteroid flux and an averaged solar analogue flux. In each row, the reflectance spectrum of a given asteroid is given at a given wavelength. Entries for all asteroids are concatenated into a single table.
///
#[allow(non_camel_case_types)]
pub struct sso_reflectance_spectrum;

impl Table for sso_reflectance_spectrum {
    fn string(&self) -> String {
        "sso_reflectance_spectrum".to_string()
    }
}

/// The columns in the sso_reflectance_spectrum table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Source identifier
    source_id,
    /// Solution Identifier
    solution_id,
    /// Minor Planet number
    number_mp,
    /// standard MPC denomination of the asteroid
    denomination,
    /// Nb samples in spectrum
    nb_samples,
    /// number of epoch spectra used to compute the average
    num_of_spectra,
    /// Reflectance spectrum
    reflectance_spectrum,
    /// Error in reflectance spectrum
    reflectance_spectrum_err,
    /// Internally-calibrated wavelength of reflectance spectrum
    wavelength,
    /// Reflectance spectrum value flag
    reflectance_spectrum_flag,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the sso_reflectance_spectrum table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::number_mp.to_string());
    col_strings.push(Col::denomination.to_string());
    col_strings.push(Col::nb_samples.to_string());
    col_strings.push(Col::num_of_spectra.to_string());
    col_strings.push(Col::reflectance_spectrum.to_string());
    col_strings.push(Col::reflectance_spectrum_err.to_string());
    col_strings.push(Col::wavelength.to_string());
    col_strings.push(Col::reflectance_spectrum_flag.to_string());
    map.insert(sso_reflectance_spectrum.string(), col_strings);
}
