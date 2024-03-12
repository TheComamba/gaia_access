// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct ravedr5_neighbourhood;

impl Table for ravedr5_neighbourhood {
    fn string(&self) -> String {
        "ravedr5_neighbourhood".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    original_ext_source_id,
    angular_distance,
    score,
    xm_flag,
    clean_ravedr5_oid,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::original_ext_source_id.to_string());
    col_strings.push(Col::angular_distance.to_string());
    col_strings.push(Col::score.to_string());
    col_strings.push(Col::xm_flag.to_string());
    col_strings.push(Col::clean_ravedr5_oid.to_string());
    map.insert(ravedr5_neighbourhood.string(), col_strings);
}
