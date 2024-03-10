use crate::schema::Schema;

pub struct HipparcosNewreduction;

impl Schema for HipparcosNewreduction {
    fn string(&self) -> String {
        "hipparcos_newreduction".to_string()
    }
}

#[cfg(test)]
pub(crate) fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    map.insert(HipparcosNewreduction.string(), vec![]);
}
