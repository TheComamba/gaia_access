use crate::schema::Schema;

pub struct IgslSource;

impl Schema for IgslSource {
    fn string(&self) -> String {
        "igsl_source".to_string()
    }
}
