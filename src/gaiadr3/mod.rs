use crate::schema::Schema;

pub struct Gaiadr3;

impl Schema for Gaiadr3 {
    fn string(&self) -> String {
        "gaiadr3".to_string()
    }
}

pub mod gaia_source;

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    let mut tables = std::collections::HashMap::new();
    map.insert(Gaiadr3.string(), tables);
}
