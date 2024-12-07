// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the vari_cepheid table.

use crate::traits::{Column, Table};

/// This table describes the Cepheid stars.
#[allow(non_camel_case_types)]
pub struct vari_cepheid;

impl Table for vari_cepheid {
    fn string(&self) -> String {
        "vari_cepheid".to_string()
    }
}

/// The columns in the vari_cepheid table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Solution Identifier
    solution_id,
    /// Unique source identifier
    source_id,
    /// Period corresponding to the fundamental pulsation mode (for multi mode pulsators) in  the G band time series
    pf,
    /// Uncertainty of the pf period
    pf_error,
    /// Period corresponding to the first overtone pulsation mode (for multi mode pulsators) in the G band time series
    p1_o,
    /// Uncertainty of the p1O period
    p1_o_error,
    /// Period corresponding to the second overtone pulsation mode (for multi mode pulsators) in  the G band time series
    p2_o,
    /// Uncertainty of the p2O period
    p2_o_error,
    /// Period corresponding to the third overtone pulsation mode (for multi mode pulsators) in  the G band time series
    p3_o,
    /// Uncertainty of the p3O period
    p3_o_error,
    /// Epoch of the maximum of the light curve in the G band
    epoch_g,
    /// Uncertainty on the epoch parameter epochG
    epoch_g_error,
    /// Epoch of the maximum of the light curve in the BP band
    epoch_bp,
    /// Uncertainty on the epoch parameter epochBp
    epoch_bp_error,
    /// Epoch of the maximum of the light curve in the RP band
    epoch_rp,
    /// Uncertainty on the epoch parameter epochRp
    epoch_rp_error,
    /// Intensity-averaged magnitude in the G band
    int_average_g,
    /// Uncertainty on intAverageG parameter
    int_average_g_error,
    /// intensity-averaged magnitude in the BP band
    int_average_bp,
    /// Uncertainty on intAverageBp parameter
    int_average_bp_error,
    /// intensity-averaged magnitude in the RP band
    int_average_rp,
    /// Uncertainty on intAverageRp parameter
    int_average_rp_error,
    /// Peak-to-peak amplitude of the G band light curve
    peak_to_peak_g,
    /// Uncertainty on the peakToPeakG parameter
    peak_to_peak_g_error,
    /// Peak-to-peak amplitude of the BP band light curve
    peak_to_peak_bp,
    /// Uncertainty on the peakToPeakBp parameter
    peak_to_peak_bp_error,
    /// Peak-to-peak amplitude of the RP band light curve
    peak_to_peak_rp,
    /// Uncertainty on the peakToPeakRp parameter
    peak_to_peak_rp_error,
    /// Metallicity of the star from the Fourier parameters of the light curve
    metallicity,
    /// Uncertainty of the metallicity parameter
    metallicity_error,
    /// Fourier decomposition parameter r21G: A2/A1 (for G band)
    r21_g,
    /// Uncertainty on the r21G parameter: A2/A1 (for G band)
    r21_g_error,
    /// Fourier decomposition parameter r31G: A3/A1 (for G band)
    r31_g,
    /// Uncertainty on the r31G parameter: A3/A1 (for G band)
    r31_g_error,
    /// Fourier decomposition parameter phi21G: phi2 - 2*phi1 (for G band)
    phi21_g,
    /// Uncertainty on the phi21G parameter: phi2 - 2*phi1 (for G band)
    phi21_g_error,
    /// Fourier decomposition parameter phi31G: phi3 - 3*phi1 (for G band)
    phi31_g,
    /// Uncertainty on the phi31G parameter: phi3 - 3*phi1 (for G band)
    phi31_g_error,
    /// Number of G FoV epochs used in the fitting algorithm
    num_clean_epochs_g,
    /// Number of BP epochs used in the fitting algorithm
    num_clean_epochs_bp,
    /// Number of RP epochs used in the fitting algorithm
    num_clean_epochs_rp,
    /// Interstellar absorption in the G-band
    g_absorption,
    /// Error on the interstellar absorption in the G-band
    g_absorption_error,
    /// Best type classification estimate out of: {"DCEP", "T2CEP", "ACEP"}
    type_best_classification,
    /// Best subclassification estimate for typeBestClassification="T2CEP" out of: {"BL_HER", "W_VIR","RV_TAU"}
    type2_best_sub_classification,
    /// Best mode classification estimate   out of: { "FUNDAMENTAL", "FIRST_OVERTONE","SECOND_OVERTONE","MULTI,"UNDEFINED","NOT_APPLICABLE"}
    mode_best_classification,
    /// Best multi mode DCEP classification out of: { "F/1O", "F/2O","1O/2O","1O/3O,"2O/3O",F/1O/2O,"1O/2O/3O"}
    multi_mode_best_classification,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the vari_cepheid table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::pf.to_string());
    col_strings.push(Col::pf_error.to_string());
    col_strings.push(Col::p1_o.to_string());
    col_strings.push(Col::p1_o_error.to_string());
    col_strings.push(Col::p2_o.to_string());
    col_strings.push(Col::p2_o_error.to_string());
    col_strings.push(Col::p3_o.to_string());
    col_strings.push(Col::p3_o_error.to_string());
    col_strings.push(Col::epoch_g.to_string());
    col_strings.push(Col::epoch_g_error.to_string());
    col_strings.push(Col::epoch_bp.to_string());
    col_strings.push(Col::epoch_bp_error.to_string());
    col_strings.push(Col::epoch_rp.to_string());
    col_strings.push(Col::epoch_rp_error.to_string());
    col_strings.push(Col::int_average_g.to_string());
    col_strings.push(Col::int_average_g_error.to_string());
    col_strings.push(Col::int_average_bp.to_string());
    col_strings.push(Col::int_average_bp_error.to_string());
    col_strings.push(Col::int_average_rp.to_string());
    col_strings.push(Col::int_average_rp_error.to_string());
    col_strings.push(Col::peak_to_peak_g.to_string());
    col_strings.push(Col::peak_to_peak_g_error.to_string());
    col_strings.push(Col::peak_to_peak_bp.to_string());
    col_strings.push(Col::peak_to_peak_bp_error.to_string());
    col_strings.push(Col::peak_to_peak_rp.to_string());
    col_strings.push(Col::peak_to_peak_rp_error.to_string());
    col_strings.push(Col::metallicity.to_string());
    col_strings.push(Col::metallicity_error.to_string());
    col_strings.push(Col::r21_g.to_string());
    col_strings.push(Col::r21_g_error.to_string());
    col_strings.push(Col::r31_g.to_string());
    col_strings.push(Col::r31_g_error.to_string());
    col_strings.push(Col::phi21_g.to_string());
    col_strings.push(Col::phi21_g_error.to_string());
    col_strings.push(Col::phi31_g.to_string());
    col_strings.push(Col::phi31_g_error.to_string());
    col_strings.push(Col::num_clean_epochs_g.to_string());
    col_strings.push(Col::num_clean_epochs_bp.to_string());
    col_strings.push(Col::num_clean_epochs_rp.to_string());
    col_strings.push(Col::g_absorption.to_string());
    col_strings.push(Col::g_absorption_error.to_string());
    col_strings.push(Col::type_best_classification.to_string());
    col_strings.push(Col::type2_best_sub_classification.to_string());
    col_strings.push(Col::mode_best_classification.to_string());
    col_strings.push(Col::multi_mode_best_classification.to_string());
    map.insert(vari_cepheid.string(), col_strings);
}
