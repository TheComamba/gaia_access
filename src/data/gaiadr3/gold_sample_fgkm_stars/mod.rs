// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct gold_sample_fgkm_stars;

impl Table for gold_sample_fgkm_stars {
    fn string(&self) -> String {
        "gold_sample_fgkm_stars".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    teff_gspphot,
    logg_gspphot,
    mh_gspphot,
    ag_gspphot,
    ebpminrp_gspphot,
    alphafe_gspspec,
    teff_gspspec,
    logg_gspspec,
    mh_gspspec,
    radius_flame,
    lum_flame,
    mass_flame,
    age_flame,
    evolstage_flame,
    radius_flame_spec,
    lum_flame_spec,
    mass_flame_spec,
    age_flame_spec,
    evolstage_flame_spec,
    spectraltype_esphs,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::teff_gspphot.to_string());
    col_strings.push(Col::logg_gspphot.to_string());
    col_strings.push(Col::mh_gspphot.to_string());
    col_strings.push(Col::ag_gspphot.to_string());
    col_strings.push(Col::ebpminrp_gspphot.to_string());
    col_strings.push(Col::alphafe_gspspec.to_string());
    col_strings.push(Col::teff_gspspec.to_string());
    col_strings.push(Col::logg_gspspec.to_string());
    col_strings.push(Col::mh_gspspec.to_string());
    col_strings.push(Col::radius_flame.to_string());
    col_strings.push(Col::lum_flame.to_string());
    col_strings.push(Col::mass_flame.to_string());
    col_strings.push(Col::age_flame.to_string());
    col_strings.push(Col::evolstage_flame.to_string());
    col_strings.push(Col::radius_flame_spec.to_string());
    col_strings.push(Col::lum_flame_spec.to_string());
    col_strings.push(Col::mass_flame_spec.to_string());
    col_strings.push(Col::age_flame_spec.to_string());
    col_strings.push(Col::evolstage_flame_spec.to_string());
    col_strings.push(Col::spectraltype_esphs.to_string());
    map.insert(gold_sample_fgkm_stars.string(), col_strings);
}
