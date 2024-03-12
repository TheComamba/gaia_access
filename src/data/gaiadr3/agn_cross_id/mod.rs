// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct agn_cross_id;

impl Table for agn_cross_id {
    fn string(&self) -> String {
        "agn_cross_id".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_name_in_catalogue,
    source_id,
    catalogue_name,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_name_in_catalogue.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::catalogue_name.to_string());
    map.insert(agn_cross_id.string(), col_strings);
}
