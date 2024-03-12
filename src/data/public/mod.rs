// This code is generated by generate_code.py, do not modify it manually
use crate::schema::Schema;

#[allow(non_camel_case_types)]
pub struct public;

impl Schema for public {
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
    // Some tables do not have any columns. Disabling compiler warnings for these cases
    #[allow(unused_mut)]
    let mut tables = std::collections::HashMap::new();
    hipparcos::collect_known(&mut tables);
    hipparcos_newreduction::collect_known(&mut tables);
    hubble_sc::collect_known(&mut tables);
    igsl_source::collect_known(&mut tables);
    igsl_source_catalog_ids::collect_known(&mut tables);
    tycho2::collect_known(&mut tables);
    dual::collect_known(&mut tables);
    map.insert(public.string(), tables);
}
