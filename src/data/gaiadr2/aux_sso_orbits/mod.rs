// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the aux_sso_orbits table.

use crate::traits::{Column, Table};

/// Auxiliary information on asteroid orbits and basic photometric
/// parameters. Ready to ingest the astorb data base:
/// ftp://cdsarc.u-strasbg.fr/pub/cats/B/astorb/astorb.html
#[allow(non_camel_case_types)]
pub struct aux_sso_orbits;

impl Table for aux_sso_orbits {
    fn string(&self) -> String {
        "aux_sso_orbits".to_string()
    }
}

/// The columns in the aux_sso_orbits table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Solution Identifier
    solution_id,
    /// Asteroid MPC number
    number_mp,
    /// MPC name
    designation,
    /// absolute magnitude
    mag_h,
    /// slope parameter
    slope_g,
    /// object specific flags
    code,
    /// arc spanned by the observations
    obs_arc,
    /// number of observations used
    obs_num,
    /// epoch of osculation
    osc_epoch,
    /// mean anomaly
    orb_m,
    /// argument of perihelion
    omega,
    /// longitude of ascending node
    node_omega,
    /// orbit inclination
    inclination,
    /// orbit eccentricity
    eccentricity,
    /// semimajor axis
    a,
    /// date of orbit computation
    orb_date,
    /// current orbit uncertainty
    ceu,
    /// rate of change of the orbit uncertainty
    ceu_rate,
    /// date of the CEU
    ceu_epoch,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the aux_sso_orbits table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::number_mp.to_string());
    col_strings.push(Col::designation.to_string());
    col_strings.push(Col::mag_h.to_string());
    col_strings.push(Col::slope_g.to_string());
    col_strings.push(Col::code.to_string());
    col_strings.push(Col::obs_arc.to_string());
    col_strings.push(Col::obs_num.to_string());
    col_strings.push(Col::osc_epoch.to_string());
    col_strings.push(Col::orb_m.to_string());
    col_strings.push(Col::omega.to_string());
    col_strings.push(Col::node_omega.to_string());
    col_strings.push(Col::inclination.to_string());
    col_strings.push(Col::eccentricity.to_string());
    col_strings.push(Col::a.to_string());
    col_strings.push(Col::orb_date.to_string());
    col_strings.push(Col::ceu.to_string());
    col_strings.push(Col::ceu_rate.to_string());
    col_strings.push(Col::ceu_epoch.to_string());
    map.insert(aux_sso_orbits.string(), col_strings);
}
