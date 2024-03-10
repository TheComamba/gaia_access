use crate::schema::Schema;

pub struct Gaiadr2;

impl Schema for Gaiadr2 {
    fn string(&self) -> String {
        "gaiadr2".to_string()
    }
}
