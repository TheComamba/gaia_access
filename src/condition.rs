use crate::column::GaiaColumn;

pub enum GaiaCondition {
    LessThan(GaiaColumn, f64),
    GreaterThan(GaiaColumn, f64),
    Between(GaiaColumn, f64, f64),
}
impl ToString for GaiaCondition {
    fn to_string(&self) -> String {
        let str = match self {
            GaiaCondition::LessThan(column, value) => format!("{} < {}", column, value),
            GaiaCondition::GreaterThan(column, value) => format!("{} > {}", column, value),
            GaiaCondition::Between(column, value1, value2) => {
                format!("{} BETWEEN {} AND {}", column, value1, value2)
            }
        };
        format!("({})", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        gaiadr3::{gaia_source::GaiaSource, GaiaDr3},
        query::GaiaQueryBuilder,
        result::get_float,
    };

    const NUMBER_OF_COLUMNS: usize = 20;

    #[test]
    fn request_less_than_condition() {
        let col = GaiaColumn::ecl_lon;
        let cond = GaiaCondition::LessThan(col, 10.0);
        let result = GaiaQueryBuilder::new(GaiaDr3, GaiaSource)
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
        let col = GaiaColumn::ecl_lon;
        let cond = GaiaCondition::GreaterThan(col, 10.0);
        let result = GaiaQueryBuilder::new(GaiaDr3, GaiaSource)
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
        let col = GaiaColumn::ecl_lon;
        let cond = GaiaCondition::Between(col, 10.0, 20.0);
        let result = GaiaQueryBuilder::new(GaiaDr3, GaiaSource)
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
