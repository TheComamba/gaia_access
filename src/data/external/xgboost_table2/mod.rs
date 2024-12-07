// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the xgboost_table2 table.

use crate::traits::{Column, Table};

/// This is table 2 of Andrae et al. (2023), "Robust Data-driven Metallicities for 175 Million Stars from Gaia XP Spectra" (https://ui.adsabs.harvard.edu/abs/2023ApJS..267....8A/abstract). This table contains 17,558,141 XGBoost parameters (temperature, surface gravity, and metallicity) for bright (G < 16 mag) red giants whose [M/H] values are vetted to be precise and pure. XGBoost draws on a number of data features: the full set of Gaia DR3 XP spectral coefficients, narrowband fluxes derived from XP spectra, broadband Gaia DR3 magnitudes, Gaia DR3 parallaxes, and CatWISE 2020 W1 and W2 magnitudes. The mean stellar parameter precision is 0.1 dex in [M/H], 50 K in Teff, and 0.08 dex in log(g). This table has been created in November 2023 based on version 2.1 of the dataset on Zenodo (https://doi.org/10.5281/zenodo.7945154).
#[allow(non_camel_case_types)]
pub struct xgboost_table2;

impl Table for xgboost_table2 {
    fn string(&self) -> String {
        "xgboost_table2".to_string()
    }
}

/// The columns in the xgboost_table2 table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Gaia DR3 source identifier
    source_id,
    /// Galactic longitude
    l,
    /// Galactic latitude
    b,
    /// Right ascension (ICRS with epoch J2016.0)
    ra,
    /// Declination (ICRS with epoch J2016.0)
    dec,
    /// Parallax with zero-point correction from Lindegren et al. (2021; https://ui.adsabs.harvard.edu/abs/2021A%26A...649A...4L/abstract)
    parallax_corrected,
    /// Standard error of parallax
    parallax_error,
    /// Proper motion in right ascension direction
    pmra,
    /// Standard error of proper motion in right ascension direction
    pmra_error,
    /// Proper motion in declination direction
    pmdec,
    /// Standard error of proper motion in declination direction
    pmdec_error,
    /// Renormalised unit weight error
    ruwe,
    /// Radial velocity
    radial_velocity,
    /// Radial velocity error
    radial_velocity_error,
    /// G-band mean magnitude
    phot_g_mean_mag,
    /// Integrated BP mean magnitude
    phot_bp_mean_mag,
    /// Integrated RP mean magnitude
    phot_rp_mean_mag,
    /// Apparent CatWISE 2020 W1 magnitude
    catwise_w1,
    /// Apparent CatWISE 2020 W2 magnitude
    catwise_w2,
    /// XGBoost estimate of [M/H]
    mh_xgboost,
    /// XGBoost estimate of Teff
    teff_xgboost,
    /// XGBoost estimate of log(g)
    logg_xgboost,
    /// Flag for membership in training sample
    in_training_sample,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the xgboost_table2 table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::l.to_string());
    col_strings.push(Col::b.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::parallax_corrected.to_string());
    col_strings.push(Col::parallax_error.to_string());
    col_strings.push(Col::pmra.to_string());
    col_strings.push(Col::pmra_error.to_string());
    col_strings.push(Col::pmdec.to_string());
    col_strings.push(Col::pmdec_error.to_string());
    col_strings.push(Col::ruwe.to_string());
    col_strings.push(Col::radial_velocity.to_string());
    col_strings.push(Col::radial_velocity_error.to_string());
    col_strings.push(Col::phot_g_mean_mag.to_string());
    col_strings.push(Col::phot_bp_mean_mag.to_string());
    col_strings.push(Col::phot_rp_mean_mag.to_string());
    col_strings.push(Col::catwise_w1.to_string());
    col_strings.push(Col::catwise_w2.to_string());
    col_strings.push(Col::mh_xgboost.to_string());
    col_strings.push(Col::teff_xgboost.to_string());
    col_strings.push(Col::logg_xgboost.to_string());
    col_strings.push(Col::in_training_sample.to_string());
    map.insert(xgboost_table2.string(), col_strings);
}
