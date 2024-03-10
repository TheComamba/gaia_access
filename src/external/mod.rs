use crate::schema::Schema;

pub struct External;

impl Schema for External {
    fn string(&self) -> String {
        "external".to_string()
    }
}
