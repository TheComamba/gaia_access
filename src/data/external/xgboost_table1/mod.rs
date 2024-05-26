// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the xgboost_table1 table.

use crate::traits::{Column, Table};

/// The xgboost_table1 table.
#[allow(non_camel_case_types)]
pub struct xgboost_table1;

impl Table for xgboost_table1 {
    fn string(&self) -> String {
        "xgboost_table1".to_string()
    }
}

/// The columns in the xgboost_table1 table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    catwise_w1,
    catwise_w2,
    in_training_sample,
    mh_xgboost,
    teff_xgboost,
    logg_xgboost,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::catwise_w1.to_string());
    col_strings.push(Col::catwise_w2.to_string());
    col_strings.push(Col::in_training_sample.to_string());
    col_strings.push(Col::mh_xgboost.to_string());
    col_strings.push(Col::teff_xgboost.to_string());
    col_strings.push(Col::logg_xgboost.to_string());
    map.insert(xgboost_table1.string(), col_strings);
}
