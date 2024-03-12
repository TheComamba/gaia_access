// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct nss_two_body_orbit;

impl Table for nss_two_body_orbit {
    fn string(&self) -> String {
        "nss_two_body_orbit".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    nss_solution_type,
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
    a_thiele_innes,
    a_thiele_innes_error,
    b_thiele_innes,
    b_thiele_innes_error,
    f_thiele_innes,
    f_thiele_innes_error,
    g_thiele_innes,
    g_thiele_innes_error,
    c_thiele_innes,
    c_thiele_innes_error,
    h_thiele_innes,
    h_thiele_innes_error,
    period,
    period_error,
    t_periastron,
    t_periastron_error,
    eccentricity,
    eccentricity_error,
    center_of_mass_velocity,
    center_of_mass_velocity_error,
    semi_amplitude_primary,
    semi_amplitude_primary_error,
    semi_amplitude_secondary,
    semi_amplitude_secondary_error,
    mass_ratio,
    mass_ratio_error,
    fill_factor_primary,
    fill_factor_primary_error,
    fill_factor_secondary,
    fill_factor_secondary_error,
    inclination,
    inclination_error,
    arg_periastron,
    arg_periastron_error,
    temperature_ratio,
    temperature_ratio_error,
    temperature_ratio_definition,
    astrometric_n_obs_al,
    astrometric_n_good_obs_al,
    rv_n_obs_primary,
    rv_n_good_obs_primary,
    rv_n_obs_secondary,
    rv_n_good_obs_secondary,
    phot_g_n_obs,
    phot_g_n_good_obs,
    bit_index,
    corr_vec,
    obj_func,
    goodness_of_fit,
    efficiency,
    significance,
    flags,
    conf_spectro_period,
    r_pole_sum,
    r_l1_point_sum,
    r_spher_sum,
    ecl_time_primary,
    ecl_time_secondary,
    ecl_dur_primary,
    ecl_dur_secondary,
    g_luminosity_ratio,
    input_period_error,
    g_rank,
    astrometric_jitter,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::nss_solution_type.to_string());
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
    col_strings.push(Col::a_thiele_innes.to_string());
    col_strings.push(Col::a_thiele_innes_error.to_string());
    col_strings.push(Col::b_thiele_innes.to_string());
    col_strings.push(Col::b_thiele_innes_error.to_string());
    col_strings.push(Col::f_thiele_innes.to_string());
    col_strings.push(Col::f_thiele_innes_error.to_string());
    col_strings.push(Col::g_thiele_innes.to_string());
    col_strings.push(Col::g_thiele_innes_error.to_string());
    col_strings.push(Col::c_thiele_innes.to_string());
    col_strings.push(Col::c_thiele_innes_error.to_string());
    col_strings.push(Col::h_thiele_innes.to_string());
    col_strings.push(Col::h_thiele_innes_error.to_string());
    col_strings.push(Col::period.to_string());
    col_strings.push(Col::period_error.to_string());
    col_strings.push(Col::t_periastron.to_string());
    col_strings.push(Col::t_periastron_error.to_string());
    col_strings.push(Col::eccentricity.to_string());
    col_strings.push(Col::eccentricity_error.to_string());
    col_strings.push(Col::center_of_mass_velocity.to_string());
    col_strings.push(Col::center_of_mass_velocity_error.to_string());
    col_strings.push(Col::semi_amplitude_primary.to_string());
    col_strings.push(Col::semi_amplitude_primary_error.to_string());
    col_strings.push(Col::semi_amplitude_secondary.to_string());
    col_strings.push(Col::semi_amplitude_secondary_error.to_string());
    col_strings.push(Col::mass_ratio.to_string());
    col_strings.push(Col::mass_ratio_error.to_string());
    col_strings.push(Col::fill_factor_primary.to_string());
    col_strings.push(Col::fill_factor_primary_error.to_string());
    col_strings.push(Col::fill_factor_secondary.to_string());
    col_strings.push(Col::fill_factor_secondary_error.to_string());
    col_strings.push(Col::inclination.to_string());
    col_strings.push(Col::inclination_error.to_string());
    col_strings.push(Col::arg_periastron.to_string());
    col_strings.push(Col::arg_periastron_error.to_string());
    col_strings.push(Col::temperature_ratio.to_string());
    col_strings.push(Col::temperature_ratio_error.to_string());
    col_strings.push(Col::temperature_ratio_definition.to_string());
    col_strings.push(Col::astrometric_n_obs_al.to_string());
    col_strings.push(Col::astrometric_n_good_obs_al.to_string());
    col_strings.push(Col::rv_n_obs_primary.to_string());
    col_strings.push(Col::rv_n_good_obs_primary.to_string());
    col_strings.push(Col::rv_n_obs_secondary.to_string());
    col_strings.push(Col::rv_n_good_obs_secondary.to_string());
    col_strings.push(Col::phot_g_n_obs.to_string());
    col_strings.push(Col::phot_g_n_good_obs.to_string());
    col_strings.push(Col::bit_index.to_string());
    col_strings.push(Col::corr_vec.to_string());
    col_strings.push(Col::obj_func.to_string());
    col_strings.push(Col::goodness_of_fit.to_string());
    col_strings.push(Col::efficiency.to_string());
    col_strings.push(Col::significance.to_string());
    col_strings.push(Col::flags.to_string());
    col_strings.push(Col::conf_spectro_period.to_string());
    col_strings.push(Col::r_pole_sum.to_string());
    col_strings.push(Col::r_l1_point_sum.to_string());
    col_strings.push(Col::r_spher_sum.to_string());
    col_strings.push(Col::ecl_time_primary.to_string());
    col_strings.push(Col::ecl_time_secondary.to_string());
    col_strings.push(Col::ecl_dur_primary.to_string());
    col_strings.push(Col::ecl_dur_secondary.to_string());
    col_strings.push(Col::g_luminosity_ratio.to_string());
    col_strings.push(Col::input_period_error.to_string());
    col_strings.push(Col::g_rank.to_string());
    col_strings.push(Col::astrometric_jitter.to_string());
    map.insert(nss_two_body_orbit.string(), col_strings);
}
