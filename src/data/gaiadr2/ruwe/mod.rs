// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the ruwe table.

use crate::traits::{Column, Table};

/// This table contains the Renormalised Unit Weight Error (RUWE) associated
/// to each source in gaia_source.
///
/// The RUWE is expected to be around 1.0 for sources where the single-star
/// model provides a good fit to the astrometric observations. A value
/// significantly greater than 1.0 (say, >1.4) could indicate that the
/// source is non-single or otherwise problematic for the astrometric
/// solution.
///
/// The desciption of how this parameter is calculated is described in the
/// document ”Re-normalising the astrometric chi-square in Gaia DR2”, which
/// can be downloaded from:
/// https://www.cosmos.esa.int/web/gaia/public-dpac-documents
#[allow(non_camel_case_types)]
pub struct ruwe;

impl Table for ruwe {
    fn string(&self) -> String {
        "ruwe".to_string()
    }
}

/// The columns in the ruwe table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Unique source identifier (unique within a particular Data Release)
    source_id,
    /// Renormalised unit weight error
    ruwe,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the ruwe table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::ruwe.to_string());
    map.insert(ruwe.string(), col_strings);
}
