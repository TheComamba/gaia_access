use crate::schema::Schema;

pub struct Public;

impl Schema for Public {
    fn string(&self) -> String {
        "public".to_string()
    }
}

pub mod dual;
pub mod hipparcos;
pub mod hipparcos_newreduction;
pub mod hubble_sc;
pub mod igsl_source;
pub mod igsl_source_catalog_ids;
pub mod tycho2;
