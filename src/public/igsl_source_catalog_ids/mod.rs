use crate::schema::Schema;

pub struct IgslSourceCatalogIds;

impl Schema for IgslSourceCatalogIds {
    fn string(&self) -> String {
        "igsl_source_catalog_ids".to_string()
    }
}

#[cfg(test)]
pub(crate) fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    map.insert(IgslSourceCatalogIds.string(), vec![]);
}
