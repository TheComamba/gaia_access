// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the tycho2 table.

use crate::traits::{Column, Table};

/// The tycho2 table.
#[allow(non_camel_case_types)]
pub struct tycho2;

impl Table for tycho2 {
    fn string(&self) -> String {
        "tycho2".to_string()
    }
}

/// The columns in the tycho2 table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
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

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::id.to_string());
    col_strings.push(Col::hip.to_string());
    col_strings.push(Col::tyc1.to_string());
    col_strings.push(Col::tyc2.to_string());
    col_strings.push(Col::tyc3.to_string());
    col_strings.push(Col::id_tycho.to_string());
    col_strings.push(Col::tyc.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::ra_deg.to_string());
    col_strings.push(Col::de_deg.to_string());
    col_strings.push(Col::ra_mdeg.to_string());
    col_strings.push(Col::de_mdeg.to_string());
    col_strings.push(Col::pm_ra.to_string());
    col_strings.push(Col::pm_de.to_string());
    col_strings.push(Col::ep_ra1990.to_string());
    col_strings.push(Col::ep_de1990.to_string());
    col_strings.push(Col::ep_ra_m.to_string());
    col_strings.push(Col::ep_de_m.to_string());
    col_strings.push(Col::num.to_string());
    col_strings.push(Col::e_ra_deg.to_string());
    col_strings.push(Col::e_de_deg.to_string());
    col_strings.push(Col::corr.to_string());
    col_strings.push(Col::e_ra_mdeg.to_string());
    col_strings.push(Col::e_de_mdeg.to_string());
    col_strings.push(Col::e_pm_ra.to_string());
    col_strings.push(Col::e_pm_de.to_string());
    col_strings.push(Col::q_ra_mdeg.to_string());
    col_strings.push(Col::q_de_mdeg.to_string());
    col_strings.push(Col::q_pm_de.to_string());
    col_strings.push(Col::q_pm_ra.to_string());
    col_strings.push(Col::pflag.to_string());
    col_strings.push(Col::posflg.to_string());
    col_strings.push(Col::ccdm.to_string());
    col_strings.push(Col::prox.to_string());
    col_strings.push(Col::bt_mag.to_string());
    col_strings.push(Col::vt_mag.to_string());
    col_strings.push(Col::e_bt_mag.to_string());
    col_strings.push(Col::e_vt_mag.to_string());
    col_strings.push(Col::hip_old.to_string());
    map.insert(tycho2.string(), col_strings);
}
