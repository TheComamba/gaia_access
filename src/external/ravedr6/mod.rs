use crate::{column::Column, schema::Schema};

pub struct Ravedr6;

impl Schema for Ravedr6 {
    fn string(&self) -> String {
        "ravedr6".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
    rave_obs_id,
    ra_input,
    dec_input,
    hrv_sparv,
    hrv_error_sparv,
    snr_med_sparv,
    age_bdasp,
    age_error_bdasp,
    teff_bdasp,
    teff_error_bdasp,
    logg_bdasp,
    logg_error_bdasp,
    mass_bdasp,
    mass_error_bdasp,
    log_a_v_bdasp,
    log_a_v_error_bdasp,
    m_h_cal_madera,
    m_h_error_madera,
    fe_h_gauguin,
    fe_h_error_gauguin,
    fe_h_chisq_gauguin,
    fe_h_nl_gauguin,
    al_h_gauguin,
    al_h_error_gauguin,
    al_h_chisq_gauguin,
    al_h_nl_gauguin,
    ni_h_gauguin,
    ni_h_error_gauguin,
    ni_h_chisq_gauguin,
    ni_h_nl_gauguin,
    alpha_fe_gauguin,
    alpha_fe_error_gauguin,
    alpha_fe_chisq_gauguin,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(Ravedr6.string(), col_strings);
}
