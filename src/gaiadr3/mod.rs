use crate::schema::Schema;

pub struct Gaiadr3;

impl Schema for Gaiadr3 {
    fn string(&self) -> String {
        "gaiadr3".to_string()
    }
}

pub mod  nss_two_body_orbit; pub mod gaia_universe_model; pub mod gold_sample_fgkm_stars; pub mod tmass_psc_xsc_neighbourhood; pub mod ravedr5_join; pub mod agn_cross_id; pub mod gsc23_best_neighbour; pub mod sdssdr13_best_neighbour; pub mod panstarrs1_neighbourhood; pub mod total_galactic_extinction_map; pub mod gaia_source; pub mod allwise_best_neighbour; pub mod sso_orbits; pub mod sso_reflectance_spectrum; pub mod xp_summary; pub mod vari_compact_companion; pub mod skymapperdr2_neighbourhood; pub mod astrophysical_parameters_supp; pub mod qso_catalogue_name; pub mod frame_rotator_source; pub mod gaia_source_lite; pub mod apassdr9_neighbourhood; pub mod panstarrs1_best_neighbour; pub mod qso_candidates; pub mod ravedr5_best_neighbour; pub mod tycho2tdsc_merge_best_neighbour; pub mod hipparcos2_best_neighbour; pub mod sdssdr13_join; pub mod sso_source; pub mod sso_observation; pub mod allwise_neighbourhood; pub mod nss_acceleration_astro; pub mod science_alerts; pub mod vari_agn; pub mod astrophysical_parameters; pub mod gsc23_neighbourhood; pub mod vari_cepheid; pub mod skymapperdr2_join; pub mod galaxy_candidates; pub mod vari_planetary_transit_13june2022; pub mod vari_epoch_radial_velocity; pub mod chemical_cartography; pub mod vari_rrlyrae; pub mod vari_summary; pub mod gold_sample_ucd; pub mod dr2_neighbourhood; pub mod gaia_source_simulation; pub mod sdssdr13_neighbourhood; pub mod urat1_best_neighbour; pub mod alerts_mixedin_sourceids; pub mod vari_classifier_definition; pub mod vari_microlensing; pub mod galaxy_catalogue_name; pub mod gold_sample_carbon_stars; pub mod vari_eclipsing_binary; pub mod vari_ms_oscillator; pub mod vari_rotation_modulation; pub mod apassdr9_best_neighbour; pub mod ravedr6_join; pub mod nss_non_linear_spectro; pub mod vari_classifier_result; pub mod hipparcos2_neighbourhood; pub mod vari_long_period_variable; pub mod oa_neuron_information; pub mod tmass_psc_xsc_join; pub mod urat1_neighbourhood; pub mod tmass_psc_xsc_best_neighbour; pub mod vari_classifier_class_definition; pub mod gold_sample_spss; pub mod vari_rad_vel_statistics; pub mod nss_vim_fl; pub mod synthetic_photometry_gspc; pub mod tycho2tdsc_merge_neighbourhood; pub mod vari_planetary_transit; pub mod tycho2tdsc_merge; pub mod oa_neuron_xp_spectra; pub mod vari_spurious_signals; pub mod gaia_crf3_xm; pub mod ravedr5_neighbourhood; pub mod binary_masses; pub mod ravedr6_neighbourhood; pub mod total_galactic_extinction_map_opt; pub mod gsc23_join; pub mod panstarrs1_join; pub mod gold_sample_solar_analogues; pub mod ravedr6_best_neighbour; pub mod skymapperdr2_best_neighbour; pub mod apassdr9_join; pub mod vari_short_timescale; pub mod commanded_scan_law; pub mod gold_sample_oba_stars;

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    let mut tables = std::collections::HashMap::new();
    map.insert(Gaiadr3.string(), tables);
}
