// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct ppmxl_original_valid;

impl Table for ppmxl_original_valid {
    fn string(&self) -> String {
        "ppmxl_original_valid".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    ppmxl_oid,
    ipix,
    ra,
    dec,
    ra_error_epra,
    dec_error_epde,
    pmra,
    pmde,
    pmra_error,
    pmde_error,
    n_epochs,
    epra,
    epde,
    b1mag,
    b2mag,
    r1mag,
    r2mag,
    imag,
    flags,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::ppmxl_oid.to_string());
    col_strings.push(Col::ipix.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::ra_error_epra.to_string());
    col_strings.push(Col::dec_error_epde.to_string());
    col_strings.push(Col::pmra.to_string());
    col_strings.push(Col::pmde.to_string());
    col_strings.push(Col::pmra_error.to_string());
    col_strings.push(Col::pmde_error.to_string());
    col_strings.push(Col::n_epochs.to_string());
    col_strings.push(Col::epra.to_string());
    col_strings.push(Col::epde.to_string());
    col_strings.push(Col::b1mag.to_string());
    col_strings.push(Col::b2mag.to_string());
    col_strings.push(Col::r1mag.to_string());
    col_strings.push(Col::r2mag.to_string());
    col_strings.push(Col::imag.to_string());
    col_strings.push(Col::flags.to_string());
    map.insert(ppmxl_original_valid.string(), col_strings);
}