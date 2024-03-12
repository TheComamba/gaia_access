// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct vari_rrlyrae;

impl Table for vari_rrlyrae {
    fn string(&self) -> String {
        "vari_rrlyrae".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    pf,
    pf_error,
    p1_o,
    p1_o_error,
    p2_o,
    p2_o_error,
    p3_o,
    p3_o_error,
    epoch_g,
    epoch_g_error,
    epoch_bp,
    epoch_bp_error,
    epoch_rp,
    epoch_rp_error,
    int_average_g,
    int_average_g_error,
    int_average_bp,
    int_average_bp_error,
    int_average_rp,
    int_average_rp_error,
    peak_to_peak_g,
    peak_to_peak_g_error,
    peak_to_peak_bp,
    peak_to_peak_bp_error,
    peak_to_peak_rp,
    peak_to_peak_rp_error,
    metallicity,
    metallicity_error,
    r21_g,
    r21_g_error,
    r31_g,
    r31_g_error,
    phi21_g,
    phi21_g_error,
    phi31_g,
    phi31_g_error,
    num_clean_epochs_g,
    num_clean_epochs_bp,
    num_clean_epochs_rp,
    g_absorption,
    g_absorption_error,
    best_classification,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::pf.to_string());
    col_strings.push(Col::pf_error.to_string());
    col_strings.push(Col::p1_o.to_string());
    col_strings.push(Col::p1_o_error.to_string());
    col_strings.push(Col::p2_o.to_string());
    col_strings.push(Col::p2_o_error.to_string());
    col_strings.push(Col::p3_o.to_string());
    col_strings.push(Col::p3_o_error.to_string());
    col_strings.push(Col::epoch_g.to_string());
    col_strings.push(Col::epoch_g_error.to_string());
    col_strings.push(Col::epoch_bp.to_string());
    col_strings.push(Col::epoch_bp_error.to_string());
    col_strings.push(Col::epoch_rp.to_string());
    col_strings.push(Col::epoch_rp_error.to_string());
    col_strings.push(Col::int_average_g.to_string());
    col_strings.push(Col::int_average_g_error.to_string());
    col_strings.push(Col::int_average_bp.to_string());
    col_strings.push(Col::int_average_bp_error.to_string());
    col_strings.push(Col::int_average_rp.to_string());
    col_strings.push(Col::int_average_rp_error.to_string());
    col_strings.push(Col::peak_to_peak_g.to_string());
    col_strings.push(Col::peak_to_peak_g_error.to_string());
    col_strings.push(Col::peak_to_peak_bp.to_string());
    col_strings.push(Col::peak_to_peak_bp_error.to_string());
    col_strings.push(Col::peak_to_peak_rp.to_string());
    col_strings.push(Col::peak_to_peak_rp_error.to_string());
    col_strings.push(Col::metallicity.to_string());
    col_strings.push(Col::metallicity_error.to_string());
    col_strings.push(Col::r21_g.to_string());
    col_strings.push(Col::r21_g_error.to_string());
    col_strings.push(Col::r31_g.to_string());
    col_strings.push(Col::r31_g_error.to_string());
    col_strings.push(Col::phi21_g.to_string());
    col_strings.push(Col::phi21_g_error.to_string());
    col_strings.push(Col::phi31_g.to_string());
    col_strings.push(Col::phi31_g_error.to_string());
    col_strings.push(Col::num_clean_epochs_g.to_string());
    col_strings.push(Col::num_clean_epochs_bp.to_string());
    col_strings.push(Col::num_clean_epochs_rp.to_string());
    col_strings.push(Col::g_absorption.to_string());
    col_strings.push(Col::g_absorption_error.to_string());
    col_strings.push(Col::best_classification.to_string());
    map.insert(vari_rrlyrae.string(), col_strings);
}
