//! This module implements the builder for Gaia queries.
//!
//! The query builder is a struct that allows the user to build a query to the Gaia archive in a type-safe way. The query is built by chaining methods that add clauses to the query, such as selecting columns, adding conditions, and setting the number of rows to return.
//! The `do_query` method sends the query to the Gaia archive and returns the result.
//!
//! # Example
//!
//! ```
//! use gaia_access::{
//!     condition::GaiaCondition,
//!     data::gaiadr3::{
//!         gaia_source::{gaia_source, Col},
//!         gaiadr3,
//!     },
//!     query::GaiaQueryBuilder,
//!     result::{get_float, get_string, GaiaResult},
//! };
//!
//! const NUMBER_OF_COLUMNS: usize = 20;
//!
//! let col = Col::ecl_lon;
//! let cond = GaiaCondition::LessThan(col, 10.0);
//! let result = GaiaQueryBuilder::new(gaiadr3, gaia_source)
//!     .top(NUMBER_OF_COLUMNS)
//!     .select(vec![col])
//!     .where_clause(cond)
//!     .do_query()
//!     .unwrap();
//! assert_eq!(result.columns[0], col);
//! let ecl_lon = get_float(result.data[0].get(&col).unwrap()).unwrap();
//! assert!(ecl_lon < 10.0);
//! ```

use crate::{
    condition::GaiaCondition,
    error::GaiaError,
    result::{GaiaResponse, GaiaResult},
    traits::{Column, Schema, Table},
};

/// The query builder for Gaia queries.
///
/// This struct is templated over the schema, table, and column types. The schema and table types are used to specify which data to query. The Column type is an enum that represents the columns of the table.
pub struct GaiaQueryBuilder<S: Schema, T: Table, C: Column> {
    schema: S,
    table: T,
    top: Option<usize>,
    columns: Vec<C>,
    conditions: Vec<GaiaCondition<C>>,
}

impl<S: Schema, T: Table, C: Column> GaiaQueryBuilder<S, T, C> {
    /// Create a new GaiaQueryBuilder.
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
    /// };
    ///
    /// // Type annotations are needed in this example because no method involving the columns is called.
    /// let query_builder: GaiaQueryBuilder<_, _, Col> = GaiaQueryBuilder::new(gaiadr3, gaia_source);
    /// ```
    pub fn new(schema: S, table: T) -> Self {
        GaiaQueryBuilder {
            schema,
            table,
            top: None,
            columns: Vec::new(),
            conditions: Vec::new(),
        }
    }

    /// Choose which columns to return from the query.
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
    /// };
    ///
    /// let query_builder = GaiaQueryBuilder::new(gaiadr3, gaia_source)
    ///     .select(vec![Col::designation, Col::teff_gspphot]);
    /// ```
    pub fn select(mut self, mut columns: Vec<C>) -> Self {
        self.columns.append(&mut columns);
        self
    }

    /// Sets the maximum number of results to return.
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
    /// };
    ///
    /// // Type annotations are needed in this example because no method involving the columns is called.
    /// let query_builder: GaiaQueryBuilder<_, _, Col> = GaiaQueryBuilder::new(gaiadr3, gaia_source)
    ///     .top(10);
    /// ```
    pub fn top(mut self, top: usize) -> Self {
        self.top = Some(top);
        self
    }

    /// Add a condition to the query.
    ///
    /// # Example
    ///
    /// ```
    /// use gaia_access::{
    ///     condition::GaiaCondition,
    ///     data::gaiadr3::{
    ///         gaia_source::{gaia_source, Col},
    ///         gaiadr3,
    ///     },
    ///     query::GaiaQueryBuilder,
    /// };
    ///
    /// let query_builder = GaiaQueryBuilder::new(gaiadr3, gaia_source)
    ///     .where_clause(GaiaCondition::LessThan(Col::phot_g_mean_mag, 10.0));
    /// ```
    pub fn where_clause(mut self, condition: GaiaCondition<C>) -> Self {
        self.conditions.push(condition);
        self
    }

    /// Returns the query string for inspection.
    ///
    /// This method returns the query string that will be sent to the Gaia archive. This can be useful for debugging or logging purposes. Note that you do not have to call this method to send the query, as the `do_query` method will do that for you.
    ///
    /// # Example
    ///
    /// ```
    /// use gaia_access::{
    ///     condition::GaiaCondition,
    ///     data::gaiadr3::{
    ///         gaia_source::{gaia_source, Col},
    ///         gaiadr3,
    ///     },
    ///     query::GaiaQueryBuilder,
    /// };
    ///
    /// let query_builder = GaiaQueryBuilder::new(gaiadr3, gaia_source)
    ///     .select(vec![Col::designation, Col::teff_gspphot])
    ///     .where_clause(GaiaCondition::LessThan(Col::phot_g_mean_mag, 10.5));
    /// let query_string = query_builder.query_string();
    /// assert_eq!(query_string, "SELECT designation, teff_gspphot FROM gaiadr3.gaia_source WHERE (phot_g_mean_mag < 10.5)");
    /// ```
    pub fn query_string(&self) -> String {
        let mut query = "SELECT".to_string();
        if let Some(top) = self.top {
            query.push_str(&format!(" TOP {}", top));
        }
        query.push_str(&format!(
            " {}",
            self.columns
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));
        query.push_str(&format!(
            " FROM {}.{}",
            self.schema.string(),
            self.table.string()
        ));
        if !self.conditions.is_empty() {
            query.push_str(&format!(
                " WHERE {}",
                self.conditions
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(" AND ")
            ));
        }
        query
    }

    /// Send the query to the Gaia archive and return the result.
    ///
    /// This method sends the query to the Gaia archive and returns the result. The result is a GaiaResult object that contains the columns and data returned by the query.
    ///
    /// # Example
    ///
    /// ```
    /// use gaia_access::{
    ///     condition::GaiaCondition,
    ///     data::gaiadr3::{
    ///         gaia_source::{gaia_source, Col},
    ///         gaiadr3,
    ///     },
    ///     query::GaiaQueryBuilder,
    /// };
    ///
    /// let query_builder = GaiaQueryBuilder::new(gaiadr3, gaia_source)
    ///     .top(10)
    ///     .select(vec![Col::designation, Col::teff_gspphot])
    ///     .where_clause(GaiaCondition::LessThan(Col::phot_g_mean_mag, 10.5));
    /// let result = query_builder.do_query().unwrap();
    /// ```
    pub fn do_query(self) -> Result<GaiaResult<C>, GaiaError> {
        let response = reqwest::blocking::Client::new()
            .get("https://gea.esac.esa.int/tap-server/tap/sync")
            .query(&[
                ("request", "doQuery"),
                ("lang", "ADQL"),
                ("format", "json"),
                ("query", &self.query_string()),
            ])
            .send()?;
        let text = response.text()?;
        let response: GaiaResponse = serde_json::from_str(&text)?;
        Ok(GaiaResult::new(response, self.columns))
    }
}
