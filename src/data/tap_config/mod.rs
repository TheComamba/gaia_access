// This code is generated by generate_code.py, do not modify it manually
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct tap_config;

impl Schema for tap_config {
    fn string(&self) -> String {
        "tap_config".to_string()
    }
}

#[cfg(any(feature = "tap_config_coord_sys", test))]
pub mod coord_sys;
#[cfg(any(feature = "tap_config_properties", test))]
pub mod properties;

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    // Some tables do not have any columns. Disabling compiler warnings for these cases
    #[allow(unused_mut)]
    let mut tables = std::collections::HashMap::new();
    coord_sys::collect_known(&mut tables);
    properties::collect_known(&mut tables);
    map.insert(tap_config.string(), tables);
}
