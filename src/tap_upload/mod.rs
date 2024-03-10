use crate::schema::Schema;

pub struct TapUpload;

impl Schema for TapUpload {
    fn string(&self) -> String {
        "tap_upload".to_string()
    }
}

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    let mut tables = std::collections::HashMap::new();
    map.insert(TapUpload.string(), tables);
}
