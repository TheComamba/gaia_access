use crate::schema::Schema;

pub struct Public;

impl Schema for Public {
    fn string(&self) -> String {
        "public".to_string()
    }
}

pub mod dual;
pub mod hipparcos;
pub mod hipparcos_newreduction;
pub mod hubble_sc;
pub mod igsl_source;
pub mod igsl_source_catalog_ids;
pub mod tycho2;

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    let mut tables = std::collections::HashMap::new();
    dual::collect_known(&mut tables);
    hipparcos::collect_known(&mut tables);
    hipparcos_newreduction::collect_known(&mut tables);
    hubble_sc::collect_known(&mut tables);
    igsl_source::collect_known(&mut tables);
    igsl_source_catalog_ids::collect_known(&mut tables);
    tycho2::collect_known(&mut tables);
    map.insert(Public.string(), tables);
}
