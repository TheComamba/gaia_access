// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the vari_classifier_result table.

use crate::traits::{Column, Table};

/// Table with variability classification results of all classifiers,
/// identified by the classifierName column. In DR2 it contains a classifier
/// with classifierName=‘nTransits:2+’, description of which can be found in
/// VariClassifierDefinition and classes in VariClassifierClassDefinition.
#[allow(non_camel_case_types)]
pub struct vari_classifier_result;

impl Table for vari_classifier_result {
    fn string(&self) -> String {
        "vari_classifier_result".to_string()
    }
}

/// The columns in the vari_classifier_result table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Solution Identifier
    solution_id,
    /// Unique source identifier
    source_id,
    /// Classifier name used to produce this result, use for lookup in VariClassifierDefinition table
    classifier_name,
    /// Name of best class, see table VariClassifierDefinition for details of the class
    best_class_name,
    /// Score of the best class
    best_class_score,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the vari_classifier_result table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::classifier_name.to_string());
    col_strings.push(Col::best_class_name.to_string());
    col_strings.push(Col::best_class_score.to_string());
    map.insert(vari_classifier_result.string(), col_strings);
}
