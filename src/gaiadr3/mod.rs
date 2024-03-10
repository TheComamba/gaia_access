use crate::schema::Schema;

pub struct GaiaDr3;

impl Schema for GaiaDr3 {
    fn string(&self) -> String {
        "gaiadr3".to_string()
    }
}

pub mod gaia_source;
