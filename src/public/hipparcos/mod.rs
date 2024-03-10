use crate::schema::Schema;

pub struct Hipparcos;

impl Schema for Hipparcos {
    fn string(&self) -> String {
        "hipparcos".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
    hip,
    proxy,
    rahms,
    dedms,
    vmag,
    varflag,
    r_vmag,
    ra,
    de,
    astroref,
    plx,
    pmra,
    pmde,
    e_radeg,
    e_dedeg,
    e_plx,
    e_pmra,
    e_pmde,
    dera,
    plxra,
    plxde,
    pmrara,
    pmrade,
    pmraplx,
    pmdera,
    pmdede,
    pmdeplx,
    pmdepmra,
    f1,
    f2,
    btmag,
    e_btmag,
    vtmag,
    e_vtmag,
    m_btmag,
    b_v,
    e_b_v,
    r_b_v,
    v_i,
    e_v_i,
    r_v_i,
    combmag,
    hpmag,
    e_hpmag,
    hpscat,
    o_hpmag,
    m_hpmag,
    hpmax,
    hpmin,
    period,
    hvartype,
    morevar,
    morephoto,
    ccdm,
    n_ccdm,
    nsys,
    ncomp,
    multflag,
    source,
    qual,
    m_hip,
    theta,
    rho,
    e_rho,
    dhp,
    e_dhp,
    survey,
    chart,
    notes,
    hd,
    bd,
    cod,
    cpd,
    v_ired,
    sptype,
    r_sptype,
}

#[cfg(test)]
pub(crate) fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(Hipparcos.string(), col_strings);
}
