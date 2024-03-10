use crate::{column::Column, schema::Schema};

pub struct Ravedr5Gra;

impl Schema for Ravedr5Gra {
    fn string(&self) -> String {
        "ravedr5_gra".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
    name,
    loggsc,
    e_loggsc,
    flag050,
    flag075,
    flagm,
    teffir,
    mg_h,
    o_mg_h,
    al_h,
    o_al_h,
    si_h,
    o_si_h,
    ti_h,
    o_ti_h,
    fe_h,
    o_fe_h,
    ni_h,
    o_ni_h,
    a_fe_c,
    chisqc,
    fracc,
    avschl,
    dist,
    e_dist,
    logav,
    e_logav,
    plx,
    e_plx,
    dm,
    e_dm,
    ffb,
    fqb,
    ngauss,
    gm1,
    gs1,
    gf1,
    gm2,
    gs2,
    gf2,
    gm3,
    gs3,
    gf3,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(Ravedr5Gra.string(), col_strings);
}
