// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the gaia_source table.

use crate::traits::{Column, Table};

/// This table has an entry for every Gaia observed source as published with this data release. It contains the basic source parameters, in their final state as processed by the Gaia Data Processing and Analysis Consortium from the raw data coming from the spacecraft. The table is complemented with others containing information specific to certain kinds of objects (e.g.~Solar--system objects, non--single stars, variables etc.) and value--added processing (e.g.~astrophysical parameters etc.). Further array data types (spectra, epoch measurements) are presented separately via Datalink resources.
#[allow(non_camel_case_types)]
pub struct gaia_source;

impl Table for gaia_source {
    fn string(&self) -> String {
        "gaia_source".to_string()
    }
}

/// The columns in the gaia_source table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Solution Identifier
    solution_id,
    /// Unique source designation (unique across all Data Releases)
    designation,
    /// Unique source identifier (unique within a particular Data Release)
    source_id,
    /// Random index for use when selecting subsets
    random_index,
    /// Reference epoch
    ref_epoch,
    /// Right ascension
    ra,
    /// Standard error of right ascension
    ra_error,
    /// Declination
    dec,
    /// Standard error of declination
    dec_error,
    /// Parallax
    parallax,
    /// Standard error of parallax
    parallax_error,
    /// Parallax divided by its standard error
    parallax_over_error,
    /// Total proper motion
    pm,
    /// Proper motion in right ascension direction
    pmra,
    /// Standard error of proper motion in right ascension direction
    pmra_error,
    /// Proper motion in declination direction
    pmdec,
    /// Standard error of proper motion in declination direction
    pmdec_error,
    /// Correlation between right ascension and declination
    ra_dec_corr,
    /// Correlation between right ascension and parallax
    ra_parallax_corr,
    /// Correlation between right ascension and proper motion in right ascension
    ra_pmra_corr,
    /// Correlation between right ascension and proper motion in declination
    ra_pmdec_corr,
    /// Correlation between declination and parallax
    dec_parallax_corr,
    /// Correlation between declination and proper motion in right ascension
    dec_pmra_corr,
    /// Correlation between declination and proper motion in declination
    dec_pmdec_corr,
    /// Correlation between parallax and proper motion in right ascension
    parallax_pmra_corr,
    /// Correlation between parallax and proper motion in declination
    parallax_pmdec_corr,
    /// Correlation between proper motion in right ascension and proper motion in declination
    pmra_pmdec_corr,
    /// Total number of observations in the along-scan (AL) direction
    astrometric_n_obs_al,
    /// Total number of observations in the across-scan (AC) direction
    astrometric_n_obs_ac,
    /// Number of good observations in the along-scan (AL) direction
    astrometric_n_good_obs_al,
    /// Number of bad observations in the along-scan (AL) direction
    astrometric_n_bad_obs_al,
    /// Goodness of fit statistic of model wrt along-scan observations
    astrometric_gof_al,
    /// AL chi-square value
    astrometric_chi2_al,
    /// Excess noise of the source
    astrometric_excess_noise,
    /// Significance of excess noise
    astrometric_excess_noise_sig,
    /// Which parameters have been solved for?
    astrometric_params_solved,
    /// Primary or seconday
    astrometric_primary_flag,
    /// Effective wavenumber of the source used in the astrometric solution
    nu_eff_used_in_astrometry,
    /// Astrometrically estimated pseudocolour of the source
    pseudocolour,
    /// Standard error of the pseudocolour of the source
    pseudocolour_error,
    /// Correlation between right ascension and pseudocolour
    ra_pseudocolour_corr,
    /// Correlation between declination and pseudocolour
    dec_pseudocolour_corr,
    /// Correlation between parallax and pseudocolour
    parallax_pseudocolour_corr,
    /// Correlation between proper motion in right asension and pseudocolour
    pmra_pseudocolour_corr,
    /// Correlation between proper motion in declination and pseudocolour
    pmdec_pseudocolour_corr,
    /// Matched FOV transits used in the AGIS solution
    astrometric_matched_transits,
    /// Number of visibility periods used in Astrometric solution
    visibility_periods_used,
    /// The longest semi-major axis of the 5-d error ellipsoid
    astrometric_sigma5d_max,
    /// The number of transits matched to this source
    matched_transits,
    /// The number of transits newly incorporated into an existing source in the current cycle
    new_matched_transits,
    /// The number of transits removed from an existing source in the current cycle
    matched_transits_removed,
    /// Amplitude of the IPD GoF versus position angle of scan
    ipd_gof_harmonic_amplitude,
    /// Phase of the IPD GoF versus position angle of scan
    ipd_gof_harmonic_phase,
    /// Percent of successful-IPD windows with more than one peak
    ipd_frac_multi_peak,
    /// Percent of transits with truncated windows or multiple gate
    ipd_frac_odd_win,
    /// Renormalised unit weight error
    ruwe,
    /// Degree of concentration of scan directions across the source
    scan_direction_strength_k1,
    /// Degree of concentration of scan directions across the source
    scan_direction_strength_k2,
    /// Degree of concentration of scan directions across the source
    scan_direction_strength_k3,
    /// Degree of concentration of scan directions across the source
    scan_direction_strength_k4,
    /// Mean position angle of scan directions across the source
    scan_direction_mean_k1,
    /// Mean position angle of scan directions across the source
    scan_direction_mean_k2,
    /// Mean position angle of scan directions across the source
    scan_direction_mean_k3,
    /// Mean position angle of scan directions across the source
    scan_direction_mean_k4,
    /// Source with multiple source identifiers
    duplicated_source,
    /// Number of observations contributing to G photometry
    phot_g_n_obs,
    /// G-band mean flux
    phot_g_mean_flux,
    /// Error on G-band mean flux
    phot_g_mean_flux_error,
    /// G-band mean flux divided by its error
    phot_g_mean_flux_over_error,
    /// G-band mean magnitude
    phot_g_mean_mag,
    /// Number of observations contributing to BP photometry
    phot_bp_n_obs,
    /// Integrated BP mean flux
    phot_bp_mean_flux,
    /// Error on the integrated BP mean flux
    phot_bp_mean_flux_error,
    /// Integrated BP mean flux divided by its error
    phot_bp_mean_flux_over_error,
    /// Integrated BP mean magnitude
    phot_bp_mean_mag,
    /// Number of observations contributing to RP photometry
    phot_rp_n_obs,
    /// Integrated RP mean flux
    phot_rp_mean_flux,
    /// Error on the integrated RP mean flux
    phot_rp_mean_flux_error,
    /// Integrated RP mean flux divided by its error
    phot_rp_mean_flux_over_error,
    /// Integrated RP mean magnitude
    phot_rp_mean_mag,
    /// BP/RP excess factor
    phot_bp_rp_excess_factor,
    /// Number of BP contaminated transits
    phot_bp_n_contaminated_transits,
    /// Number of BP blended transits
    phot_bp_n_blended_transits,
    /// Number of RP contaminated transits
    phot_rp_n_contaminated_transits,
    /// Number of RP blended transits
    phot_rp_n_blended_transits,
    /// Photometry processing mode
    phot_proc_mode,
    /// BP - RP colour
    bp_rp,
    /// BP - G colour
    bp_g,
    /// G - RP colour
    g_rp,
    /// Radial velocity
    radial_velocity,
    /// Radial velocity error
    radial_velocity_error,
    /// Method used to obtain the radial velocity
    rv_method_used,
    /// Number of transits used to compute the radial velocity
    rv_nb_transits,
    /// Number of valid transits that have undergone deblending
    rv_nb_deblended_transits,
    /// Number of visibility periods used to estimate the radial velocity
    rv_visibility_periods_used,
    /// Expected signal to noise ratio in the combination of the spectra used to obtain the radial velocity
    rv_expected_sig_to_noise,
    /// Radial velocity renormalised goodness of fit
    rv_renormalised_gof,
    /// P-value for constancy based on a chi-squared criterion
    rv_chisq_pvalue,
    /// Time coverage of the radial velocity time series
    rv_time_duration,
    /// Total amplitude in the radial velocity time series after outlier removal
    rv_amplitude_robust,
    /// Teff of the template used to compute the radial velocity
    rv_template_teff,
    /// Logg of the template used to compute the radial velocity
    rv_template_logg,
    /// [Fe/H] of the template used to compute the radial velocityy
    rv_template_fe_h,
    /// Origin of the atmospheric parameters associated to the template
    rv_atm_param_origin,
    /// Spectral line broadening parameter
    vbroad,
    /// Uncertainty on the spectral line broadening
    vbroad_error,
    /// Number of transits used to compute vbroad
    vbroad_nb_transits,
    /// Integrated Grvs magnitude
    grvs_mag,
    /// Grvs magnitude uncertainty
    grvs_mag_error,
    /// Number of transits used to compute Grvs
    grvs_mag_nb_transits,
    /// Signal to noise ratio in the mean RVS spectrum
    rvs_spec_sig_to_noise,
    /// Photometric variability flag
    phot_variable_flag,
    /// Galactic longitude
    l,
    /// Galactic latitude
    b,
    /// Ecliptic longitude
    ecl_lon,
    /// Ecliptic latitude
    ecl_lat,
    /// Flag indicating the availability of additional information in the QSO candidates table
    in_qso_candidates,
    /// Flag indicating the availability of additional information in the galaxy candidates table
    in_galaxy_candidates,
    /// Flag indicating the availability of additional information in the various Non-Single Star tables
    non_single_star,
    /// Flag indicating the availability of mean BP/RP spectrum in continuous representation for this source
    has_xp_continuous,
    /// Flag indicating the availability of mean BP/RP spectrum in sampled form for this source
    has_xp_sampled,
    /// Flag indicating the availability of mean RVS spectrum for this source
    has_rvs,
    /// Flag indicating the availability of epoch photometry for this source
    has_epoch_photometry,
    /// Flag indicating the availability of epoch radial velocity for this source
    has_epoch_rv,
    /// Flag indicating the availability of GSP-Phot MCMC samples for this source
    has_mcmc_gspphot,
    /// Flag indicating the availability of MSC MCMC samples for this source
    has_mcmc_msc,
    /// Flag indicating that the source is present in the Gaia Andromeda Photometric Survey (GAPS)
    in_andromeda_survey,
    /// Probability from DSC-Combmod of being a quasar (data used: BP/RP spectrum, photometry, astrometry)
    classprob_dsc_combmod_quasar,
    /// Probability from DSC-Combmod of being a galaxy (data used: BP/RP spectrum, photometry, astrometry)
    classprob_dsc_combmod_galaxy,
    /// Probability from DSC-Combmod of being a single star (but not a white dwarf) (data used: BP/RP spectrum, photometry, astrometry)
    classprob_dsc_combmod_star,
    /// Effective temperature from GSP-Phot Aeneas best library using BP/RP spectra
    teff_gspphot,
    /// Lower confidence level (16%) of effective temperature from GSP-Phot Aeneas best library using BP/RP spectra
    teff_gspphot_lower,
    /// Upper confidence level (84%) of effective temperature from GSP-Phot Aeneas best library using BP/RP spectra
    teff_gspphot_upper,
    /// Surface gravity from GSP-Phot Aeneas best library using BP/RP spectra
    logg_gspphot,
    /// Lower confidence level (16%) of surface gravity from GSP-Phot Aeneas best library using BP/RP spectra
    logg_gspphot_lower,
    /// Upper confidence level (84%) of surface gravity from GSP-Phot Aeneas best library using BP/RP spectra
    logg_gspphot_upper,
    /// Iron abundance from GSP-Phot Aeneas best library using BP/RP spectra
    mh_gspphot,
    /// Lower confidence level (16%) of iron abundance from GSP-Phot Aeneas best library using BP/RP spectra
    mh_gspphot_lower,
    /// Upper confidence level (84%) of iron abundance from GSP-Phot Aeneas best library using BP/RP spectra
    mh_gspphot_upper,
    /// Distance from GSP-Phot Aeneas best library using BP/RP spectra
    distance_gspphot,
    /// Lower confidence level (16%) of distance from GSP-Phot Aeneas best library using BP/RP spectra
    distance_gspphot_lower,
    /// Upper confidence level (84%) of distance from GSP-Phot Aeneas best library using BP/RP spectra
    distance_gspphot_upper,
    /// Monochromatic extinction $A_0$ at 547.7nm from GSP-Phot Aeneas best library using BP/RP spectra
    azero_gspphot,
    /// Lower confidence level (16%) of monochromatic extinction $A_0$ at 547.7nm from GSP-Phot Aeneas best library using BP/RP spectra
    azero_gspphot_lower,
    /// Upper confidence level (84%) of monochromatic extinction $A_0$ at 547.7nm from GSP-Phot Aeneas best library using BP/RP spectra
    azero_gspphot_upper,
    /// Extinction in G band from GSP-Phot Aeneas best library using BP/RP spectra
    ag_gspphot,
    /// Lower confidence level (16%) of extinction in G band from GSP-Phot Aeneas best library using BP/RP spectra
    ag_gspphot_lower,
    /// Upper confidence level (84%) of extinction in G band from GSP-Phot Aeneas best library using BP/RP spectra
    ag_gspphot_upper,
    /// Reddening $E(G_{\rm BP} - G_{\rm RP})$ from GSP-Phot Aeneas best library using BP/RP spectra
    ebpminrp_gspphot,
    /// Lower confidence level (16%) of reddening  $E(G_{\rm BP} - G_{\rm RP})$ from GSP-Phot Aeneas best library using BP/RP spectra
    ebpminrp_gspphot_lower,
    /// Upper confidence level (84%) of reddening  $E(G_{\rm BP} - G_{\rm RP})$ from GSP-Phot Aeneas best library using BP/RP spectra
    ebpminrp_gspphot_upper,
    /// Name of library that achieves the highest mean log-posterior in MCMC samples and was used to derive GSP-Phot parameters in this table
    libname_gspphot,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the gaia_source table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::designation.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::random_index.to_string());
    col_strings.push(Col::ref_epoch.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::ra_error.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::dec_error.to_string());
    col_strings.push(Col::parallax.to_string());
    col_strings.push(Col::parallax_error.to_string());
    col_strings.push(Col::parallax_over_error.to_string());
    col_strings.push(Col::pm.to_string());
    col_strings.push(Col::pmra.to_string());
    col_strings.push(Col::pmra_error.to_string());
    col_strings.push(Col::pmdec.to_string());
    col_strings.push(Col::pmdec_error.to_string());
    col_strings.push(Col::ra_dec_corr.to_string());
    col_strings.push(Col::ra_parallax_corr.to_string());
    col_strings.push(Col::ra_pmra_corr.to_string());
    col_strings.push(Col::ra_pmdec_corr.to_string());
    col_strings.push(Col::dec_parallax_corr.to_string());
    col_strings.push(Col::dec_pmra_corr.to_string());
    col_strings.push(Col::dec_pmdec_corr.to_string());
    col_strings.push(Col::parallax_pmra_corr.to_string());
    col_strings.push(Col::parallax_pmdec_corr.to_string());
    col_strings.push(Col::pmra_pmdec_corr.to_string());
    col_strings.push(Col::astrometric_n_obs_al.to_string());
    col_strings.push(Col::astrometric_n_obs_ac.to_string());
    col_strings.push(Col::astrometric_n_good_obs_al.to_string());
    col_strings.push(Col::astrometric_n_bad_obs_al.to_string());
    col_strings.push(Col::astrometric_gof_al.to_string());
    col_strings.push(Col::astrometric_chi2_al.to_string());
    col_strings.push(Col::astrometric_excess_noise.to_string());
    col_strings.push(Col::astrometric_excess_noise_sig.to_string());
    col_strings.push(Col::astrometric_params_solved.to_string());
    col_strings.push(Col::astrometric_primary_flag.to_string());
    col_strings.push(Col::nu_eff_used_in_astrometry.to_string());
    col_strings.push(Col::pseudocolour.to_string());
    col_strings.push(Col::pseudocolour_error.to_string());
    col_strings.push(Col::ra_pseudocolour_corr.to_string());
    col_strings.push(Col::dec_pseudocolour_corr.to_string());
    col_strings.push(Col::parallax_pseudocolour_corr.to_string());
    col_strings.push(Col::pmra_pseudocolour_corr.to_string());
    col_strings.push(Col::pmdec_pseudocolour_corr.to_string());
    col_strings.push(Col::astrometric_matched_transits.to_string());
    col_strings.push(Col::visibility_periods_used.to_string());
    col_strings.push(Col::astrometric_sigma5d_max.to_string());
    col_strings.push(Col::matched_transits.to_string());
    col_strings.push(Col::new_matched_transits.to_string());
    col_strings.push(Col::matched_transits_removed.to_string());
    col_strings.push(Col::ipd_gof_harmonic_amplitude.to_string());
    col_strings.push(Col::ipd_gof_harmonic_phase.to_string());
    col_strings.push(Col::ipd_frac_multi_peak.to_string());
    col_strings.push(Col::ipd_frac_odd_win.to_string());
    col_strings.push(Col::ruwe.to_string());
    col_strings.push(Col::scan_direction_strength_k1.to_string());
    col_strings.push(Col::scan_direction_strength_k2.to_string());
    col_strings.push(Col::scan_direction_strength_k3.to_string());
    col_strings.push(Col::scan_direction_strength_k4.to_string());
    col_strings.push(Col::scan_direction_mean_k1.to_string());
    col_strings.push(Col::scan_direction_mean_k2.to_string());
    col_strings.push(Col::scan_direction_mean_k3.to_string());
    col_strings.push(Col::scan_direction_mean_k4.to_string());
    col_strings.push(Col::duplicated_source.to_string());
    col_strings.push(Col::phot_g_n_obs.to_string());
    col_strings.push(Col::phot_g_mean_flux.to_string());
    col_strings.push(Col::phot_g_mean_flux_error.to_string());
    col_strings.push(Col::phot_g_mean_flux_over_error.to_string());
    col_strings.push(Col::phot_g_mean_mag.to_string());
    col_strings.push(Col::phot_bp_n_obs.to_string());
    col_strings.push(Col::phot_bp_mean_flux.to_string());
    col_strings.push(Col::phot_bp_mean_flux_error.to_string());
    col_strings.push(Col::phot_bp_mean_flux_over_error.to_string());
    col_strings.push(Col::phot_bp_mean_mag.to_string());
    col_strings.push(Col::phot_rp_n_obs.to_string());
    col_strings.push(Col::phot_rp_mean_flux.to_string());
    col_strings.push(Col::phot_rp_mean_flux_error.to_string());
    col_strings.push(Col::phot_rp_mean_flux_over_error.to_string());
    col_strings.push(Col::phot_rp_mean_mag.to_string());
    col_strings.push(Col::phot_bp_rp_excess_factor.to_string());
    col_strings.push(Col::phot_bp_n_contaminated_transits.to_string());
    col_strings.push(Col::phot_bp_n_blended_transits.to_string());
    col_strings.push(Col::phot_rp_n_contaminated_transits.to_string());
    col_strings.push(Col::phot_rp_n_blended_transits.to_string());
    col_strings.push(Col::phot_proc_mode.to_string());
    col_strings.push(Col::bp_rp.to_string());
    col_strings.push(Col::bp_g.to_string());
    col_strings.push(Col::g_rp.to_string());
    col_strings.push(Col::radial_velocity.to_string());
    col_strings.push(Col::radial_velocity_error.to_string());
    col_strings.push(Col::rv_method_used.to_string());
    col_strings.push(Col::rv_nb_transits.to_string());
    col_strings.push(Col::rv_nb_deblended_transits.to_string());
    col_strings.push(Col::rv_visibility_periods_used.to_string());
    col_strings.push(Col::rv_expected_sig_to_noise.to_string());
    col_strings.push(Col::rv_renormalised_gof.to_string());
    col_strings.push(Col::rv_chisq_pvalue.to_string());
    col_strings.push(Col::rv_time_duration.to_string());
    col_strings.push(Col::rv_amplitude_robust.to_string());
    col_strings.push(Col::rv_template_teff.to_string());
    col_strings.push(Col::rv_template_logg.to_string());
    col_strings.push(Col::rv_template_fe_h.to_string());
    col_strings.push(Col::rv_atm_param_origin.to_string());
    col_strings.push(Col::vbroad.to_string());
    col_strings.push(Col::vbroad_error.to_string());
    col_strings.push(Col::vbroad_nb_transits.to_string());
    col_strings.push(Col::grvs_mag.to_string());
    col_strings.push(Col::grvs_mag_error.to_string());
    col_strings.push(Col::grvs_mag_nb_transits.to_string());
    col_strings.push(Col::rvs_spec_sig_to_noise.to_string());
    col_strings.push(Col::phot_variable_flag.to_string());
    col_strings.push(Col::l.to_string());
    col_strings.push(Col::b.to_string());
    col_strings.push(Col::ecl_lon.to_string());
    col_strings.push(Col::ecl_lat.to_string());
    col_strings.push(Col::in_qso_candidates.to_string());
    col_strings.push(Col::in_galaxy_candidates.to_string());
    col_strings.push(Col::non_single_star.to_string());
    col_strings.push(Col::has_xp_continuous.to_string());
    col_strings.push(Col::has_xp_sampled.to_string());
    col_strings.push(Col::has_rvs.to_string());
    col_strings.push(Col::has_epoch_photometry.to_string());
    col_strings.push(Col::has_epoch_rv.to_string());
    col_strings.push(Col::has_mcmc_gspphot.to_string());
    col_strings.push(Col::has_mcmc_msc.to_string());
    col_strings.push(Col::in_andromeda_survey.to_string());
    col_strings.push(Col::classprob_dsc_combmod_quasar.to_string());
    col_strings.push(Col::classprob_dsc_combmod_galaxy.to_string());
    col_strings.push(Col::classprob_dsc_combmod_star.to_string());
    col_strings.push(Col::teff_gspphot.to_string());
    col_strings.push(Col::teff_gspphot_lower.to_string());
    col_strings.push(Col::teff_gspphot_upper.to_string());
    col_strings.push(Col::logg_gspphot.to_string());
    col_strings.push(Col::logg_gspphot_lower.to_string());
    col_strings.push(Col::logg_gspphot_upper.to_string());
    col_strings.push(Col::mh_gspphot.to_string());
    col_strings.push(Col::mh_gspphot_lower.to_string());
    col_strings.push(Col::mh_gspphot_upper.to_string());
    col_strings.push(Col::distance_gspphot.to_string());
    col_strings.push(Col::distance_gspphot_lower.to_string());
    col_strings.push(Col::distance_gspphot_upper.to_string());
    col_strings.push(Col::azero_gspphot.to_string());
    col_strings.push(Col::azero_gspphot_lower.to_string());
    col_strings.push(Col::azero_gspphot_upper.to_string());
    col_strings.push(Col::ag_gspphot.to_string());
    col_strings.push(Col::ag_gspphot_lower.to_string());
    col_strings.push(Col::ag_gspphot_upper.to_string());
    col_strings.push(Col::ebpminrp_gspphot.to_string());
    col_strings.push(Col::ebpminrp_gspphot_lower.to_string());
    col_strings.push(Col::ebpminrp_gspphot_upper.to_string());
    col_strings.push(Col::libname_gspphot.to_string());
    map.insert(gaia_source.string(), col_strings);
}
