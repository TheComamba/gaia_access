// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the urat1_original_valid table.

use crate::traits::{Column, Table};

/// URAT-1 Catalogue  
/// Reference paper:   
/// A URAT-1 release paper for the Astronomical Journal is in preparation.  
/// The first U.S. Naval Observatory Astrometric Robotic Telescope Catalog
/// (URAT1) Zacharias N., Finch C., Subasavage J., Bredthauer G., Crockett
/// C., Divittorio M., Furguson E., Harris F., Harris H., Henden A., Kilian
/// C., Munn J., Rafferty T., Rhodes A., Schultheiss M., Tilleman T., Wieder
/// G. =2015yCat.1329....0Z  
/// Original catalogue:  
/// CDS
/// Catalogue curator:
/// SSDC - ASI Space Science Data Center
/// https://www.ssdc.asi.it/
#[allow(non_camel_case_types)]
pub struct urat1_original_valid;

impl Table for urat1_original_valid {
    fn string(&self) -> String {
        "urat1_original_valid".to_string()
    }
}

/// The columns in the urat1_original_valid table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Incremental unique numeric identifier (increasing with declination).  
    /// This is the only field which was not in the original URAT1 catalogue,
    /// but was added for cross-match purposes.
    urat1_oid,
    /// Official URAT1 star ID numbers consist of 2 parts, the 3-digit zone
    /// number (zzz) and the 6-digit running record number (nnnnnn) along a
    /// zone.  
    /// Thus a URAT1 star number is given by:  
    /// URAT1-zzznnnnnn  
    /// The main catalog data are arranged in declination zones of 0.2 degree
    /// width. Zones are numbered from 1 starting at the South Pole and
    /// increasing toward north. The first zone with data in URAT1 is 326 for
    /// -25.0 to -24.8 deg Dec. There is a separate file for each zone up to
    /// zone 900 near the north celestial pole.
    urat1_identifier,
    /// Positions are on the International Celestial Reference System (ICRS) as
    /// represented by the UCAC4 catalog.  
    /// Mean observed positions are given at mean epoch of URAT observations
    /// (epoch). Thus the epoch is slightly different from star to star, but it
    /// is always in the range between 2012.311 and 2014.679.
    ra,
    /// Positions are on the International Celestial Reference System (ICRS) as
    /// represented by the UCAC4 catalog.  
    /// Mean observed positions are given at mean epoch of URAT observations
    /// (epoch). Thus the epoch is slightly different from star to star, but it
    /// is always in the range between 2012.311 and 2014.679.
    dec,
    /// raError = posError  
    /// posError gives an estimate of the error of the mean position components
    /// (ra and dec).  
    /// A mean was taken over RA and DEC component errors because they are very
    /// similar for most stars.  
    /// Here a model is used which include image profile fit (x,y data) errors,
    /// atmospheric turbulence, and astrometric reduction error propagations. A
    /// systematic error floor of 5 mas was added RMS. The model error is likely
    /// a better estimate of the true positional errors than the scatter error,
    /// at least for small numbers of observations.
    ra_error,
    /// decError = posError  
    /// posError gives an estimate of the error of the mean position components
    /// (ra and dec).  
    /// A mean was taken over RA and DEC component errors because they are very
    /// similar for most stars.  
    /// Here a model is used which include image profile fit (x,y data) errors,
    /// atmospheric turbulence, and astrometric reduction error propagations. A
    /// systematic error floor of 5 mas was added RMS. The model error is likely
    /// a better estimate of the true positional errors than the scatter error,
    /// at least for small numbers of observations.
    dec_error,
    /// epoch = mean epoch of URAT observations.
    epoch,
    /// This is the mean, observed magnitude in the 680-762 nm URAT bandpass,
    /// calibrated by APASS photometry. This bandpass is between R and I, thus
    /// further into the red than UCAC. Observations in non-photometric nights
    /// *are* included thus the URAT magnitudes need to be taken with caution.
    /// Unknown or unrealistic magnitudes are set to NULL. The faintest maybe
    /// real celestial object magnitude is about 19.0, while the URAT1 catalog
    /// should be complete to about magnitude 18.0.
    f_mag,
    /// The photometric error of URAT bandpass observations is derived from the
    /// scatter of individual observations. A systematic error floor of 0.01 mag
    /// has been RMS added. Unknown errors are indicated by NULL.
    f_mag_error,
    /// APASS B magnitude.  
    /// A custom set of APASS (The AAVSO Photometric All-Sky Survey) data was
    /// kindly provided to us by Arne Henden to include the DR8 data plus single
    /// photometric observations.  
    /// For a total of 71614 stars with no DR8 data the DR6 data was used.
    b_mag,
    /// APASS V magnitude.  
    /// A custom set of APASS (The AAVSO Photometric All-Sky Survey) data was
    /// kindly provided to us by Arne Henden to include the DR8 data plus single
    /// photometric observations.  
    /// For a total of 71614 stars with no DR8 data the DR6 data was used.
    v_mag,
    /// APASS g magnitude.  
    /// A custom set of APASS (The AAVSO Photometric All-Sky Survey) data was
    /// kindly provided to us by Arne Henden to include the DR8 data plus single
    /// photometric observations.  
    /// For a total of 71614 stars with no DR8 data the DR6 data was used.
    g_mag,
    /// APASS r magnitude.  
    /// A custom set of APASS (The AAVSO Photometric All-Sky Survey) data was
    /// kindly provided to us by Arne Henden to include the DR8 data plus single
    /// photometric observations.  
    /// For a total of 71614 stars with no DR8 data the DR6 data was used.
    r_mag,
    /// APASS i magnitude.  
    /// A custom set of APASS (The AAVSO Photometric All-Sky Survey) data was
    /// kindly provided to us by Arne Henden to include the DR8 data plus single
    /// photometric observations.  
    /// For a total of 71614 stars with no DR8 data the DR6 data was used.
    i_mag,
    /// Error on APASS B magnitude.
    b_mag_error,
    /// Error on APASS V magnitude.
    v_mag_error,
    /// Error on APASS g magnitude.
    g_mag_error,
    /// Error on APASS r magnitude.
    r_mag_error,
    /// Error on APASS i magnitude.
    i_mag_error,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the urat1_original_valid table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::urat1_oid.to_string());
    col_strings.push(Col::urat1_identifier.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::ra_error.to_string());
    col_strings.push(Col::dec_error.to_string());
    col_strings.push(Col::epoch.to_string());
    col_strings.push(Col::f_mag.to_string());
    col_strings.push(Col::f_mag_error.to_string());
    col_strings.push(Col::b_mag.to_string());
    col_strings.push(Col::v_mag.to_string());
    col_strings.push(Col::g_mag.to_string());
    col_strings.push(Col::r_mag.to_string());
    col_strings.push(Col::i_mag.to_string());
    col_strings.push(Col::b_mag_error.to_string());
    col_strings.push(Col::v_mag_error.to_string());
    col_strings.push(Col::g_mag_error.to_string());
    col_strings.push(Col::r_mag_error.to_string());
    col_strings.push(Col::i_mag_error.to_string());
    map.insert(urat1_original_valid.string(), col_strings);
}
