use crate::schema::Schema;

pub struct TapConfig;

impl Schema for TapConfig {
    fn string(&self) -> String {
        "tap_config".to_string()
    }
}
