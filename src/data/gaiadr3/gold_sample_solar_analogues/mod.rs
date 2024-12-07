// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the gold_sample_solar_analogues table.

use crate::traits::{Column, Table};

/// \textit{Gaia} DR3 source IDs of candidates for solar analogues selected from GSP-Spec via effective temperature, surface gravity and metallicity and from FLAME via mass and radius.
/// For more details please refer to \cite{DR3-DPACP-123}.
#[allow(non_camel_case_types)]
pub struct gold_sample_solar_analogues;

impl Table for gold_sample_solar_analogues {
    fn string(&self) -> String {
        "gold_sample_solar_analogues".to_string()
    }
}

/// The columns in the gold_sample_solar_analogues table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Unique source identifier (unique within a particular Data Release)
    source_id,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the gold_sample_solar_analogues table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    map.insert(gold_sample_solar_analogues.string(), col_strings);
}
