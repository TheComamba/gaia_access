use crate::schema::Schema;

pub struct Hipparcos;

impl Schema for Hipparcos {
    fn string(&self) -> String {
        "hipparcos".to_string()
    }
}

#[cfg(test)]
pub(crate) fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    map.insert(Hipparcos.string(), vec![]);
}
