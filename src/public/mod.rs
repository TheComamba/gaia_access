use crate::schema::Schema;

pub struct Public;

impl Schema for Public {
    fn string(&self) -> String {
        "public".to_string()
    }
}
