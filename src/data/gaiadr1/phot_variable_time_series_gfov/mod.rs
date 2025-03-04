// This code is generated by generate_code.py, do not modify it manually.

//! This module contains all the known columns in the phot_variable_time_series_gfov table.

use crate::traits::{Column, Table};

/// Field-of-view time series of sources that have photVariableFlag =
/// “VARIABLE” in the GaiaSource table.
#[allow(non_camel_case_types)]
pub struct phot_variable_time_series_gfov;

impl Table for phot_variable_time_series_gfov {
    fn string(&self) -> String {
        "phot_variable_time_series_gfov".to_string()
    }
}

/// The columns in the phot_variable_time_series_gfov table.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum Col {
    /// All Gaia data processed by the Data Processing and Analysis Consortium
    /// comes tagged with a solution identifier. This is a numeric field
    /// attached to each table row that can be used to unequivocally identify
    /// the version of all the subsystems that where used in the generation of
    /// the data as well as the input data used. It is mainly for internal DPAC
    /// use but is included in the published data releases to enable end users
    /// to examine the provenance of processed data products. To decode a given
    /// solution ID visit
    solution_id,
    /// A unique single numerical identifier of the source obtained from
    /// GaiaSource (for a detailed description see GaiaSource.sourceId)
    source_id,
    /// Field-of-view transit averaged observation time in units of Barycentric
    /// JD (in TCB) in days -2455197.5, computed as follows. First the
    /// observation time is converted from On-board Mission Time (OBMT) into
    /// Julian date in TCB (Temps Coordonnée Barycentrique). Next a correction
    /// is applied for the light-travel time to the Solar system barycentre,
    /// resulting in Barycentric Julian Date (BJD). Finally, an offset of
    /// 2455197.5 days is applied (corresponding to a reference time T_0 at
    /// 2010-01-01T00:00:00) to have a conveniently small numerical value.
    /// Although the centroiding time accuracy of the individual CCD
    /// observations is (much) below 1 ms, this per-FoV observation time is
    /// averaged over typically 9 CCD observations taken in a time range of
    /// about 44 sec.
    observation_time,
    /// G-band flux for the field-of-view transit observation.
    g_flux,
    /// Estimated uncertainty of G-band flux for the field-of-view transit
    /// observation.
    g_flux_error,
    /// G-band magnitude for the field-of-view transit observation, computed
    /// from the gFlux field using magnitude zero-point defined in
    /// ExtPhotZeroPoint.
    g_magnitude,
    /// Observations with this flag true have been excluded from the variability
    /// result in tables VariableSummary, Cepheid, Rrlyrae, and
    /// PhotVariableTimeSeriesGfovStatisticalParameters.
    rejected_by_variability_processing,
}

impl Column for Col {}

#[cfg(test)]
/// Collects all the known columns in the phot_variable_time_series_gfov table.
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    let mut col_strings = Vec::new();
    col_strings.push(Col::solution_id.to_string());
    col_strings.push(Col::source_id.to_string());
    col_strings.push(Col::observation_time.to_string());
    col_strings.push(Col::g_flux.to_string());
    col_strings.push(Col::g_flux_error.to_string());
    col_strings.push(Col::g_magnitude.to_string());
    col_strings.push(Col::rejected_by_variability_processing.to_string());
    map.insert(phot_variable_time_series_gfov.string(), col_strings);
}
