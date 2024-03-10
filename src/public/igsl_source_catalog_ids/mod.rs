use crate::schema::Schema;

pub struct IgslSourceCatalogIds;

impl Schema for IgslSourceCatalogIds {
    fn string(&self) -> String {
        "igsl_source_catalog_ids".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
    id_epc,
    id_gsc23,
    id_hip,
    id_lqrf,
    id_ogle,
    id_ppmxl,
    id_sdss,
    id_tmass,
    id_tycho,
    id_ucac,
    solution_id,
    source_id,
}

#[cfg(test)]
pub(crate) fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(IgslSourceCatalogIds.string(), col_strings);
}
