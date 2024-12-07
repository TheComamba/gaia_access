// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the ravedr5_on table.

use crate::traits::{Column, Table};

/// The Radial Velocity Experiment (RAVE): Fifth Data Release. Stellar parameters from Cannon. Original catalogue released by Kunder et al. 2017 AJ 153, 75K. Data retrieved using the VizieR catalogue access tool, CDS, Strasbourg, France. The original description of the VizieR service was published in A&AS 143, 23. VizieR catalogue II/279.
#[allow(non_camel_case_types)]
pub struct ravedr5_on;

impl Table for ravedr5_on {
    fn string(&self) -> String {
        "ravedr5_on".to_string()
    }
}

/// The columns in the ravedr5_on table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// ! Target designation (RAVE_OBS_ID)
    name,
    /// ! RAVE designation(JHHMMSS.S+DDMMSS) (RAVEID)
    rave,
    /// Heliocentric radial velocity (HRV)
    hrv,
    /// Error on HRV (eHRV)
    e_hrv,
    /// ? Standard deviation in HRV from 10 resampled spectra (StdDev_HRV)
    s_hrv,
    /// ? Median absolute deviation in HRV from 10 resampled spectra (MAD_HRV)
    s_med_hrv,
    /// Tonry-Davis R correlation coefficient (CorrelationCoeff)
    r,
    /// Height of correlation peak (PeakHeight)
    hcp,
    /// Width of correlation peak (PeakWidth)
    wcp,
    /// Zero point correction applied HRV (CorrectionRV)
    crv,
    /// Measured HRV of sky (SkyRV)
    hrvsky,
    /// Error HRV of sky (eSkyRV)
    e_hrvsky,
    /// Correlation Coefficient R of sky (SkyCorrelationCoeff)
    rsky,
    /// ? Rotational Velocity from SPARV pipeline (Vrot_SPARV)
    vrots,
    /// Quality Flag for ZeroPoint correction (ZeroPointFLAG) (1)
    zpflag,
    /// 1st minimum distance (c1) (2)
    c1,
    /// 2nd minimum distance (c2) (2)
    c2,
    /// 3rd minimum distance (c3) (2)
    c3,
    /// 4th minimum distance (c4) (2)
    c4,
    /// 5th minimum distance (c5) (2)
    c5,
    /// 6th minimum distance (c6) (2)
    c6,
    /// 7th minimum distance (c7) (2)
    c7,
    /// 8th minimum distance (c8) (2)
    c8,
    /// 9th minimum distance (c9) (2)
    c9,
    /// 10th minimum distance (c10) (2)
    c10,
    /// 11th minimum distance (c11) (2)
    c11,
    /// 12th minimum distance (c12) (2)
    c12,
    /// 13th minimum distance (c13) (2)
    c13,
    /// 14th minimum distance (c14) (2)
    c14,
    /// 15th minimum distance (c15) (2)
    c15,
    /// 16th minimum distance (c16) (2)
    c16,
    /// 17th minimum distance (c17) (2)
    c17,
    /// 18th minimum distance (c18) (2)
    c18,
    /// 19th minimum distance (c19) (2)
    c19,
    /// 20th minimum distance (c20) (2)
    c20,
    /// ? Effective temperature (TEFF) (3)
    teff,
    /// ? Log gravity (LOGG) (3)
    logg,
    /// ? Abundance of Fe (FE_H) (3)
    fe_h,
    /// ? Abundance of O (O_H) (3)
    o_h,
    /// ? Abundance of Mg (MG_H) (3)
    mg_h,
    /// ? Abundance of Al (AL_H) (3)
    al_h,
    /// ? Abundance of Si (SI_H) (3)
    si_h,
    /// ? Abundance of Ca (CA_H) (3)
    ca_h,
    /// ? Abundance of Ni (NI_H) (3)
    ni_h,
    /// ? Error on effective temperature (e_TEFF) (3)
    e_teff,
    /// ? Error on log gravity (E_LOGG) (3)
    e_logg,
    /// ? Error on [Fe/H] (E_FE_H) (3)
    e_fe_h,
    /// ? Error on [O/H] (E_O_H) (3)
    e_o_h,
    /// ? Error on [Mg/H] (E_MG_H) (3)
    e_mg_h,
    /// ? Error on [Al/H] (E_AL_H) (3)
    e_al_h,
    /// ? Error on [Si/H] (E_SI_H) (3)
    e_si_h,
    /// ? Error on [Ca/H] (E_CA_H) (3)
    e_ca_h,
    /// ? Error on [Ni/H] (E_NI_H) (3)
    e_ni_h,
    /// ? Signal to noise ratio (SNR) (3)
    snr,
    /// ? Chi-square (R_CHI_SQ) (3)
    rchisq,
    /// Quality flag (QC) (3)
    qc,
    /// Right ascension (J2000.0, Ep=J2000) (RAdeg)
    ra,
    /// Declination (J2000.0, Ep=J2000) (DEdeg)
    dec,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the ravedr5_on table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::name.to_string());
    col_strings.push(Col::rave.to_string());
    col_strings.push(Col::hrv.to_string());
    col_strings.push(Col::e_hrv.to_string());
    col_strings.push(Col::s_hrv.to_string());
    col_strings.push(Col::s_med_hrv.to_string());
    col_strings.push(Col::r.to_string());
    col_strings.push(Col::hcp.to_string());
    col_strings.push(Col::wcp.to_string());
    col_strings.push(Col::crv.to_string());
    col_strings.push(Col::hrvsky.to_string());
    col_strings.push(Col::e_hrvsky.to_string());
    col_strings.push(Col::rsky.to_string());
    col_strings.push(Col::vrots.to_string());
    col_strings.push(Col::zpflag.to_string());
    col_strings.push(Col::c1.to_string());
    col_strings.push(Col::c2.to_string());
    col_strings.push(Col::c3.to_string());
    col_strings.push(Col::c4.to_string());
    col_strings.push(Col::c5.to_string());
    col_strings.push(Col::c6.to_string());
    col_strings.push(Col::c7.to_string());
    col_strings.push(Col::c8.to_string());
    col_strings.push(Col::c9.to_string());
    col_strings.push(Col::c10.to_string());
    col_strings.push(Col::c11.to_string());
    col_strings.push(Col::c12.to_string());
    col_strings.push(Col::c13.to_string());
    col_strings.push(Col::c14.to_string());
    col_strings.push(Col::c15.to_string());
    col_strings.push(Col::c16.to_string());
    col_strings.push(Col::c17.to_string());
    col_strings.push(Col::c18.to_string());
    col_strings.push(Col::c19.to_string());
    col_strings.push(Col::c20.to_string());
    col_strings.push(Col::teff.to_string());
    col_strings.push(Col::logg.to_string());
    col_strings.push(Col::fe_h.to_string());
    col_strings.push(Col::o_h.to_string());
    col_strings.push(Col::mg_h.to_string());
    col_strings.push(Col::al_h.to_string());
    col_strings.push(Col::si_h.to_string());
    col_strings.push(Col::ca_h.to_string());
    col_strings.push(Col::ni_h.to_string());
    col_strings.push(Col::e_teff.to_string());
    col_strings.push(Col::e_logg.to_string());
    col_strings.push(Col::e_fe_h.to_string());
    col_strings.push(Col::e_o_h.to_string());
    col_strings.push(Col::e_mg_h.to_string());
    col_strings.push(Col::e_al_h.to_string());
    col_strings.push(Col::e_si_h.to_string());
    col_strings.push(Col::e_ca_h.to_string());
    col_strings.push(Col::e_ni_h.to_string());
    col_strings.push(Col::snr.to_string());
    col_strings.push(Col::rchisq.to_string());
    col_strings.push(Col::qc.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    map.insert(ravedr5_on.string(), col_strings);
}
