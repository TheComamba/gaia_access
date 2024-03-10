use crate::schema::Schema;

pub struct Gaiadr1;

impl Schema for Gaiadr1 {
    fn string(&self) -> String {
        "gaiadr1".to_string()
    }
}
