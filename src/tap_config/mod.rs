use crate::schema::Schema;

pub struct TapConfig;

impl Schema for TapConfig {
    fn string(&self) -> String {
        "tap_config".to_string()
    }
}

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    let mut tables = std::collections::HashMap::new();
    map.insert(TapConfig.string(), tables);
}
