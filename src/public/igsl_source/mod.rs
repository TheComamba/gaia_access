use crate::schema::Schema;

pub struct IgslSource;

impl Schema for IgslSource {
    fn string(&self) -> String {
        "igsl_source".to_string()
    }
}

#[cfg(test)]
pub(crate) fn collect_known(map: &mut std::collections::HashMap<String, Vec<String>>) {
    map.insert(IgslSource.string(), vec![]);
}
