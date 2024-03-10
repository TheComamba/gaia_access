use crate::{column::Column, schema::Schema};

pub struct GalexAis;

impl Schema for GalexAis {
    fn string(&self) -> String {
        "galex_ais".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
    raj2000,
    dej2000,
    name,
    objid,
    phid,
    cat,
    rafdeg,
    defdeg,
    fuvexp,
    nuvexp,
    glon,
    glat,
    tile,
    img,
    sv,
    r_fov,
    obs,
    b,
    e_b_v,
    sp_,
    chkf,
    fuvmag,
    e_fuvmag,
    nuvmag,
    e_nuvmag,
    fuv_a,
    e_fuv_a,
    nuv_a,
    e_nuv_a,
    fuv_4,
    e_fuv_4,
    nuv_4,
    e_nuv_4,
    fuv_6,
    e_fuv_6,
    nuv_6,
    e_nuv_6,
    fafl,
    nafl,
    fexf,
    nexf,
    fflux,
    e_fflux,
    nflux,
    e_nflux,
    fxpos,
    fypos,
    nxpos,
    nypos,
    fima,
    nima,
    fr,
    nr,
    ns_g,
    fs_g,
    nell,
    fell,
    npa,
    e_npa,
    fpa,
    e_fpa,
    fnr,
    f3r,
    nar,
    narms,
    nbrms,
    far,
    farms,
    fbrms,
    nuvw,
    fuvw,
    prob,
    sep,
    nerr,
    ferr,
    ierr,
    nperr,
    fperr,
    cv,
    g,
    n,
    primid,
    groupid,
    gd,
    nd,
    primidd,
    groupidd,
    grouptot,
    oname,
    #[strum(serialize = "\"size\"")]
    size,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(GalexAis.string(), col_strings);
}
