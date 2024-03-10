use crate::schema::Schema;

pub struct Dual;

impl Schema for Dual {
    fn string(&self) -> String {
        "dual".to_string()
    }
}

#[cfg(test)]
pub fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    map.insert(Dual.string(), vec![]);
}
