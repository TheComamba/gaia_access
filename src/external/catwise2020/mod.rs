use crate::{column::Column, schema::Schema};

pub struct Catwise2020;

impl Schema for Catwise2020 {
    fn string(&self) -> String {
        "catwise2020".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
    obj_id,
    ra_icrs,
    e_ra_icrs,
    de_icrs,
    e_de_icrs,
    name,
    e_pos,
    xpos,
    ypos,
    sky_w1,
    e_sky_w1,
    conf_w1,
    sky_w2,
    e_sky_w2,
    conf_w2,
    n_w1,
    m_w1,
    n_w2,
    m_w2,
    mjd,
    ra_pm_deg,
    e_ra_pm_deg,
    de_pm_deg,
    e_de_pm_deg,
    e_pos_pm,
    pm_ra,
    e_pm_ra,
    pm_de,
    e_pm_de,
    snr_w1pm,
    snr_w2pm,
    fw1pm,
    e_fw1pm,
    fw2pm,
    e_fw2pm,
    w1mpro_pm,
    e_w1mpro_pm,
    chi2_w1pm,
    w2mpro_pm,
    e_w2mpro_pm,
    chi2_w2pm,
    chi2pm,
    pm_qual,
    dist,
    d_w1mpro,
    chi2d_w1mpro,
    d_w2mpro,
    chi2d_w2mpro,
    elon_avg,
    e_elon_avg,
    elat_avg,
    e_elat_avg,
    d_elon,
    e_d_elon,
    d_elat,
    e_d_elat,
    snrd_elon,
    snrd_elat,
    chi2pm_ra,
    chi2pm_de,
    ka,
    k1,
    k2,
    km,
    plx1,
    e_plx1,
    plx2,
    e_plx2,
    sep,
    ccf,
    abf,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(Catwise2020.string(), col_strings);
}
