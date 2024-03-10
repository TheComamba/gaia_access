use crate::{column::Column, schema::Schema};

pub struct GaiaSourceLite;

impl Schema for GaiaSourceLite {
    fn string(&self) -> String {
        "gaia_source_lite".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter)]
pub enum Col {
}

impl Column for Col {}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    use strum::IntoEnumIterator;
    let col_strings = Col::iter().map(|col| col.to_string()).collect();
    map.insert(GaiaSourceLite.string(), col_strings);
}
