use crate::schema::Schema;

pub struct TapSchema;

impl Schema for TapSchema {
    fn string(&self) -> String {
        "tap_schema".to_string()
    }
}
