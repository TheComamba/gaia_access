// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the ravedr5_dr5 table.

use crate::traits::{Column, Table};

/// The ravedr5_dr5 table.
#[allow(non_camel_case_types)]
pub struct ravedr5_dr5;

impl Table for ravedr5_dr5 {
    fn string(&self) -> String {
        "ravedr5_dr5".to_string()
    }
}

/// The columns in the ravedr5_dr5 table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    name,
    healpix,
    rave,
    ra,
    dec,
    glon,
    glat,
    hrv,
    e_hrv,
    s_hrv,
    s_med_hrv,
    snrs,
    snrk,
    teffk,
    cteffk,
    e_teffk,
    s_med_teffk,
    s_teffk,
    loggk,
    cloggk,
    e_loggk,
    s_loggk,
    s_med_loggk,
    m_h_k,
    m_h_nk,
    e__m_h_k,
    s__m_h_k,
    s_med__m_h_k,
    chisqk,
    qk,
    teffir,
    e_teffir,
    irdirect,
    mg_h_c,
    o_mg_h_c,
    al_h_c,
    o_al_h_c,
    si_h_c,
    o_si_h_c,
    ti_h_c,
    o_ti_h_c,
    fe_h_c,
    o_fe_h_c,
    ni_h_c,
    o_ni_h_c,
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
    repflag,
    clsflag,
    fpflag,
    tgas,
    ratgas,
    detgas,
    pmratgas,
    e_pmratgas,
    pmdetgas,
    e_pmdetgas,
    plxtgas,
    e_plxtgas,
    gmagtgas,
    gfluxtgas,
    e_gfluxtgas,
    hipparcos,
    tycho2,
    distt2,
    xt2,
    btmagt2,
    e_btmagt2,
    vtmagt2,
    e_vtmagt2,
    pmrat2,
    e_pmrat2,
    pmdet2,
    e_pmdet2,
    ucac4,
    distu4,
    xu4,
    pmrau4,
    e_pmrau4,
    pmdeu4,
    e_pmdeu4,
    ppmxl,
    distp,
    xp,
    pmrap,
    e_pmrap,
    pmdep,
    e_pmdep,
    col2mass,
    dist2,
    x2,
    jmag2,
    e_jmag2,
    hmag2,
    e_hmag2,
    kmag2,
    e_kmag2,
    allwise,
    distw,
    xw,
    w1magw,
    e_w1magw,
    w2magw,
    e_w2magw,
    w3magw,
    e_w3magw,
    w4magw,
    e_w4magw,
    ccflagsw,
    extflgw,
    varflgw,
    phqualw,
    distadr9,
    xadr9,
    bmagadr9,
    e_bmagadr9,
    vmagadr9,
    e_vmagadr9,
    gpmagadr9,
    e_gpmagadr9,
    rpmagadr9,
    e_rpmagadr9,
    ipmagadr9,
    e_ipmagadr9,
    denis,
    distd,
    xd,
    imagd,
    e_imagd,
    jmagd,
    e_jmagd,
    kmagd,
    e_kmagd,
    usnob1,
    distub1,
    xub1,
    b1magub1,
    r1magub1,
    b2magub1,
    r2magub1,
    imagub1,
    pmraub1,
    e_pmraub1,
    pmdeub1,
    e_pmdeub1,
    obsdate,
    fieldname,
    fibernb,
    platenb,
    mjdobs,
    lststart,
    lstend,
    utcstart,
    utcend,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::name.to_string());
    col_strings.push(Col::healpix.to_string());
    col_strings.push(Col::rave.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::glon.to_string());
    col_strings.push(Col::glat.to_string());
    col_strings.push(Col::hrv.to_string());
    col_strings.push(Col::e_hrv.to_string());
    col_strings.push(Col::s_hrv.to_string());
    col_strings.push(Col::s_med_hrv.to_string());
    col_strings.push(Col::snrs.to_string());
    col_strings.push(Col::snrk.to_string());
    col_strings.push(Col::teffk.to_string());
    col_strings.push(Col::cteffk.to_string());
    col_strings.push(Col::e_teffk.to_string());
    col_strings.push(Col::s_med_teffk.to_string());
    col_strings.push(Col::s_teffk.to_string());
    col_strings.push(Col::loggk.to_string());
    col_strings.push(Col::cloggk.to_string());
    col_strings.push(Col::e_loggk.to_string());
    col_strings.push(Col::s_loggk.to_string());
    col_strings.push(Col::s_med_loggk.to_string());
    col_strings.push(Col::m_h_k.to_string());
    col_strings.push(Col::m_h_nk.to_string());
    col_strings.push(Col::e__m_h_k.to_string());
    col_strings.push(Col::s__m_h_k.to_string());
    col_strings.push(Col::s_med__m_h_k.to_string());
    col_strings.push(Col::chisqk.to_string());
    col_strings.push(Col::qk.to_string());
    col_strings.push(Col::teffir.to_string());
    col_strings.push(Col::e_teffir.to_string());
    col_strings.push(Col::irdirect.to_string());
    col_strings.push(Col::mg_h_c.to_string());
    col_strings.push(Col::o_mg_h_c.to_string());
    col_strings.push(Col::al_h_c.to_string());
    col_strings.push(Col::o_al_h_c.to_string());
    col_strings.push(Col::si_h_c.to_string());
    col_strings.push(Col::o_si_h_c.to_string());
    col_strings.push(Col::ti_h_c.to_string());
    col_strings.push(Col::o_ti_h_c.to_string());
    col_strings.push(Col::fe_h_c.to_string());
    col_strings.push(Col::o_fe_h_c.to_string());
    col_strings.push(Col::ni_h_c.to_string());
    col_strings.push(Col::o_ni_h_c.to_string());
    col_strings.push(Col::a_fe_c.to_string());
    col_strings.push(Col::chisqc.to_string());
    col_strings.push(Col::fracc.to_string());
    col_strings.push(Col::avschl.to_string());
    col_strings.push(Col::dist.to_string());
    col_strings.push(Col::e_dist.to_string());
    col_strings.push(Col::logav.to_string());
    col_strings.push(Col::e_logav.to_string());
    col_strings.push(Col::plx.to_string());
    col_strings.push(Col::e_plx.to_string());
    col_strings.push(Col::dm.to_string());
    col_strings.push(Col::e_dm.to_string());
    col_strings.push(Col::ffb.to_string());
    col_strings.push(Col::fqb.to_string());
    col_strings.push(Col::ngauss.to_string());
    col_strings.push(Col::gm1.to_string());
    col_strings.push(Col::gs1.to_string());
    col_strings.push(Col::gf1.to_string());
    col_strings.push(Col::gm2.to_string());
    col_strings.push(Col::gs2.to_string());
    col_strings.push(Col::gf2.to_string());
    col_strings.push(Col::gm3.to_string());
    col_strings.push(Col::gs3.to_string());
    col_strings.push(Col::gf3.to_string());
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
    col_strings.push(Col::repflag.to_string());
    col_strings.push(Col::clsflag.to_string());
    col_strings.push(Col::fpflag.to_string());
    col_strings.push(Col::tgas.to_string());
    col_strings.push(Col::ratgas.to_string());
    col_strings.push(Col::detgas.to_string());
    col_strings.push(Col::pmratgas.to_string());
    col_strings.push(Col::e_pmratgas.to_string());
    col_strings.push(Col::pmdetgas.to_string());
    col_strings.push(Col::e_pmdetgas.to_string());
    col_strings.push(Col::plxtgas.to_string());
    col_strings.push(Col::e_plxtgas.to_string());
    col_strings.push(Col::gmagtgas.to_string());
    col_strings.push(Col::gfluxtgas.to_string());
    col_strings.push(Col::e_gfluxtgas.to_string());
    col_strings.push(Col::hipparcos.to_string());
    col_strings.push(Col::tycho2.to_string());
    col_strings.push(Col::distt2.to_string());
    col_strings.push(Col::xt2.to_string());
    col_strings.push(Col::btmagt2.to_string());
    col_strings.push(Col::e_btmagt2.to_string());
    col_strings.push(Col::vtmagt2.to_string());
    col_strings.push(Col::e_vtmagt2.to_string());
    col_strings.push(Col::pmrat2.to_string());
    col_strings.push(Col::e_pmrat2.to_string());
    col_strings.push(Col::pmdet2.to_string());
    col_strings.push(Col::e_pmdet2.to_string());
    col_strings.push(Col::ucac4.to_string());
    col_strings.push(Col::distu4.to_string());
    col_strings.push(Col::xu4.to_string());
    col_strings.push(Col::pmrau4.to_string());
    col_strings.push(Col::e_pmrau4.to_string());
    col_strings.push(Col::pmdeu4.to_string());
    col_strings.push(Col::e_pmdeu4.to_string());
    col_strings.push(Col::ppmxl.to_string());
    col_strings.push(Col::distp.to_string());
    col_strings.push(Col::xp.to_string());
    col_strings.push(Col::pmrap.to_string());
    col_strings.push(Col::e_pmrap.to_string());
    col_strings.push(Col::pmdep.to_string());
    col_strings.push(Col::e_pmdep.to_string());
    col_strings.push(Col::col2mass.to_string());
    col_strings.push(Col::dist2.to_string());
    col_strings.push(Col::x2.to_string());
    col_strings.push(Col::jmag2.to_string());
    col_strings.push(Col::e_jmag2.to_string());
    col_strings.push(Col::hmag2.to_string());
    col_strings.push(Col::e_hmag2.to_string());
    col_strings.push(Col::kmag2.to_string());
    col_strings.push(Col::e_kmag2.to_string());
    col_strings.push(Col::allwise.to_string());
    col_strings.push(Col::distw.to_string());
    col_strings.push(Col::xw.to_string());
    col_strings.push(Col::w1magw.to_string());
    col_strings.push(Col::e_w1magw.to_string());
    col_strings.push(Col::w2magw.to_string());
    col_strings.push(Col::e_w2magw.to_string());
    col_strings.push(Col::w3magw.to_string());
    col_strings.push(Col::e_w3magw.to_string());
    col_strings.push(Col::w4magw.to_string());
    col_strings.push(Col::e_w4magw.to_string());
    col_strings.push(Col::ccflagsw.to_string());
    col_strings.push(Col::extflgw.to_string());
    col_strings.push(Col::varflgw.to_string());
    col_strings.push(Col::phqualw.to_string());
    col_strings.push(Col::distadr9.to_string());
    col_strings.push(Col::xadr9.to_string());
    col_strings.push(Col::bmagadr9.to_string());
    col_strings.push(Col::e_bmagadr9.to_string());
    col_strings.push(Col::vmagadr9.to_string());
    col_strings.push(Col::e_vmagadr9.to_string());
    col_strings.push(Col::gpmagadr9.to_string());
    col_strings.push(Col::e_gpmagadr9.to_string());
    col_strings.push(Col::rpmagadr9.to_string());
    col_strings.push(Col::e_rpmagadr9.to_string());
    col_strings.push(Col::ipmagadr9.to_string());
    col_strings.push(Col::e_ipmagadr9.to_string());
    col_strings.push(Col::denis.to_string());
    col_strings.push(Col::distd.to_string());
    col_strings.push(Col::xd.to_string());
    col_strings.push(Col::imagd.to_string());
    col_strings.push(Col::e_imagd.to_string());
    col_strings.push(Col::jmagd.to_string());
    col_strings.push(Col::e_jmagd.to_string());
    col_strings.push(Col::kmagd.to_string());
    col_strings.push(Col::e_kmagd.to_string());
    col_strings.push(Col::usnob1.to_string());
    col_strings.push(Col::distub1.to_string());
    col_strings.push(Col::xub1.to_string());
    col_strings.push(Col::b1magub1.to_string());
    col_strings.push(Col::r1magub1.to_string());
    col_strings.push(Col::b2magub1.to_string());
    col_strings.push(Col::r2magub1.to_string());
    col_strings.push(Col::imagub1.to_string());
    col_strings.push(Col::pmraub1.to_string());
    col_strings.push(Col::e_pmraub1.to_string());
    col_strings.push(Col::pmdeub1.to_string());
    col_strings.push(Col::e_pmdeub1.to_string());
    col_strings.push(Col::obsdate.to_string());
    col_strings.push(Col::fieldname.to_string());
    col_strings.push(Col::fibernb.to_string());
    col_strings.push(Col::platenb.to_string());
    col_strings.push(Col::mjdobs.to_string());
    col_strings.push(Col::lststart.to_string());
    col_strings.push(Col::lstend.to_string());
    col_strings.push(Col::utcstart.to_string());
    col_strings.push(Col::utcend.to_string());
    map.insert(ravedr5_dr5.string(), col_strings);
}
