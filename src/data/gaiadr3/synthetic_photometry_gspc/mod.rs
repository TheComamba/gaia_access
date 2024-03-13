// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct synthetic_photometry_gspc;

impl Table for synthetic_photometry_gspc {
    fn string(&self) -> String {
        "synthetic_photometry_gspc".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    source_id,
    c_star,
    u_jkc_flux,
    u_jkc_flux_error,
    u_jkc_mag,
    u_jkc_flag,
    b_jkc_flux,
    b_jkc_flux_error,
    b_jkc_mag,
    b_jkc_flag,
    v_jkc_flux,
    v_jkc_flux_error,
    v_jkc_mag,
    v_jkc_flag,
    r_jkc_flux,
    r_jkc_flux_error,
    r_jkc_mag,
    r_jkc_flag,
    i_jkc_flux,
    i_jkc_flux_error,
    i_jkc_mag,
    i_jkc_flag,
    u_sdss_flux,
    u_sdss_flux_error,
    u_sdss_mag,
    u_sdss_flag,
    g_sdss_flux,
    g_sdss_flux_error,
    g_sdss_mag,
    g_sdss_flag,
    r_sdss_flux,
    r_sdss_flux_error,
    r_sdss_mag,
    r_sdss_flag,
    i_sdss_flux,
    i_sdss_flux_error,
    i_sdss_mag,
    i_sdss_flag,
    z_sdss_flux,
    z_sdss_flux_error,
    z_sdss_mag,
    z_sdss_flag,
    y_ps1_flux,
    y_ps1_flux_error,
    y_ps1_mag,
    y_ps1_flag,
    f606w_acswfc_flux,
    f606w_acswfc_flux_error,
    f606w_acswfc_mag,
    f606w_acswfc_flag,
    f814w_acswfc_flux,
    f814w_acswfc_flux_error,
    f814w_acswfc_mag,
    f814w_acswfc_flag,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::c_star.to_string());
    col_strings.push(Col::u_jkc_flux.to_string());
    col_strings.push(Col::u_jkc_flux_error.to_string());
    col_strings.push(Col::u_jkc_mag.to_string());
    col_strings.push(Col::u_jkc_flag.to_string());
    col_strings.push(Col::b_jkc_flux.to_string());
    col_strings.push(Col::b_jkc_flux_error.to_string());
    col_strings.push(Col::b_jkc_mag.to_string());
    col_strings.push(Col::b_jkc_flag.to_string());
    col_strings.push(Col::v_jkc_flux.to_string());
    col_strings.push(Col::v_jkc_flux_error.to_string());
    col_strings.push(Col::v_jkc_mag.to_string());
    col_strings.push(Col::v_jkc_flag.to_string());
    col_strings.push(Col::r_jkc_flux.to_string());
    col_strings.push(Col::r_jkc_flux_error.to_string());
    col_strings.push(Col::r_jkc_mag.to_string());
    col_strings.push(Col::r_jkc_flag.to_string());
    col_strings.push(Col::i_jkc_flux.to_string());
    col_strings.push(Col::i_jkc_flux_error.to_string());
    col_strings.push(Col::i_jkc_mag.to_string());
    col_strings.push(Col::i_jkc_flag.to_string());
    col_strings.push(Col::u_sdss_flux.to_string());
    col_strings.push(Col::u_sdss_flux_error.to_string());
    col_strings.push(Col::u_sdss_mag.to_string());
    col_strings.push(Col::u_sdss_flag.to_string());
    col_strings.push(Col::g_sdss_flux.to_string());
    col_strings.push(Col::g_sdss_flux_error.to_string());
    col_strings.push(Col::g_sdss_mag.to_string());
    col_strings.push(Col::g_sdss_flag.to_string());
    col_strings.push(Col::r_sdss_flux.to_string());
    col_strings.push(Col::r_sdss_flux_error.to_string());
    col_strings.push(Col::r_sdss_mag.to_string());
    col_strings.push(Col::r_sdss_flag.to_string());
    col_strings.push(Col::i_sdss_flux.to_string());
    col_strings.push(Col::i_sdss_flux_error.to_string());
    col_strings.push(Col::i_sdss_mag.to_string());
    col_strings.push(Col::i_sdss_flag.to_string());
    col_strings.push(Col::z_sdss_flux.to_string());
    col_strings.push(Col::z_sdss_flux_error.to_string());
    col_strings.push(Col::z_sdss_mag.to_string());
    col_strings.push(Col::z_sdss_flag.to_string());
    col_strings.push(Col::y_ps1_flux.to_string());
    col_strings.push(Col::y_ps1_flux_error.to_string());
    col_strings.push(Col::y_ps1_mag.to_string());
    col_strings.push(Col::y_ps1_flag.to_string());
    col_strings.push(Col::f606w_acswfc_flux.to_string());
    col_strings.push(Col::f606w_acswfc_flux_error.to_string());
    col_strings.push(Col::f606w_acswfc_mag.to_string());
    col_strings.push(Col::f606w_acswfc_flag.to_string());
    col_strings.push(Col::f814w_acswfc_flux.to_string());
    col_strings.push(Col::f814w_acswfc_flux_error.to_string());
    col_strings.push(Col::f814w_acswfc_mag.to_string());
    col_strings.push(Col::f814w_acswfc_flag.to_string());
    map.insert(synthetic_photometry_gspc.string(), col_strings);
}