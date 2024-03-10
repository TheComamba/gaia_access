use crate::schema::Schema;

pub struct Tycho2;

impl Schema for Tycho2 {
    fn string(&self) -> String {
        "tycho2".to_string()
    }
}

#[cfg(test)]
pub(crate) fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    map.insert(Tycho2.string(), vec![]);
}
