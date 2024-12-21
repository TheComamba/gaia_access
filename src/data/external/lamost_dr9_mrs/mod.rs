// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the lamost_dr9_mrs table.

use crate::traits::{Column, Table};

/// LAMOST Medium-Resolution Spectroscopic Survey General Catalogue
///
/// The Large Sky Area Multi-Object Fiber Spectroscopic Telescope (LAMOST) is a Chinese national scientific research facility operated by the National Astronomical Observatories, Chinese Academy of Sciences. It is a special reflecting Schmidt telescope with 4000 fibers in a field of view of 20 degrees. The LAMOST survey provides flux - and wavelength-calibrated, sky-subtracted spectra in the wavelength range of 3700-9000 angstrom for many types of astronomical objects. Since October 2018, LAMOST started the second stage survey program containing both low- and medium-resolution spectroscopic surveys, and the medium-resolution spectroscopic survey includes two surveys, i.e., the time-domain and the non time-domain surveys.
///
/// The ninth LAMOST data release (LAMOST DR9) includes observations until June 2021. The Medium-Resolution spectroscopic survey (MRS) General Catalog contains 8,259,362 spectra, where there are 1,846,438 and 6,412,924 spectra for the non time-domain and time-domain surveys, respectively.
#[allow(non_camel_case_types)]
pub struct lamost_dr9_mrs;

impl Table for lamost_dr9_mrs {
    fn string(&self) -> String {
        "lamost_dr9_mrs".to_string()
    }
}

/// The columns in the lamost_dr9_mrs table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// The product_url column. (No further description available)
    product_url,
    /// Unique ID for each medium-resolution spectrum in each exposure
    mobsid,
    /// Unique spectrum ID for each observation of each target
    obsid,
    /// Unique source identifier calculated with the "ura" and "udec" in table 9
    uid,
    /// For each LAMOST source, the equatorial coordinates ("ura" and "udec" in table 9) used to calculate "uid" were from which survey ((Pan-STARRS, Gaia or LAMOST), "gp_id" gives the corresponding source identifier in that survey.
    gp_id,
    /// Target designation
    designation,
    /// Target Observation Date
    obsdate,
    /// Local Modified Julian Day
    lmjd,
    /// Modified Julian Day
    mjd,
    /// Plan Name
    planid,
    /// Spectrograph ID
    spid,
    /// Fiber ID
    fiberid,
    /// Local modified Julian minute
    lmjm,
    /// Having two values of B and R, which represent B and R band spectra
    band,
    /// Fiber pointing right ascension
    ra_obs,
    /// Fiber pointing declination
    dec_obs,
    /// The median value of all pixel S/Ns in B band spectrum or R band
    snr,
    /// Fiber Type of target [Obj, Sky, F-std, Unused, PosErr, Dead]
    fibertype,
    /// Whether there is a fiber offset during observation
    offsets,
    /// If the "offsets" field is true, "offsets_v" gives the offset distance from the target's coordinator in input catalog
    offsets_v,
    /// Right Ascension from input catalog
    ra,
    /// Declination from input catalog
    dec,
    /// Radial velocity of B band spectra measured with 483 selected KURUCZ synthetic templates
    rv_b0,
    /// Uncertainty of rv_b0 measured by the method LASP used
    rv_b0_err,
    /// Radial velocity after zero-point correction of rv_b0
    rv_b1,
    /// Uncertainty of rv_b1 measured by the error propagation
    rv_b1_err,
    /// Radial velocity flag for B band spectra
    rv_b_flag,
    /// Radial velocity of R band spectra measured with 483 selected KURUCZ synthetic templates
    rv_r0,
    /// Uncertainty of rv_r0 measured by the method LASP used
    rv_r0_err,
    /// Radial velocity after zero-point correction of rv_r0
    rv_r1,
    /// Uncertainty of rv_r1 measured by the error propagation
    rv_r1_err,
    /// Radial velocity flag for R band spectra
    rv_r_flag,
    /// Radial velocity of B and R band spectra measured with 483 selected KURUCZ synthetic templates
    rv_br0,
    /// Uncertainty of rv_br0 measured by the method LASP used
    rv_br0_err,
    /// Radial velocity after zero-point correction of rv_br0
    rv_br1,
    /// Uncertainty of rv_br1 measured by the error propagation
    rv_br1_err,
    /// Radial velocity flag for B and R band spectra
    rv_br_flag,
    /// Radial velocity measured by the LASP
    rv_lasp0,
    /// Uncertainty of rv_lasp0 also given by the LASP
    rv_lasp0_err,
    /// Radial velocity after zero-point correction of rv_lasp0
    rv_lasp1,
    /// Uncertainty of rv_lasp1 measured by the error propagation
    rv_lasp1_err,
    /// A flag to show whether it is a coadd spectrum
    coadd,
    /// Possible fiber problems
    fibermask,
    /// A flag to show whether the B band spectra is problematic
    bad_b,
    /// A flag to show whether the R band spectra is problematic
    bad_r,
    /// The angular distance of each target to the moon
    moon_angle,
    /// The date of Chinese lunar calendar, which represents the moon phase, and its value is from 1 to 30.
    lunardate,
    /// This is a flag for "moon_angle", and it has two values of 0 and 1, which represent the range of angular distance. If "moon_flg" equals to 1, it indicates that angular distance is in the range of [10 degree, 30 degree], and it represents that the angular distance is larger than 30 degree when "moon_flg" equals to 0.
    moon_flg,
    /// The sub-project names of LAMOST Medium-Resolution Spectroscopic Survey
    subproject,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the lamost_dr9_mrs table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::product_url.to_string());
    col_strings.push(Col::mobsid.to_string());
    col_strings.push(Col::obsid.to_string());
    col_strings.push(Col::uid.to_string());
    col_strings.push(Col::gp_id.to_string());
    col_strings.push(Col::designation.to_string());
    col_strings.push(Col::obsdate.to_string());
    col_strings.push(Col::lmjd.to_string());
    col_strings.push(Col::mjd.to_string());
    col_strings.push(Col::planid.to_string());
    col_strings.push(Col::spid.to_string());
    col_strings.push(Col::fiberid.to_string());
    col_strings.push(Col::lmjm.to_string());
    col_strings.push(Col::band.to_string());
    col_strings.push(Col::ra_obs.to_string());
    col_strings.push(Col::dec_obs.to_string());
    col_strings.push(Col::snr.to_string());
    col_strings.push(Col::fibertype.to_string());
    col_strings.push(Col::offsets.to_string());
    col_strings.push(Col::offsets_v.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::rv_b0.to_string());
    col_strings.push(Col::rv_b0_err.to_string());
    col_strings.push(Col::rv_b1.to_string());
    col_strings.push(Col::rv_b1_err.to_string());
    col_strings.push(Col::rv_b_flag.to_string());
    col_strings.push(Col::rv_r0.to_string());
    col_strings.push(Col::rv_r0_err.to_string());
    col_strings.push(Col::rv_r1.to_string());
    col_strings.push(Col::rv_r1_err.to_string());
    col_strings.push(Col::rv_r_flag.to_string());
    col_strings.push(Col::rv_br0.to_string());
    col_strings.push(Col::rv_br0_err.to_string());
    col_strings.push(Col::rv_br1.to_string());
    col_strings.push(Col::rv_br1_err.to_string());
    col_strings.push(Col::rv_br_flag.to_string());
    col_strings.push(Col::rv_lasp0.to_string());
    col_strings.push(Col::rv_lasp0_err.to_string());
    col_strings.push(Col::rv_lasp1.to_string());
    col_strings.push(Col::rv_lasp1_err.to_string());
    col_strings.push(Col::coadd.to_string());
    col_strings.push(Col::fibermask.to_string());
    col_strings.push(Col::bad_b.to_string());
    col_strings.push(Col::bad_r.to_string());
    col_strings.push(Col::moon_angle.to_string());
    col_strings.push(Col::lunardate.to_string());
    col_strings.push(Col::moon_flg.to_string());
    col_strings.push(Col::subproject.to_string());
    map.insert(lamost_dr9_mrs.string(), col_strings);
}
