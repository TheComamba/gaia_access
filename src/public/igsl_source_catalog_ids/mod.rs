use crate::schema::Schema;

pub struct IgslSourceCatalogIds;

impl Schema for IgslSourceCatalogIds {
    fn string(&self) -> String {
        "igsl_source_catalog_ids".to_string()
    }
}
