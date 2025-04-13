// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the catwise2020 table.

use crate::traits::{Column, Table};

/// The CatWISE2020 Catalogue contains 1,890,715,640 sources over the entire sky selected from WISE and NEOWISE survey data at 3.4 and 4.6 micrometer (W1 and W2) collected from 7 January 2010 to 13 December 2018. This data set adds two years to that used for the CatWISE Preliminary Catalogue (Eisenhardt+, 2020, ApJS, 247, 69), bringing the total to six times as many exposures spanning over sixteen times as large a time baseline as the AllWISE catalogue. The other major change from the CatWISE Preliminary Catalogue is that the detection list for the CatWISE2020 Catalogue was generated using "crowdsource" software (Schlafly+, 2019, ApJS, 240, 30), while the CatWISE Preliminary Catalogue used the detection software used for AllWISE. These two factors result in roughly twice as many sources in the CatWISE2020 Catalogue. This table was created in May 2023 based on the VizieR version of the CatWISE2020 Catalogue (VizieR catalogue: II/365). The corrections to the astrometry (positions and proper motions) identified in Marocco+ (2021, ApJS, 253, 8) have been included by CDS/VizieR and are therefore also included in this table. Reference paper: https://ui.adsabs.harvard.edu/abs/2021ApJS..253....8M/abstract (DOI: 10.3847/1538-4365/abd805)
#[allow(non_camel_case_types)]
pub struct catwise2020;

impl Table for catwise2020 {
    fn string(&self) -> String {
        "catwise2020".to_string()
    }
}

/// The columns in the catwise2020 table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Tile name + processing code + wphot index (source_id)
    obj_id,
    /// Right ascension (ICRS) (1)
    ra_icrs,
    /// [0/74.3] Uncertainty in ra (sigra)
    e_ra_icrs,
    /// Declination (ICRS) (1)
    de_icrs,
    /// [0/83.3] Uncertainty in dec (sigdec)
    e_de_icrs,
    /// Source name (JHHMMSS.ss+DDMMSS.s; <CWISE JHHMMSS.ss+DDMMSS.s> in Simbad) (source_name)
    name,
    /// [-25.7/44.2] Uncertainty cross-term (sigradec)
    e_pos,
    /// X-pixel coordinate in the unWISE full depth coadd (wx)
    xpos,
    /// Y-pixel coordinate in the unWISE full depth coadd (wy)
    ypos,
    /// [-98.2/4997]? Frame sky background value, band-1; in dn units (w1sky) (2)
    sky_w1,
    /// [0.4/999]? Frame sky background value uncertainty; band-1 (w1sigsk) (2)
    e_sky_w1,
    /// [0/999]? Frame sky confusion based on the UNC images; in dn units (w1conf) (2)
    conf_w1,
    /// [-99/5000]? Frame sky background value, band-2; in dn (w2sky) (2)
    sky_w2,
    /// [0.7/999]? Frame sky background value uncertainty; band-2 (w2sigsk) (2)
    e_sky_w2,
    /// [0/999]? Frame sky confusion based on the UNC images; in dn units (w2conf) (2)
    conf_w2,
    /// [0/221]? Number of profile-fit flux measurements for source with SNR>=3, band 1 (w1NM)
    n_w1,
    /// [0/221]? Number of profile-fit flux measurements for source, band-1 (w1M)
    m_w1,
    /// [0/221]? Number of profile-fit flux measurements for source with SNR>=3, band-2 (w2NM)
    n_w2,
    /// [0/221]? Number of profile-fit flux measurements for source, band-2 (w2M)
    m_w2,
    /// [55205/58465] Mean observation epoch (MeanObsMJD)
    mjd,
    /// Right ascension (ICRS) at Ep=2015.4 (ra_pm) (3)
    ra_pm_deg,
    /// [1e-4/99.94]? Uncertainty in ra_pm (sigra_pm)
    e_ra_pm_deg,
    /// Declination (ICRS) at Ep=2015.4 (dec_pm) (3)
    de_pm_deg,
    /// [0/99.8]? Uncertainty in dec_pm (sigdec_pm)
    e_de_pm_deg,
    /// [-99.2/99.9]? Uncertainty cross-term (sigradec_pm)
    e_pos_pm,
    /// [-100/100] Proper motion in ra (PMRA) (1)
    pm_ra,
    /// [7e-4/1000] Uncertainty in PMRA (sigPMRA)
    e_pm_ra,
    /// [-100/100] Proper motion in dec (PMDec) (1)
    pm_de,
    /// [7e-4/1000] Uncertainty in PMDec (sigPMDec)
    e_pm_de,
    /// [0/1000]? Flux S/N ratio; band-1 (w1snr_pm)
    snr_w1pm,
    /// [0/1000]? Flux S/N ratio; band-2 (w2snr_pm)
    snr_w2pm,
    /// [-1.2e8/2e14]? WPRO raw flux, band-1 (W1, 3.35um) in dn units (w1flux_pm)
    fw1pm,
    /// [3.7e-4/3.5e12]? WPRO raw flux uncertainty; band-1 (w1sigflux_pm)
    e_fw1pm,
    /// [-4.8e8/2e10]? WPRO raw flux, band-2 (W2, 4.6um); in dn units (w2flux_pm)
    fw2pm,
    /// [1e-4/3.2e13]? Fit WPRO raw flux uncertainty; band-2 (w2sigflux_pm)
    e_fw2pm,
    /// [-10/27]? WPRO magnitude in band-1 (W1: 3.35um) (w1mpro_pm) (4)
    w1mpro_pm,
    /// [0/0.6]? W1mproPM uncertainty (w1sigmpro_pm)
    e_w1mpro_pm,
    /// [8.3e-7/6.4e21]? WPRO reduced chi^2^; band-1 (w1rchi2_pm)
    chi2_w1pm,
    /// [-10/25.5]? WPRO magnitude in band-2 (W2: 4.6um) (w2mpro_pm) (4)
    w2mpro_pm,
    /// [0/0.6]? W2mproPM uncertainty (w2sigmpro_pm)
    e_w2mpro_pm,
    /// [2.6e-6/375600]? WPRO reduced chi^2^; band-2 (w2rchi2_pm)
    chi2_w2pm,
    /// [8.3e-7/92260]? Reduced chi squared; total (rchi2_pm)
    chi2pm,
    /// Quality of the PM solution (pmcode) (5)
    pm_qual,
    /// [0/20]? Radial distance between apparitions (dist)
    dist,
    /// [-9.2/9.3]? W1mpro difference (dw1mag)
    d_w1mpro,
    /// [0/100]? Chi-square for dw1mag (1 DF) (rch2w1)
    chi2d_w1mpro,
    /// [-9.4/10.2]? W2mpro difference (dw2mag)
    d_w2mpro,
    /// [0/100]? Chi-square for dw2mag (1 DF) (rch2w2)
    chi2d_w2mpro,
    /// ? Averaged ecliptic longitude (elon_avg)
    elon_avg,
    /// [0.001/1.6]? One-sigma uncertainty in elon (elonSig)
    e_elon_avg,
    /// ? Averaged ecliptic latitude (elat_avg)
    elat_avg,
    /// [0.001/1.7]? One-sigma uncertainty in elat (elatSig)
    e_elat_avg,
    /// ? Descending-ascending ecliptic longitude (Delon) (6)
    d_elon,
    /// [0.001/101.5]? One-sigma uncertainty in Delon (DelonSig)
    e_d_elon,
    /// ? Descending-ascending ecliptic latitude (Delat)
    d_elat,
    /// [0.001/105.8]? One-sigma uncertainty in Delat (DelatSig)
    e_d_elat,
    /// [0/1065]? |Delon|/DelonSig (DelonSNR)
    snrd_elon,
    /// [0/475]? |Delat|/DelatSig (DelatSNR)
    snrd_elat,
    /// [0/7.3e7]? Chi-square for PMRA difference (1 DF) (chi2pmra)
    chi2pm_ra,
    /// [0/2.2e8]? Chi-square for PMRA difference (1 DF) (chi2pmdec)
    chi2pm_de,
    /// [0123n] Astrometry usage code (7)
    ka,
    /// [0123n] W1 photometry usage code (7)
    k1,
    /// [0123n] W2 photometry usage code (7)
    k2,
    /// [0123n] Proper motion usage code (7)
    km,
    /// [-831/569]? Parallax from PM desc-asce elon (par_pm)
    plx1,
    /// [0.001/53]? One-sigma uncertainty in par_pm (par_pmSig)
    e_plx1,
    /// [-831/504]? Parallax estimate from stationary solution (par_stat)
    plx2,
    /// [0.001/189]? One-sigma uncertainty in par_stat (par_sigma)
    e_plx2,
    /// [0/2.75]? Distance between CatWISE and AllWISE source (dist_x)
    sep,
    /// [0DHOPdhop] Worst case 4 character cc_flag from AllWISE (cc_flags) (8)
    ccf,
    /// [0DHOP] Two character (W1 W2) artifact flag (ab_flags) (8)
    abf,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the catwise2020 table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::obj_id.to_string());
    col_strings.push(Col::ra_icrs.to_string());
    col_strings.push(Col::e_ra_icrs.to_string());
    col_strings.push(Col::de_icrs.to_string());
    col_strings.push(Col::e_de_icrs.to_string());
    col_strings.push(Col::name.to_string());
    col_strings.push(Col::e_pos.to_string());
    col_strings.push(Col::xpos.to_string());
    col_strings.push(Col::ypos.to_string());
    col_strings.push(Col::sky_w1.to_string());
    col_strings.push(Col::e_sky_w1.to_string());
    col_strings.push(Col::conf_w1.to_string());
    col_strings.push(Col::sky_w2.to_string());
    col_strings.push(Col::e_sky_w2.to_string());
    col_strings.push(Col::conf_w2.to_string());
    col_strings.push(Col::n_w1.to_string());
    col_strings.push(Col::m_w1.to_string());
    col_strings.push(Col::n_w2.to_string());
    col_strings.push(Col::m_w2.to_string());
    col_strings.push(Col::mjd.to_string());
    col_strings.push(Col::ra_pm_deg.to_string());
    col_strings.push(Col::e_ra_pm_deg.to_string());
    col_strings.push(Col::de_pm_deg.to_string());
    col_strings.push(Col::e_de_pm_deg.to_string());
    col_strings.push(Col::e_pos_pm.to_string());
    col_strings.push(Col::pm_ra.to_string());
    col_strings.push(Col::e_pm_ra.to_string());
    col_strings.push(Col::pm_de.to_string());
    col_strings.push(Col::e_pm_de.to_string());
    col_strings.push(Col::snr_w1pm.to_string());
    col_strings.push(Col::snr_w2pm.to_string());
    col_strings.push(Col::fw1pm.to_string());
    col_strings.push(Col::e_fw1pm.to_string());
    col_strings.push(Col::fw2pm.to_string());
    col_strings.push(Col::e_fw2pm.to_string());
    col_strings.push(Col::w1mpro_pm.to_string());
    col_strings.push(Col::e_w1mpro_pm.to_string());
    col_strings.push(Col::chi2_w1pm.to_string());
    col_strings.push(Col::w2mpro_pm.to_string());
    col_strings.push(Col::e_w2mpro_pm.to_string());
    col_strings.push(Col::chi2_w2pm.to_string());
    col_strings.push(Col::chi2pm.to_string());
    col_strings.push(Col::pm_qual.to_string());
    col_strings.push(Col::dist.to_string());
    col_strings.push(Col::d_w1mpro.to_string());
    col_strings.push(Col::chi2d_w1mpro.to_string());
    col_strings.push(Col::d_w2mpro.to_string());
    col_strings.push(Col::chi2d_w2mpro.to_string());
    col_strings.push(Col::elon_avg.to_string());
    col_strings.push(Col::e_elon_avg.to_string());
    col_strings.push(Col::elat_avg.to_string());
    col_strings.push(Col::e_elat_avg.to_string());
    col_strings.push(Col::d_elon.to_string());
    col_strings.push(Col::e_d_elon.to_string());
    col_strings.push(Col::d_elat.to_string());
    col_strings.push(Col::e_d_elat.to_string());
    col_strings.push(Col::snrd_elon.to_string());
    col_strings.push(Col::snrd_elat.to_string());
    col_strings.push(Col::chi2pm_ra.to_string());
    col_strings.push(Col::chi2pm_de.to_string());
    col_strings.push(Col::ka.to_string());
    col_strings.push(Col::k1.to_string());
    col_strings.push(Col::k2.to_string());
    col_strings.push(Col::km.to_string());
    col_strings.push(Col::plx1.to_string());
    col_strings.push(Col::e_plx1.to_string());
    col_strings.push(Col::plx2.to_string());
    col_strings.push(Col::e_plx2.to_string());
    col_strings.push(Col::sep.to_string());
    col_strings.push(Col::ccf.to_string());
    col_strings.push(Col::abf.to_string());
    map.insert(catwise2020.string(), col_strings);
}
