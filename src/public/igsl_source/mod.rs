use crate::schema::Schema;

pub struct IgslSource;

impl Schema for IgslSource {
    fn string(&self) -> String {
        "igsl_source".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
    solution_id,
    source_id,
    ra,
    dec,
    ra_error,
    dec_error,
    ra_epoch,
    dec_epoch,
    source_position,
    pm_ra,
    pm_dec,
    pm_ra_error,
    pm_dec_error,
    source_mu,
    galactic_lon,
    galactic_lat,
    ecliptic_lon,
    ecliptic_lat,
    mag_bj,
    mag_bj_error,
    source_mag_bj,
    mag_rf,
    mag_rf_error,
    source_mag_rf,
    mag_g,
    mag_g_error,
    source_mag_g,
    mag_grvs,
    mag_grvs_error,
    source_mag_grvs,
    classification,
    source_classification,
    toggle_asc,
    aux_gsc23,
    aux_sdss,
    aux_ucac,
    aux_lqrf,
    aux_tycho,
    aux_hip,
    aux_ppmxl,
    aux_ogle,
    aux_tmass,
    aux_epc,
}

#[cfg(test)]
pub(crate) fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(IgslSource.string(), col_strings);
}
