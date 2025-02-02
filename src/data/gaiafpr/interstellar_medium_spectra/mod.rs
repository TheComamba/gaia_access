// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the interstellar_medium_spectra table.

use crate::traits::{Column, Table};

/// Table of the stacked Interstellar Medium Spectra
#[allow(non_camel_case_types)]
pub struct interstellar_medium_spectra;

impl Table for interstellar_medium_spectra {
    fn string(&self) -> String {
        "interstellar_medium_spectra".to_string()
    }
}

/// The columns in the interstellar_medium_spectra table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Solution Identifier
    solution_id,
    /// The HEALPix Identification
    healpix,
    /// Central galactic longitude of voxel
    lc,
    /// Central galactic latitude of voxel
    bc,
    /// Central heliocentric distance of voxel
    dc,
    /// Wavelength
    lambda,
    /// Normalised flux
    flux,
    /// Uncertainty in the flux parameter
    flux_uncertainty,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the interstellar_medium_spectra table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::healpix.to_string());
    col_strings.push(Col::lc.to_string());
    col_strings.push(Col::bc.to_string());
    col_strings.push(Col::dc.to_string());
    col_strings.push(Col::lambda.to_string());
    col_strings.push(Col::flux.to_string());
    col_strings.push(Col::flux_uncertainty.to_string());
    map.insert(interstellar_medium_spectra.string(), col_strings);
}
