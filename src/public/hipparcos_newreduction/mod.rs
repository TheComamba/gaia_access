use crate::schema::Schema;

pub struct HipparcosNewreduction;

impl Schema for HipparcosNewreduction {
    fn string(&self) -> String {
        "hipparcos_newreduction".to_string()
    }
}
