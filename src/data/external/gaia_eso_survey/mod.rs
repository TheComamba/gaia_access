// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the gaia_eso_survey table.

use crate::traits::{Column, Table};

/// This table contains a selection of fields from data release 5.1 of the Gaia-ESO Public Spectroscopic Survey (GES). Gaia-ESO is a large public spectroscopic survey carried out with FLAMES, targeting 114,916 stars, systematically covering all major components of the Milky Way, from halo to star-forming regions, providing a homogeneous overview of the distributions of kinematics and elemental abundances. The catalogue contains radial and projected rotational velocities, stellar parameters (effective temperature, surface gravity, and metallicity), abundances of several elements, specific parameters for tracing accretion and activity in young stars, and - for the targets of the cluster fields - the probability to be members of the cluster calculated by combining GES radial velocities with astrometry from Gaia EDR3 by Jackson+ (2022, MNRAS, 509, 1164). The full GES data release can be accessed from the ESO Science Portal at https://eso.org/rm/publicAccess#/dataReleases?wcmmode=disabled. Please, see the two main GES data release papers for details: https://ui.adsabs.harvard.edu/abs/2022A%26A...666A.120G/abstract (DOI: 10.1051/0004-6361/202243134) and https://ui.adsabs.harvard.edu/abs/2022A%26A...666A.121R/abstract (DOI: 10.1051/0004-6361/202243141). This table has been created by Germano Sacco (INAF-Osservatorio Astrofisico di Arcetri) in July 2023 for the Gaia ESA Archive.
#[allow(non_camel_case_types)]
pub struct gaia_eso_survey;

impl Table for gaia_eso_survey {
    fn string(&self) -> String {
        "gaia_eso_survey".to_string()
    }
}

/// The columns in the gaia_eso_survey table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// GES object name from coordinates
    object,
    /// GES field name
    ges_fld,
    /// GES Classification System of Target Programmes
    ges_type,
    /// Right Ascension
    ra,
    /// Declination
    dec,
    /// Coordinate epoch
    epoch,
    /// Effective Temperature
    teff,
    /// Error on TEFF
    e_teff,
    /// Log Surface Gravity (gravity in cm/s**2)
    logg,
    /// Error on LOGG
    e_logg,
    /// Metallicity from Fe lines
    feh,
    /// Error on FEH
    e_feh,
    /// Radial Velocity
    vrad,
    /// Error on VRAD
    e_vrad,
    /// Rotational broadening
    vsini,
    /// Error on VSINI
    e_vsini,
    /// Flag on VSINI (0 = detection, 1 = lower limit)
    lim_vsini,
    /// Neutral Lithium Abundance
    li1,
    /// Error on LI1
    e_li1,
    /// Flag on LI Measurement type (0 = detection, 1 = lower limit)
    lim_li1,
    /// LI(6708A) equivalent width
    ew_li,
    /// Error on EW_LI
    e_ew_li,
    /// Flag on EW_LI (0 = detection, 1 = lower limit)
    lim_ew_li,
    /// Oxgen Abundance
    o1,
    /// Error O1
    e_o1,
    /// Magnesium Abundance
    mg1,
    /// Error on MG1
    e_mg1,
    /// Aluminium Abundance
    al1,
    /// Error on ALI
    e_al1,
    /// Silicon Abundance
    si1,
    /// Error on SI1
    e_si1,
    /// Calcium Abundance
    ca1,
    /// Error on CA1
    e_ca1,
    /// Titanium Abundance
    ti1,
    /// Error on TI1
    e_ti1,
    /// Nitrogen Abundance
    ni1,
    /// Error on NI1
    e_ni1,
    /// Ionized Barium Abundance
    ba2,
    /// Error on BA2
    e_ba2,
    /// Ionized Europium Abundance
    eu2,
    /// Error on EU2
    e_eu2,
    /// Simplified Flags
    sflags,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the gaia_eso_survey table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::object.to_string());
    col_strings.push(Col::ges_fld.to_string());
    col_strings.push(Col::ges_type.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::epoch.to_string());
    col_strings.push(Col::teff.to_string());
    col_strings.push(Col::e_teff.to_string());
    col_strings.push(Col::logg.to_string());
    col_strings.push(Col::e_logg.to_string());
    col_strings.push(Col::feh.to_string());
    col_strings.push(Col::e_feh.to_string());
    col_strings.push(Col::vrad.to_string());
    col_strings.push(Col::e_vrad.to_string());
    col_strings.push(Col::vsini.to_string());
    col_strings.push(Col::e_vsini.to_string());
    col_strings.push(Col::lim_vsini.to_string());
    col_strings.push(Col::li1.to_string());
    col_strings.push(Col::e_li1.to_string());
    col_strings.push(Col::lim_li1.to_string());
    col_strings.push(Col::ew_li.to_string());
    col_strings.push(Col::e_ew_li.to_string());
    col_strings.push(Col::lim_ew_li.to_string());
    col_strings.push(Col::o1.to_string());
    col_strings.push(Col::e_o1.to_string());
    col_strings.push(Col::mg1.to_string());
    col_strings.push(Col::e_mg1.to_string());
    col_strings.push(Col::al1.to_string());
    col_strings.push(Col::e_al1.to_string());
    col_strings.push(Col::si1.to_string());
    col_strings.push(Col::e_si1.to_string());
    col_strings.push(Col::ca1.to_string());
    col_strings.push(Col::e_ca1.to_string());
    col_strings.push(Col::ti1.to_string());
    col_strings.push(Col::e_ti1.to_string());
    col_strings.push(Col::ni1.to_string());
    col_strings.push(Col::e_ni1.to_string());
    col_strings.push(Col::ba2.to_string());
    col_strings.push(Col::e_ba2.to_string());
    col_strings.push(Col::eu2.to_string());
    col_strings.push(Col::e_eu2.to_string());
    col_strings.push(Col::sflags.to_string());
    map.insert(gaia_eso_survey.string(), col_strings);
}
