// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the apassdr9 table.

use crate::traits::{Column, Table};

/// The AAVSO Photometric All-Sky Survey - Data Release 9
///     This publication makes use of data products from the AAVSO
///     Photometric All Sky Survey (APASS). Funded by the Robert Martin Ayers
///     Sciences Fund and the National Science Foundation. Original catalogue released by Henden et al. 2015 AAS Meeting #225, id.336.16. Data retrieved using the VizieR catalogue access tool, CDS, Strasbourg, France. The original description of the VizieR service was published in A&AS 143, 23. VizieR catalogue II/336.
#[allow(non_camel_case_types)]
pub struct apassdr9;

impl Table for apassdr9 {
    fn string(&self) -> String {
        "apassdr9".to_string()
    }
}

/// The columns in the apassdr9 table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Record number assigned by the VizieR team. Should Not be used for identification. However, the lack of a unique ID in APASS has made the VizieR recno the de-facto source identifier. For instance, it is used as such in the Gaia DR2 cross-match tables.
    recno,
    /// Right ascension in decimal degrees (J2000)
    raj2000,
    /// Declination in decimal degrees (J2000)
    dej2000,
    /// [0/2.4] RA uncertainty
    e_ra,
    /// [0/2.4] DEC uncertainty
    e_dec,
    /// [20110001/9999988888] Field name
    field,
    /// [2/387] Number of observed nights
    nobs,
    /// [2/3476] Number of images for this field, usually nobs*5
    mobs,
    /// [-7.5/13]? B-V color index
    b_v,
    /// [0/10.1]? B-V uncertainty
    e_b_v,
    /// [5.5/27.4]? Johnson V-band magnitude
    vmag,
    /// [0/7]? Vmag uncertainty
    e_vmag,
    /// [0/1]? Uncertainty flag on e_Vmag (1)
    u_e_vmag,
    /// [5.4/27.3]? Johnson B-band magnitude
    bmag,
    /// [0/10]? Bmag uncertainty
    e_bmag,
    /// [0/1]? Uncertainty flag on e_Bmag (1)
    u_e_bmag,
    /// [5.9/24.2]? g'-band AB magnitude, Sloan filter
    g_mag,
    /// [0/9.7]? g'mag uncertainty
    e_g_mag,
    /// [0/1]? Uncertainty flag on e_g'mag (1)
    u_e_g_mag,
    /// [5.1/23.9]? r'-band AB magnitude, Sloan filter
    r_mag,
    /// [0/6.5]? r'mag uncertainty
    e_r_mag,
    /// [0/1]? Uncertainty flag on e_r'mag (1)
    u_e_r_mag,
    /// [4.2/29.1]? i'-band AB magnitude, Sloan filter
    i_mag,
    /// [0/9.6]? i'mag uncertainty
    e_i_mag,
    /// [0/1]? Uncertainty flag on e_i'mag (1)
    u_e_i_mag,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the apassdr9 table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::recno.to_string());
    col_strings.push(Col::raj2000.to_string());
    col_strings.push(Col::dej2000.to_string());
    col_strings.push(Col::e_ra.to_string());
    col_strings.push(Col::e_dec.to_string());
    col_strings.push(Col::field.to_string());
    col_strings.push(Col::nobs.to_string());
    col_strings.push(Col::mobs.to_string());
    col_strings.push(Col::b_v.to_string());
    col_strings.push(Col::e_b_v.to_string());
    col_strings.push(Col::vmag.to_string());
    col_strings.push(Col::e_vmag.to_string());
    col_strings.push(Col::u_e_vmag.to_string());
    col_strings.push(Col::bmag.to_string());
    col_strings.push(Col::e_bmag.to_string());
    col_strings.push(Col::u_e_bmag.to_string());
    col_strings.push(Col::g_mag.to_string());
    col_strings.push(Col::e_g_mag.to_string());
    col_strings.push(Col::u_e_g_mag.to_string());
    col_strings.push(Col::r_mag.to_string());
    col_strings.push(Col::e_r_mag.to_string());
    col_strings.push(Col::u_e_r_mag.to_string());
    col_strings.push(Col::i_mag.to_string());
    col_strings.push(Col::e_i_mag.to_string());
    col_strings.push(Col::u_e_i_mag.to_string());
    map.insert(apassdr9.string(), col_strings);
}
