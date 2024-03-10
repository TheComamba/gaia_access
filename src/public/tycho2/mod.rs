use crate::schema::Schema;

pub struct Tycho2;

impl Schema for Tycho2 {
    fn string(&self) -> String {
        "tycho2".to_string()
    }
}
