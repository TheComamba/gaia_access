use crate::schema::Schema;

pub struct HubbleSc;

impl Schema for HubbleSc {
    fn string(&self) -> String {
        "hubble_sc".to_string()
    }
}
