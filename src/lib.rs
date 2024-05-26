#![warn(missing_docs)]

//! A simple, type-safe crate to access data from the [Gaia ESA Archive](https://gea.esac.esa.int/archive/), which contains data on hundreds of thousands of stars.
//!
//! ## Usage
//!
//! To keep the code that needs to be compiled at a manageable amount, every data table is hidden behind a feature flag. They follow the naming convention `<schema>_<table>`. See this project's `Cargo.toml` for a list of all features.
//!
//! In your `Cargo.toml`, you need to enable the tables you want to access, e.g.:
//!
//! ```toml
//! [dependencies]
//! gaia_access = { version = "0.1.0", features = ["gaiadr3_gaia_source"] }
//! ```
//!
//! There are cumulative feature flags available for schemas. If you want to use all or most of the gaiadr3 tables for example, you could depend on this crate via:
//!
//! ```toml
//! [dependencies]
//! gaia_access = { version = "0.1.0", features = ["gaiadr3"] }
//! ```
//!
//! The data query in your code is created via a builder pattern. The returned object contains a data Vec, which for all table rows contains a HashMap from column to value. The type of that value is a union of string, float and null. Knowing what type is expected for a column, it can be extracted.
//!
//! # Example
//! ```
//! use gaia_access::{
//!     condition::GaiaCondition,
//!     data::gaiadr3::{
//!         gaia_source::{gaia_source, Col},
//!         gaiadr3,
//!     },
//!     query::GaiaQueryBuilder,
//!     result::{get_float, get_string, GaiaCellData, GaiaResult},
//! };
//!
//! // Get the designation and temperature of the first three stars that have a visible magnitude brighter than 5.
//! let magnitude_threshold = 5.0;
//! let query_result = GaiaQueryBuilder::new(gaiadr3, gaia_source)
//!         .top(3)
//!         .select(vec![
//!             Col::designation,
//!             Col::teff_gspphot,
//!         ])
//!         .where_clause(GaiaCondition::LessThan(
//!             Col::phot_g_mean_mag,
//!             magnitude_threshold,
//!         ))
//!         .do_query()
//!         .unwrap();
//!
//! // From the first entry of the returned data, get the temperature value.
//! let temperature: &GaiaCellData = query_result
//!     .data[0]
//!     .get(&Col::teff_gspphot)
//!     .unwrap();
//!
//! // Convert the data to a type that can be used by the rest of rust. There is no guarantee that the data exists, hence the Option return type.
//! let temperature: Option<f64> = get_float(temperature);
//! ```

pub mod condition;
pub mod data;
pub mod error;
pub mod query;
pub mod result;
pub(crate) mod traits;

#[cfg(test)]
mod test_completeness;
