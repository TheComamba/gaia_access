// This code is generated by generate_code.py, do not modify it manually.
use crate::traits::{Column, Table};

#[allow(non_camel_case_types)]
pub struct vari_classifier_definition;

impl Table for vari_classifier_definition {
    fn string(&self) -> String {
        "vari_classifier_definition".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    classifier_name,
    classifier_description,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::classifier_name.to_string());
    col_strings.push(Col::classifier_description.to_string());
    map.insert(vari_classifier_definition.string(), col_strings);
}
