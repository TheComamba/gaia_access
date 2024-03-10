use crate::{column::Column, schema::Schema};

pub struct Gaiaedr3Spurious;

impl Schema for Gaiaedr3Spurious {
    fn string(&self) -> String {
        "gaiaedr3_spurious".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
    source_id,
    fidelity_v2,
    theta_arcsec_worst_source,
    norm_dg,
    dist_nearest_neighbor_at_least_m2_brighter,
    dist_nearest_neighbor_at_least_0_brighter,
    dist_nearest_neighbor_at_least_2_brighter,
    dist_nearest_neighbor_at_least_4_brighter,
    dist_nearest_neighbor_at_least_6_brighter,
    dist_nearest_neighbor_at_least_10_brighter,
    fidelity_v1,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(Gaiaedr3Spurious.string(), col_strings);
}
