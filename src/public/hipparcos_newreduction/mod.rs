use crate::schema::Schema;

pub struct HipparcosNewreduction;

impl Schema for HipparcosNewreduction {
    fn string(&self) -> String {
        "hipparcos_newreduction".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
    hip,
    ic,
    ra,
    dec,
    ra_rad,
    de_rad,
    plx,
    pm_ra,
    pm_de,
    e_ra_rad,
    e_de_rad,
    e_plx,
    e_pm_ra,
    e_pm_de,
    f1,
    f2,
    nc,
    ntr,
    u3,
    u4,
    u5,
    u6,
    u7,
    u8,
    u9,
    sn,
    so,
    var,
    u1,
    u2,
    u10,
    u11,
    u12,
    u13,
    u14,
    u15,
    hp_mag,
    b_v,
    v_i,
    e_hp_mag,
    e_b_v,
    s_hp,
    va,
}

#[cfg(test)]
pub(crate) fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(HipparcosNewreduction.string(), col_strings);
}
