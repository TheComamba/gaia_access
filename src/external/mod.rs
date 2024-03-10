use crate::schema::Schema;

pub struct External;

impl Schema for External {
    fn string(&self) -> String {
        "external".to_string()
    }
}

pub mod apassdr9;
pub mod catwise2020;
pub mod gaia_eso_survey;
pub mod gaiadr2_astrophysical_parameters;
pub mod gaiadr2_geometric_distance;
pub mod gaiaedr3_distance;
pub mod gaiaedr3_gcns_main_1;
pub mod gaiaedr3_gcns_rejected_1;
pub mod gaiaedr3_spurious;
pub mod galex_ais;
pub mod ravedr5_com;
pub mod ravedr5_dr5;
pub mod ravedr5_gra;
pub mod ravedr5_on;
pub mod ravedr6;
pub mod sdssdr13_photoprimary;
pub mod skymapperdr1_master;
pub mod skymapperdr2_master;
pub mod tmass_xsc;

#[cfg(test)]
pub(crate) fn collect_known(
    map: &mut std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>,
) {
    let mut tables = std::collections::HashMap::new();
    apassdr9::collect_known(&mut tables);
    catwise2020::collect_known(&mut tables);
    gaia_eso_survey::collect_known(&mut tables);
    gaiadr2_astrophysical_parameters::collect_known(&mut tables);
    gaiadr2_geometric_distance::collect_known(&mut tables);
    gaiaedr3_distance::collect_known(&mut tables);
    gaiaedr3_gcns_main_1::collect_known(&mut tables);
    gaiaedr3_gcns_rejected_1::collect_known(&mut tables);
    gaiaedr3_spurious::collect_known(&mut tables);
    galex_ais::collect_known(&mut tables);
    ravedr5_com::collect_known(&mut tables);
    ravedr5_dr5::collect_known(&mut tables);
    ravedr5_gra::collect_known(&mut tables);
    ravedr5_on::collect_known(&mut tables);
    ravedr6::collect_known(&mut tables);
    sdssdr13_photoprimary::collect_known(&mut tables);
    skymapperdr1_master::collect_known(&mut tables);
    skymapperdr2_master::collect_known(&mut tables);
    tmass_xsc::collect_known(&mut tables);
    map.insert(External.string(), tables);
}
