use crate::schema::Schema;

pub struct Dual;

impl Schema for Dual {
    fn string(&self) -> String {
        "dual".to_string()
    }
}
