// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the gaia_universe_model table.

use crate::traits::{Column, Table};

/// Table of simulated galactic stars according to the Gaia Universal Model
/// Simulation (GUMS). True values of the intrinsic simulated quantities
/// (astrometry, photometry and physical parameters) for the sources
/// generated by GOG using the Universe Model are given. No errors are
/// added.
#[allow(non_camel_case_types)]
pub struct gaia_universe_model;

impl Table for gaia_universe_model {
    fn string(&self) -> String {
        "gaia_universe_model".to_string()
    }
}

/// The columns in the gaia_universe_model table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Extended source identifier
    source_extended_id,
    /// Long Identifier
    source_id,
    /// Solution Identifier
    solution_id,
    /// Right Ascension
    ra,
    /// Declination
    dec,
    /// Barycentric distance to the simulated source
    barycentric_distance,
    /// Proper motion along right ascension
    pmra,
    /// Proper motion along declination
    pmdec,
    /// Radial Velocity
    radial_velocity,
    /// Mean Apparent G magnitude
    mag_g,
    /// Mean Apparent BP magnitude
    mag_bp,
    /// Mean Apparent RP magnitude
    mag_rp,
    /// Mean Apparent RVS magnitude
    mag_rvs,
    /// (V-I) color
    v_i,
    /// Mean Absolute V magnitude
    mean_absolute_v,
    /// Absorption in G
    ag,
    /// Absorption in V
    av,
    /// Effective temperature
    teff,
    /// spectral class + luminosity class
    spectral_type,
    /// Surface gravity
    logg,
    /// Metallicity
    feh,
    /// Alpha elements
    alphafe,
    /// Absolute bolometric magnitude
    mbol,
    /// Age
    age,
    /// Mass
    mass,
    /// Radius
    radius,
    /// Rotational velocity
    vsini,
    /// Population
    population,
    /// Boolean describing if the photocenter has or not motion
    has_photocenter_motion,
    /// Number of components
    nc,
    /// Total number of object
    nt,
    /// Semi major axis
    semimajor_axis,
    /// Eccentricity
    eccentricity,
    /// Inclination
    inclination,
    /// Longitude of ascending node
    longitude_ascending_node,
    /// Period of the orbit
    orbit_period,
    /// Date of periastron
    periastron_date,
    /// Periastron argument
    periastron_argument,
    /// Variability type
    variability_type,
    /// Amplitude of variability
    variability_amplitude,
    /// Period of variability
    variability_period,
    /// Phase of variability
    variability_phase,
    /// Envelope characterisic for Be stars
    r_env_r_star,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the gaia_universe_model table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::source_extended_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::barycentric_distance.to_string());
    col_strings.push(Col::pmra.to_string());
    col_strings.push(Col::pmdec.to_string());
    col_strings.push(Col::radial_velocity.to_string());
    col_strings.push(Col::mag_g.to_string());
    col_strings.push(Col::mag_bp.to_string());
    col_strings.push(Col::mag_rp.to_string());
    col_strings.push(Col::mag_rvs.to_string());
    col_strings.push(Col::v_i.to_string());
    col_strings.push(Col::mean_absolute_v.to_string());
    col_strings.push(Col::ag.to_string());
    col_strings.push(Col::av.to_string());
    col_strings.push(Col::teff.to_string());
    col_strings.push(Col::spectral_type.to_string());
    col_strings.push(Col::logg.to_string());
    col_strings.push(Col::feh.to_string());
    col_strings.push(Col::alphafe.to_string());
    col_strings.push(Col::mbol.to_string());
    col_strings.push(Col::age.to_string());
    col_strings.push(Col::mass.to_string());
    col_strings.push(Col::radius.to_string());
    col_strings.push(Col::vsini.to_string());
    col_strings.push(Col::population.to_string());
    col_strings.push(Col::has_photocenter_motion.to_string());
    col_strings.push(Col::nc.to_string());
    col_strings.push(Col::nt.to_string());
    col_strings.push(Col::semimajor_axis.to_string());
    col_strings.push(Col::eccentricity.to_string());
    col_strings.push(Col::inclination.to_string());
    col_strings.push(Col::longitude_ascending_node.to_string());
    col_strings.push(Col::orbit_period.to_string());
    col_strings.push(Col::periastron_date.to_string());
    col_strings.push(Col::periastron_argument.to_string());
    col_strings.push(Col::variability_type.to_string());
    col_strings.push(Col::variability_amplitude.to_string());
    col_strings.push(Col::variability_period.to_string());
    col_strings.push(Col::variability_phase.to_string());
    col_strings.push(Col::r_env_r_star.to_string());
    map.insert(gaia_universe_model.string(), col_strings);
}
