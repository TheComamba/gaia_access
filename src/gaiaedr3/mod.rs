use crate::schema::Schema;

pub struct Gaiaedr3;

impl Schema for Gaiaedr3 {
    fn string(&self) -> String {
        "gaiaedr3".to_string()
    }
}

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    let mut tables = std::collections::HashMap::new();
    map.insert(Gaiaedr3.string(), tables);
}
