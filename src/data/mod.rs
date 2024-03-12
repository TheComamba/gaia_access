#[cfg(any(external, test))]
pub mod external;
#[cfg(any(gaiadr1, test))]
pub mod gaiadr1;
#[cfg(any(gaiadr2, test))]
pub mod gaiadr2;
#[cfg(any(gaiadr3, test))]
pub mod gaiadr3;
#[cfg(any(gaiaedr3, test))]
pub mod gaiaedr3;
#[cfg(any(gaiafpr, test))]
pub mod gaiafpr;
#[cfg(any(job_upload, test))]
pub mod job_upload;
#[cfg(any(public, test))]
pub mod public;
#[cfg(any(tap_config, test))]
pub mod tap_config;
#[cfg(any(tap_schema, test))]
pub mod tap_schema;
#[cfg(any(tap_upload, test))]
pub mod tap_upload;

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
