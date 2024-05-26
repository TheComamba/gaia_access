// This code is generated by generate_code.py, do not modify it manually.
use crate::traits::Schema;

#[allow(non_camel_case_types)]
pub struct tap_schema;

impl Schema for tap_schema {
    fn string(&self) -> String {
        "tap_schema".to_string()
    }
}

#[cfg(any(feature = "tap_schema_columns", test))]
pub mod columns;
#[cfg(any(feature = "tap_schema_key_columns", test))]
pub mod key_columns;
#[cfg(any(feature = "tap_schema_keys", test))]
pub mod keys;
#[cfg(any(feature = "tap_schema_schemas", test))]
pub mod schemas;
#[cfg(any(feature = "tap_schema_tables", test))]
pub mod tables;

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    // Some tables do not have any columns. Disabling compiler warnings for these cases
    #[allow(unused_mut)]
    let mut tables = std::collections::HashMap::new();
    columns::collect_known(&mut tables);
    key_columns::collect_known(&mut tables);
    keys::collect_known(&mut tables);
    schemas::collect_known(&mut tables);
    tables::collect_known(&mut tables);
    map.insert(tap_schema.string(), tables);
}
