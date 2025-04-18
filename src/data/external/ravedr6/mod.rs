// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the ravedr6 table.

use crate::traits::{Column, Table};

/// This summary table contains a selection of values from several RAVE DR6 tables. The selection of fields is motivated by the findings in the RAVE DR6 papers (DOI:10.3847/1538-3881/ab9ab9, DOI:10.3847/1538-3881/ab9ab8). The RAVE DR6 release contains 22 tables and many more results from other pipelines. The summary in this table draws from the following RAVE DR6 tables: ravedr6.dr6_rave_sparv (DOI:10.17876/rave/dr.6/001): radial velocities and errors; ravedr6.dr6_obsdata (DOI:10.17876/rave/dr.6/002): input coordinates of the RAVE observations; rave_dr6.dr6_bdasp (DOI:10.107876/rave/dr.6/008): stellar atmospheric parameters as derived by pipeline BDASP ("Enhanced stellar atmospheric parameters inferred via a Bayesian pipeline using Gaia DR2 astrometric priors", DOI:10.3847/1538-3881/ab9ab8); ravedr6.dr6_madera (DOI:10.17876/rave/dr.6/006): values [M/H]; ravedr6.dr6_gauguin_bdasp (DOI:10.17876/rave/dr.6/010): element abundances for selected elements ("Abundances of the elements Fe, Al, and Ni, as well as an overall [alpha/Fe] ratio obtained using a new pipeline based on the GAUGUIN optimization method that is able to deal with variable signal-to-noise ratios", DOI:10.3847/1538-3881/ab9ab8). This table has been created in March 2022 based on an earlier version created by Harry Enke and Yori Fournier (AIP) in October 2021 for the Gaia ESA Archive using a custom query on the AIP server.
#[allow(non_camel_case_types)]
pub struct ravedr6;

impl Table for ravedr6 {
    fn string(&self) -> String {
        "ravedr6".to_string()
    }
}

/// The columns in the ravedr6 table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Unique Identifier for RAVE objects, Observation Date, Fieldname, Fibernumber
    rave_obs_id,
    /// Right ascension (J2000)
    ra_input,
    /// Declination (J2000)
    dec_input,
    /// Heliocentric radial velocity
    hrv_sparv,
    /// Error of heliocentric radial velocity
    hrv_error_sparv,
    /// Signal/Noise Ratio
    snr_med_sparv,
    /// Age estimation
    age_bdasp,
    /// Error of age estimation
    age_error_bdasp,
    /// Effective temperature
    teff_bdasp,
    /// Error of Effective Temperature
    teff_error_bdasp,
    /// Log gravity
    logg_bdasp,
    /// Error of Log gravity
    logg_error_bdasp,
    /// Mass
    mass_bdasp,
    /// Error of Mass
    mass_error_bdasp,
    /// log_10(Av) optical extinction
    log_a_v_bdasp,
    /// error log_10(Av) optical extinction
    log_a_v_error_bdasp,
    /// Calibrated abundance of [M/H]
    m_h_cal_madera,
    /// Error of abundance of [M/H]
    m_h_error_madera,
    /// [Fe/H] abundance of Fe
    fe_h_gauguin,
    /// Error of [Fe/H] abundance of Fe
    fe_h_error_gauguin,
    /// Chi-square of [Fe/H] abundance of Fe
    fe_h_chisq_gauguin,
    /// Number of used spectral lines in calc. of [Fe/H]
    fe_h_nl_gauguin,
    /// [Al/H] abundance of Al
    al_h_gauguin,
    /// Error of [Al/H] abundance of Al
    al_h_error_gauguin,
    /// Chi-square of [Al/H] abundance of Al
    al_h_chisq_gauguin,
    /// Number of used spectral lines in calc. of [Al/H]
    al_h_nl_gauguin,
    /// [Ni/H] abundance of Ni
    ni_h_gauguin,
    /// Error of [Ni/H] abundance of Ni
    ni_h_error_gauguin,
    /// Chi-square of [Ni/H] abundance of Ni
    ni_h_chisq_gauguin,
    /// Number of used spectral lines in calc. of [Ni/H]
    ni_h_nl_gauguin,
    /// [Alpha/Fe]
    alpha_fe_gauguin,
    /// Error of [Alpha/Fe]
    alpha_fe_error_gauguin,
    /// Chi-square of [Alpha/Fe]
    alpha_fe_chisq_gauguin,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the ravedr6 table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::rave_obs_id.to_string());
    col_strings.push(Col::ra_input.to_string());
    col_strings.push(Col::dec_input.to_string());
    col_strings.push(Col::hrv_sparv.to_string());
    col_strings.push(Col::hrv_error_sparv.to_string());
    col_strings.push(Col::snr_med_sparv.to_string());
    col_strings.push(Col::age_bdasp.to_string());
    col_strings.push(Col::age_error_bdasp.to_string());
    col_strings.push(Col::teff_bdasp.to_string());
    col_strings.push(Col::teff_error_bdasp.to_string());
    col_strings.push(Col::logg_bdasp.to_string());
    col_strings.push(Col::logg_error_bdasp.to_string());
    col_strings.push(Col::mass_bdasp.to_string());
    col_strings.push(Col::mass_error_bdasp.to_string());
    col_strings.push(Col::log_a_v_bdasp.to_string());
    col_strings.push(Col::log_a_v_error_bdasp.to_string());
    col_strings.push(Col::m_h_cal_madera.to_string());
    col_strings.push(Col::m_h_error_madera.to_string());
    col_strings.push(Col::fe_h_gauguin.to_string());
    col_strings.push(Col::fe_h_error_gauguin.to_string());
    col_strings.push(Col::fe_h_chisq_gauguin.to_string());
    col_strings.push(Col::fe_h_nl_gauguin.to_string());
    col_strings.push(Col::al_h_gauguin.to_string());
    col_strings.push(Col::al_h_error_gauguin.to_string());
    col_strings.push(Col::al_h_chisq_gauguin.to_string());
    col_strings.push(Col::al_h_nl_gauguin.to_string());
    col_strings.push(Col::ni_h_gauguin.to_string());
    col_strings.push(Col::ni_h_error_gauguin.to_string());
    col_strings.push(Col::ni_h_chisq_gauguin.to_string());
    col_strings.push(Col::ni_h_nl_gauguin.to_string());
    col_strings.push(Col::alpha_fe_gauguin.to_string());
    col_strings.push(Col::alpha_fe_error_gauguin.to_string());
    col_strings.push(Col::alpha_fe_chisq_gauguin.to_string());
    map.insert(ravedr6.string(), col_strings);
}
