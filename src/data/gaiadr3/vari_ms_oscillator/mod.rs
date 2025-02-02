// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the vari_ms_oscillator table.

use crate::traits::{Column, Table};

/// This table describes the main-sequence oscillators.
#[allow(non_camel_case_types)]
pub struct vari_ms_oscillator;

impl Table for vari_ms_oscillator {
    fn string(&self) -> String {
        "vari_ms_oscillator".to_string()
    }
}

/// The columns in the vari_ms_oscillator table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Solution Identifier
    solution_id,
    /// Unique source identifier
    source_id,
    /// Best frequency
    frequency1,
    /// False alarm probability corresponding to the best frequency
    fap_g_freq1,
    /// Half peak-to-peak amplitude in the G band of the best frequency
    amplitude_g_freq1,
    /// Phase of the oscillation in the G band corresponding to best frequency
    phase_g_freq1,
    /// Number of significant harmonics of the best frequency
    num_harmonics,
    /// Half peak-to-peak amplitude in the G band of the second harmonic of the best frequency
    amplitude_g_freq1_harm2,
    /// Phase of the oscillation in the G band corresponding to the second harmonic of the best frequency
    phase_g_freq1_harm2,
    /// Half peak-to-peak amplitude in the G band of the third harmonic of the best frequency
    amplitude_g_freq1_harm3,
    /// Phase of the oscillation in the G band corresponding to the third harmonic of the best frequency
    phase_g_freq1_harm3,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the vari_ms_oscillator table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::frequency1.to_string());
    col_strings.push(Col::fap_g_freq1.to_string());
    col_strings.push(Col::amplitude_g_freq1.to_string());
    col_strings.push(Col::phase_g_freq1.to_string());
    col_strings.push(Col::num_harmonics.to_string());
    col_strings.push(Col::amplitude_g_freq1_harm2.to_string());
    col_strings.push(Col::phase_g_freq1_harm2.to_string());
    col_strings.push(Col::amplitude_g_freq1_harm3.to_string());
    col_strings.push(Col::phase_g_freq1_harm3.to_string());
    map.insert(vari_ms_oscillator.string(), col_strings);
}
