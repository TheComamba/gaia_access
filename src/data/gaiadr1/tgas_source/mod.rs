// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the tgas_source table.

use crate::traits::{Column, Table};

/// This table is a subset of GaiaSource comprising those stars in the
/// Hipparcos and Tycho-2 Catalogues for which a full 5-parameter
/// astrometric solution has been possible in Gaia Data Release 1. This is
/// possible because the early Hipparcos epoch positions break some
/// degeneracies due to the limited Gaia time coverage.
///
/// This table contains a substantial fraction of the around 2.5 million
/// stars in the Hipparcos and Tycho-2 catalogue. Many stars have been
/// excluded due to several reasons, such as saturation, cross-match errors
/// or bad astrometric solution.
#[allow(non_camel_case_types)]
pub struct tgas_source;

impl Table for tgas_source {
    fn string(&self) -> String {
        "tgas_source".to_string()
    }
}

/// The columns in the tgas_source table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// Hipparcos identifier (this field will be empty if the source was not in
    /// the Hipparcos catalogue).
    hip,
    /// Tycho 2 identifier. The TYC identifier is constructed from the GSC
    /// region number (TYC1), the running number within the region (TYC2) and a
    /// component identifier (TYC3) which is normally 1. Some non-GSC running
    /// numbers were constructed for the first Tycho Catalogue and for Tycho-2.
    /// The recommended star designation contains a hyphen between the TYC
    /// numbers, e.g. TYC 1-13-1.To see if Hipparcos or Tycho measurements where
    /// used as a prior constraint in the astrometic solution for a given source
    /// please see astrometricPriorsUsed.
    tycho2_id,
    /// All Gaia data processed by the Data Processing and Analysis Consortium
    /// comes tagged with a solution identifier. This is a numeric field
    /// attached to each table row that can be used to unequivocally identify
    /// the version of all the subsystems that where used in the generation of
    /// the data as well as the input data used. It is mainly for internal DPAC
    /// use but is included in the published data releases to enable end users
    /// to examine the provenance of processed data products. To decode a given
    /// solution ID visit
    solution_id,
    /// A unique single numerical identifier of the source.
    ///
    /// For the contents of Gaia DR1, which does not include Solar System
    /// objects, the source ID consists consists of a 64-bit integer, least
    /// significant bit = 1 and most significant bit = 64, comprising:
    ///
    /// -   a HEALPix index number (sky pixel) in bits 36 - 63; by definition
    ///     the smallest HEALPix index number is zero.
    ///
    /// -   a 2-bit Data Processing Centre code in bits 34 - 35; for example
    ///     MOD(sourceId / 4294967296, 8) can be used to distinguish between
    ///     sources initialised via the Initial Gaia Source List by the Torino
    ///     DPC (code = 0) and sources otherwise detected and assigned by Gaia
    ///     observations (code >0)
    ///
    /// -   a 25-bit plus 7 bit sequence number within the HEALPix pixel in bits
    ///     1 - 32 split into:
    ///
    ///     -   a 25 bit running number in bits 8 – 32; the running numbers are
    ///         defined to be positive, i.e. never zero (except in the case of
    ///         forced empty windows)
    ///
    ///     -   a 7-bit component number in bits 1 – 7
    ///
    /// -   one spare bit in bit 33
    ///
    /// This means that the HEALpix index level 12 of a given source is
    /// contained in the most significant bits. HEALpix index of 12 and lower
    /// levels can thus be retrieved as follows:
    ///
    /// -   HEALpix level 12 = source_id / 34359738368
    ///
    /// -   HEALpix level 11 = source_id / 137438953472
    ///
    /// -   HEALpix level 10 = source_id / 549755813888
    ///
    /// -   HEALpix level n = source_id / 2 ^ 35 * 4 ^ (12 - level).
    ///
    /// Additional details can be found in the Gaia DPAC public document Source
    /// Identifiers — Assignment and Usage throughout DPAC (document code
    /// GAIA–C3–TN–ARI–BAS–020) available from
    source_id,
    /// Random index which can be used to select smaller subsets of the data
    /// that are still representative. The column contains a random permutation
    /// of the numbers from 0 to N-1, where N is the number of rows.
    ///
    /// The random index can be useful for validation (testing on 10 different
    /// random subsets), visualization (displaying 1% of the data), and
    /// statistical exploration of the data, without the need to download all
    /// the data.
    random_index,
    /// Reference epoch to which the astrometic source parameters are referred,
    /// expressed as a Julian Year in TCB.
    ref_epoch,
    /// Barycentric right ascension \alpha of the source in ICRS at the
    /// reference epoch refEpoch
    ra,
    /// Standard error \sigma_{\alpha *} \equiv \sigma_\alpha\cos\delta of the
    /// right ascension of the source in ICRS at the reference epoch refEpoch.
    ra_error,
    /// Barycentric declination \delta of the source in ICRS at the reference
    /// epoch refEpoch
    dec,
    /// Standard error \sigma_\delta of the declination of the source in ICRS at
    /// the reference epoch refEpoch
    dec_error,
    /// Absolute barycentric stellar parallax \varpi of the soure at the
    /// reference epoch refEpoch
    parallax,
    /// Standard error \sigma_\varpi of the stellar parallax at the reference
    /// epoch refEpoch
    parallax_error,
    /// Proper motion in right ascension \mu_{\alpha *} of the source in ICRS at
    /// the reference epoch refEpoch. This is the projection of the proper
    /// motion vector in the direction of increasing right ascension.
    pmra,
    /// Standard error \sigma_{\mu\alpha *} of the proper motion vector in right
    /// ascension at the reference epoch refEpoch
    pmra_error,
    /// Proper motion in declination \mu_\delta of the source at the reference
    /// epoch refEpoch. This is the projection of the proper motion vector in
    /// the direction of increasing declination.
    pmdec,
    /// Standard error \sigma_{\mu\delta} of the proper motion in declination at
    /// the reference epoch refEpoch
    pmdec_error,
    /// Correlation between right ascension and declination, in dimensionless
    /// units [-1:+1]
    ra_dec_corr,
    /// Correlation between right ascension and parallax, in dimensionless units
    /// [-1:+1]
    ra_parallax_corr,
    /// Correlation between right ascension and proper motion in right
    /// ascension, in dimensionless units [-1:+1]
    ra_pmra_corr,
    /// Correlation between right ascension and proper motion in declination, in
    /// dimensionless units [-1:+1]
    ra_pmdec_corr,
    /// Correlation between declination and parallax, in dimensionless units
    /// [-1:+1]
    dec_parallax_corr,
    /// Correlation between declination and proper motion in right ascension, in
    /// dimensionless units [-1:+1]
    dec_pmra_corr,
    /// Correlation between declination and proper motion in declination, in
    /// dimensionless units [-1:+1]
    dec_pmdec_corr,
    /// Correlation between parallax and proper motion in right ascension, in
    /// dimensionless units [-1:+1]
    parallax_pmra_corr,
    /// Correlation between parallax and proper motion in declination, in
    /// dimensionless units [-1:+1]
    parallax_pmdec_corr,
    /// Correlation between proper motion in right ascension and proper motion
    /// in declination, in dimensionless units [-1:+1]
    pmra_pmdec_corr,
    /// Total number of AL observations (= CCD transits) used in the astrometric
    /// solution of the source, independent of their weight. Note that some
    /// observations may be strongly downweighted (see astrometricNBadObsAl).
    astrometric_n_obs_al,
    /// Total number of AC observations (= CCD transits) used in the astrometric
    /// solution of the source, independent of their weight. Note that some
    /// observations may be strongly downweighted (see astrometricNBadObsAc).
    /// Nearly all sources having G <13 will have AC observations from 2d
    /// windows, while fainter than that limit only \sim1% of stars (the
    /// so–called ‘calibration faint stars’) are assigned 2d windows resulting
    /// in AC observations.
    astrometric_n_obs_ac,
    /// Number of AL observations (= CCD transits) that were not strongly
    /// downweighted in the astrometric solution of the source. Strongly
    /// downweighted observations (with downweighting factor w<0.2) are instead
    /// counted in astrometricNBadObsAl. The sum of astrometricNGoodObsAl and
    /// astrometricNBadObsAl equals astrometricNObsAl, the total number of AL
    /// observations used in the astrometric solution of the source.
    astrometric_n_good_obs_al,
    /// Number of AC observations (= CCD transits) that were not strongly
    /// downweighted in the astrometric solution of the source. Strongly
    /// downweighted observations (with downweighting factor w<0.2) are instead
    /// counted in astrometricNBadObsAc. The sum of astrometricNGoodObsAc and
    /// astrometricNBadObsAc equals astrometricNObsAc, the total number of AC
    /// observations used in the astrometric solution of the source.
    astrometric_n_good_obs_ac,
    /// Number of AL observations (= CCD transits) that were strongly
    /// downweighted in the astrometric solution of the source, and therefore
    /// contributed little to the determination of the astrometric parameters.
    /// An observation is considered to be strongly downweighted if its
    /// downweighting factor w<0.2, which means that the absolute value of the
    /// astrometric residual exceeds 4.83 times the total uncertainty of the
    /// observation, calculated as the quadratic sum of the centroiding
    /// uncertainty, excess source noise, and excess attitude noise.
    astrometric_n_bad_obs_al,
    /// Number of AC observations (= CCD transits) that were strongly
    /// downweighted in the astrometric solution of the source, and therefore
    /// contributed little to the determination of the astrometric parameters.
    /// An observation is considered to be strongly downweighted if its
    /// downweighting factor w<0.2, which means that the absolute value of the
    /// astrometric residual exceeds 4.83 times the total uncertainty of the
    /// observation, calculated as the quadratic sum of the centroiding
    /// uncertainty, excess source noise, and excess attitude noise.
    astrometric_n_bad_obs_ac,
    /// In the TGAS solution \tt astrometricDeltaQ (\Delta Q) indicates the
    /// discrepancy between the Hipparcos proper motion and the TGAS proper
    /// motion. A large value of \tt deltaQ could indicate non-linear motion
    /// (e.g. in a binary).
    ///
    /// The precise definition is
    ///
    /// \Delta Q =
    /// \begin{bmatrix}
    /// \Delta\mu_{\alpha *} & \Delta\mu_{\delta}
    /// \end{bmatrix}
    /// \left(\vec{C}_\text{pm,\,T}+\vec{C}_\text{pm,\,H}\right)^{-1}
    /// \begin{bmatrix}
    /// \Delta\mu_{\alpha *} \\ \Delta\mu_{\delta}
    /// \end{bmatrix}
    ///
    /// where \Delta\mu_{\alpha *} = \mu_{\alpha *,\rm T}-\mu_{\alpha *,\rm H},
    /// \Delta\mu_{\delta} = \mu_{\delta,\rm T}-\mu_{\delta,\rm H}, with T and H
    /// indicating values from the Gaia DR1 (TGAS) solution and Hipparcos
    /// catalogue. \vec{C}_\text{pm,\,T} and \vec{C}_\text{pm,\,H} are the
    /// corresponding 2\times 2 covariance matrices.
    ///
    /// In order to compute \Delta Q the two sets of proper motions must use the
    /// same reference frame and the same reference epoch. Thus, the proper
    /// motion components as given in the Hipparcos catalogue were rotated to
    /// the Gaia DR1 reference frame, and then propagated to the Gaia reference
    /// epoch.
    astrometric_delta_q,
    /// This is the excess noise \epsilon_i of the source. It measures the
    /// disagreement, expressed as an angle, between the observations of a
    /// source and the best-fitting standard astrometric model (using five
    /// astrometric parameters). The assumed observational noise in each
    /// observation is quadratically increased by \epsilon_i in order to
    /// statistically match the residuals in the astrometric solution. A value
    /// of 0 signifies that the source is astrometrically well-behaved, i.e.
    /// that the residuals of the fit statistically agree with the assumed
    /// observational noise. A positive value signifies that the residuals are
    /// statistically larger than expected.
    ///
    /// The significance of \epsilon_i is given by \tt
    /// astrometricExcessNoiseSig (D). If D\le 2 then \epsilon_i is probably not
    /// significant, and the source may be astrometrically well-behaved even if
    /// \epsilon_i is large.
    ///
    /// The excess noise \epsilon_i may absorb all kinds of modelling errors
    /// that are not accounted for by the observational noise (image centroiding
    /// error) or the excess attitude noise. Such modelling errors include LSF
    /// and PSF calibration errors, geometric instrument calibration errors, and
    /// part of the high-frequency attitude noise. These modelling errors are
    /// particularly important in the early data releases, but should decrease
    /// as the astrometric modelling of the instrument and attitude improves
    /// over the years.
    ///
    /// Additionally, sources that deviate from the standard five-parameter
    /// astrometric model (e.g. unresolved binaries, exoplanet systems, etc.)
    /// may have positive \epsilon_i. Given the many other possible
    /// contributions to the excess noise, the user must study the empirical
    /// distributions of \epsilon_i and D to make sensible cutoffs before
    /// filtering out sources for their particular application.
    ///
    /// In Gaia DR1, the excess source noise has the same interpretation as
    /// described above for both the primary (TGAS) and secondary data sets. It
    /// measures the disagreement between the five-parameter model and the
    /// observations, augmented by the different priors used. Thus, in TGAS the
    /// excess noise may be increased if the proper motion seen during the 14
    /// months of Gaia observations are not in agreement with the proper motion
    /// inferred from the Tycho-2/Gaia comparison. In the secondary solution the
    /// excess noise may be increased if the Gaia observations indicate a proper
    /// motion or parallax several times larger than the prior uncertainty.
    ///
    /// The excess source noise is further explained in Sects. 3.6 and 5.1.2 of
    /// Lindegren et al. (2012).
    ///
    /// Lindegren, L., U. Lammers, D. Hobbs, O’MullaneW., U. Bastian, and J.
    /// Hernandez. 2012. “The Astrometric Core Solution for the Gaia Mission.
    /// Overview of Models, Algorithms, and Software Implementation.” Astronomy
    /// and Astrophysics 538 (February).
    astrometric_excess_noise,
    /// A dimensionless measure (D) of the significance of the calculated \tt
    /// astrometricExcessNoise (\epsilon_i). A value D>2 indicates that the
    /// given \epsilon_i is probably significant.
    ///
    /// For good fits in the limit of a large number of observations, D should
    /// be zero in half of the cases and approximately follow the positive half
    /// of a normal distribution with zero mean and unit standard deviation for
    /// the other half. Consequently, D is expected to be greater than 2 for
    /// only a few percent of the sources with well-behaved astrometric
    /// solutions.
    ///
    /// In the early data releases \epsilon_i will however include instrument
    /// and attitude modelling errors that are statistically significant and
    /// could result in large values of \epsilon_i and D. The user must study
    /// the empirical distributions of these statistics and make sensible
    /// cutoffs before filtering out sources for their particular application.
    ///
    /// The excess noise significance is further explained in Sect. 5.1.2 of
    /// Lindegren et al. (2012).
    ///
    /// Lindegren, L., U. Lammers, D. Hobbs, O’MullaneW., U. Bastian, and J.
    /// Hernandez. 2012. “The Astrometric Core Solution for the Gaia Mission.
    /// Overview of Models, Algorithms, and Software Implementation.” Astronomy
    /// and Astrophysics 538 (February).
    astrometric_excess_noise_sig,
    /// Flag indicating if this source was used as a primary source (\tt true)
    /// or secondary source (\tt false). Only primary sources contribute to the
    /// estimation of attitude, calibration, and global parameters. The
    /// estimation of source parameters is otherwise done in exactly the same
    /// way for primary and secondary sources.
    astrometric_primary_flag,
    /// Relegation factor of the source calculated as per Eq. (118) in Lindegren
    /// et al. (2012) used for the primary selection process.
    ///
    /// Lindegren, L., U. Lammers, D. Hobbs, O’MullaneW., U. Bastian, and J.
    /// Hernandez. 2012. “The Astrometric Core Solution for the Gaia Mission.
    /// Overview of Models, Algorithms, and Software Implementation.” Astronomy
    /// and Astrophysics 538 (February).
    astrometric_relegation_factor,
    /// Mean astrometric weight of the source in the AL direction.
    ///
    /// The mean astrometric weight of the source is calculated as per Eq. (119)
    /// in Lindegren et al. (2012).
    ///
    /// Lindegren, L., U. Lammers, D. Hobbs, O’MullaneW., U. Bastian, and J.
    /// Hernandez. 2012. “The Astrometric Core Solution for the Gaia Mission.
    /// Overview of Models, Algorithms, and Software Implementation.” Astronomy
    /// and Astrophysics 538 (February).
    astrometric_weight_al,
    /// Mean astrometric weight of the source in the AC direction
    ///
    /// The mean astrometric weight of the source is calculated as per Eq. (119)
    /// in Lindegren et al. (2012).
    ///
    /// Lindegren, L., U. Lammers, D. Hobbs, O’MullaneW., U. Bastian, and J.
    /// Hernandez. 2012. “The Astrometric Core Solution for the Gaia Mission.
    /// Overview of Models, Algorithms, and Software Implementation.” Astronomy
    /// and Astrophysics 538 (February).
    astrometric_weight_ac,
    /// Type of prior used in the astrometric solution:
    ///
    /// -   0: No prior used
    ///
    /// -   1: Galaxy Bayesian Prior for parallax and proper motion
    ///
    /// -   2: Galaxy Bayesian Prior for parallax and proper motion relaxed by
    ///     factor 10
    ///
    /// -   3: Hipparcos prior for position
    ///
    /// -   4: Hipparcos prior for position and proper motion
    ///
    /// -   5: Tycho2 prior for position
    ///
    /// -   6: Quasar prior for proper motion
    ///
    /// The Galaxy Bayesian Prior is defined in , where it is denoted
    /// \sigma_{\varpi,F90} (for the parallax) and
    /// \sigma_{\mu,F90}={\cal R}\sigma_{\varpi,F90}, with {\cal
    /// R}=10 yr^{-1} (for proper motion). The Galaxy Bayesian Prior relaxed by
    /// a factor 10 is 10\sigma_{\varpi,F90} and 10\sigma_{\mu,F90},
    /// respectively.
    ///
    /// For Gaia DR1 the only types of priors used are 2 (for the secondary data
    /// set), 3 (for the Hipparcos subset of the primary data set), or 5 (for
    /// the non-Hipparcos subset of the primary data set). Type 6 was used for
    /// internal calibration purposes and alignment of the reference frame, but
    /// the corresponding astrometric results are in general not published.
    astrometric_priors_used,
    /// This field indicates the number of observations (detection transits)
    /// that have been matched to a given source during the last internal
    /// crossmatch revision.
    matched_observations,
    /// During data processing, this source happened to been duplicated and one
    /// source only has been kept. This may indicate observational,
    /// cross-matching or processing problems, or stellar multiplicity, and
    /// probable astrometric or photometric problems in all cases. In DR1, for
    /// close doubles with separations below some 2 arcsec, truncated windows
    /// have not been processed, neither in astrometry and photometry. The
    /// transmitted window is centred on the brighter part of the acquired
    /// window, so the brighter component has a better chance to be selected,
    /// even when processing the fainter transit. If more than two images are
    /// contained in a window, the result of the image parameter determination
    /// is un-predictable in the sense that it might refer to either (or
    /// neither) image, and no consistency is assured.
    duplicated_source,
    /// The scanDirectionStrength and scanDirectionMean quantify the
    /// distribution of AL scan directions across the source.
    /// scanDirectionStrength[k-1] (k=1,2,3,4) is the absolute value of the
    /// trigonometric moments m_k=\langle\exp(ik\theta)\rangle, where \theta is
    /// the position angle of the scan and the mean value is taken over the
    /// nObs[0] AL observations contributing to the astrometric parameters of
    /// the source. \theta is defined in the usual astronomical sense: \theta=0
    /// when the FoV is moving towards local North, and \theta=90^\circ towards
    /// local East.
    ///
    /// The scanDirectionStrength is a number between 0 and 1, where 0 means
    /// that the scan directions are well spread out in different directions,
    /// while 1 means that they are concentrated in a single direction (given by
    /// scanAngleMean).
    ///
    /// The different orders k are statistics of the scan directions modulo
    /// 360^\circ/k. For example, at first order (k=1), \theta=10^\circ and
    /// \theta=190^\circ count as different directions, but at second order
    /// (k=2) they are the same. Thus, scanDirectionStrength[0] is the degree of
    /// concentration when the sense of direction is taken into account, while
    /// scanDirectionStrength[1] is the degree of concentration without regard
    /// to the sense of direction. A large value of scanDirectionStrength[3]
    /// indicates that the scans are concentrated in two nearly orthogonal
    /// directions.
    scan_direction_strength_k1,
    /// The scanDirectionStrength and scanDirectionMean quantify the
    /// distribution of AL scan directions across the source.
    /// scanDirectionStrength[k-1] (k=1,2,3,4) is the absolute value of the
    /// trigonometric moments m_k=\langle\exp(ik\theta)\rangle, where \theta is
    /// the position angle of the scan and the mean value is taken over the
    /// nObs[0] AL observations contributing to the astrometric parameters of
    /// the source. \theta is defined in the usual astronomical sense: \theta=0
    /// when the FoV is moving towards local North, and \theta=90^\circ towards
    /// local East.
    ///
    /// The scanDirectionStrength is a number between 0 and 1, where 0 means
    /// that the scan directions are well spread out in different directions,
    /// while 1 means that they are concentrated in a single direction (given by
    /// scanAngleMean).
    ///
    /// The different orders k are statistics of the scan directions modulo
    /// 360^\circ/k. For example, at first order (k=1), \theta=10^\circ and
    /// \theta=190^\circ count as different directions, but at second order
    /// (k=2) they are the same. Thus, scanDirectionStrength[0] is the degree of
    /// concentration when the sense of direction is taken into account, while
    /// scanDirectionStrength[1] is the degree of concentration without regard
    /// to the sense of direction. A large value of scanDirectionStrength[3]
    /// indicates that the scans are concentrated in two nearly orthogonal
    /// directions.
    scan_direction_strength_k2,
    /// The scanDirectionStrength and scanDirectionMean quantify the
    /// distribution of AL scan directions across the source.
    /// scanDirectionStrength[k-1] (k=1,2,3,4) is the absolute value of the
    /// trigonometric moments m_k=\langle\exp(ik\theta)\rangle, where \theta is
    /// the position angle of the scan and the mean value is taken over the
    /// nObs[0] AL observations contributing to the astrometric parameters of
    /// the source. \theta is defined in the usual astronomical sense: \theta=0
    /// when the FoV is moving towards local North, and \theta=90^\circ towards
    /// local East.
    ///
    /// The scanDirectionStrength is a number between 0 and 1, where 0 means
    /// that the scan directions are well spread out in different directions,
    /// while 1 means that they are concentrated in a single direction (given by
    /// scanAngleMean).
    ///
    /// The different orders k are statistics of the scan directions modulo
    /// 360^\circ/k. For example, at first order (k=1), \theta=10^\circ and
    /// \theta=190^\circ count as different directions, but at second order
    /// (k=2) they are the same. Thus, scanDirectionStrength[0] is the degree of
    /// concentration when the sense of direction is taken into account, while
    /// scanDirectionStrength[1] is the degree of concentration without regard
    /// to the sense of direction. A large value of scanDirectionStrength[3]
    /// indicates that the scans are concentrated in two nearly orthogonal
    /// directions.
    scan_direction_strength_k3,
    /// The scanDirectionStrength and scanDirectionMean quantify the
    /// distribution of AL scan directions across the source.
    /// scanDirectionStrength[k-1] (k=1,2,3,4) is the absolute value of the
    /// trigonometric moments m_k=\langle\exp(ik\theta)\rangle, where \theta is
    /// the position angle of the scan and the mean value is taken over the
    /// nObs[0] AL observations contributing to the astrometric parameters of
    /// the source. \theta is defined in the usual astronomical sense: \theta=0
    /// when the FoV is moving towards local North, and \theta=90^\circ towards
    /// local East.
    ///
    /// The scanDirectionStrength is a number between 0 and 1, where 0 means
    /// that the scan directions are well spread out in different directions,
    /// while 1 means that they are concentrated in a single direction (given by
    /// scanAngleMean).
    ///
    /// The different orders k are statistics of the scan directions modulo
    /// 360^\circ/k. For example, at first order (k=1), \theta=10^\circ and
    /// \theta=190^\circ count as different directions, but at second order
    /// (k=2) they are the same. Thus, scanDirectionStrength[0] is the degree of
    /// concentration when the sense of direction is taken into account, while
    /// scanDirectionStrength[1] is the degree of concentration without regard
    /// to the sense of direction. A large value of scanDirectionStrength[3]
    /// indicates that the scans are concentrated in two nearly orthogonal
    /// directions.
    scan_direction_strength_k4,
    /// The scanDirectionStrength and scanDirectionMean quantify the
    /// distribution of AL scan directions across the source.
    /// scanDirectionMean[k-1] (k=1,2,3,4) is 1/k times the argument of the
    /// trigonometric moments m_k=\langle\exp(ik\theta)\rangle, where \theta is
    /// the position angle of the scan and the mean value is taken over the
    /// nObs[0] AL observations contributing to the astrometric parameters of
    /// the source. \theta is defined in the usual astronomical sense: \theta=0
    /// when the FoV is moving towards local North, and \theta=90^\circ towards
    /// local East.
    ///
    /// scanDirectionMean[k-1] is an angle between -180^\circ/k and
    /// +180^\circ/k, giving the mean position angle of the scans at order k.
    ///
    /// The different orders k are statistics of the scan directions modulo
    /// 360^\circ/k. For example, at first order (k=1), \theta=10^\circ and
    /// \theta=190^\circ count as different directions, but at second order
    /// (k=2) they are the same. Thus, scanDirectionMean[0] is the mean
    /// direction when the sense of direction is taken into account, while
    /// scanDirectionMean[1] is the mean direction without regard to the sense
    /// of the direction. For example, scanDirectionMean[0] = 0 means that the
    /// scans preferentially go towards North, while scanDirectionMean[1] = 0
    /// means that they preferentially go in the North-South direction, and
    /// scanDirectionMean[4] = 0 that they preferentially go either in the
    /// North-South or in the East-West direction.
    scan_direction_mean_k1,
    /// The scanDirectionStrength and scanDirectionMean quantify the
    /// distribution of AL scan directions across the source.
    /// scanDirectionMean[k-1] (k=1,2,3,4) is 1/k times the argument of the
    /// trigonometric moments m_k=\langle\exp(ik\theta)\rangle, where \theta is
    /// the position angle of the scan and the mean value is taken over the
    /// nObs[0] AL observations contributing to the astrometric parameters of
    /// the source. \theta is defined in the usual astronomical sense: \theta=0
    /// when the FoV is moving towards local North, and \theta=90^\circ towards
    /// local East.
    ///
    /// scanDirectionMean[k-1] is an angle between -180^\circ/k and
    /// +180^\circ/k, giving the mean position angle of the scans at order k.
    ///
    /// The different orders k are statistics of the scan directions modulo
    /// 360^\circ/k. For example, at first order (k=1), \theta=10^\circ and
    /// \theta=190^\circ count as different directions, but at second order
    /// (k=2) they are the same. Thus, scanDirectionMean[0] is the mean
    /// direction when the sense of direction is taken into account, while
    /// scanDirectionMean[1] is the mean direction without regard to the sense
    /// of the direction. For example, scanDirectionMean[0] = 0 means that the
    /// scans preferentially go towards North, while scanDirectionMean[1] = 0
    /// means that they preferentially go in the North-South direction, and
    /// scanDirectionMean[4] = 0 that they preferentially go either in the
    /// North-South or in the East-West direction.
    scan_direction_mean_k2,
    /// The scanDirectionStrength and scanDirectionMean quantify the
    /// distribution of AL scan directions across the source.
    /// scanDirectionMean[k-1] (k=1,2,3,4) is 1/k times the argument of the
    /// trigonometric moments m_k=\langle\exp(ik\theta)\rangle, where \theta is
    /// the position angle of the scan and the mean value is taken over the
    /// nObs[0] AL observations contributing to the astrometric parameters of
    /// the source. \theta is defined in the usual astronomical sense: \theta=0
    /// when the FoV is moving towards local North, and \theta=90^\circ towards
    /// local East.
    ///
    /// scanDirectionMean[k-1] is an angle between -180^\circ/k and
    /// +180^\circ/k, giving the mean position angle of the scans at order k.
    ///
    /// The different orders k are statistics of the scan directions modulo
    /// 360^\circ/k. For example, at first order (k=1), \theta=10^\circ and
    /// \theta=190^\circ count as different directions, but at second order
    /// (k=2) they are the same. Thus, scanDirectionMean[0] is the mean
    /// direction when the sense of direction is taken into account, while
    /// scanDirectionMean[1] is the mean direction without regard to the sense
    /// of the direction. For example, scanDirectionMean[0] = 0 means that the
    /// scans preferentially go towards North, while scanDirectionMean[1] = 0
    /// means that they preferentially go in the North-South direction, and
    /// scanDirectionMean[4] = 0 that they preferentially go either in the
    /// North-South or in the East-West direction.
    scan_direction_mean_k3,
    /// The scanDirectionStrength and scanDirectionMean quantify the
    /// distribution of AL scan directions across the source.
    /// scanDirectionMean[k-1] (k=1,2,3,4) is 1/k times the argument of the
    /// trigonometric moments m_k=\langle\exp(ik\theta)\rangle, where \theta is
    /// the position angle of the scan and the mean value is taken over the
    /// nObs[0] AL observations contributing to the astrometric parameters of
    /// the source. \theta is defined in the usual astronomical sense: \theta=0
    /// when the FoV is moving towards local North, and \theta=90^\circ towards
    /// local East.
    ///
    /// scanDirectionMean[k-1] is an angle between -180^\circ/k and
    /// +180^\circ/k, giving the mean position angle of the scans at order k.
    ///
    /// The different orders k are statistics of the scan directions modulo
    /// 360^\circ/k. For example, at first order (k=1), \theta=10^\circ and
    /// \theta=190^\circ count as different directions, but at second order
    /// (k=2) they are the same. Thus, scanDirectionMean[0] is the mean
    /// direction when the sense of direction is taken into account, while
    /// scanDirectionMean[1] is the mean direction without regard to the sense
    /// of the direction. For example, scanDirectionMean[0] = 0 means that the
    /// scans preferentially go towards North, while scanDirectionMean[1] = 0
    /// means that they preferentially go in the North-South direction, and
    /// scanDirectionMean[4] = 0 that they preferentially go either in the
    /// North-South or in the East-West direction.
    scan_direction_mean_k4,
    /// Number of observations (CCD transits) that contributed to the G mean
    /// flux and mean flux error.
    phot_g_n_obs,
    /// Mean flux in the G-band.
    phot_g_mean_flux,
    /// Error on the mean flux in the G-band.
    phot_g_mean_flux_error,
    /// Mean magnitude in the G band. This is computed from the G-band mean flux
    /// applying the magnitude zero-point in the Vega scale.
    phot_g_mean_mag,
    /// Flag indicating if variability was identified in the photometric G band:
    ///
    /// -   source not processed and/or exported to catalogue
    ///
    /// -   Source not identified as variable
    ///
    /// -   source identified and processed as variable, see tables
    ///     PhotVariableSummary, PhotVariableTimeSeriesGfov,
    ///     PhotVariableTimeSeriesGfovStatisticalParameters, and Cepheid or
    ///     Rrlyrae for more details.
    ///
    /// Note that for this data release only a small subset of (variable)
    /// sources was processed and/or exported, so for many (known) variable
    /// sources this flag is set to “NOT AVAILABLE”. No “CONSTANT” sources were
    /// exported either.
    phot_variable_flag,
    /// Galactic Longitude of the object at reference epoch refEpoch, see ESA,
    /// 1997, ’The Hipparcos and Tycho Catalogues’, ESA SP-1200, Volume 1,
    /// Section 1.5.3, for the conversion details.
    l,
    /// Galactic Latitude of the object at reference epoch refEpoch, see ESA,
    /// 1997, ’The Hipparcos and Tycho Catalogues’, ESA SP-1200, Volume 1,
    /// Section 1.5.3, for the conversion details.
    b,
    /// Ecliptic Longitude of the object at reference epoch refEpoch, see ESA,
    /// 1997, ’The Hipparcos and Tycho Catalogues’, ESA SP-1200, Volume 1,
    /// Section 1.5.3, for the conversion details.
    ecl_lon,
    /// Ecliptic Latitude of the object at reference epoch refEpoch, see ESA,
    /// 1997, ’The Hipparcos and Tycho Catalogues’, ESA SP-1200, Volume 1,
    /// Section 1.5.3, for the conversion details.
    ecl_lat,
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::hip.to_string());
    col_strings.push(Col::tycho2_id.to_string());
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::random_index.to_string());
    col_strings.push(Col::ref_epoch.to_string());
    col_strings.push(Col::ra.to_string());
    col_strings.push(Col::ra_error.to_string());
    col_strings.push(Col::dec.to_string());
    col_strings.push(Col::dec_error.to_string());
    col_strings.push(Col::parallax.to_string());
    col_strings.push(Col::parallax_error.to_string());
    col_strings.push(Col::pmra.to_string());
    col_strings.push(Col::pmra_error.to_string());
    col_strings.push(Col::pmdec.to_string());
    col_strings.push(Col::pmdec_error.to_string());
    col_strings.push(Col::ra_dec_corr.to_string());
    col_strings.push(Col::ra_parallax_corr.to_string());
    col_strings.push(Col::ra_pmra_corr.to_string());
    col_strings.push(Col::ra_pmdec_corr.to_string());
    col_strings.push(Col::dec_parallax_corr.to_string());
    col_strings.push(Col::dec_pmra_corr.to_string());
    col_strings.push(Col::dec_pmdec_corr.to_string());
    col_strings.push(Col::parallax_pmra_corr.to_string());
    col_strings.push(Col::parallax_pmdec_corr.to_string());
    col_strings.push(Col::pmra_pmdec_corr.to_string());
    col_strings.push(Col::astrometric_n_obs_al.to_string());
    col_strings.push(Col::astrometric_n_obs_ac.to_string());
    col_strings.push(Col::astrometric_n_good_obs_al.to_string());
    col_strings.push(Col::astrometric_n_good_obs_ac.to_string());
    col_strings.push(Col::astrometric_n_bad_obs_al.to_string());
    col_strings.push(Col::astrometric_n_bad_obs_ac.to_string());
    col_strings.push(Col::astrometric_delta_q.to_string());
    col_strings.push(Col::astrometric_excess_noise.to_string());
    col_strings.push(Col::astrometric_excess_noise_sig.to_string());
    col_strings.push(Col::astrometric_primary_flag.to_string());
    col_strings.push(Col::astrometric_relegation_factor.to_string());
    col_strings.push(Col::astrometric_weight_al.to_string());
    col_strings.push(Col::astrometric_weight_ac.to_string());
    col_strings.push(Col::astrometric_priors_used.to_string());
    col_strings.push(Col::matched_observations.to_string());
    col_strings.push(Col::duplicated_source.to_string());
    col_strings.push(Col::scan_direction_strength_k1.to_string());
    col_strings.push(Col::scan_direction_strength_k2.to_string());
    col_strings.push(Col::scan_direction_strength_k3.to_string());
    col_strings.push(Col::scan_direction_strength_k4.to_string());
    col_strings.push(Col::scan_direction_mean_k1.to_string());
    col_strings.push(Col::scan_direction_mean_k2.to_string());
    col_strings.push(Col::scan_direction_mean_k3.to_string());
    col_strings.push(Col::scan_direction_mean_k4.to_string());
    col_strings.push(Col::phot_g_n_obs.to_string());
    col_strings.push(Col::phot_g_mean_flux.to_string());
    col_strings.push(Col::phot_g_mean_flux_error.to_string());
    col_strings.push(Col::phot_g_mean_mag.to_string());
    col_strings.push(Col::phot_variable_flag.to_string());
    col_strings.push(Col::l.to_string());
    col_strings.push(Col::b.to_string());
    col_strings.push(Col::ecl_lon.to_string());
    col_strings.push(Col::ecl_lat.to_string());
    map.insert(tgas_source.string(), col_strings);
}
