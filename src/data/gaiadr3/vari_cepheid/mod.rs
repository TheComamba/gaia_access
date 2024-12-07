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
    /// Period corresponding to the fundamental pulsation mode in the G band time series
    pf,
    /// Uncertainty of the \texttt{pf} period
    pf_error,
    /// Period corresponding to the first overtone pulsation mode in the G band time series
    p1_o,
    /// Uncertainty of the \texttt{p1O} period
    p1_o_error,
    /// Epoch of the maximum of the light curve in the G band
    epoch_g,
    /// Uncertainty on the epoch parameter \texttt{epochG}
    epoch_g_error,
    /// Epoch of the maximum of the light curve in the BP band
    epoch_bp,
    /// Uncertainty on the epoch parameter \texttt{epochBp}
    epoch_bp_error,
    /// Epoch of the maximum of the light curve in the RP band
    epoch_rp,
    /// Uncertainty on the epoch parameter \texttt{epochRp}
    epoch_rp_error,
    /// Epoch of the minimum of the radial velocity curve
    epoch_rv,
    /// Uncertainty on the epoch parameter \texttt{epochRv}
    epoch_rv_error,
    /// Intensity-averaged magnitude in the G band
    int_average_g,
    /// Uncertainty on \texttt{intAverageG} parameter
    int_average_g_error,
    /// Intensity-averaged magnitude in the BP band
    int_average_bp,
    /// Uncertainty on \texttt{intAverageBp} parameter
    int_average_bp_error,
    /// Intensity-averaged magnitude in the RP band
    int_average_rp,
    /// Uncertainty on \texttt{intAverageRp} parameter
    int_average_rp_error,
    /// Mean radial velocity
    average_rv,
    /// Uncertainty on \texttt{averageRv} parameter
    average_rv_error,
    /// Peak-to-peak amplitude of the G band light curve
    peak_to_peak_g,
    /// Uncertainty on the \texttt{peakToPeakG} parameter
    peak_to_peak_g_error,
    /// Peak-to-peak amplitude of the BP band light curve
    peak_to_peak_bp,
    /// Uncertainty on the \texttt{peakToPeakBp} parameter
    peak_to_peak_bp_error,
    /// Peak-to-peak amplitude of the RP band light curve
    peak_to_peak_rp,
    /// Uncertainty on the \texttt{peakToPeakRp} parameter
    peak_to_peak_rp_error,
    /// Peak-to-peak amplitude of the radial velocity curve
    peak_to_peak_rv,
    /// Uncertainty on the \texttt{peakToPeakRv} parameter
    peak_to_peak_rv_error,
    /// Metallicity of the star from the Fourier parameters of the light curve
    metallicity,
    /// Uncertainty of the \texttt{metallicity} parameter
    metallicity_error,
    /// Fourier decomposition parameter \texttt{r21G}: A2/A1 (for G band)
    r21_g,
    /// Uncertainty on the \texttt{r21G} parameter: A2/A1 (for G band)
    r21_g_error,
    /// Fourier decomposition parameter \texttt{r31G}: A3/A1 (for G band)
    r31_g,
    /// Uncertainty on the \texttt{r31G} parameter: A3/A1 (for G band)
    r31_g_error,
    /// Fourier decomposition parameter \texttt{phi21G}: phi2 - 2*phi1 (for G band)
    phi21_g,
    /// Uncertainty on the \texttt{phi21G} parameter: phi2 - 2*phi1 (for G band)
    phi21_g_error,
    /// Fourier decomposition parameter \texttt{phi31G}: phi3 - 3*phi1 (for G band)
    phi31_g,
    /// Uncertainty on the \texttt{phi31G} parameter: phi3 - 3*phi1 (for G band)
    phi31_g_error,
    /// Number of G FoV epochs used in the fitting algorithm
    num_clean_epochs_g,
    /// Number of BP epochs used in the fitting algorithm
    num_clean_epochs_bp,
    /// Number of RP epochs used in the fitting algorithm
    num_clean_epochs_rp,
    /// Number of radial velocity epochs used in the fitting algorithm
    num_clean_epochs_rv,
    /// Zero point (mag) of the final model of the G band light curve
    zp_mag_g,
    /// Zero point (mag) of the final model of the BP band light curve
    zp_mag_bp,
    /// Zero point (mag) of the final model of the RP band light curve
    zp_mag_rp,
    /// Number of harmonics used to model the first periodicity of the G-band light curve
    num_harmonics_for_p1_g,
    /// Number of harmonics used to model the first periodicity of the BP-band light curve
    num_harmonics_for_p1_bp,
    /// Number of harmonics used to model the first periodicity of the RP-band light curve
    num_harmonics_for_p1_rp,
    /// Number of harmonics used to model the first periodicity of the radial velocity curve
    num_harmonics_for_p1_rv,
    /// Reference time of the Fourier modelled G-band light curve
    reference_time_g,
    /// Reference time of the Fourier modelled BP-band light curve
    reference_time_bp,
    /// Reference time of the Fourier modelled RP-band light curve
    reference_time_rp,
    /// Reference time of the Fourier modelled radial velocity curve
    reference_time_rv,
    /// First frequency of the non-linear Fourier modelling
    fund_freq1,
    /// Error of the first frequency of the non-linear Fourier modelling
    fund_freq1_error,
    /// Second frequency of the non-linear Fourier modelling in the G band
    fund_freq2,
    /// Error of the second frequency of the non-linear Fourier modelling in the G band
    fund_freq2_error,
    /// Amplitudes of the Fourier model for the first frequency in the G band
    fund_freq1_harmonic_ampl_g,
    /// Errors of the amplitudes of the Fourier model for the first frequency in the G band
    fund_freq1_harmonic_ampl_g_error,
    /// Phases of the Fourier model for the first frequency in the G band
    fund_freq1_harmonic_phase_g,
    /// Errors of the phases of the Fourier model for the first frequency in the G band
    fund_freq1_harmonic_phase_g_error,
    /// Amplitudes of the Fourier model for the first frequency in the BP band
    fund_freq1_harmonic_ampl_bp,
    /// Errors of the amplitudes of the Fourier model for the first frequency in the BP band
    fund_freq1_harmonic_ampl_bp_error,
    /// Phases of the Fourier model for the first frequency in the BP band
    fund_freq1_harmonic_phase_bp,
    /// Errors of the phases of the Fourier model for the first frequency in the BP band
    fund_freq1_harmonic_phase_bp_error,
    /// Amplitudes of the Fourier model for the first frequency in the RP band
    fund_freq1_harmonic_ampl_rp,
    /// Errors of the amplitudes of the Fourier model for the first frequency in the RP band
    fund_freq1_harmonic_ampl_rp_error,
    /// Phases of the Fourier model for the first frequency in the RP band
    fund_freq1_harmonic_phase_rp,
    /// Errors of the phases of the Fourier model for the first frequency in the RP band
    fund_freq1_harmonic_phase_rp_error,
    /// Amplitudes of the Fourier model for the first frequency of the radial velocity curve
    fund_freq1_harmonic_ampl_rv,
    /// Errors of the amplitudes of the Fourier model for the first frequency of the radial velocity curve
    fund_freq1_harmonic_ampl_rv_error,
    /// Phases of the Fourier model for the first frequency of the radial velocity curve
    fund_freq1_harmonic_phase_rv,
    /// Errors of the phases of the Fourier model for the first frequency of the radial velocity curve
    fund_freq1_harmonic_phase_rv_error,
    /// Period corresponding to the second overtone pulsation mode (for multi mode pulsators) in  the G band time series
    p2_o,
    /// Uncertainty of the p2O period
    p2_o_error,
    /// Best type classification estimate out of: `DCEP, `T2CEP, `ACEP
    type_best_classification,
    /// Best subclassification estimate for typeBestClassification=`T2CEP out of: `BL_HER, `W_VIR,`RV_TAU
    type2_best_sub_classification,
    /// Best mode classification estimate   out of: `FUNDAMENTAL, `FIRST_OVERTONE, `SECOND_OVERTONE, `MULTI,`UNDEFINED, `NOT_APPLICABLE
    mode_best_classification,
    /// Best multi mode DCEP classification out of: `F/1O, `F/2O, `1O/2O, `1O/3O, `2O/3O, `F/1O/2O, `1O/2O/3O
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
    col_strings.push(Col::epoch_g.to_string());
    col_strings.push(Col::epoch_g_error.to_string());
    col_strings.push(Col::epoch_bp.to_string());
    col_strings.push(Col::epoch_bp_error.to_string());
    col_strings.push(Col::epoch_rp.to_string());
    col_strings.push(Col::epoch_rp_error.to_string());
    col_strings.push(Col::epoch_rv.to_string());
    col_strings.push(Col::epoch_rv_error.to_string());
    col_strings.push(Col::int_average_g.to_string());
    col_strings.push(Col::int_average_g_error.to_string());
    col_strings.push(Col::int_average_bp.to_string());
    col_strings.push(Col::int_average_bp_error.to_string());
    col_strings.push(Col::int_average_rp.to_string());
    col_strings.push(Col::int_average_rp_error.to_string());
    col_strings.push(Col::average_rv.to_string());
    col_strings.push(Col::average_rv_error.to_string());
    col_strings.push(Col::peak_to_peak_g.to_string());
    col_strings.push(Col::peak_to_peak_g_error.to_string());
    col_strings.push(Col::peak_to_peak_bp.to_string());
    col_strings.push(Col::peak_to_peak_bp_error.to_string());
    col_strings.push(Col::peak_to_peak_rp.to_string());
    col_strings.push(Col::peak_to_peak_rp_error.to_string());
    col_strings.push(Col::peak_to_peak_rv.to_string());
    col_strings.push(Col::peak_to_peak_rv_error.to_string());
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
    col_strings.push(Col::num_clean_epochs_rv.to_string());
    col_strings.push(Col::zp_mag_g.to_string());
    col_strings.push(Col::zp_mag_bp.to_string());
    col_strings.push(Col::zp_mag_rp.to_string());
    col_strings.push(Col::num_harmonics_for_p1_g.to_string());
    col_strings.push(Col::num_harmonics_for_p1_bp.to_string());
    col_strings.push(Col::num_harmonics_for_p1_rp.to_string());
    col_strings.push(Col::num_harmonics_for_p1_rv.to_string());
    col_strings.push(Col::reference_time_g.to_string());
    col_strings.push(Col::reference_time_bp.to_string());
    col_strings.push(Col::reference_time_rp.to_string());
    col_strings.push(Col::reference_time_rv.to_string());
    col_strings.push(Col::fund_freq1.to_string());
    col_strings.push(Col::fund_freq1_error.to_string());
    col_strings.push(Col::fund_freq2.to_string());
    col_strings.push(Col::fund_freq2_error.to_string());
    col_strings.push(Col::fund_freq1_harmonic_ampl_g.to_string());
    col_strings.push(Col::fund_freq1_harmonic_ampl_g_error.to_string());
    col_strings.push(Col::fund_freq1_harmonic_phase_g.to_string());
    col_strings.push(Col::fund_freq1_harmonic_phase_g_error.to_string());
    col_strings.push(Col::fund_freq1_harmonic_ampl_bp.to_string());
    col_strings.push(Col::fund_freq1_harmonic_ampl_bp_error.to_string());
    col_strings.push(Col::fund_freq1_harmonic_phase_bp.to_string());
    col_strings.push(Col::fund_freq1_harmonic_phase_bp_error.to_string());
    col_strings.push(Col::fund_freq1_harmonic_ampl_rp.to_string());
    col_strings.push(Col::fund_freq1_harmonic_ampl_rp_error.to_string());
    col_strings.push(Col::fund_freq1_harmonic_phase_rp.to_string());
    col_strings.push(Col::fund_freq1_harmonic_phase_rp_error.to_string());
    col_strings.push(Col::fund_freq1_harmonic_ampl_rv.to_string());
    col_strings.push(Col::fund_freq1_harmonic_ampl_rv_error.to_string());
    col_strings.push(Col::fund_freq1_harmonic_phase_rv.to_string());
    col_strings.push(Col::fund_freq1_harmonic_phase_rv_error.to_string());
    col_strings.push(Col::p2_o.to_string());
    col_strings.push(Col::p2_o_error.to_string());
    col_strings.push(Col::type_best_classification.to_string());
    col_strings.push(Col::type2_best_sub_classification.to_string());
    col_strings.push(Col::mode_best_classification.to_string());
    col_strings.push(Col::multi_mode_best_classification.to_string());
    map.insert(vari_cepheid.string(), col_strings);
}
