use crate::schema::Schema;

pub struct Public;

impl Schema for Public {
    fn string(&self) -> String {
        "public".to_string()
    }
}

#[cfg(any(public_dual, test))]
pub mod dual;
#[cfg(any(public_hipparcos, test))]
pub mod hipparcos;
#[cfg(any(public_hipparcos_newreduction, test))]
pub mod hipparcos_newreduction;
#[cfg(any(public_hubble_sc, test))]
pub mod hubble_sc;
#[cfg(any(public_igsl_source, test))]
pub mod igsl_source;
#[cfg(any(public_igsl_source_catalog_ids, test))]
pub mod igsl_source_catalog_ids;
#[cfg(any(public_tycho2, test))]
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
