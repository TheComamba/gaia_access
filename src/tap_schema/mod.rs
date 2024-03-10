use crate::schema::Schema;

pub struct TapSchema;

impl Schema for TapSchema {
    fn string(&self) -> String {
        "tap_schema".to_string()
    }
}

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    let mut tables = std::collections::HashMap::new();
    map.insert(TapSchema.string(), tables);
}
