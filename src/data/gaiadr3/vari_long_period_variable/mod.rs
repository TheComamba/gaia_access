// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct vari_long_period_variable;

impl Table for vari_long_period_variable {
    fn string(&self) -> String {
        "vari_long_period_variable".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    frequency,
    frequency_error,
    amplitude,
    median_delta_wl_rp,
    is_cstar,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::frequency.to_string());
    col_strings.push(Col::frequency_error.to_string());
    col_strings.push(Col::amplitude.to_string());
    col_strings.push(Col::median_delta_wl_rp.to_string());
    col_strings.push(Col::is_cstar.to_string());
    map.insert(vari_long_period_variable.string(), col_strings);
}
