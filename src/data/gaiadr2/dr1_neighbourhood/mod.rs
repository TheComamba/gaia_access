// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct dr1_neighbourhood;

impl Table for dr1_neighbourhood {
    fn string(&self) -> String {
        "dr1_neighbourhood".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    dr1_source_id,
    dr2_source_id,
    angular_distance,
    magnitude_difference,
    rank,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::dr1_source_id.to_string());
    col_strings.push(Col::dr2_source_id.to_string());
    col_strings.push(Col::angular_distance.to_string());
    col_strings.push(Col::magnitude_difference.to_string());
    col_strings.push(Col::rank.to_string());
    map.insert(dr1_neighbourhood.string(), col_strings);
}