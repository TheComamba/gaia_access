// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the ravedr5_dr5 table.

use crate::traits::{Column, Table};

/// The Radial Velocity Experiment (RAVE): fifth data release (DR5). Main catalogue. RAVE DR5 is the fifth data release from a magnitude-limited (9 < I < 12 mag) survey of stars randomly selected in the Southern Hemisphere. The RAVE medium-resolution spectra (R ~ 7500) covering the Ca-triplet region (8410-8795 Angstrom) span the complete time frame from the start of RAVE observations in 2003 to their completion in 2013. Radial velocities from 520,781 spectra of 457,588 unique stars are presented, of which 255,922 stellar observations have parallaxes and proper motions from the Tycho-Gaia astrometric solution in Gaia DR1. For the main DR5 catalogue, stellar parameters (effective temperature, surface gravity, and overall metallicity) are computed using the RAVE DR4 stellar pipeline, but calibrated using recent K2 Campaign 1 seismic gravities and Gaia benchmark stars, as well as results obtained from high-resolution studies. Also included are temperatures from the Infrared Flux Method, and a catalogue of red giant stars in the de-reddened color {(J-{Ks})}_0 interval (0.50, 0.85) for which the gravities were calibrated based only on seismology. Further data products for subsamples of the RAVE stars include individual abundances for Mg, Al, Si, Ca, Ti, Fe, and Ni, and distances found using isochrones. Each RAVE spectrum is complemented by an error spectrum, which has been used to determine uncertainties on the parameters. Data retrieved using the VizieR catalogue access tool, CDS, Strasbourg, France (VizieR catalogue: III/279). Reference paper: https://ui.adsabs.harvard.edu/abs/2017AJ....153...75K/abstract (DOI: 10.3847/1538-3881/153/2/75)
#[allow(non_camel_case_types)]
pub struct ravedr5_dr5;

impl Table for ravedr5_dr5 {
    fn string(&self) -> String {
        "ravedr5_dr5".to_string()
    }
}

/// The columns in the ravedr5_dr5 table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// ! Target designation (RAVE_OBS_ID)
    name,
    /// Hierarchical Equal-Area iso-Latitude Pixelisation value (N_side=4096) (HEALPix) (1)
    healpix,
    /// ! RAVE designation(JHHMMSS.S+DDMMSS) (RAVEID)
    rave,
    /// Right ascension (J2000.0, Ep=J2000) (RAdeg)
    ra,
    /// Declination (J2000.0, Ep=J2000) (DEdeg)
    dec,
    /// Galactic Longitude
    glon,
    /// Galactic Latitude
    glat,
    /// Heliocentric radial velocity (HRV)
    hrv,
    /// Error on HRV (eHRV)
    e_hrv,
    /// ? Standard deviation in HRV from 10 resampled spectra (StdDev_HRV)
    s_hrv,
    /// ? Median absolute deviation in HRV from 10 resampled spectra (MAD_HRV)
    s_med_hrv,
    /// Signal/Noise Ratio of SPARV pipeline (STN_SPARV) (2)
    snrs,
    /// ? Signal/Noise Ratio of Kordopatis pipeline (SNR_K) (2)
    snrk,
    /// ? Effective temperature (Teff_K) (2)
    teffk,
    /// ? Calibrated effective temperature (Teff_N_K) (2)
    cteffk,
    /// ? Effective Temperature error (eTeff_K)
    e_teffk,
    /// ? Median absolute deviation in TeffK from 10 resampled spectra (MAD_Teff_K)
    s_med_teffk,
    /// ? Standard deviation in Teff_K from 10 resampled spectra (StdDev_Teff_K)
    s_teffk,
    /// ? Log gravity (logg_K) (2)
    loggk,
    /// ? Calibrated log gravity (logg_N_K) (2)
    cloggk,
    /// ? Error Log gravity (elogg_K) (2)
    e_loggk,
    /// ? Standard deviation in loggK from 10 resampled spectra (MAD_logg_K)
    s_loggk,
    /// ? Median absolute deviation in loggK from 10 resampled spectra (StdDev_logg_K)
    s_med_loggk,
    /// ? Metallicity [m/H] (Met_K) (2)
    m_h_k,
    /// ? Calibrated metallicity (Met_N_K) (2)
    m_h_nk,
    /// ? Error on metallicity (eMet_K) (2)
    e__m_h_k,
    /// ? Standard deviation in [M/H]K from 10 resampled spectra (MAD_Met_K)
    s__m_h_k,
    /// ? Median absolute deviation in [M/H]K from 10 resampled spectra (StdDev_Met_K)
    s_med__m_h_k,
    /// ? Chi-square from the stellar parameter pipeline (CHISQ_K) (2)
    chisqk,
    /// [0/4]? Quality flag for stellar parameter pipeline (Algo_Conv_K) (2) (3)
    qk,
    /// Temperature from infrared flux method (Teff_IR)
    teffir,
    /// ? Internal error on TeffIR (eTeff_IR)
    e_teffir,
    /// Infrared flux method flag (IR_direct) (4)
    irdirect,
    /// ? Abundance of Mg (Mg) (2)
    mg_h_c,
    /// [0/6]? Number of used spectral lines for calculation (Mg_N) (2)
    o_mg_h_c,
    /// ? Abundance of Al (Al) (2)
    al_h_c,
    /// ? Number of used spectral lines for calculation (Al_N) (2)
    o_al_h_c,
    /// ? Abundance of Si (Si) (2)
    si_h_c,
    /// ? Number of used spectral lines for calculation (Si_N) (2)
    o_si_h_c,
    /// ? Abundance of Ti (Ti) (2)
    ti_h_c,
    /// ? Number of used spectral lines for calculation (Ti_N) (2)
    o_ti_h_c,
    /// ? Abundance of Fe (Fe) (2)
    fe_h_c,
    /// ? Number of used spectral lines for calculation (Fe_N) (2)
    o_fe_h_c,
    /// ? Abundance of Ni (Ni) (2)
    ni_h_c,
    /// ? Number of used spectral lines for calculation (Ni_N) (2)
    o_ni_h_c,
    /// ? Alpha-enhancement from chemi cal pipeline (Alpha_c) (2)
    a_fe_c,
    /// ? Chi-square of chemi cal pipeline (CHISQ_c) (2)
    chisqc,
    /// ? Fraction of spectrum used for calculation of abundances (frac_c) (2)
    fracc,
    /// ? Total extinction in V-band from Schlegel et al. (1998ApJ...500..525S) (AV_Schlegel)
    avschl,
    /// ? Spectrophotometric distance (distance) (10)
    dist,
    /// ? Error on distance (edistance) (10)
    e_dist,
    /// ? Log Av extinction (log_Av) (10)
    logav,
    /// ? Error on LogAv (elog_Av) (10)
    e_logav,
    /// ? Spectrophotometric parallax (parallax) (10)
    plx,
    /// ? Error on plx (eparallax) (10)
    e_plx,
    /// ? Distance modulus (DistanceModulus_Binne) (10)
    dm,
    /// ? Error on DM (eDistanceModulus_Binne) (10)
    e_dm,
    /// ? Fit flag see sec.3 of Binney et al. (2014MNRAS.437..351B) (Fit_Flag_Binney)
    ffb,
    /// ? Fit quality "F" given by Eq. 15 of Binney et al. (2014MNRAS.437..351B) (FitQuality_Binney)
    fqb,
    /// ? Number of components required for multi-Gaussian distance modulus fit (N_Gauss_fit)
    ngauss,
    /// ? Mean of 1st Gaussian (Gauss_mean_1)
    gm1,
    /// ? Sigma of 1st Gaussian (Gauss_sigma_1)
    gs1,
    /// ? Normalisation of 1st Gaussian (Gauss_frac_1)
    gf1,
    /// ? Mean of 2nd Gaussian (Gauss_mean_2)
    gm2,
    /// ? Sigma of 2nd Gaussian (Gauss_sigma_2)
    gs2,
    /// ? Normalisation of 2nd Gaussian (Gauss_frac_2)
    gf2,
    /// ? Mean of 3rd Gaussian (Gauss_mean_3)
    gm3,
    /// ? Sigma of 3rd Gaussian (Gauss_sigma_3)
    gs3,
    /// ? Normalisation of 3rd Gaussian (Gauss_frac_3)
    gf3,
    /// 1st minimum distance (c1) (5)
    c1,
    /// 2nd minimum distance (c2) (5)
    c2,
    /// 3rd minimum distance (c3) (5)
    c3,
    /// 4th minimum distance (c4) (5)
    c4,
    /// 5th minimum distance (c5) (5)
    c5,
    /// 6th minimum distance (c6) (5)
    c6,
    /// 7th minimum distance (c7) (5)
    c7,
    /// 8th minimum distance (c8) (5)
    c8,
    /// 9th minimum distance (c9) (5)
    c9,
    /// 10th minimum distance (c10) (5)
    c10,
    /// 11th minimum distance (c11) (5)
    c11,
    /// 12th minimum distance (c12) (5)
    c12,
    /// 13th minimum distance (c13) (5)
    c13,
    /// 14th minimum distance (c14) (5)
    c14,
    /// 15th minimum distance (c15) (5)
    c15,
    /// 16th minimum distance (c16) (5)
    c16,
    /// 17th minimum distance (c17) (5)
    c17,
    /// 18th minimum distance (c18) (5)
    c18,
    /// 19th minimum distance (c19) (5)
    c19,
    /// 20th minimum distance (c20) (5)
    c20,
    /// [1]? Repetition flag (Rep_Flag) (6)
    repflag,
    /// [1]? Cluster star flag (CluStar_Flag) (7)
    clsflag,
    /// [1]? Footprint flag FootPrint_Flag) (8)
    fpflag,
    /// ? TGAS target designation (ID_TGAS_source)
    tgas,
    /// ? TGAS right ascension (Ep=2015.0) (RA_TGAS)
    ratgas,
    /// ? TGAS declination (Ep=2015.0) (DE_TGAS)
    detgas,
    /// ? Proper motion RA from TGAS (pmRA_TGAS)
    pmratgas,
    /// ? Error on proper motion RA from TGAS (pmRA_error_TGAS)
    e_pmratgas,
    /// ? Proper motion DE from TGAS (pmDE_TGAS)
    pmdetgas,
    /// ? Error on proper motion DE from TGAS (pmDE_error_TGAS)
    e_pmdetgas,
    /// ? Parallax from TGAS (parallax_TGAS)
    plxtgas,
    /// ? Error on parallax from TGAS (parallax_error_TGAS)
    e_plxtgas,
    /// ? TGAS G magnitude (phot_g_mean_mag_TGAS)
    gmagtgas,
    /// ? TGAS flux in G band (phot_g_mean_flux_TGAS)
    gfluxtgas,
    /// ? Error on G band flux from TGAS (phot_g_mean_flux_error_TGAS)
    e_gfluxtgas,
    /// ? Hipparcos target designation (ID_Hipparcos)
    hipparcos,
    /// Tycho-2 target designation (ID_TYCHO2)
    tycho2,
    /// ? Distance to Tycho-2 source (Dist_TYCHO2)
    distt2,
    /// Cross-match quality flag Tycho-2 (MatchFlag_TYCHO2) (9)
    xt2,
    /// ? Tycho-2 BT magnitude (BTmag_TYCHO2)
    btmagt2,
    /// ? Error on Tycho-2 BT magnitude (eBTmag_TYCHO2)
    e_btmagt2,
    /// ? Tycho-2 VT magnitude (VTmag_TYCHO2)
    vtmagt2,
    /// ? Error on Tycho-2 VT magnitude (eVTmag_TYCHO2)
    e_vtmagt2,
    /// ? Proper motion RA from Tycho-2 (pmRA_TYCHO2)
    pmrat2,
    /// ? Error on proper motion RA from Tycho-2 (epmRA_TYCHO2)
    e_pmrat2,
    /// ? Proper motion De from Tycho-2 (pmDE_TYCHO2)
    pmdet2,
    /// ? Error on proper motion DE from Tycho-2 (epmDE_TYCHO2)
    e_pmdet2,
    /// UCAC4 target designation (ID_UCAC4)
    ucac4,
    /// ? Distance to UCAC4 source (Dist_UCAC4)
    distu4,
    /// Cross-match quality flag UCAC4 (MatchFlag_UCAC4) (9)
    xu4,
    /// ? Proper motion RA from UCAC4 (pmRA_UCAC4)
    pmrau4,
    /// ? Error on proper motion RA from UCAC4 (epmRA_UCAC4)
    e_pmrau4,
    /// ? Proper motion DE from UCAC4 (pmDE_UCAC4)
    pmdeu4,
    /// ? Error on proper motion DE from UCAC4 (epmDE_UCAC4)
    e_pmdeu4,
    /// ? PPMXL target designation (ID_PPMXL)
    ppmxl,
    /// ? Distance to PPMXL source (Dist_PPMXL)
    distp,
    /// Cross-match quality flag PPMXL (MatchFlag_PPMXL) (9)
    xp,
    /// ? Proper motion RA from PPMXL (pmRA_PPMXL)
    pmrap,
    /// ? Error on proper motion RA from PPMXL (epmRA_PPMXL)
    e_pmrap,
    /// ? Proper motion DE from PPMXL (pmDE_PPMXL)
    pmdep,
    /// ? Error on proper motion DE from PPMXL (epmDE_PPMXL)
    e_pmdep,
    /// 2MASS target designation (ID_2MASS)
    col2mass,
    /// ? Distance to 2MASS source (Dist_2MASS)
    dist2,
    /// Cross-match quality flag 2MASS (MatchFlag_2MASS) (9)
    x2,
    /// ? 2MASS J magnitude (Jmag_2MASS)
    jmag2,
    /// ? Error on 2MASS J magnitude (eJmag_2MASS)
    e_jmag2,
    /// ? 2MASS H magnitude (Hmag_2MASS)
    hmag2,
    /// ? Error on 2MASS H magnitude (eHmag_2MASS)
    e_hmag2,
    /// ? 2MASS K magnitude (Kmag_2MASS)
    kmag2,
    /// ? Error on 2MASS K magnitude (eKmag_2MASS)
    e_kmag2,
    /// ALLWISE target designation (ID_ALLWISE)
    allwise,
    /// ? Distance to ALLWISE source (Dist_ALLWISE)
    distw,
    /// Cross-match quality flag ALLWISE (MatchFlag_ALLWISE) (9)
    xw,
    /// ? ALLWISE W1 magnitude (W1mag_ALLWISE)
    w1magw,
    /// ? Error on ALLWISE W1 magnitude (eW1mag_ALLWISE)
    e_w1magw,
    /// ? ALLWISE W2 magnitude (W2mag_ALLWISE)
    w2magw,
    /// ? Error on ALLWISE W2 magnitude (eW2mag_ALLWISE)
    e_w2magw,
    /// ? ALLWISE W3 magnitude (W3mag_ALLWISE)
    w3magw,
    /// ? Error on ALLWISE W3 magnitude (eW3mag_ALLWISE)
    e_w3magw,
    /// ? ALLWISE W4 magnitude (W4mag_ALLWISE)
    w4magw,
    /// ? Error on ALLWISE W4 magnitude (eW4mag_ALLWISE)
    e_w4magw,
    /// Prioritized artifacts affecting the source in each band (cc_flags_ALLWISE)
    ccflagsw,
    /// ? Probability source morphology is not consistent with single PSF (ext_flg_ALLWISE)
    extflgw,
    /// Probability that flux varied in any band greater than amount expected (var_flg_ALLWISE)
    varflgw,
    /// Photometric quality of each band A=highest, U=upper limit (ph_qual_ALLWISE)
    phqualw,
    /// ? Distance to APASS DR9 source (Dist_APASSDR9)
    distadr9,
    /// Cross-match quality flag APASSDR9 (MatchFlag_APASSDR9) (9)
    xadr9,
    /// ? APASS DR9 B magnitude (Bmag_APASSDR9)
    bmagadr9,
    /// ? Error APASS DR9 B magnitude (eBmag_APASSDR9)
    e_bmagadr9,
    /// ? APASS DR9 V magnitude (Vmag_APASSDR9)
    vmagadr9,
    /// ? Error APASS DR9 V magnitude (eVmag_APASSDR9)
    e_vmagadr9,
    /// ? APASS DR9 g' magnitude (gpmag_APASSDR9)
    gpmagadr9,
    /// ? Error APASS DR9 g' magnitude (egpmag_APASSDR9)
    e_gpmagadr9,
    /// ? APASS DR9 r' magnitude (rpmag_APASSDR9)
    rpmagadr9,
    /// ? Error APASS DR9 r' magnitude (erpmag_APASSDR9)
    e_rpmagadr9,
    /// ? APASS DR9 i' magnitude (ipmag_APASSDR9)
    ipmagadr9,
    /// ? Error APASS DR9 i' magnitude (eipmag_APASSDR9)
    e_ipmagadr9,
    /// DENIS target designation (ID_DENIS)
    denis,
    /// ? Distance to DENIS source (Dist_DENIS)
    distd,
    /// Cross-match quality flag DENIS (MatchFlag_DENIS) (9)
    xd,
    /// ? DENIS I magnitude (Imag_DENIS)
    imagd,
    /// ? Error DENIS I magnitude (eImag_DENIS)
    e_imagd,
    /// ? DENIS J magnitude (Jmag_DENIS)
    jmagd,
    /// ? Error DENIS J magnitude (eJmag_DENIS)
    e_jmagd,
    /// ? DENIS K magnitude (Kmag_DENIS)
    kmagd,
    /// ? Error DENIS K magnitude (eKmag_DENIS)
    e_kmagd,
    /// USNO-B1 target designation (ID_USNOB1)
    usnob1,
    /// ? Distance to USNO-B1 source (Dist_USNOB1)
    distub1,
    /// Cross-match quality flag USNO-B1 (MatchFlag_USNOB1) (9)
    xub1,
    /// ? USNO-B1 B1 magnitude (B1mag_USNOB1)
    b1magub1,
    /// ? USNO-B1 R1 magnitude (R1mag_USNOB1)
    r1magub1,
    /// ? USNO-B1 B2 magnitude (B2mag_USNOB1)
    b2magub1,
    /// ? USNO-B1 R2 magnitude (R2mag_USNOB1)
    r2magub1,
    /// ? USNO-B1 I magnitude (Imag_USNOB1)
    imagub1,
    /// ? Proper motion RA from USNO-B1 (pmRA_USNOB1)
    pmraub1,
    /// ? Error on proper motion RA from USNO-B1
    e_pmraub1,
    /// ? Proper motion DE from USNO-B1 (pmDE_USNOB1)
    pmdeub1,
    /// ? Error on proper motion DE from USNO-B1
    e_pmdeub1,
    /// ! Observation date (Obsdate)
    obsdate,
    /// ! Name of RAVE field (FieldName)
    fieldname,
    /// [1/150] Fiber number (FiberNumber)
    fibernb,
    /// [1/3] Plate number (PlateNumber)
    platenb,
    /// Modified Julian date of observation (MJD_OBS)
    mjdobs,
    /// Exposure start in Local sideral time (LST_start)
    lststart,
    /// Exposure end in local sideral time (LST_end)
    lstend,
    /// Exposure start in UTC (UTC_start)
    utcstart,
    /// Exposure end in UTC (UTC_end)
    utcend,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the ravedr5_dr5 table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::name.to_string());
    col_strings.push(Col::healpix.to_string());
    col_strings.push(Col::rave.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::glon.to_string());
    col_strings.push(Col::glat.to_string());
    col_strings.push(Col::hrv.to_string());
    col_strings.push(Col::e_hrv.to_string());
    col_strings.push(Col::s_hrv.to_string());
    col_strings.push(Col::s_med_hrv.to_string());
    col_strings.push(Col::snrs.to_string());
    col_strings.push(Col::snrk.to_string());
    col_strings.push(Col::teffk.to_string());
    col_strings.push(Col::cteffk.to_string());
    col_strings.push(Col::e_teffk.to_string());
    col_strings.push(Col::s_med_teffk.to_string());
    col_strings.push(Col::s_teffk.to_string());
    col_strings.push(Col::loggk.to_string());
    col_strings.push(Col::cloggk.to_string());
    col_strings.push(Col::e_loggk.to_string());
    col_strings.push(Col::s_loggk.to_string());
    col_strings.push(Col::s_med_loggk.to_string());
    col_strings.push(Col::m_h_k.to_string());
    col_strings.push(Col::m_h_nk.to_string());
    col_strings.push(Col::e__m_h_k.to_string());
    col_strings.push(Col::s__m_h_k.to_string());
    col_strings.push(Col::s_med__m_h_k.to_string());
    col_strings.push(Col::chisqk.to_string());
    col_strings.push(Col::qk.to_string());
    col_strings.push(Col::teffir.to_string());
    col_strings.push(Col::e_teffir.to_string());
    col_strings.push(Col::irdirect.to_string());
    col_strings.push(Col::mg_h_c.to_string());
    col_strings.push(Col::o_mg_h_c.to_string());
    col_strings.push(Col::al_h_c.to_string());
    col_strings.push(Col::o_al_h_c.to_string());
    col_strings.push(Col::si_h_c.to_string());
    col_strings.push(Col::o_si_h_c.to_string());
    col_strings.push(Col::ti_h_c.to_string());
    col_strings.push(Col::o_ti_h_c.to_string());
    col_strings.push(Col::fe_h_c.to_string());
    col_strings.push(Col::o_fe_h_c.to_string());
    col_strings.push(Col::ni_h_c.to_string());
    col_strings.push(Col::o_ni_h_c.to_string());
    col_strings.push(Col::a_fe_c.to_string());
    col_strings.push(Col::chisqc.to_string());
    col_strings.push(Col::fracc.to_string());
    col_strings.push(Col::avschl.to_string());
    col_strings.push(Col::dist.to_string());
    col_strings.push(Col::e_dist.to_string());
    col_strings.push(Col::logav.to_string());
    col_strings.push(Col::e_logav.to_string());
    col_strings.push(Col::plx.to_string());
    col_strings.push(Col::e_plx.to_string());
    col_strings.push(Col::dm.to_string());
    col_strings.push(Col::e_dm.to_string());
    col_strings.push(Col::ffb.to_string());
    col_strings.push(Col::fqb.to_string());
    col_strings.push(Col::ngauss.to_string());
    col_strings.push(Col::gm1.to_string());
    col_strings.push(Col::gs1.to_string());
    col_strings.push(Col::gf1.to_string());
    col_strings.push(Col::gm2.to_string());
    col_strings.push(Col::gs2.to_string());
    col_strings.push(Col::gf2.to_string());
    col_strings.push(Col::gm3.to_string());
    col_strings.push(Col::gs3.to_string());
    col_strings.push(Col::gf3.to_string());
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
    col_strings.push(Col::repflag.to_string());
    col_strings.push(Col::clsflag.to_string());
    col_strings.push(Col::fpflag.to_string());
    col_strings.push(Col::tgas.to_string());
    col_strings.push(Col::ratgas.to_string());
    col_strings.push(Col::detgas.to_string());
    col_strings.push(Col::pmratgas.to_string());
    col_strings.push(Col::e_pmratgas.to_string());
    col_strings.push(Col::pmdetgas.to_string());
    col_strings.push(Col::e_pmdetgas.to_string());
    col_strings.push(Col::plxtgas.to_string());
    col_strings.push(Col::e_plxtgas.to_string());
    col_strings.push(Col::gmagtgas.to_string());
    col_strings.push(Col::gfluxtgas.to_string());
    col_strings.push(Col::e_gfluxtgas.to_string());
    col_strings.push(Col::hipparcos.to_string());
    col_strings.push(Col::tycho2.to_string());
    col_strings.push(Col::distt2.to_string());
    col_strings.push(Col::xt2.to_string());
    col_strings.push(Col::btmagt2.to_string());
    col_strings.push(Col::e_btmagt2.to_string());
    col_strings.push(Col::vtmagt2.to_string());
    col_strings.push(Col::e_vtmagt2.to_string());
    col_strings.push(Col::pmrat2.to_string());
    col_strings.push(Col::e_pmrat2.to_string());
    col_strings.push(Col::pmdet2.to_string());
    col_strings.push(Col::e_pmdet2.to_string());
    col_strings.push(Col::ucac4.to_string());
    col_strings.push(Col::distu4.to_string());
    col_strings.push(Col::xu4.to_string());
    col_strings.push(Col::pmrau4.to_string());
    col_strings.push(Col::e_pmrau4.to_string());
    col_strings.push(Col::pmdeu4.to_string());
    col_strings.push(Col::e_pmdeu4.to_string());
    col_strings.push(Col::ppmxl.to_string());
    col_strings.push(Col::distp.to_string());
    col_strings.push(Col::xp.to_string());
    col_strings.push(Col::pmrap.to_string());
    col_strings.push(Col::e_pmrap.to_string());
    col_strings.push(Col::pmdep.to_string());
    col_strings.push(Col::e_pmdep.to_string());
    col_strings.push(Col::col2mass.to_string());
    col_strings.push(Col::dist2.to_string());
    col_strings.push(Col::x2.to_string());
    col_strings.push(Col::jmag2.to_string());
    col_strings.push(Col::e_jmag2.to_string());
    col_strings.push(Col::hmag2.to_string());
    col_strings.push(Col::e_hmag2.to_string());
    col_strings.push(Col::kmag2.to_string());
    col_strings.push(Col::e_kmag2.to_string());
    col_strings.push(Col::allwise.to_string());
    col_strings.push(Col::distw.to_string());
    col_strings.push(Col::xw.to_string());
    col_strings.push(Col::w1magw.to_string());
    col_strings.push(Col::e_w1magw.to_string());
    col_strings.push(Col::w2magw.to_string());
    col_strings.push(Col::e_w2magw.to_string());
    col_strings.push(Col::w3magw.to_string());
    col_strings.push(Col::e_w3magw.to_string());
    col_strings.push(Col::w4magw.to_string());
    col_strings.push(Col::e_w4magw.to_string());
    col_strings.push(Col::ccflagsw.to_string());
    col_strings.push(Col::extflgw.to_string());
    col_strings.push(Col::varflgw.to_string());
    col_strings.push(Col::phqualw.to_string());
    col_strings.push(Col::distadr9.to_string());
    col_strings.push(Col::xadr9.to_string());
    col_strings.push(Col::bmagadr9.to_string());
    col_strings.push(Col::e_bmagadr9.to_string());
    col_strings.push(Col::vmagadr9.to_string());
    col_strings.push(Col::e_vmagadr9.to_string());
    col_strings.push(Col::gpmagadr9.to_string());
    col_strings.push(Col::e_gpmagadr9.to_string());
    col_strings.push(Col::rpmagadr9.to_string());
    col_strings.push(Col::e_rpmagadr9.to_string());
    col_strings.push(Col::ipmagadr9.to_string());
    col_strings.push(Col::e_ipmagadr9.to_string());
    col_strings.push(Col::denis.to_string());
    col_strings.push(Col::distd.to_string());
    col_strings.push(Col::xd.to_string());
    col_strings.push(Col::imagd.to_string());
    col_strings.push(Col::e_imagd.to_string());
    col_strings.push(Col::jmagd.to_string());
    col_strings.push(Col::e_jmagd.to_string());
    col_strings.push(Col::kmagd.to_string());
    col_strings.push(Col::e_kmagd.to_string());
    col_strings.push(Col::usnob1.to_string());
    col_strings.push(Col::distub1.to_string());
    col_strings.push(Col::xub1.to_string());
    col_strings.push(Col::b1magub1.to_string());
    col_strings.push(Col::r1magub1.to_string());
    col_strings.push(Col::b2magub1.to_string());
    col_strings.push(Col::r2magub1.to_string());
    col_strings.push(Col::imagub1.to_string());
    col_strings.push(Col::pmraub1.to_string());
    col_strings.push(Col::e_pmraub1.to_string());
    col_strings.push(Col::pmdeub1.to_string());
    col_strings.push(Col::e_pmdeub1.to_string());
    col_strings.push(Col::obsdate.to_string());
    col_strings.push(Col::fieldname.to_string());
    col_strings.push(Col::fibernb.to_string());
    col_strings.push(Col::platenb.to_string());
    col_strings.push(Col::mjdobs.to_string());
    col_strings.push(Col::lststart.to_string());
    col_strings.push(Col::lstend.to_string());
    col_strings.push(Col::utcstart.to_string());
    col_strings.push(Col::utcend.to_string());
    map.insert(ravedr5_dr5.string(), col_strings);
}
