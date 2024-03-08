use crate::column::GaiaColumn;

pub enum GaiaCondition {
    LessThan(GaiaColumn, f64),
    GreaterThan(GaiaColumn, f64),
    Between(GaiaColumn, f64, f64),
}
impl ToString for GaiaCondition {
    fn to_string(&self) -> String {
        let str = match self {
            GaiaCondition::LessThan(column, value) => format!("{} < {}", column.to_string(), value),
            GaiaCondition::GreaterThan(column, value) => {
                format!("{} > {}", column.to_string(), value)
            }
            GaiaCondition::Between(column, value1, value2) => {
                format!("{} BETWEEN {} AND {}", column.to_string(), value1, value2)
            }
        };
        format!("({})", str)
    }
}
