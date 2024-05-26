// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the sdss_dr9_neighbourhood table.

use crate::traits::{Column, Table};

/// The sdss_dr9_neighbourhood table.
#[allow(non_camel_case_types)]
pub struct sdss_dr9_neighbourhood;

impl Table for sdss_dr9_neighbourhood {
    fn string(&self) -> String {
        "sdss_dr9_neighbourhood".to_string()
    }
}

/// The columns in the sdss_dr9_neighbourhood table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    sdssdr9_oid,
    source_id,
    original_ext_source_id,
    angular_distance,
    score,
    proper_motion_flag,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::sdssdr9_oid.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::original_ext_source_id.to_string());
    col_strings.push(Col::angular_distance.to_string());
    col_strings.push(Col::score.to_string());
    col_strings.push(Col::proper_motion_flag.to_string());
    map.insert(sdss_dr9_neighbourhood.string(), col_strings);
}
