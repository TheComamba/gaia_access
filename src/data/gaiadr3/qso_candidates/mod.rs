// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the qso_candidates table.

use crate::traits::{Column, Table};

/// This table contains parameters derived from various modules dedicated to the classification and characterisation of sources considered as QSO candidates. Together with those, the QSOs used to define the Gaia-CRF3 are also listed in this table. This table has been constructed with the intention to be complete rather than pure and, as such, it will contain a large fraction of non-genuine extragalactic sources. Purer samples can be drawn using dedicated flags or queries. Please refer to Chapter~\ref{chap:cu3qso} of the on-line documentation for details about how this table was built, its content, and for recommendations regarding its exploitation.
#[allow(non_camel_case_types)]
pub struct qso_candidates;

impl Table for qso_candidates {
    fn string(&self) -> String {
        "qso_candidates".to_string()
    }
}

/// The columns in the qso_candidates table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Solution Identifier
    solution_id,
    /// Unique source identifier (unique within a particular Data Release)
    source_id,
    /// Flag indicating if the source is part of the astrometric selection
    astrometric_selection_flag,
    /// Flag indicative of whether the source was used define the Gaia-CRF3
    gaia_crf_source,
    /// Name of best class, see table VariClassifierClassDefinition for details of the class
    vari_best_class_name,
    /// Score of the best class
    vari_best_class_score,
    /// Fractional variability in the G band
    fractional_variability_g,
    /// Index of the first-order structure function in the G band
    structure_function_index,
    /// Standard deviation of the index of the structure function
    structure_function_index_scatter,
    /// Quasar variability metric in the G band
    qso_variability,
    /// Non-quasar variability metric in the G band
    non_qso_variability,
    /// Membership score (0=lowest,1=highest) of source to be of AGN type
    vari_agn_membership_score,
    /// Probability from DSC-Combmod of being a quasar (data used: BP/RP spectrum, photometry, astrometry)
    classprob_dsc_combmod_quasar,
    /// Probability from DSC-Combmod of being a galaxy (data used: BP/RP spectrum, photometry, astrometry)
    classprob_dsc_combmod_galaxy,
    /// Class assigned by DSC based on the probability from its Combmod classifier
    classlabel_dsc,
    /// Class assigned by DSC based on the probability from its Specmod and Allosmod classifiers
    classlabel_dsc_joint,
    /// Class assigned by OA the neuron that represents the source
    classlabel_oa,
    /// Redshift from QSOC
    redshift_qsoc,
    /// Redshift lower confidence level from QSOC
    redshift_qsoc_lower,
    /// Redshift upper confidence level from QSOC
    redshift_qsoc_upper,
    /// Value of the cross-correlation function used to derive the redshift from QSOC, relative to the maximum value
    ccfratio_qsoc,
    /// Redshift zscore from QSOC
    zscore_qsoc,
    /// Processing flags for the analysis based on BP/RP Spectra from QSOC
    flags_qsoc,
    /// Number of transits used for the morphological analysis
    n_transits,
    /// Fitted intensity of the quasar at its center
    intensity_quasar,
    /// Error on the fitted intensity of the quasar at its center
    intensity_quasar_error,
    /// Fitted intensity of the host galaxy at the effective radius
    intensity_hostgalaxy,
    /// Error on the fitted intensity of the host galaxy at effective radius
    intensity_hostgalaxy_error,
    /// Fitted effective radius of the host galaxy
    radius_hostgalaxy,
    /// Error on the fitted effective radius of the host galaxy
    radius_hostgalaxy_error,
    /// Fitted sersic Index
    sersic_index,
    /// Error on the fitted sersic Index
    sersic_index_error,
    /// Fitted ellipticity of the host galaxy
    ellipticity_hostgalaxy,
    /// Error on the fitted ellipticity of the host galaxy
    ellipticity_hostgalaxy_error,
    /// Fitted position angle of the host galaxy
    posangle_hostgalaxy,
    /// Error on the fitted position angle of the host galaxy
    posangle_hostgalaxy_error,
    /// Flag indicating whether a host galaxy has been detected
    host_galaxy_detected,
    /// L2 norm for the fitted Sersic profile
    l2_norm,
    /// Vector form of the upper triangle of the correlation matrix for the fitted morphological parameters
    morph_params_corr_vec,
    /// Flag indicative of processing or scientific quality for the morphological parameters fitting
    host_galaxy_flag,
    /// Bit indicative of whether the input data from a given module met the source list eligibility criteria for the source of interest
    source_selection_flags,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the qso_candidates table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::astrometric_selection_flag.to_string());
    col_strings.push(Col::gaia_crf_source.to_string());
    col_strings.push(Col::vari_best_class_name.to_string());
    col_strings.push(Col::vari_best_class_score.to_string());
    col_strings.push(Col::fractional_variability_g.to_string());
    col_strings.push(Col::structure_function_index.to_string());
    col_strings.push(Col::structure_function_index_scatter.to_string());
    col_strings.push(Col::qso_variability.to_string());
    col_strings.push(Col::non_qso_variability.to_string());
    col_strings.push(Col::vari_agn_membership_score.to_string());
    col_strings.push(Col::classprob_dsc_combmod_quasar.to_string());
    col_strings.push(Col::classprob_dsc_combmod_galaxy.to_string());
    col_strings.push(Col::classlabel_dsc.to_string());
    col_strings.push(Col::classlabel_dsc_joint.to_string());
    col_strings.push(Col::classlabel_oa.to_string());
    col_strings.push(Col::redshift_qsoc.to_string());
    col_strings.push(Col::redshift_qsoc_lower.to_string());
    col_strings.push(Col::redshift_qsoc_upper.to_string());
    col_strings.push(Col::ccfratio_qsoc.to_string());
    col_strings.push(Col::zscore_qsoc.to_string());
    col_strings.push(Col::flags_qsoc.to_string());
    col_strings.push(Col::n_transits.to_string());
    col_strings.push(Col::intensity_quasar.to_string());
    col_strings.push(Col::intensity_quasar_error.to_string());
    col_strings.push(Col::intensity_hostgalaxy.to_string());
    col_strings.push(Col::intensity_hostgalaxy_error.to_string());
    col_strings.push(Col::radius_hostgalaxy.to_string());
    col_strings.push(Col::radius_hostgalaxy_error.to_string());
    col_strings.push(Col::sersic_index.to_string());
    col_strings.push(Col::sersic_index_error.to_string());
    col_strings.push(Col::ellipticity_hostgalaxy.to_string());
    col_strings.push(Col::ellipticity_hostgalaxy_error.to_string());
    col_strings.push(Col::posangle_hostgalaxy.to_string());
    col_strings.push(Col::posangle_hostgalaxy_error.to_string());
    col_strings.push(Col::host_galaxy_detected.to_string());
    col_strings.push(Col::l2_norm.to_string());
    col_strings.push(Col::morph_params_corr_vec.to_string());
    col_strings.push(Col::host_galaxy_flag.to_string());
    col_strings.push(Col::source_selection_flags.to_string());
    map.insert(qso_candidates.string(), col_strings);
}
