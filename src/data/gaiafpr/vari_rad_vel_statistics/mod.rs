// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct vari_rad_vel_statistics;

impl Table for vari_rad_vel_statistics {
    fn string(&self) -> String {
        "vari_rad_vel_statistics".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    num_selected_rv,
    mean_obs_time_rv,
    time_duration_rv,
    min_rv,
    max_rv,
    mean_rv,
    median_rv,
    range_rv,
    std_dev_rv,
    skewness_rv,
    kurtosis_rv,
    mad_rv,
    abbe_rv,
    iqr_rv,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::num_selected_rv.to_string());
    col_strings.push(Col::mean_obs_time_rv.to_string());
    col_strings.push(Col::time_duration_rv.to_string());
    col_strings.push(Col::min_rv.to_string());
    col_strings.push(Col::max_rv.to_string());
    col_strings.push(Col::mean_rv.to_string());
    col_strings.push(Col::median_rv.to_string());
    col_strings.push(Col::range_rv.to_string());
    col_strings.push(Col::std_dev_rv.to_string());
    col_strings.push(Col::skewness_rv.to_string());
    col_strings.push(Col::kurtosis_rv.to_string());
    col_strings.push(Col::mad_rv.to_string());
    col_strings.push(Col::abbe_rv.to_string());
    col_strings.push(Col::iqr_rv.to_string());
    map.insert(vari_rad_vel_statistics.string(), col_strings);
}
