use crate::schema::Schema;

pub struct Gaiafpr;

impl Schema for Gaiafpr {
    fn string(&self) -> String {
        "gaiafpr".to_string()
    }
}
