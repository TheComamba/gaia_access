// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the nss_two_body_orbit table.

use crate::traits::{Column, Table};

/// This table contains non-single-star orbital models for sources compatible with an orbital two-body solution. This covers astrometric binaries, spectroscopic binaries, eclipsing binaries and certain combinations thereof. Several possible models are hosted within the same table and they are indicated by the field \texttt{nssSolutionType}. The description of this latter lists all possible solution types considered for this release. Only a selection of parameters hosted in this table are provided here, depending on the solution. The details of those is given in the description of field \texttt{bitIndex}, which can also be used to extract the relevant elements of the correlation vector \texttt{corrVec}. Details about the formalism used to derive the parameters in this table are given in the on-line documentation, see Chapter~\ref{chap:cu4nss}. Warning: as a source may receive independent astrometric, spectroscopic or photometric orbits, a query using a given sourceId may return several solutions. This has to be accounted for when doing a crossmatch by sourceId.
#[allow(non_camel_case_types)]
pub struct nss_two_body_orbit;

impl Table for nss_two_body_orbit {
    fn string(&self) -> String {
        "nss_two_body_orbit".to_string()
    }
}

/// The columns in the nss_two_body_orbit table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Solution Identifier
    solution_id,
    /// Source Identifier
    source_id,
    /// NSS model adopted
    nss_solution_type,
    /// Right ascension
    ra,
    /// Standard error of right ascension
    ra_error,
    /// Declination
    dec,
    /// Standard error of declination
    dec_error,
    /// Parallax
    parallax,
    /// Standard error of parallax
    parallax_error,
    /// Proper motion in right ascension direction
    pmra,
    /// Standard error of proper motion in right ascension direction
    pmra_error,
    /// Proper motion in declination direction
    pmdec,
    /// Standard error of proper motion in declination direction
    pmdec_error,
    /// Thiele-Innes element A
    a_thiele_innes,
    /// Standard error of Thiele-Innes element A
    a_thiele_innes_error,
    /// Thiele-Innes element B
    b_thiele_innes,
    /// Standard error of Thiele-Innes element B
    b_thiele_innes_error,
    /// Thiele-Innes element F
    f_thiele_innes,
    /// Standard error of Thiele-Innes element F
    f_thiele_innes_error,
    /// Thiele-Innes element G
    g_thiele_innes,
    /// Standard error of Thiele-Innes element G
    g_thiele_innes_error,
    /// C element of Thiele-Innes
    c_thiele_innes,
    /// Standard error of C element of Thiele-Innes
    c_thiele_innes_error,
    /// H element of Thiele-Innes
    h_thiele_innes,
    /// Standard error of H element of Thiele-Innes
    h_thiele_innes_error,
    /// Orbital Period
    period,
    /// Standard error of Orbital Period
    period_error,
    /// Periastron epoch
    t_periastron,
    /// Standard error of Periastron epoch
    t_periastron_error,
    /// eccentricity
    eccentricity,
    /// Standard error of eccentricity
    eccentricity_error,
    /// The velocity of the center of mass
    center_of_mass_velocity,
    /// Standard error of The velocity of the center of mass
    center_of_mass_velocity_error,
    /// Semi-amplitude of the center of mass
    semi_amplitude_primary,
    /// Standard error of Semi-amplitude of the center of mass
    semi_amplitude_primary_error,
    /// The semiamplitude of the radial velocity curve for second component
    semi_amplitude_secondary,
    /// Standard error of The semiamplitude of the radial velocity curve for second component
    semi_amplitude_secondary_error,
    /// Mass ratio
    mass_ratio,
    /// Standard error of Mass ratio
    mass_ratio_error,
    /// Fill factor of primary
    fill_factor_primary,
    /// Standard error of Fill factor of primary
    fill_factor_primary_error,
    /// Fill factor of secondary
    fill_factor_secondary,
    /// Standard error of Fill factor of secondary
    fill_factor_secondary_error,
    /// Orbital inclination
    inclination,
    /// Standard error of Orbital inclination
    inclination_error,
    /// Argument of periastron
    arg_periastron,
    /// Standard error of Argument of periastron
    arg_periastron_error,
    /// Ratio of the effective temperatures
    temperature_ratio,
    /// Standard error of the ratio of the effective temperatures
    temperature_ratio_error,
    /// Code defining which fitting scenario did apply to the effective temperature
    temperature_ratio_definition,
    /// Total astrometric CCD observations in AL considered
    astrometric_n_obs_al,
    /// Total astrometric CCD observations in AL actually used
    astrometric_n_good_obs_al,
    /// Total number of radial velocities considered for the primary
    rv_n_obs_primary,
    /// Total number of radial velocities actually used for the primary
    rv_n_good_obs_primary,
    /// Total number of radial velocities considered for the secondary in the case of SB2
    rv_n_obs_secondary,
    /// Total number of radial velocities actually used for the secondary in the case of SB2
    rv_n_good_obs_secondary,
    /// Total number of G photometry measurements considered
    phot_g_n_obs,
    /// Total number of G photometry measurements actually used
    phot_g_n_good_obs,
    /// boolean mask for the fields above in the corrVec matrix
    bit_index,
    /// Vector form of the upper triangle of the correlation matrix (column-major ordered)
    corr_vec,
    /// value of the objective function at the solution
    obj_func,
    /// goodness of fit in the Hipparcos sense
    goodness_of_fit,
    /// Efficiency of the solution
    efficiency,
    /// The significance of the solution (i.e. how worth keeping a model is)
    significance,
    /// Quality flag for the achieved NSS solution
    flags,
    /// The probability of the period for not being due to (gaussian white) noise. Relevant for SB1, SB1C, SB2 and SB2C models. To be ignored otherwise.
    conf_spectro_period,
    /// Sum of the polar radii of primary and secondary (in units of the semi-major axis)
    r_pole_sum,
    /// L1-pointing radii of primary and secondary (in units of the semi-major axis)
    r_l1_point_sum,
    /// Sum of the radii of sphere having the same volume as the primary and secondary (in units of the semi-major axis
    r_spher_sum,
    /// Time of mid-eclipse of the primary by the secondary
    ecl_time_primary,
    /// Time of mid-eclipse of the secondary by the primary
    ecl_time_secondary,
    /// Duration of primary eclipse assuming spherical components
    ecl_dur_primary,
    /// Duration of secondary eclipse assuming spherical components
    ecl_dur_secondary,
    /// Ratio of the G-band luminosity of the secondary over the primary
    g_luminosity_ratio,
    /// Standard error of the period taken from VariEclipsingBinary.frequencyError
    input_period_error,
    /// Rank of the G-band solution
    g_rank,
    /// Uncorrelated astrometric jitter term
    astrometric_jitter,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the nss_two_body_orbit table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::nss_solution_type.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::ra_error.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::dec_error.to_string());
    col_strings.push(Col::parallax.to_string());
    col_strings.push(Col::parallax_error.to_string());
    col_strings.push(Col::pmra.to_string());
    col_strings.push(Col::pmra_error.to_string());
    col_strings.push(Col::pmdec.to_string());
    col_strings.push(Col::pmdec_error.to_string());
    col_strings.push(Col::a_thiele_innes.to_string());
    col_strings.push(Col::a_thiele_innes_error.to_string());
    col_strings.push(Col::b_thiele_innes.to_string());
    col_strings.push(Col::b_thiele_innes_error.to_string());
    col_strings.push(Col::f_thiele_innes.to_string());
    col_strings.push(Col::f_thiele_innes_error.to_string());
    col_strings.push(Col::g_thiele_innes.to_string());
    col_strings.push(Col::g_thiele_innes_error.to_string());
    col_strings.push(Col::c_thiele_innes.to_string());
    col_strings.push(Col::c_thiele_innes_error.to_string());
    col_strings.push(Col::h_thiele_innes.to_string());
    col_strings.push(Col::h_thiele_innes_error.to_string());
    col_strings.push(Col::period.to_string());
    col_strings.push(Col::period_error.to_string());
    col_strings.push(Col::t_periastron.to_string());
    col_strings.push(Col::t_periastron_error.to_string());
    col_strings.push(Col::eccentricity.to_string());
    col_strings.push(Col::eccentricity_error.to_string());
    col_strings.push(Col::center_of_mass_velocity.to_string());
    col_strings.push(Col::center_of_mass_velocity_error.to_string());
    col_strings.push(Col::semi_amplitude_primary.to_string());
    col_strings.push(Col::semi_amplitude_primary_error.to_string());
    col_strings.push(Col::semi_amplitude_secondary.to_string());
    col_strings.push(Col::semi_amplitude_secondary_error.to_string());
    col_strings.push(Col::mass_ratio.to_string());
    col_strings.push(Col::mass_ratio_error.to_string());
    col_strings.push(Col::fill_factor_primary.to_string());
    col_strings.push(Col::fill_factor_primary_error.to_string());
    col_strings.push(Col::fill_factor_secondary.to_string());
    col_strings.push(Col::fill_factor_secondary_error.to_string());
    col_strings.push(Col::inclination.to_string());
    col_strings.push(Col::inclination_error.to_string());
    col_strings.push(Col::arg_periastron.to_string());
    col_strings.push(Col::arg_periastron_error.to_string());
    col_strings.push(Col::temperature_ratio.to_string());
    col_strings.push(Col::temperature_ratio_error.to_string());
    col_strings.push(Col::temperature_ratio_definition.to_string());
    col_strings.push(Col::astrometric_n_obs_al.to_string());
    col_strings.push(Col::astrometric_n_good_obs_al.to_string());
    col_strings.push(Col::rv_n_obs_primary.to_string());
    col_strings.push(Col::rv_n_good_obs_primary.to_string());
    col_strings.push(Col::rv_n_obs_secondary.to_string());
    col_strings.push(Col::rv_n_good_obs_secondary.to_string());
    col_strings.push(Col::phot_g_n_obs.to_string());
    col_strings.push(Col::phot_g_n_good_obs.to_string());
    col_strings.push(Col::bit_index.to_string());
    col_strings.push(Col::corr_vec.to_string());
    col_strings.push(Col::obj_func.to_string());
    col_strings.push(Col::goodness_of_fit.to_string());
    col_strings.push(Col::efficiency.to_string());
    col_strings.push(Col::significance.to_string());
    col_strings.push(Col::flags.to_string());
    col_strings.push(Col::conf_spectro_period.to_string());
    col_strings.push(Col::r_pole_sum.to_string());
    col_strings.push(Col::r_l1_point_sum.to_string());
    col_strings.push(Col::r_spher_sum.to_string());
    col_strings.push(Col::ecl_time_primary.to_string());
    col_strings.push(Col::ecl_time_secondary.to_string());
    col_strings.push(Col::ecl_dur_primary.to_string());
    col_strings.push(Col::ecl_dur_secondary.to_string());
    col_strings.push(Col::g_luminosity_ratio.to_string());
    col_strings.push(Col::input_period_error.to_string());
    col_strings.push(Col::g_rank.to_string());
    col_strings.push(Col::astrometric_jitter.to_string());
    map.insert(nss_two_body_orbit.string(), col_strings);
}
