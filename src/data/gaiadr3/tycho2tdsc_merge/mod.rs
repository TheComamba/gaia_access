// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the tycho2tdsc_merge table.

use crate::traits::{Column, Table};

/// Tycho-2 merged with the TDSC catalog and TDSC supplement.
///
/// The Tycho Double Star Catalogue, (TDSC, ) contains 98482 components of
/// double and multiple systems processed in the Tycho-2 context. It
/// includes either original Tycho-2 data or results from a dedicated
/// re-processing aimed at binaries. As pointed out by P. Marrese (personal
/// communication), the TDSC star with identifier 29583 is redundant and it
/// was therefore skipped. The TDSC supplement contains data from an
/// additional 4777 components from either Hipparcos or Tycho-1.
///
/// The Tycho-2 main catalogue  contains 2539913 sources, including many
/// binaries, but a minimum separation of 0.8 arcsec was imposed during the
/// catalogue construction. The Tycho-2 supplement-1 contains data for 17588
/// Hipparcos and Tycho-1 stars, which do not appear in the main catalogue,
/// but only the 4777 stars relevant for TDSC are included here. The Tycho-2
/// supplement-2 contains an additional 1146 Tycho-1 stars of poor quality.
///
/// We have merged Tycho-2 (main) with the TDSC main and supplement, keeping
/// the Tycho-2 data model and extending it to include TDSC–specific fields.
/// This is only possible to a certain degree as specified in detail below.
/// In particular, Tycho-2 includes fields for mean position, proper motion
/// etc. for a combination of Tycho-2 and several ground based catalogues.
/// These fields are inherited for TDSC main stars, but were not derived for
/// the TDSC supplement, where we only provide HIP proper motions. For
/// Tycho-2 stars not in TDSC, blank TDSC fields were appended.
///
/// For Tycho-2 stars in TDSC, the following fields were replaced by the
/// corresponding TDSC fields:
///
/// -   tyc1, tyc2, tyc3;
///
/// -   hip;
///
/// -   ccdm.
///
/// In addition, if the TDSC contains a new solution:
///
/// -   btMag, eBtMag, vtMag, eVtMag, raDeg, deDeg, epRa1990, epDe1990,
///     eRaDeg, eDeDec;
///
/// -   The mean position flag, pflag, is set to ‘P’ (“photocentre") if
///     resolved and there are Tycho-2 mean positions. Mean position
///     etc. are then repeated for both new components;
///
/// -   The proximity indicator, prox, is set to blank if the Tycho-2 star
///     was resolved in TDSC;
///
/// -   The type-of-solution flag, posflg, set to ‘N’;
///
/// -   For resolved Tycho-2 stars, two records are given.
///
/// For the few TDSC stars with new solutions, but not in Tycho-2, and for
/// the TDSC supplement stars (Hipparcos or Tycho-1), the Tycho-2 part of
/// the merged record was populated in the following way:
///
/// -   TYC1..3 from TDSC;
///
/// -   pflag set to TDSC pmflg (‘H’ or ‘X’), ‘H’ indicating Hip proper
///     motions;
///
/// -   no mean position and related fields;
///
/// -   proper motions only for Hipparcos stars;
///
/// -   $B_{\rm T}, V_{\rm T}$ photometry from TDSC;
///
/// -   Tycho-1 flag, tyc, is set to posflg, i.e. ‘T’ or ‘H’ although
///     Hipparcos stars may well be in Tycho-1;
///
/// -   Hipparcos, hip, and CCDM, ccdm, identifiers from TDSC;
///
/// -   astrometry from TDSC, but the (ra, dec)-correlation, corr, is set to
///     blank.
///
/// New TDSC specific fields added after the Tycho-2 part of the record:
///
/// -   sysNo, cmp, nMain, nSup, magflg, wds, note, hd;
///
/// -   rcmp, pa, sep, ePa, ePaSep, eSep.
#[allow(non_camel_case_types)]
pub struct tycho2tdsc_merge;

impl Table for tycho2tdsc_merge {
    fn string(&self) -> String {
        "tycho2tdsc_merge".to_string()
    }
}

/// The columns in the tycho2tdsc_merge table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Tycho-2 identifier
    id,
    /// Hipparcos number
    hip,
    /// TYC1 component from TYC or GSC
    tyc1,
    /// TYC2 component from TYC or GSC
    tyc2,
    /// TYC3 component from TYC or TDSC
    tyc3,
    /// Numeric Tycho-2 identifier
    id_tycho,
    /// Tycho-1 star
    tyc,
    /// Observed Tycho-2 Right Ascension, ICRS
    ra,
    /// Observed Tycho-2 Declination, ICRS
    dec,
    /// Observed Tycho-2 Right Ascension, ICRS
    ra_deg,
    /// Observed Tycho-2 Declination, ICRS
    de_deg,
    /// Mean Right Ascension, ICRS, epoch=J2000
    ra_mdeg,
    /// Mean Declination, ICRS, at epoch=J2000
    de_mdeg,
    /// Proper motion in RA*cos(dec)
    pm_ra,
    /// Proper motion in Dec
    pm_de,
    /// Epoch--1990 of raDeg
    ep_ra1990,
    /// Epoch--1990 of deDeg
    ep_de1990,
    /// Mean epoch of RA.
    ep_ra_m,
    /// Mean epoch of Dec.
    ep_de_m,
    /// Number of positions used for forming mean data
    num,
    /// Uncertainty RA*cos(dec), of observed Tycho-2 RA.
    e_ra_deg,
    /// Uncertainty of observed Tycho-2 Dec.
    e_de_deg,
    /// Correlation (RAdeg,DEdeg)
    corr,
    /// Uncertainty RA*cos(dec),at mean epoch.
    e_ra_mdeg,
    /// Uncertainty of Dec at mean epoch.
    e_de_mdeg,
    /// Uncertainty proper motion in RA*cos(dec).
    e_pm_ra,
    /// Uncertainty of proper motion in Dec.
    e_pm_de,
    /// Goodness of fit for mean RA
    q_ra_mdeg,
    /// Goodness of fit for mean Dec
    q_de_mdeg,
    /// Goodness of fit for pmDe
    q_pm_de,
    /// Goodness of fit for pmRa
    q_pm_ra,
    /// Mean position flag
    pflag,
    /// Type of Tycho-2 solution
    posflg,
    /// CCDM component identifier for HIP stars
    ccdm,
    /// Proximity indicator
    prox,
    /// Tycho-2 BT magnitude
    bt_mag,
    /// Tycho-2 VT magnitude
    vt_mag,
    /// Uncertainty of BT
    e_bt_mag,
    /// Uncertainty of VT
    e_vt_mag,
    /// TDSC identifier for the system
    sys_no,
    /// Component designation
    cmp,
    /// Number of components in TDSC main catalogue
    n_main,
    /// Number of components in the TDSC supplement
    n_sup,
    /// TDSC photometry flag
    magflg,
    /// WDS identifier for the system
    wds,
    /// TDSC notes
    note,
    /// HD identifier for TDSC entries
    hd,
    /// Reference component for position angle and separation
    rcmp,
    /// Position angle
    pa,
    /// Separation
    sep,
    /// Uncertainty of the position angle
    e_pa,
    /// Uncertainty of the position angle * separation
    e_pa_sep,
    /// Uncertainty of the separation
    e_sep,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the tycho2tdsc_merge table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::id.to_string());
    col_strings.push(Col::hip.to_string());
    col_strings.push(Col::tyc1.to_string());
    col_strings.push(Col::tyc2.to_string());
    col_strings.push(Col::tyc3.to_string());
    col_strings.push(Col::id_tycho.to_string());
    col_strings.push(Col::tyc.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::ra_deg.to_string());
    col_strings.push(Col::de_deg.to_string());
    col_strings.push(Col::ra_mdeg.to_string());
    col_strings.push(Col::de_mdeg.to_string());
    col_strings.push(Col::pm_ra.to_string());
    col_strings.push(Col::pm_de.to_string());
    col_strings.push(Col::ep_ra1990.to_string());
    col_strings.push(Col::ep_de1990.to_string());
    col_strings.push(Col::ep_ra_m.to_string());
    col_strings.push(Col::ep_de_m.to_string());
    col_strings.push(Col::num.to_string());
    col_strings.push(Col::e_ra_deg.to_string());
    col_strings.push(Col::e_de_deg.to_string());
    col_strings.push(Col::corr.to_string());
    col_strings.push(Col::e_ra_mdeg.to_string());
    col_strings.push(Col::e_de_mdeg.to_string());
    col_strings.push(Col::e_pm_ra.to_string());
    col_strings.push(Col::e_pm_de.to_string());
    col_strings.push(Col::q_ra_mdeg.to_string());
    col_strings.push(Col::q_de_mdeg.to_string());
    col_strings.push(Col::q_pm_de.to_string());
    col_strings.push(Col::q_pm_ra.to_string());
    col_strings.push(Col::pflag.to_string());
    col_strings.push(Col::posflg.to_string());
    col_strings.push(Col::ccdm.to_string());
    col_strings.push(Col::prox.to_string());
    col_strings.push(Col::bt_mag.to_string());
    col_strings.push(Col::vt_mag.to_string());
    col_strings.push(Col::e_bt_mag.to_string());
    col_strings.push(Col::e_vt_mag.to_string());
    col_strings.push(Col::sys_no.to_string());
    col_strings.push(Col::cmp.to_string());
    col_strings.push(Col::n_main.to_string());
    col_strings.push(Col::n_sup.to_string());
    col_strings.push(Col::magflg.to_string());
    col_strings.push(Col::wds.to_string());
    col_strings.push(Col::note.to_string());
    col_strings.push(Col::hd.to_string());
    col_strings.push(Col::rcmp.to_string());
    col_strings.push(Col::pa.to_string());
    col_strings.push(Col::sep.to_string());
    col_strings.push(Col::e_pa.to_string());
    col_strings.push(Col::e_pa_sep.to_string());
    col_strings.push(Col::e_sep.to_string());
    map.insert(tycho2tdsc_merge.string(), col_strings);
}
