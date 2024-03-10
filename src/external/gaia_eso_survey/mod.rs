use crate::{column::Column, schema::Schema};

pub struct GaiaEsoSurvey;

impl Schema for GaiaEsoSurvey {
    fn string(&self) -> String {
        "gaia_eso_survey".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
    object,
    ges_fld,
    ges_type,
    ra,
    dec,
    epoch,
    teff,
    e_teff,
    logg,
    e_logg,
    feh,
    e_feh,
    vrad,
    e_vrad,
    vsini,
    e_vsini,
    lim_vsini,
    li1,
    e_li1,
    lim_li1,
    ew_li,
    e_ew_li,
    lim_ew_li,
    o1,
    e_o1,
    mg1,
    e_mg1,
    al1,
    e_al1,
    si1,
    e_si1,
    ca1,
    e_ca1,
    ti1,
    e_ti1,
    ni1,
    e_ni1,
    ba2,
    e_ba2,
    eu2,
    e_eu2,
    sflags,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(GaiaEsoSurvey.string(), col_strings);
}
