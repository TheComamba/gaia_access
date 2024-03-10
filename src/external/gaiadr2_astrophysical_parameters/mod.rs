use crate::{column::Column, schema::Schema};

pub struct Gaiadr2AstrophysicalParameters;

impl Schema for Gaiadr2AstrophysicalParameters {
    fn string(&self) -> String {
        "gaiadr2_astrophysical_parameters".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
    source_id,
    a0_best,
    a0_p50,
    a0_dist,
    r0_best,
    r0_p50,
    r0_dist,
    loga_best,
    loga_p50,
    loga_dist,
    logl_best,
    logl_p50,
    logl_dist,
    logm_best,
    logm_p50,
    logm_dist,
    logt_best,
    logt_p50,
    logt_dist,
    logg_best,
    logg_p50,
    logg_dist,
    a_bp_best,
    a_bp_p50,
    a_bp_dist,
    a_g_best,
    a_g_p50,
    a_g_dist,
    a_rp_best,
    a_rp_p50,
    a_rp_dist,
    mag_bp,
    err_bp,
    bp_best,
    bp_p50,
    bp_dist,
    mag_g,
    err_g,
    g_best,
    g_p50,
    g_dist,
    mag_rp,
    err_rp,
    rp_best,
    rp_p50,
    rp_dist,
    mag_j,
    err_j,
    j_best,
    j_p50,
    j_dist,
    mag_h,
    err_h,
    h_best,
    h_p50,
    h_dist,
    mag_ks,
    err_ks,
    ks_best,
    ks_p50,
    ks_dist,
    mag_w1,
    err_w1,
    w1_best,
    w1_p50,
    w1_dist,
    mag_w2,
    err_w2,
    w2_best,
    w2_p50,
    w2_dist,
    dmod_best,
    dmod_p50,
    dmod_dist,
    lnlike_best,
    lnlike_p50,
    lnlike_dist,
    lnp_best,
    lnp_p50,
    lnp_dist,
    log10jitter_best,
    log10jitter_p50,
    log10jitter_dist,
    parallax,
    err_parallax,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(Gaiadr2AstrophysicalParameters.string(), col_strings);
}
