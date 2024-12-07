// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the skymapperdr2_master table.

use crate::traits::{Column, Table};

/// SkyMapper Data Release 2 - MAIN TABLE: Primary table of mean astrometric, photometric and shape measurements per object, with cross-match information to external tables.
///
/// Reference: Onken et al. 2019 PASA, 36, 33O.
///
/// Data replicated from dr2.master table NCI bulk download files and TAP metadata as of April 2020.
#[allow(non_camel_case_types)]
pub struct skymapperdr2_master;

impl Table for skymapperdr2_master {
    fn string(&self) -> String {
        "skymapperdr2_master".to_string()
    }
}

/// The columns in the skymapperdr2_master table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Global unique object ID in the master table.
    object_id,
    /// Mean ICRS Right Ascension of the object
    raj2000,
    /// Mean ICRS Declination of the object
    dej2000,
    /// RMS variation around the mean Right Ascension, in milliarcseconds
    e_raj2000,
    /// RMS variation around the mean Declination, in milliarcseconds
    e_dej2000,
    /// SkyMapper Southern Survey designation of the form SMSS Jhhmmss.ss+/-ddmmss.s, derived from mean ICRS coordinates
    smss_j,
    /// Mean MJD epoch of the observations
    mean_epoch,
    /// RMS variation around the mean epoch
    rms_epoch,
    /// Galactic longitude derived from ICRS coordinates. Not to be used as primary astrometric reference.
    glon,
    /// Galactic latitude derived from ICRS coordinates. Not to be used as primary astrometric reference.
    glat,
    /// Bitwise OR of Source Extractor flags across all observations
    flags,
    /// Total number of flagged pixels from bad, saturated, and crosstalk pixel masks across all observations
    nimaflags,
    /// Number of observations used across all filters
    ngood,
    /// Minimum number of observations used in any filter (excluding filters with 0 observations)
    ngood_min,
    /// Maximum number of child sources combined into this global object_id in any filter
    nch_max,
    /// Number of DR2 sources within 15 arcsec (including this source)
    density,
    /// Bitwise OR of Source Extractor flags from u-band measurements in photometry table
    u_flags,
    /// Number of flagged pixels from bad, saturated, and crosstalk pixel masks from u-band measurements in photometry table
    u_nimaflags,
    /// Number of u-band observations used
    u_ngood,
    /// Number of u-band child sources combined into this object_id
    u_nch,
    /// Number of u-band observations with magnitudes clipped from the final PSF magnitude estimate
    u_nclip,
    /// Bitwise OR of Source Extractor flags from v-band measurements in photometry table
    v_flags,
    /// Number of flagged pixels from bad, saturated, and crosstalk pixel masks from v-band measurements in photometry table
    v_nimaflags,
    /// Number of v-band observations used
    v_ngood,
    /// Number of v-band child sources combined into this object_id
    v_nch,
    /// Number of v-band observations with magnitudes clipped from the final PSF magnitude estimate
    v_nclip,
    /// Bitwise OR of Source Extractor flags from g-band measurements in photometry table
    g_flags,
    /// Number of flagged pixels from bad, saturated, and crosstalk pixel masks from g-band measurements in photometry table
    g_nimaflags,
    /// Number of g-band observations used
    g_ngood,
    /// Number of g-band child sources combined into this object_id
    g_nch,
    /// Number of g-band observations with magnitudes clipped from the final PSF magnitude estimate
    g_nclip,
    /// Bitwise OR of Source Extractor flags from r-band measurements in photometry table
    r_flags,
    /// Number of flagged pixels from bad, saturated, and crosstalk pixel masks from r-band measurements in photometry table
    r_nimaflags,
    /// Number of r-band observations used
    r_ngood,
    /// Number of r-band child sources combined into this object_id
    r_nch,
    /// Number of r-band observations with magnitudes clipped from the final PSF magnitude estimate
    r_nclip,
    /// Bitwise OR of Source Extractor flags from i-band measurements in photometry table
    i_flags,
    /// Number of flagged pixels from bad, saturated, and crosstalk pixel masks from i-band measurements in photometry table
    i_nimaflags,
    /// Number of i-band observations used
    i_ngood,
    /// Number of i-band child sources combined into this object_id
    i_nch,
    /// Number of i-band observations with magnitudes clipped from the final PSF magnitude estimate
    i_nclip,
    /// Bitwise OR of Source Extractor flags from z-band measurements in photometry table
    z_flags,
    /// Number of flagged pixels from bad, saturated, and crosstalk pixel masks from z-band measurements in photometry table
    z_nimaflags,
    /// Number of z-band observations used
    z_ngood,
    /// Number of z-band child sources combined into this object_id
    z_nch,
    /// Number of z-band observations with magnitudes clipped from the final PSF magnitude estimate
    z_nclip,
    /// Maximum stellarity index from photometry table (between 0=no star and 1=star)
    class_star,
    /// Bitmask indicating whether photometry is likely biased by neighbours at >1%; bits 0-5 correspond to filters z-u
    flags_psf,
    /// Mean r-band Petrosian radius
    radius_petro,
    /// Mean u-band PSF magnitude
    u_psf,
    /// Error in u-band PSF magnitude
    e_u_psf,
    /// Reduced chi-squared for a constant-magnitude model of the u-band PSF magnitude, including clipped sources
    u_rchi2var,
    /// Mean u-band Petrosian magnitude
    u_petro,
    /// Error in u-band Petrosian magnitude
    e_u_petro,
    /// Mean v-band PSF magnitude
    v_psf,
    /// Error in v-band PSF magnitude
    e_v_psf,
    /// Reduced chi-squared for a constant-magnitude model of the v-band PSF magnitude, including clipped sources
    v_rchi2var,
    /// Mean v-band Petrosian magnitude
    v_petro,
    /// Error in v-band Petrosian magnitude
    e_v_petro,
    /// Mean g-band PSF magnitude
    g_psf,
    /// Error in g-band PSF magnitude
    e_g_psf,
    /// Reduced chi-squared for a constant-magnitude model of the g-band PSF magnitude, including clipped sources
    g_rchi2var,
    /// Mean g-band Petrosian magnitude
    g_petro,
    /// Error in g-band Petrosian magnitude
    e_g_petro,
    /// Mean r-band PSF magnitude
    r_psf,
    /// Error in r-band PSF magnitude
    e_r_psf,
    /// Reduced chi-squared for a constant-magnitude model of the r-band PSF magnitude, including clipped sources
    r_rchi2var,
    /// Mean r-band Petrosian magnitude
    r_petro,
    /// Error in r-band Petrosian magnitude
    e_r_petro,
    /// Mean i-band PSF magnitude
    i_psf,
    /// Error in i-band PSF magnitude
    e_i_psf,
    /// Reduced chi-squared for a constant-magnitude model of the i-band PSF magnitude, including clipped sources
    i_rchi2var,
    /// Mean i-band Petrosian magnitude
    i_petro,
    /// Error in i-band Petrosian magnitude
    e_i_petro,
    /// Mean z-band PSF magnitude
    z_psf,
    /// Error in z-band PSF magnitude
    e_z_psf,
    /// Reduced chi-squared for a constant-magnitude model of the z-band PSF magnitude, including clipped sources
    z_rchi2var,
    /// Mean z-band Petrosian magnitude
    z_petro,
    /// Error in z-band Petrosian magnitude
    e_z_petro,
    /// E(B-V) from Schlegel+1998 extinction maps at the ICRS coordinates
    ebmv_sfd,
    /// Distance to next-closest DR2 source within 15 arcsec (or 15 if no such sources)
    prox,
    /// object_id of next-closest DR2 source
    prox_id,
    /// object_id of closest SkyMapper Data Release 1 source
    dr1_id,
    /// Distance to closest SkyMapper Data Release 1 source
    dr1_dist,
    /// Unique identifier (pts_key) of closest 2MASS PSC source
    twomass_key,
    /// Distance on sky to closest 2MASS PSC source
    twomass_dist,
    /// Unique identifier (cntr) of closest AllWISE source
    allwise_cntr,
    /// Distance on sky to closest AllWISE source
    allwise_dist,
    /// Unique identifier (mpos) of closest UCAC4 source
    ucac4_mpos,
    /// Distance on sky to closest UCAC4 source
    ucac4_dist,
    /// Unique identifier (objid) of closest ATLAS Refcat2 source (only useful for this copy of Refcat2)
    refcat2_id,
    /// Distance on sky to closest ATLAS Refcat2 source
    refcat2_dist,
    /// Unique identifier (objid) of closest Pan-STARRS1 DR1 source
    ps1_dr1_id,
    /// Distance on sky to closest Pan-STARRS1 DR1 source
    ps1_dr1_dist,
    /// Unique identifier (objid) of closest GALEX GUVcat AIS source (Bianchi et al. 2017)
    galex_guv_id,
    /// Distance on sky to closest GALEX GUVcat AIS source
    galex_guv_dist,
    /// Unique identifier (source_id) of closest Gaia DR2 source
    gaia_dr2_id1,
    /// Distance on sky to closest Gaia DR2 source
    gaia_dr2_dist1,
    /// Unique identifier (source_id) of second closest Gaia DR2 source
    gaia_dr2_id2,
    /// Distance on sky to second closest Gaia DR2 source
    gaia_dr2_dist2,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the skymapperdr2_master table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::object_id.to_string());
    col_strings.push(Col::raj2000.to_string());
    col_strings.push(Col::dej2000.to_string());
    col_strings.push(Col::e_raj2000.to_string());
    col_strings.push(Col::e_dej2000.to_string());
    col_strings.push(Col::smss_j.to_string());
    col_strings.push(Col::mean_epoch.to_string());
    col_strings.push(Col::rms_epoch.to_string());
    col_strings.push(Col::glon.to_string());
    col_strings.push(Col::glat.to_string());
    col_strings.push(Col::flags.to_string());
    col_strings.push(Col::nimaflags.to_string());
    col_strings.push(Col::ngood.to_string());
    col_strings.push(Col::ngood_min.to_string());
    col_strings.push(Col::nch_max.to_string());
    col_strings.push(Col::density.to_string());
    col_strings.push(Col::u_flags.to_string());
    col_strings.push(Col::u_nimaflags.to_string());
    col_strings.push(Col::u_ngood.to_string());
    col_strings.push(Col::u_nch.to_string());
    col_strings.push(Col::u_nclip.to_string());
    col_strings.push(Col::v_flags.to_string());
    col_strings.push(Col::v_nimaflags.to_string());
    col_strings.push(Col::v_ngood.to_string());
    col_strings.push(Col::v_nch.to_string());
    col_strings.push(Col::v_nclip.to_string());
    col_strings.push(Col::g_flags.to_string());
    col_strings.push(Col::g_nimaflags.to_string());
    col_strings.push(Col::g_ngood.to_string());
    col_strings.push(Col::g_nch.to_string());
    col_strings.push(Col::g_nclip.to_string());
    col_strings.push(Col::r_flags.to_string());
    col_strings.push(Col::r_nimaflags.to_string());
    col_strings.push(Col::r_ngood.to_string());
    col_strings.push(Col::r_nch.to_string());
    col_strings.push(Col::r_nclip.to_string());
    col_strings.push(Col::i_flags.to_string());
    col_strings.push(Col::i_nimaflags.to_string());
    col_strings.push(Col::i_ngood.to_string());
    col_strings.push(Col::i_nch.to_string());
    col_strings.push(Col::i_nclip.to_string());
    col_strings.push(Col::z_flags.to_string());
    col_strings.push(Col::z_nimaflags.to_string());
    col_strings.push(Col::z_ngood.to_string());
    col_strings.push(Col::z_nch.to_string());
    col_strings.push(Col::z_nclip.to_string());
    col_strings.push(Col::class_star.to_string());
    col_strings.push(Col::flags_psf.to_string());
    col_strings.push(Col::radius_petro.to_string());
    col_strings.push(Col::u_psf.to_string());
    col_strings.push(Col::e_u_psf.to_string());
    col_strings.push(Col::u_rchi2var.to_string());
    col_strings.push(Col::u_petro.to_string());
    col_strings.push(Col::e_u_petro.to_string());
    col_strings.push(Col::v_psf.to_string());
    col_strings.push(Col::e_v_psf.to_string());
    col_strings.push(Col::v_rchi2var.to_string());
    col_strings.push(Col::v_petro.to_string());
    col_strings.push(Col::e_v_petro.to_string());
    col_strings.push(Col::g_psf.to_string());
    col_strings.push(Col::e_g_psf.to_string());
    col_strings.push(Col::g_rchi2var.to_string());
    col_strings.push(Col::g_petro.to_string());
    col_strings.push(Col::e_g_petro.to_string());
    col_strings.push(Col::r_psf.to_string());
    col_strings.push(Col::e_r_psf.to_string());
    col_strings.push(Col::r_rchi2var.to_string());
    col_strings.push(Col::r_petro.to_string());
    col_strings.push(Col::e_r_petro.to_string());
    col_strings.push(Col::i_psf.to_string());
    col_strings.push(Col::e_i_psf.to_string());
    col_strings.push(Col::i_rchi2var.to_string());
    col_strings.push(Col::i_petro.to_string());
    col_strings.push(Col::e_i_petro.to_string());
    col_strings.push(Col::z_psf.to_string());
    col_strings.push(Col::e_z_psf.to_string());
    col_strings.push(Col::z_rchi2var.to_string());
    col_strings.push(Col::z_petro.to_string());
    col_strings.push(Col::e_z_petro.to_string());
    col_strings.push(Col::ebmv_sfd.to_string());
    col_strings.push(Col::prox.to_string());
    col_strings.push(Col::prox_id.to_string());
    col_strings.push(Col::dr1_id.to_string());
    col_strings.push(Col::dr1_dist.to_string());
    col_strings.push(Col::twomass_key.to_string());
    col_strings.push(Col::twomass_dist.to_string());
    col_strings.push(Col::allwise_cntr.to_string());
    col_strings.push(Col::allwise_dist.to_string());
    col_strings.push(Col::ucac4_mpos.to_string());
    col_strings.push(Col::ucac4_dist.to_string());
    col_strings.push(Col::refcat2_id.to_string());
    col_strings.push(Col::refcat2_dist.to_string());
    col_strings.push(Col::ps1_dr1_id.to_string());
    col_strings.push(Col::ps1_dr1_dist.to_string());
    col_strings.push(Col::galex_guv_id.to_string());
    col_strings.push(Col::galex_guv_dist.to_string());
    col_strings.push(Col::gaia_dr2_id1.to_string());
    col_strings.push(Col::gaia_dr2_dist1.to_string());
    col_strings.push(Col::gaia_dr2_id2.to_string());
    col_strings.push(Col::gaia_dr2_dist2.to_string());
    map.insert(skymapperdr2_master.string(), col_strings);
}
