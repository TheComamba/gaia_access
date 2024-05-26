// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the ravedr5_on table.

use crate::traits::{Column, Table};

/// The ravedr5_on table.
#[allow(non_camel_case_types)]
pub struct ravedr5_on;

impl Table for ravedr5_on {
    fn string(&self) -> String {
        "ravedr5_on".to_string()
    }
}

/// The columns in the ravedr5_on table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    name,
    rave,
    hrv,
    e_hrv,
    s_hrv,
    s_med_hrv,
    r,
    hcp,
    wcp,
    crv,
    hrvsky,
    e_hrvsky,
    rsky,
    vrots,
    zpflag,
    c1,
    c2,
    c3,
    c4,
    c5,
    c6,
    c7,
    c8,
    c9,
    c10,
    c11,
    c12,
    c13,
    c14,
    c15,
    c16,
    c17,
    c18,
    c19,
    c20,
    teff,
    logg,
    fe_h,
    o_h,
    mg_h,
    al_h,
    si_h,
    ca_h,
    ni_h,
    e_teff,
    e_logg,
    e_fe_h,
    e_o_h,
    e_mg_h,
    e_al_h,
    e_si_h,
    e_ca_h,
    e_ni_h,
    snr,
    rchisq,
    qc,
    ra,
    dec,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::name.to_string());
    col_strings.push(Col::rave.to_string());
    col_strings.push(Col::hrv.to_string());
    col_strings.push(Col::e_hrv.to_string());
    col_strings.push(Col::s_hrv.to_string());
    col_strings.push(Col::s_med_hrv.to_string());
    col_strings.push(Col::r.to_string());
    col_strings.push(Col::hcp.to_string());
    col_strings.push(Col::wcp.to_string());
    col_strings.push(Col::crv.to_string());
    col_strings.push(Col::hrvsky.to_string());
    col_strings.push(Col::e_hrvsky.to_string());
    col_strings.push(Col::rsky.to_string());
    col_strings.push(Col::vrots.to_string());
    col_strings.push(Col::zpflag.to_string());
    col_strings.push(Col::c1.to_string());
    col_strings.push(Col::c2.to_string());
    col_strings.push(Col::c3.to_string());
    col_strings.push(Col::c4.to_string());
    col_strings.push(Col::c5.to_string());
    col_strings.push(Col::c6.to_string());
    col_strings.push(Col::c7.to_string());
    col_strings.push(Col::c8.to_string());
    col_strings.push(Col::c9.to_string());
    col_strings.push(Col::c10.to_string());
    col_strings.push(Col::c11.to_string());
    col_strings.push(Col::c12.to_string());
    col_strings.push(Col::c13.to_string());
    col_strings.push(Col::c14.to_string());
    col_strings.push(Col::c15.to_string());
    col_strings.push(Col::c16.to_string());
    col_strings.push(Col::c17.to_string());
    col_strings.push(Col::c18.to_string());
    col_strings.push(Col::c19.to_string());
    col_strings.push(Col::c20.to_string());
    col_strings.push(Col::teff.to_string());
    col_strings.push(Col::logg.to_string());
    col_strings.push(Col::fe_h.to_string());
    col_strings.push(Col::o_h.to_string());
    col_strings.push(Col::mg_h.to_string());
    col_strings.push(Col::al_h.to_string());
    col_strings.push(Col::si_h.to_string());
    col_strings.push(Col::ca_h.to_string());
    col_strings.push(Col::ni_h.to_string());
    col_strings.push(Col::e_teff.to_string());
    col_strings.push(Col::e_logg.to_string());
    col_strings.push(Col::e_fe_h.to_string());
    col_strings.push(Col::e_o_h.to_string());
    col_strings.push(Col::e_mg_h.to_string());
    col_strings.push(Col::e_al_h.to_string());
    col_strings.push(Col::e_si_h.to_string());
    col_strings.push(Col::e_ca_h.to_string());
    col_strings.push(Col::e_ni_h.to_string());
    col_strings.push(Col::snr.to_string());
    col_strings.push(Col::rchisq.to_string());
    col_strings.push(Col::qc.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    map.insert(ravedr5_on.string(), col_strings);
}
