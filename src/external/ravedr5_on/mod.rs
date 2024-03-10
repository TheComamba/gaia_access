use crate::{column::Column, schema::Schema};

pub struct Ravedr5On;

impl Schema for Ravedr5On {
    fn string(&self) -> String {
        "ravedr5_on".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
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
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(Ravedr5On.string(), col_strings);
}
