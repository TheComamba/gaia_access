// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the alerts_mixedin_sourceids table.

use crate::traits::{Column, Table};

/// Some photometric science alerts mix transits from their primary source and other sources in the catalogue. This is done when those sources are believed to be associated with a single, astrophysical source.
///
/// This table lists the identifier for these mixed-in sources, linking them to the primary sourceIds listed in table \texttt{ScienceAlerts}.
#[allow(non_camel_case_types)]
pub struct alerts_mixedin_sourceids;

impl Table for alerts_mixedin_sourceids {
    fn string(&self) -> String {
        "alerts_mixedin_sourceids".to_string()
    }
}

/// The columns in the alerts_mixedin_sourceids table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Solution Identifier
    solution_id,
    /// Primary sourceId associated to the alert
    alert_source_id,
    /// Additional sourceId, if any, associated to the alert
    mixed_in_source_id,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::alert_source_id.to_string());
    col_strings.push(Col::mixed_in_source_id.to_string());
    map.insert(alerts_mixedin_sourceids.string(), col_strings);
}
