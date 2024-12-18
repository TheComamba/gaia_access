// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the gsc23_original_valid table.

use crate::traits::{Column, Table};

/// GSC 2.3 Catalogue  
/// Reference paper: Lasker et al. 2008, AJ 136,735  
/// Original catalogue: R. Smart private communication
/// Catalogue curator:
/// SSDC - ASI Space Science Data Center
/// https://www.ssdc.asi.it/
#[allow(non_camel_case_types)]
pub struct gsc23_original_valid;

impl Table for gsc23_original_valid {
    fn string(&self) -> String {
        "gsc23_original_valid".to_string()
    }
}

/// The columns in the gsc23_original_valid table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Incremental unique numeric identifier (increasing with declination).
    /// This is the only field which was not in the original GSC2.3 catalogue,
    /// but was added for cross-match purposes.
    gsc23_oid,
    /// The GSC2.3 identification is made of 10 characters, the first four
    /// representing the level-6 HTM (Hierarchical Triangular Mesh) coded in
    /// base 36 (0..9 and A..Z), and the last 6 represent a zero-filled sequence
    /// number assigned to each source upon initial detection.
    gsc23_identifier,
    /// J2000 right ascension with respect to the ICRS.
    ra,
    /// J2000 declination with respect to the ICRS.
    dec,
    /// Plate epoch for GSC-II objects. For Tycho-2 objects, for which RA epoch
    /// and DEC epoch are different, the RA epoch is given.
    position_epoch,
    /// Reference error in RA*cos(DEC) at position epoch (positionEpoch). These
    /// astrometric errors are not formal statistical uncertainties but raw and
    /// conservative estimates to be used for telescope operations.
    ra_error,
    /// Reference error in DEC at position epoch (positionEpoch). These
    /// astrometric errors are not formal statistical uncertainties but raw and
    /// conservative estimates to be used for telescope operations.
    dec_error,
    /// Magnitude in Rf photographic band (red).
    fpg_mag,
    /// Reference error in Rf photographic band (red).  
    /// These photometric errors are not formal statistical uncertainties but
    /// raw and conservative estimates to be used for telescope operations.
    fpg_mag_error,
    /// coded emulsion / bandpass / filter:  
    /// ———————————————————————-  
    /// bcode Name Emulsion/Filter  
    /// ———————————————————————-  
    /// 35 Fpg IIIaF+RG610 (POSS-II Red)  
    /// 36 Fpg IIIaF+OG590 (SERC-ER/SR, AAO-R/GR)  
    /// ———————————————————————-
    fpg_mag_code,
    /// Magnitude in Bj photographic band (blue).
    jpg_mag,
    /// Reference error in Bj photographic band (blue).  
    /// These photometric errors are not formal statistical uncertainties but
    /// raw and conservative estimates to be used for telescope operations.
    jpg_mag_error,
    /// coded emulsion / bandpass / filter:  
    /// ———————————————————————-  
    /// bcode Name Emulsion/Filter  
    /// ———————————————————————-  
    /// 0 Jpg IIIaJ+GG395 (SERC-J/EJ)  
    /// 18 Jpg IIIaJ+GG385 (POSS-II Blue)  
    /// ———————————————————————-
    jpg_mag_code,
    /// Magnitude in V band.  
    /// This magnitude may include:
    ///
    /// -   photographic V_12 or V_485 from IIaD plates,
    ///
    /// -   V_T of Tycho-2 stars, or
    ///
    /// -   Johnson V from SKY2000
    v_mag,
    /// Reference error in V band.  
    /// These photometric errors are not formal statistical uncertainties but
    /// raw and conservative estimates to be used for telescope operations.
    v_mag_error,
    /// coded emulsion / bandpass / filter:  
    /// ———————————————————————-  
    /// bcode Name Emulsion/Filter  
    /// ———————————————————————-  
    /// 1 V IIaD+W12 (Pal Quick-V)  
    /// 4 V (Johnson)  
    /// 6 V495 IIaD+GG495 (Pal QV/AAO XV)  
    /// 42 VT TYCHO-V  
    /// ———————————————————————-
    v_mag_code,
    /// Magnitude in In photographic band.
    npg_mag,
    /// Reference error in In photographic band.  
    /// These photometric errors are not formal statistical uncertainties but
    /// raw and conservative estimates to be used for telescope operations.
    npg_mag_error,
    /// coded emulsion / bandpass / filter:  
    /// ———————————————————————-  
    /// bcode Name Emulsion/Filter  
    /// ———————————————————————-  
    /// 37 Npg IVN+RG9 (POSS-II IR)  
    /// 38 Npg IVN+RG715 (SERC-IR)  
    /// ———————————————————————-
    npg_mag_code,
    /// Magnitude in B band.  
    /// This filter may include:
    ///
    /// -   B_T of Tycho-2 stars,
    ///
    /// -   Johnson B from SKY2000, or
    ///
    /// -   photographic O from pOSS-I.
    b_mag,
    /// Reference error in B band.  
    /// These photometric errors are not formal statistical uncertainties but
    /// raw and conservative estimates to be used for telescope operations.
    b_mag_error,
    /// coded emulsion / bandpass / filter:  
    /// ———————————————————————-  
    /// bcode Name Emulsion/Filter  
    /// ———————————————————————-  
    /// 3 B (Johnson)  
    /// 7 O 103aO+no filter (POSS-I Blue)  
    /// 41 BT TYCHO-B  
    /// ———————————————————————-
    b_mag_code,
    /// Image classification:  
    /// 0 = “star”, i.e. point-like object  
    /// 3 = “nonstar”, i.e. extended object
    classification,
    /// The status code is a 10-digit field encoding the properties of the
    /// catalog object.  
    /// This flag is 99999900 for a Tycho object and 88888800 for SKY2000
    /// object.  
    ///   
    ///   
    ///   
    /// FPA = Fractional Pixel Allocation)
    status,
    /// This flag concerns only the Tycho stars (Status=99999900).
    mult_flag,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the gsc23_original_valid table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::gsc23_oid.to_string());
    col_strings.push(Col::gsc23_identifier.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::position_epoch.to_string());
    col_strings.push(Col::ra_error.to_string());
    col_strings.push(Col::dec_error.to_string());
    col_strings.push(Col::fpg_mag.to_string());
    col_strings.push(Col::fpg_mag_error.to_string());
    col_strings.push(Col::fpg_mag_code.to_string());
    col_strings.push(Col::jpg_mag.to_string());
    col_strings.push(Col::jpg_mag_error.to_string());
    col_strings.push(Col::jpg_mag_code.to_string());
    col_strings.push(Col::v_mag.to_string());
    col_strings.push(Col::v_mag_error.to_string());
    col_strings.push(Col::v_mag_code.to_string());
    col_strings.push(Col::npg_mag.to_string());
    col_strings.push(Col::npg_mag_error.to_string());
    col_strings.push(Col::npg_mag_code.to_string());
    col_strings.push(Col::b_mag.to_string());
    col_strings.push(Col::b_mag_error.to_string());
    col_strings.push(Col::b_mag_code.to_string());
    col_strings.push(Col::classification.to_string());
    col_strings.push(Col::status.to_string());
    col_strings.push(Col::mult_flag.to_string());
    map.insert(gsc23_original_valid.string(), col_strings);
}
