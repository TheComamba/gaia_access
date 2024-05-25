use std::fmt;

use crate::traits::Column;

pub enum GaiaCondition<C: Column> {
    LessThan(C, f64),
    GreaterThan(C, f64),
    Between(C, f64, f64),
}

impl<C: Column> fmt::Display for GaiaCondition<C> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            GaiaCondition::LessThan(column, value) => format!("{} < {}", column.to_string(), value),
            GaiaCondition::GreaterThan(column, value) => {
                format!("{} > {}", column.to_string(), value)
            }
            GaiaCondition::Between(column, value1, value2) => {
                format!("{} BETWEEN {} AND {}", column.to_string(), value1, value2)
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
}
