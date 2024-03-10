use crate::{column::Column, schema::Schema};

pub struct Apassdr9;

impl Schema for Apassdr9 {
    fn string(&self) -> String {
        "apassdr9".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
    recno,
    raj2000,
    dej2000,
    e_ra,
    e_dec,
    field,
    nobs,
    mobs,
    b_v,
    e_b_v,
    vmag,
    e_vmag,
    u_e_vmag,
    bmag,
    e_bmag,
    u_e_bmag,
    g_mag,
    e_g_mag,
    u_e_g_mag,
    r_mag,
    e_r_mag,
    u_e_r_mag,
    i_mag,
    e_i_mag,
    u_e_i_mag,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(Apassdr9.string(), col_strings);
}
