use crate::schema::Schema;

pub struct Gaiadr1;

impl Schema for Gaiadr1 {
    fn string(&self) -> String {
        "gaiadr1".to_string()
    }
}

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    let mut tables = std::collections::HashMap::new();
    map.insert(Gaiadr1.string(), tables);
}
