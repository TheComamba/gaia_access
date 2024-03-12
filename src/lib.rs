pub mod column;
pub mod condition;
pub mod error;
pub mod query;
pub mod result;
pub mod schema;
pub mod table;
pub mod data;

#[cfg(test)]
mod completeness;
#[cfg(test)]
pub(crate) fn collect_known_schemas(
) -> std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>> {
    let mut known = std::collections::HashMap::new();
    external::collect_known(&mut known);
    gaiadr1::collect_known(&mut known);
    gaiadr2::collect_known(&mut known);
    gaiadr3::collect_known(&mut known);
    gaiaedr3::collect_known(&mut known);
    gaiafpr::collect_known(&mut known);
    job_upload::collect_known(&mut known);
    public::collect_known(&mut known);
    tap_config::collect_known(&mut known);
    tap_schema::collect_known(&mut known);
    tap_upload::collect_known(&mut known);
    known
}
