// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the vari_microlensing table.

use crate::traits::{Column, Table};

/// The vari_microlensing table.
#[allow(non_camel_case_types)]
pub struct vari_microlensing;

impl Table for vari_microlensing {
    fn string(&self) -> String {
        "vari_microlensing".to_string()
    }
}

/// The columns in the vari_microlensing table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    paczynski0_g0,
    paczynski0_g0_error,
    paczynski0_bp0,
    paczynski0_bp0_error,
    paczynski0_rp0,
    paczynski0_rp0_error,
    paczynski0_u0,
    paczynski0_u0_error,
    paczynski0_te,
    paczynski0_te_error,
    paczynski0_tmax,
    paczynski0_tmax_error,
    paczynski0_chi2,
    paczynski0_chi2_dof,
    paczynski1_g0,
    paczynski1_g0_error,
    paczynski1_bp0,
    paczynski1_bp0_error,
    paczynski1_rp0,
    paczynski1_rp0_error,
    paczynski1_u0,
    paczynski1_u0_error,
    paczynski1_te,
    paczynski1_te_error,
    paczynski1_tmax,
    paczynski1_tmax_error,
    paczynski1_fs_g,
    paczynski1_fs_g_error,
    paczynski1_fs_bp,
    paczynski1_fs_bp_error,
    paczynski1_fs_rp,
    paczynski1_fs_rp_error,
    paczynski1_chi2,
    paczynski1_chi2_dof,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::paczynski0_g0.to_string());
    col_strings.push(Col::paczynski0_g0_error.to_string());
    col_strings.push(Col::paczynski0_bp0.to_string());
    col_strings.push(Col::paczynski0_bp0_error.to_string());
    col_strings.push(Col::paczynski0_rp0.to_string());
    col_strings.push(Col::paczynski0_rp0_error.to_string());
    col_strings.push(Col::paczynski0_u0.to_string());
    col_strings.push(Col::paczynski0_u0_error.to_string());
    col_strings.push(Col::paczynski0_te.to_string());
    col_strings.push(Col::paczynski0_te_error.to_string());
    col_strings.push(Col::paczynski0_tmax.to_string());
    col_strings.push(Col::paczynski0_tmax_error.to_string());
    col_strings.push(Col::paczynski0_chi2.to_string());
    col_strings.push(Col::paczynski0_chi2_dof.to_string());
    col_strings.push(Col::paczynski1_g0.to_string());
    col_strings.push(Col::paczynski1_g0_error.to_string());
    col_strings.push(Col::paczynski1_bp0.to_string());
    col_strings.push(Col::paczynski1_bp0_error.to_string());
    col_strings.push(Col::paczynski1_rp0.to_string());
    col_strings.push(Col::paczynski1_rp0_error.to_string());
    col_strings.push(Col::paczynski1_u0.to_string());
    col_strings.push(Col::paczynski1_u0_error.to_string());
    col_strings.push(Col::paczynski1_te.to_string());
    col_strings.push(Col::paczynski1_te_error.to_string());
    col_strings.push(Col::paczynski1_tmax.to_string());
    col_strings.push(Col::paczynski1_tmax_error.to_string());
    col_strings.push(Col::paczynski1_fs_g.to_string());
    col_strings.push(Col::paczynski1_fs_g_error.to_string());
    col_strings.push(Col::paczynski1_fs_bp.to_string());
    col_strings.push(Col::paczynski1_fs_bp_error.to_string());
    col_strings.push(Col::paczynski1_fs_rp.to_string());
    col_strings.push(Col::paczynski1_fs_rp_error.to_string());
    col_strings.push(Col::paczynski1_chi2.to_string());
    col_strings.push(Col::paczynski1_chi2_dof.to_string());
    map.insert(vari_microlensing.string(), col_strings);
}
