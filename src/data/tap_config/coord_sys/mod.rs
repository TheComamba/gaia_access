// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the coord_sys table.

use crate::traits::{Column, Table};

/// The coord_sys table. (No further description available)
#[allow(non_camel_case_types)]
pub struct coord_sys;

impl Table for coord_sys {
    fn string(&self) -> String {
        "coord_sys".to_string()
    }
}

/// The columns in the coord_sys table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// The epoch column. (No further description available)
    epoch,
    /// The equinox column. (No further description available)
    equinox,
    /// The id column. (No further description available)
    id,
    /// The system column. (No further description available)
    system,
    /// The xml_output column. (No further description available)
    xml_output,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the coord_sys table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::epoch.to_string());
    col_strings.push(Col::equinox.to_string());
    col_strings.push(Col::id.to_string());
    col_strings.push(Col::system.to_string());
    col_strings.push(Col::xml_output.to_string());
    map.insert(coord_sys.string(), col_strings);
}
