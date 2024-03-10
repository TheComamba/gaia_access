use crate::schema::Schema;

pub struct Hipparcos;

impl Schema for Hipparcos {
    fn string(&self) -> String {
        "hipparcos".to_string()
    }
}
