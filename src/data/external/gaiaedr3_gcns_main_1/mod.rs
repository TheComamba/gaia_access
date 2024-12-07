// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the gaiaedr3_gcns_main_1 table.

use crate::traits::{Column, Table};

/// Gaia Collaboration, Smart, et al. (2021): Gaia Early Data Release 3. The Gaia Catalogue of Nearby Stars (2021, A&A, 649, A6), table1c with 331312 selected objects and table1r with 880428 rejected objects. Data replicated from gcns.main and gcns.rejected tables at GAVO Data Center TAP service https://dc.g-vo.org/tap and TAP metadata as of November 2021.</p>Original table description.</p>We produce a clean and well-characterised catalogue of objects within 100 pc of the Sun from the Gaia Early Data Release 3. We characterise the catalogue through comparisons to the full data release, external catalogues, and simulations. We carry out a first analysis of the science that is possible with this sample to demonstrate its potential and best practices for its use. The selection of objects within 100 pc from the full catalogue used selected training sets, machine-learning procedures, astrometric quantities, and solution quality indicators to determine a probability that the astrometric solution is reliable. The training set construction exploited the astrometric data, quality flags, and external photometry. For all candidates we calculated distance posterior probability densities using Bayesian procedures and mock catalogues to define priors. Any object with reliable astrometry and a non-zero probability of being within 100 pc is included in the catalogue. We have produced a catalogue of 331312 objects that we estimate contains at least 92% of stars of stellar type M9 within 100 pc of the Sun. We estimate that 9% of the stars in this catalogue probably lie outside 100 pc, but when the distance probability function is used, a correct treatment of this contamination is possible. We produced luminosity functions with a high signal-to-noise ratio for the main-sequence stars, giants, and white dwarfs. We examined in detail the Hyades cluster, the white dwarf population, and wide-binary systems and produced candidate lists for all three samples. We detected local manifestations of several streams, superclusters, and halo objects, in which we identified 12 members of Gaia Enceladus. We present the first direct parallaxes of five objects in multiple systems within 10 pc of the Sun. We provide the community with a large, well-characterised catalogue of objects in the solar neighbourhood. This is a primary benchmark for measuring and understanding fundamental parameters and descriptive functions in astronomy.</p>
#[allow(non_camel_case_types)]
pub struct gaiaedr3_gcns_main_1;

impl Table for gaiaedr3_gcns_main_1 {
    fn string(&self) -> String {
        "gaiaedr3_gcns_main_1".to_string()
    }
}

/// The columns in the gaiaedr3_gcns_main_1 table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Gaia eDR3 unique source identifier. Note that this *cannot* be matched against the DR1 or DR2 source_ids.
    source_id,
    /// ICRS right ascension from Gaia eDR3.
    ra,
    /// ICRS declination from Gaia eDR3.
    dec,
    /// Standard error of ra (with cos δ applied).
    ra_error,
    /// Standard error of dec
    dec_error,
    /// Absolute barycentric stellar parallax of the source at the reference epoch J2016.0. If looking for a distance, consider joining with gedr3dist.main and using the distances from there.
    parallax,
    /// Standard error of parallax
    parallax_error,
    /// Proper motion in right ascension of the source in ICRS at J2016.0. This is the tangent plane projection of the proper motion vector in the direction of increasing right ascension.
    pmra,
    /// Standard error of pmra
    pmra_error,
    /// Proper motion in declination at J2016.0.
    pmdec,
    /// Standard error of pmdec
    pmdec_error,
    /// Mean magnitude in the G band. This is computed from the G-band mean flux applying the magnitude zero-point in the Vega scale. To obtain error estimates, see phot_g_mean_flux_over_error.
    phot_g_mean_mag,
    /// Integrated mean G flux divided by its error. Errors are computed from the dispersion about the weighted mean of the input calibrated photometry.
    phot_g_mean_flux_over_error,
    /// Mean magnitude in the integrated BP band. This is computed from the BP-band mean flux applying the magnitude zero-point in the Vega scale. To obtain error estimates, see phot_bp_mean_flux_over_error.
    phot_bp_mean_mag,
    /// Integrated mean BP flux divided by its error. Errors are computed from the dispersion about the weighted mean of the input calibrated photometry.
    phot_bp_mean_flux_over_error,
    /// Mean magnitude in the integrated RP band. This is computed from the RP-band mean flux applying the magnitude zero-point in the Vega scale. To obtain error estimates, see phot_rp_mean_flux_over_error.
    phot_rp_mean_mag,
    /// Integrated mean RP flux divided by its error. Errors are computed from the dispersion about the weighted mean of the input calibrated photometry.
    phot_rp_mean_flux_over_error,
    /// BP/RP excess factor estimated from the comparison of the sum of integrated BP and RP fluxes with respect to the flux in the G band. This measures the excess of flux in the BP and RP integrated photometry with respect to the G band. This excess is believed to be caused by background and contamination issues affecting the BP and RP data. Therefore a large value of this factor for a given source indicates systematic errors in the BP and RP photometry.
    phot_bp_rp_excess_factor,
    /// Renormalized Unit Weight Error; this is a revised measure for the overall consistency of the solution as defined by GAIA-C3-TN-LU-LL-124-01. A suggested cut on this is RUWE <1.40) See the note for details.
    ruwe,
    /// Percentage of distance windows in which a double peak was seen (high values mean high likelihood of a resolved double star).
    ipd_frac_multi_peak,
    /// Adopted Radial Velocity
    adoptedrv,
    /// Error in adopted RV
    adoptedrv_error,
    /// Bibcode for the source of the radial velocity
    adoptedrv_refname,
    /// 1 if this object has a radial velocity in eDR3, 0 otherwise
    radial_velocity_is_valid,
    /// Probability that the astrometry is reliable
    gcns_prob,
    /// probability that this is a white dwarf.
    wd_prob,
    /// 1st percentile of the distance PDF, used in GCNS selection
    dist_1,
    /// 16th percentile of the distance PDF (1 σ lower bound)
    dist_16,
    /// Median of the distance PDF
    dist_50,
    /// 84th percentile of the distance PDF (1 σ upper bound)
    dist_84,
    /// Median x coordinate in the Galactic frame assuming dist_50
    xcoord_50,
    /// 1 σ lower bound of Galactic frame x coordinate
    xcoord_16,
    /// 1 σ upper bound of Galactic frame x coordinate
    xcoord_84,
    /// Median y coordinate in the Galactic frame assuming dist_50
    ycoord_50,
    /// 1 σ lower bound of Galactic frame y coordinate
    ycoord_16,
    /// 1 σ upper bound of Galactic frame y coordinate
    ycoord_84,
    /// Median z coordinate in the Galactic frame assuming dist_50
    zcoord_50,
    /// 1 σ lower bound of Galactic frame z coordinate
    zcoord_16,
    /// 1 σ upper bound of Galactic frame z coordinate
    zcoord_84,
    /// Median velocity u in the Galactic frame, direction positive x
    uvel_50,
    /// 1 σ lower bound for u
    uvel_16,
    /// 1 σ upper bound for u
    uvel_84,
    /// Median velocity v in the Galactic frame, direction positive y
    vvel_50,
    /// 1 σ lower bound for v
    vvel_16,
    /// 1 σ upper bound for v
    vvel_84,
    /// Median velocity w in the Galactic frame, direction positive z
    wvel_50,
    /// 1 σ lower bound for w
    wvel_16,
    /// 1 σ upper bound for w
    wvel_84,
    /// Object Name from PanSTARRS/SDSS/SkyMapper survey
    name_gunn,
    /// Reference for the source of the Gunn photometry
    refname_gunn,
    /// Gunn G band magnitude (when from SDSS, g, when from Skymapper, g_psf)
    gmag_gunn,
    /// Uncertainty in G band magnitude
    e_gmag_gunn,
    /// Gunn R band magnitude (when from SDSS, g, when from Skymapper, g_psf)
    rmag_gunn,
    /// Uncertainty in R band magnitude
    e_rmag_gunn,
    /// Gunn I band magnitude (when from SDSS, g, when from Skymapper, g_psf)
    imag_gunn,
    /// Uncertainty in I band magnitude
    e_imag_gunn,
    /// Gunn Z band magnitude (when from SDSS, g, when from Skymapper, g_psf)
    zmag_gunn,
    /// Uncertainty in Z band magnitude
    e_zmag_gunn,
    /// Name of this object in 2MASS
    name_2mass,
    /// 2MASS J band magnitude
    j_m_2mass,
    /// Uncertainty in 2MASS J band magnitude
    j_msig_2mass,
    /// 2MASS H band magnitude
    h_m_2mass,
    /// Uncertainty in 2MASS H band magnitude
    h_msig_2mass,
    /// 2MASS K band magnitude
    k_m_2mass,
    /// Uncertainty in 2MASS K band magnitude
    k_msig_2mass,
    /// Name of this object in WISE
    name_wise,
    /// CATWISE W1 band magnitude
    w1mpro_pm_wise,
    /// Uncertainty in CATWISE W1 band magnitude
    w1sigmpro_pm_wise,
    /// CATWISE W2 band magnitude
    w2mpro_pm_wise,
    /// Uncertainty in CATWISE W2 band magnitude
    w2sigmpro_pm_wise,
    /// CATWISE W3 band magnitude
    w3mpro_wise,
    /// Uncertainty in CATWISE W3 band magnitude
    w3sigmpro_wise,
    /// ALLWISE W4 band magnitude
    w4mpro_wise,
    /// Uncertainty in ALLWISE W4 band magnitude
    w4sigmpro_wise,
    /// The gncs_main_oid column. (No further description available)
    gncs_main_oid,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the gaiaedr3_gcns_main_1 table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::ra_error.to_string());
    col_strings.push(Col::dec_error.to_string());
    col_strings.push(Col::parallax.to_string());
    col_strings.push(Col::parallax_error.to_string());
    col_strings.push(Col::pmra.to_string());
    col_strings.push(Col::pmra_error.to_string());
    col_strings.push(Col::pmdec.to_string());
    col_strings.push(Col::pmdec_error.to_string());
    col_strings.push(Col::phot_g_mean_mag.to_string());
    col_strings.push(Col::phot_g_mean_flux_over_error.to_string());
    col_strings.push(Col::phot_bp_mean_mag.to_string());
    col_strings.push(Col::phot_bp_mean_flux_over_error.to_string());
    col_strings.push(Col::phot_rp_mean_mag.to_string());
    col_strings.push(Col::phot_rp_mean_flux_over_error.to_string());
    col_strings.push(Col::phot_bp_rp_excess_factor.to_string());
    col_strings.push(Col::ruwe.to_string());
    col_strings.push(Col::ipd_frac_multi_peak.to_string());
    col_strings.push(Col::adoptedrv.to_string());
    col_strings.push(Col::adoptedrv_error.to_string());
    col_strings.push(Col::adoptedrv_refname.to_string());
    col_strings.push(Col::radial_velocity_is_valid.to_string());
    col_strings.push(Col::gcns_prob.to_string());
    col_strings.push(Col::wd_prob.to_string());
    col_strings.push(Col::dist_1.to_string());
    col_strings.push(Col::dist_16.to_string());
    col_strings.push(Col::dist_50.to_string());
    col_strings.push(Col::dist_84.to_string());
    col_strings.push(Col::xcoord_50.to_string());
    col_strings.push(Col::xcoord_16.to_string());
    col_strings.push(Col::xcoord_84.to_string());
    col_strings.push(Col::ycoord_50.to_string());
    col_strings.push(Col::ycoord_16.to_string());
    col_strings.push(Col::ycoord_84.to_string());
    col_strings.push(Col::zcoord_50.to_string());
    col_strings.push(Col::zcoord_16.to_string());
    col_strings.push(Col::zcoord_84.to_string());
    col_strings.push(Col::uvel_50.to_string());
    col_strings.push(Col::uvel_16.to_string());
    col_strings.push(Col::uvel_84.to_string());
    col_strings.push(Col::vvel_50.to_string());
    col_strings.push(Col::vvel_16.to_string());
    col_strings.push(Col::vvel_84.to_string());
    col_strings.push(Col::wvel_50.to_string());
    col_strings.push(Col::wvel_16.to_string());
    col_strings.push(Col::wvel_84.to_string());
    col_strings.push(Col::name_gunn.to_string());
    col_strings.push(Col::refname_gunn.to_string());
    col_strings.push(Col::gmag_gunn.to_string());
    col_strings.push(Col::e_gmag_gunn.to_string());
    col_strings.push(Col::rmag_gunn.to_string());
    col_strings.push(Col::e_rmag_gunn.to_string());
    col_strings.push(Col::imag_gunn.to_string());
    col_strings.push(Col::e_imag_gunn.to_string());
    col_strings.push(Col::zmag_gunn.to_string());
    col_strings.push(Col::e_zmag_gunn.to_string());
    col_strings.push(Col::name_2mass.to_string());
    col_strings.push(Col::j_m_2mass.to_string());
    col_strings.push(Col::j_msig_2mass.to_string());
    col_strings.push(Col::h_m_2mass.to_string());
    col_strings.push(Col::h_msig_2mass.to_string());
    col_strings.push(Col::k_m_2mass.to_string());
    col_strings.push(Col::k_msig_2mass.to_string());
    col_strings.push(Col::name_wise.to_string());
    col_strings.push(Col::w1mpro_pm_wise.to_string());
    col_strings.push(Col::w1sigmpro_pm_wise.to_string());
    col_strings.push(Col::w2mpro_pm_wise.to_string());
    col_strings.push(Col::w2sigmpro_pm_wise.to_string());
    col_strings.push(Col::w3mpro_wise.to_string());
    col_strings.push(Col::w3sigmpro_wise.to_string());
    col_strings.push(Col::w4mpro_wise.to_string());
    col_strings.push(Col::w4sigmpro_wise.to_string());
    col_strings.push(Col::gncs_main_oid.to_string());
    map.insert(gaiaedr3_gcns_main_1.string(), col_strings);
}
