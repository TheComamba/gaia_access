pub mod column;
pub mod condition;
pub mod error;
pub mod query;
pub mod result;
pub mod schema;
pub mod table;

pub mod external;
pub mod gaiadr1;
pub mod gaiadr2;
pub mod gaiadr3;
pub mod gaiaedr3;
pub mod gaiafpr;
pub mod job_upload;
pub mod public;
pub mod tap_config;
pub mod tap_schema;
pub mod tap_upload;

#[cfg(test)]
mod completeness;
#[cfg(test)]
pub(crate) fn collect_known_schemas(
) -> std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>> {
    let mut known = std::collections::HashMap::new();
    public::collect_known(&mut known);
    known
}
