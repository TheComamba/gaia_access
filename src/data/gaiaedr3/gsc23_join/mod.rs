// This code is generated by generate_code.py, do not modify it manually.
use crate::traits::{Column, Table};

#[allow(non_camel_case_types)]
pub struct gsc23_join;

impl Table for gsc23_join {
    fn string(&self) -> String {
        "gsc23_join".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    original_ext_source_id,
    clean_gsc23_oid,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::original_ext_source_id.to_string());
    col_strings.push(Col::clean_gsc23_oid.to_string());
    map.insert(gsc23_join.string(), col_strings);
}
