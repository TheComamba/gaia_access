use crate::{column::Column, schema::Schema};

pub struct Gaiaedr3Distance;

impl Schema for Gaiaedr3Distance {
    fn string(&self) -> String {
        "gaiaedr3_distance".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
    source_id,
    r_med_geo,
    r_lo_geo,
    r_hi_geo,
    r_med_photogeo,
    r_lo_photogeo,
    r_hi_photogeo,
    flag,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(Gaiaedr3Distance.string(), col_strings);
}
