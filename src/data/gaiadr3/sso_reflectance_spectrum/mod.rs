// This code is generated by generate_code.py, do not modify it manually.
use crate::traits::{Column, Table};

#[allow(non_camel_case_types)]
pub struct sso_reflectance_spectrum;

impl Table for sso_reflectance_spectrum {
    fn string(&self) -> String {
        "sso_reflectance_spectrum".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    solution_id,
    number_mp,
    denomination,
    nb_samples,
    num_of_spectra,
    reflectance_spectrum,
    reflectance_spectrum_err,
    wavelength,
    reflectance_spectrum_flag,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::number_mp.to_string());
    col_strings.push(Col::denomination.to_string());
    col_strings.push(Col::nb_samples.to_string());
    col_strings.push(Col::num_of_spectra.to_string());
    col_strings.push(Col::reflectance_spectrum.to_string());
    col_strings.push(Col::reflectance_spectrum_err.to_string());
    col_strings.push(Col::wavelength.to_string());
    col_strings.push(Col::reflectance_spectrum_flag.to_string());
    map.insert(sso_reflectance_spectrum.string(), col_strings);
}
