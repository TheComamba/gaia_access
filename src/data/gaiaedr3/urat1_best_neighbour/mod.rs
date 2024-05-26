// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the urat1_best_neighbour table.

use crate::traits::{Column, Table};

/// The urat1_best_neighbour table.
#[allow(non_camel_case_types)]
pub struct urat1_best_neighbour;

impl Table for urat1_best_neighbour {
    fn string(&self) -> String {
        "urat1_best_neighbour".to_string()
    }
}

/// The columns in the urat1_best_neighbour table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    original_ext_source_id,
    angular_distance,
    xm_flag,
    urat1_oid,
    number_of_neighbours,
    number_of_mates,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::original_ext_source_id.to_string());
    col_strings.push(Col::angular_distance.to_string());
    col_strings.push(Col::xm_flag.to_string());
    col_strings.push(Col::urat1_oid.to_string());
    col_strings.push(Col::number_of_neighbours.to_string());
    col_strings.push(Col::number_of_mates.to_string());
    map.insert(urat1_best_neighbour.string(), col_strings);
}
