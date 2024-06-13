//! Contains the definition of the GaiaCondition enum that allows filtering query results.

use std::fmt;

use crate::traits::Column;

/// The GaiaCondition enum can be used in the QueryBuilder to conditionally filter results.
pub enum GaiaCondition<C: Column> {
    /// Between(column, lower, upper) is rendered as "column BETWEEN lower AND upper" in ADQL.
    Between(C, f64, f64),
    /// GreaterThan(column, threshold) is rendered as "column > threshold" in ADQL.
    GreaterThan(C, f64),
    /// GreaterThanOrEqual(column, threshold) is rendered as "column >= threshold" in ADQL.
    GreaterThanOrEqual(C, f64),
    /// LessThan(column, threshold) is rendered as "column < threshold" in ADQL.
    LessThan(C, f64),
    /// LessThanOrEqual(column, threshold) is rendered as "column <= threshold" in ADQL.
    LessThanOrEqual(C, f64),
}

impl<C: Column> fmt::Display for GaiaCondition<C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            GaiaCondition::Between(column, value1, value2) => {
                format!("{} BETWEEN {} AND {}", column.to_string(), value1, value2)
            }
            GaiaCondition::GreaterThan(column, value) => {
                format!("{} > {}", column.to_string(), value)
            }
            GaiaCondition::GreaterThanOrEqual(column, value) => {
                format!("{} >= {}", column.to_string(), value)
            }
            GaiaCondition::LessThan(column, value) => format!("{} < {}", column.to_string(), value),
            GaiaCondition::LessThanOrEqual(column, value) => {
                format!("{} <= {}", column.to_string(), value)
            }
        };
        write!(f, "({})", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::gaiadr3::{gaia_source::*, gaiadr3},
        query::GaiaQueryBuilder,
        result::get_float,
    };

    const NUMBER_OF_COLUMNS: usize = 20;

    #[test]
    fn request_between_condition() {
        let col = Col::ecl_lon;
        let cond = GaiaCondition::Between(col, 10.0, 20.0);
        let result = GaiaQueryBuilder::new(gaiadr3, gaia_source)
            .top(NUMBER_OF_COLUMNS)
            .select(vec![col])
            .where_clause(cond)
            .do_query()
            .unwrap();
        assert_eq!(result.columns[0], col);
        let ecl_lon = get_float(result.data[0].get(&col).unwrap()).unwrap();
        assert!(ecl_lon > 10.0 && ecl_lon < 20.0);
    }

    #[test]
    fn request_greater_than_condition() {
        let col = Col::ecl_lon;
        let cond = GaiaCondition::GreaterThan(col, 10.0);
        let result = GaiaQueryBuilder::new(gaiadr3, gaia_source)
            .top(NUMBER_OF_COLUMNS)
            .select(vec![col])
            .where_clause(cond)
            .do_query()
            .unwrap();
        assert_eq!(result.columns[0], col);
        let ecl_lon = get_float(result.data[0].get(&col).unwrap()).unwrap();
        assert!(ecl_lon > 10.0);
    }

    #[test]
    fn request_greater_than_or_equal_condition() {
        let col = Col::ecl_lon;
        let cond = GaiaCondition::GreaterThanOrEqual(col, 10.0);
        let result = GaiaQueryBuilder::new(gaiadr3, gaia_source)
            .top(NUMBER_OF_COLUMNS)
            .select(vec![col])
            .where_clause(cond)
            .do_query()
            .unwrap();
        assert_eq!(result.columns[0], col);
        let ecl_lon = get_float(result.data[0].get(&col).unwrap()).unwrap();
        assert!(ecl_lon >= 10.0);
    }

    #[test]
    fn request_less_than_condition() {
        let col = Col::ecl_lon;
        let cond = GaiaCondition::LessThan(col, 10.0);
        let result = GaiaQueryBuilder::new(gaiadr3, gaia_source)
            .top(NUMBER_OF_COLUMNS)
            .select(vec![col])
            .where_clause(cond)
            .do_query()
            .unwrap();
        assert_eq!(result.columns[0], col);
        let ecl_lon = get_float(result.data[0].get(&col).unwrap()).unwrap();
        assert!(ecl_lon < 10.0);
    }

    #[test]
    fn request_less_than_or_equal_condition() {
        let col = Col::ecl_lon;
        let cond = GaiaCondition::LessThanOrEqual(col, 10.0);
        let result = GaiaQueryBuilder::new(gaiadr3, gaia_source)
            .top(NUMBER_OF_COLUMNS)
            .select(vec![col])
            .where_clause(cond)
            .do_query()
            .unwrap();
        assert_eq!(result.columns[0], col);
        let ecl_lon = get_float(result.data[0].get(&col).unwrap()).unwrap();
        assert!(ecl_lon <= 10.0);
    }
}
