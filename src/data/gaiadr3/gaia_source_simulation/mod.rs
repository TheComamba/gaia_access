// This code is generated by generate_code.py, do not modify it manually.
use crate::traits::{Column, Table};

#[allow(non_camel_case_types)]
pub struct gaia_source_simulation;

impl Table for gaia_source_simulation {
    fn string(&self) -> String {
        "gaia_source_simulation".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    ra,
    ra_error,
    dec,
    dec_error,
    parallax,
    parallax_error,
    pmra,
    pmra_error,
    pmdec,
    pmdec_error,
    n_obs_al,
    n_outliers_al,
    phot_g_mean_flux,
    phot_g_mean_flux_error,
    phot_g_mean_mag,
    phot_bp_mean_flux,
    phot_bp_mean_flux_error,
    phot_bp_mean_mag,
    phot_rp_mean_flux,
    phot_rp_mean_flux_error,
    phot_rp_mean_mag,
    phot_rvs_mean_flux,
    phot_rvs_mean_flux_error,
    phot_rvs_mean_mag,
    radial_velocity,
    radial_velocity_error,
    teff,
    teff_error,
    vsini,
    vsini_error,
    a0,
    a0_error,
    feh,
    feh_error,
    logg,
    logg_error,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::ra_error.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::dec_error.to_string());
    col_strings.push(Col::parallax.to_string());
    col_strings.push(Col::parallax_error.to_string());
    col_strings.push(Col::pmra.to_string());
    col_strings.push(Col::pmra_error.to_string());
    col_strings.push(Col::pmdec.to_string());
    col_strings.push(Col::pmdec_error.to_string());
    col_strings.push(Col::n_obs_al.to_string());
    col_strings.push(Col::n_outliers_al.to_string());
    col_strings.push(Col::phot_g_mean_flux.to_string());
    col_strings.push(Col::phot_g_mean_flux_error.to_string());
    col_strings.push(Col::phot_g_mean_mag.to_string());
    col_strings.push(Col::phot_bp_mean_flux.to_string());
    col_strings.push(Col::phot_bp_mean_flux_error.to_string());
    col_strings.push(Col::phot_bp_mean_mag.to_string());
    col_strings.push(Col::phot_rp_mean_flux.to_string());
    col_strings.push(Col::phot_rp_mean_flux_error.to_string());
    col_strings.push(Col::phot_rp_mean_mag.to_string());
    col_strings.push(Col::phot_rvs_mean_flux.to_string());
    col_strings.push(Col::phot_rvs_mean_flux_error.to_string());
    col_strings.push(Col::phot_rvs_mean_mag.to_string());
    col_strings.push(Col::radial_velocity.to_string());
    col_strings.push(Col::radial_velocity_error.to_string());
    col_strings.push(Col::teff.to_string());
    col_strings.push(Col::teff_error.to_string());
    col_strings.push(Col::vsini.to_string());
    col_strings.push(Col::vsini_error.to_string());
    col_strings.push(Col::a0.to_string());
    col_strings.push(Col::a0_error.to_string());
    col_strings.push(Col::feh.to_string());
    col_strings.push(Col::feh_error.to_string());
    col_strings.push(Col::logg.to_string());
    col_strings.push(Col::logg_error.to_string());
    map.insert(gaia_source_simulation.string(), col_strings);
}
