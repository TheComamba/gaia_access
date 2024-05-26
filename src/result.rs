//! This module contains the `GaiaResult` and `GaiaCellData` structs, which are used to store the results of a database query.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::traits::Column;

#[derive(Serialize, Deserialize)]
struct GaiaMetadataLine {
    name: String,
    datatype: String,
    xtype: Option<String>,
    arraysize: Option<String>,
    description: String,
    unit: Option<String>,
    ucd: String,
    utype: Option<String>,
}

/// The data type of a cell in a Gaia query result.
///
/// This enum is used to store the data of a cell in a Gaia query result. The data can be a string, a float, or null.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum GaiaCellData {
    /// A string value.
    String(String),
    /// A float value.
    Float(f64),
    /// A null value.
    Null,
}

/// Get the string value from a GaiaCellData.
///
/// # Example
///
/// ```
/// use gaia_access::{
///     data::gaiadr3::{
///         gaia_source::{gaia_source, Col},
///         gaiadr3,
///     },
///     query::GaiaQueryBuilder,
///     result::{get_float, get_string},
/// };
///
/// let col = Col::designation;
/// let result = GaiaQueryBuilder::new(gaiadr3, gaia_source)
///     .top(1)
///     .select(vec![col])
///     .do_query()
///     .unwrap();
/// let first_cell = result.data[0].get(&col).unwrap();
/// assert_eq!(result.columns[0], col);
/// assert!(result.data[0].contains_key(&col));
/// assert!(get_string(first_cell).is_some());
/// ```
pub fn get_string(data: &GaiaCellData) -> Option<String> {
    match data {
        GaiaCellData::String(string) => Some(string.clone()),
        _ => None,
    }
}

/// Get the float value from a GaiaCellData.
///
/// # Example
///
/// ```
/// use gaia_access::{
///     data::gaiadr3::{
///         gaia_source::{gaia_source, Col},
///         gaiadr3,
///     },
///     query::GaiaQueryBuilder,
///     result::{get_float, get_string},
/// };
///
/// let col = Col::ecl_lon;
/// let result = GaiaQueryBuilder::new(gaiadr3, gaia_source)
///     .top(1)
///     .select(vec![col])
///     .do_query()
///     .unwrap();
/// let first_cell = result.data[0].get(&col).unwrap();
/// assert_eq!(result.columns[0], col);
/// assert!(result.data[0].contains_key(&col));
/// assert!(get_float(first_cell).is_some());
/// ```
pub fn get_float(data: &GaiaCellData) -> Option<f64> {
    match data {
        GaiaCellData::Float(float) => Some(*float),
        _ => None,
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct GaiaResponse {
    metadata: Vec<GaiaMetadataLine>,
    data: Vec<Vec<GaiaCellData>>,
}

/// The result of a Gaia query.
///
/// This struct is used to store the result of a Gaia query. It contains the columns of the result and the data of the result.
///
/// # Example
///
/// ```
/// use gaia_access::{
///     data::gaiadr3::{
///         gaia_source::{gaia_source, Col},
///         gaiadr3,
///     },
///     query::GaiaQueryBuilder,
///     result::{get_float, get_string, GaiaResult},
/// };
/// let cols = vec![Col::designation, Col::teff_gspphot];
/// let result = GaiaQueryBuilder::new(gaiadr3, gaia_source)
///     .top(2)
///     .select(cols.clone())
///     .do_query()
///     .unwrap();
/// for col in cols {
///    assert!(result.columns.contains(&col));
/// }
/// ```
pub struct GaiaResult<C: Column> {
    /// The columns of the result. These are the sorted keys of the HashMaps in the data field.
    pub columns: Vec<C>,
    /// The data of the result.
    ///
    /// The data is stored as a vector of HashMaps from column to GaiaCellData, where each vector entry corresponds to one row.
    pub data: Vec<HashMap<C, GaiaCellData>>,
}

impl<C: Column> GaiaResult<C> {
    pub(super) fn new(response: GaiaResponse, columns: Vec<C>) -> Self {
        let mut data = Vec::new();
        for row in response.data {
            let mut star = HashMap::new();
            for (i, cell) in row.into_iter().enumerate() {
                star.insert(columns[i], cell);
            }
            data.push(star);
        }
        GaiaResult { columns, data }
    }
}
