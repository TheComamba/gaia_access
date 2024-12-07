// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the key_columns table.

use crate::traits::{Column, Table};

/// TAP SCHEMA key columns
#[allow(non_camel_case_types)]
pub struct key_columns;

impl Table for key_columns {
    fn string(&self) -> String {
        "key_columns".to_string()
    }
}

/// The columns in the key_columns table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// The from_column column. (No further description available)
    from_column,
    /// The key_id column. (No further description available)
    key_id,
    /// The target_column column. (No further description available)
    target_column,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the key_columns table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::from_column.to_string());
    col_strings.push(Col::key_id.to_string());
    col_strings.push(Col::target_column.to_string());
    map.insert(key_columns.string(), col_strings);
}
