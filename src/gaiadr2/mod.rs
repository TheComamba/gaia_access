use crate::schema::Schema;

pub struct Gaiadr2;

impl Schema for Gaiadr2 {
    fn string(&self) -> String {
        "gaiadr2".to_string()
    }
}

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    let mut tables = std::collections::HashMap::new();
    map.insert(Gaiadr2.string(), tables);
}
