use crate::schema::Schema;

pub struct HubbleSc;

impl Schema for HubbleSc {
    fn string(&self) -> String {
        "hubble_sc".to_string()
    }
}

#[cfg(test)]
pub(crate) fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    map.insert(HubbleSc.string(), vec![]);
}
