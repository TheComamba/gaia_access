use crate::schema::Schema;

pub struct Tycho2;

impl Schema for Tycho2 {
    fn string(&self) -> String {
        "tycho2".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
    id,
    hip,
    tyc1,
    tyc2,
    tyc3,
    id_tycho,
    tyc,
    ra,
    dec,
    ra_deg,
    de_deg,
    ra_mdeg,
    de_mdeg,
    pm_ra,
    pm_de,
    ep_ra1990,
    ep_de1990,
    ep_ra_m,
    ep_de_m,
    num,
    e_ra_deg,
    e_de_deg,
    corr,
    e_ra_mdeg,
    e_de_mdeg,
    e_pm_ra,
    e_pm_de,
    q_ra_mdeg,
    q_de_mdeg,
    q_pm_de,
    q_pm_ra,
    pflag,
    posflg,
    ccdm,
    prox,
    bt_mag,
    vt_mag,
    e_bt_mag,
    e_vt_mag,
    hip_old,
}

#[cfg(test)]
pub(crate) fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(Tycho2.string(), col_strings);
}
