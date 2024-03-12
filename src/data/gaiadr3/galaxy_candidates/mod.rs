// This code is generated by generate_code.py, do not modify it manually
use crate::column::Column;
use crate::table::Table;

#[allow(non_camel_case_types)]
pub struct galaxy_candidates;

impl Table for galaxy_candidates {
    fn string(&self) -> String {
        "galaxy_candidates".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    solution_id,
    source_id,
    vari_best_class_name,
    vari_best_class_score,
    classprob_dsc_combmod_galaxy,
    classprob_dsc_combmod_quasar,
    classlabel_dsc,
    classlabel_dsc_joint,
    classlabel_oa,
    redshift_ugc,
    redshift_ugc_lower,
    redshift_ugc_upper,
    n_transits,
    posangle_sersic,
    posangle_sersic_error,
    intensity_sersic,
    intensity_sersic_error,
    radius_sersic,
    radius_sersic_error,
    ellipticity_sersic,
    ellipticity_sersic_error,
    n_sersic,
    n_sersic_error,
    l2_sersic,
    morph_params_corr_vec_sersic,
    flags_sersic,
    posangle_de_vaucouleurs,
    posangle_de_vaucouleurs_error,
    intensity_de_vaucouleurs,
    intensity_de_vaucouleurs_error,
    radius_de_vaucouleurs,
    radius_de_vaucouleurs_error,
    ellipticity_de_vaucouleurs,
    ellipticity_de_vaucouleurs_error,
    l2_de_vaucouleurs,
    morph_params_corr_vec_de_vaucouleurs,
    flags_de_vaucouleurs,
    source_selection_flags,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::vari_best_class_name.to_string());
    col_strings.push(Col::vari_best_class_score.to_string());
    col_strings.push(Col::classprob_dsc_combmod_galaxy.to_string());
    col_strings.push(Col::classprob_dsc_combmod_quasar.to_string());
    col_strings.push(Col::classlabel_dsc.to_string());
    col_strings.push(Col::classlabel_dsc_joint.to_string());
    col_strings.push(Col::classlabel_oa.to_string());
    col_strings.push(Col::redshift_ugc.to_string());
    col_strings.push(Col::redshift_ugc_lower.to_string());
    col_strings.push(Col::redshift_ugc_upper.to_string());
    col_strings.push(Col::n_transits.to_string());
    col_strings.push(Col::posangle_sersic.to_string());
    col_strings.push(Col::posangle_sersic_error.to_string());
    col_strings.push(Col::intensity_sersic.to_string());
    col_strings.push(Col::intensity_sersic_error.to_string());
    col_strings.push(Col::radius_sersic.to_string());
    col_strings.push(Col::radius_sersic_error.to_string());
    col_strings.push(Col::ellipticity_sersic.to_string());
    col_strings.push(Col::ellipticity_sersic_error.to_string());
    col_strings.push(Col::n_sersic.to_string());
    col_strings.push(Col::n_sersic_error.to_string());
    col_strings.push(Col::l2_sersic.to_string());
    col_strings.push(Col::morph_params_corr_vec_sersic.to_string());
    col_strings.push(Col::flags_sersic.to_string());
    col_strings.push(Col::posangle_de_vaucouleurs.to_string());
    col_strings.push(Col::posangle_de_vaucouleurs_error.to_string());
    col_strings.push(Col::intensity_de_vaucouleurs.to_string());
    col_strings.push(Col::intensity_de_vaucouleurs_error.to_string());
    col_strings.push(Col::radius_de_vaucouleurs.to_string());
    col_strings.push(Col::radius_de_vaucouleurs_error.to_string());
    col_strings.push(Col::ellipticity_de_vaucouleurs.to_string());
    col_strings.push(Col::ellipticity_de_vaucouleurs_error.to_string());
    col_strings.push(Col::l2_de_vaucouleurs.to_string());
    col_strings.push(Col::morph_params_corr_vec_de_vaucouleurs.to_string());
    col_strings.push(Col::flags_de_vaucouleurs.to_string());
    col_strings.push(Col::source_selection_flags.to_string());
    map.insert(galaxy_candidates.string(), col_strings);
}
