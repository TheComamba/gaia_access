use crate::{column::Column, schema::Schema};

pub struct Gaiaedr3GcnsRejected1;

impl Schema for Gaiaedr3GcnsRejected1 {
    fn string(&self) -> String {
        "gaiaedr3_gcns_rejected_1".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
    source_id,
    ra,
    dec,
    ra_error,
    dec_error,
    parallax,
    parallax_error,
    pmra,
    pmra_error,
    pmdec,
    pmdec_error,
    phot_g_mean_mag,
    phot_g_mean_flux_over_error,
    phot_bp_mean_mag,
    phot_bp_mean_flux_over_error,
    phot_rp_mean_mag,
    phot_rp_mean_flux_over_error,
    phot_bp_rp_excess_factor,
    ruwe,
    ipd_frac_multi_peak,
    adoptedrv,
    adoptedrv_error,
    adoptedrv_refname,
    radial_velocity_is_valid,
    gcns_prob,
    wd_prob,
    dist_1,
    dist_16,
    dist_50,
    dist_84,
    xcoord_50,
    xcoord_16,
    xcoord_84,
    ycoord_50,
    ycoord_16,
    ycoord_84,
    zcoord_50,
    zcoord_16,
    zcoord_84,
    uvel_50,
    uvel_16,
    uvel_84,
    vvel_50,
    vvel_16,
    vvel_84,
    wvel_50,
    wvel_16,
    wvel_84,
    name_gunn,
    refname_gunn,
    gmag_gunn,
    e_gmag_gunn,
    rmag_gunn,
    e_rmag_gunn,
    imag_gunn,
    e_imag_gunn,
    zmag_gunn,
    e_zmag_gunn,
    name_2mass,
    j_m_2mass,
    j_msig_2mass,
    h_m_2mass,
    h_msig_2mass,
    k_m_2mass,
    k_msig_2mass,
    name_wise,
    w1mpro_pm_wise,
    w1sigmpro_pm_wise,
    w2mpro_pm_wise,
    w2sigmpro_pm_wise,
    w3mpro_wise,
    w3sigmpro_wise,
    w4mpro_wise,
    w4sigmpro_wise,
    gncs_rejected_oid,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(Gaiaedr3GcnsRejected1.string(), col_strings);
}
