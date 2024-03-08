pub enum GaiaTable {
    GaiaDr3GaiaSource,
}

impl ToString for GaiaTable {
    fn to_string(&self) -> String {
        match self {
            GaiaTable::GaiaDr3GaiaSource => "gaiadr3.gaia_source".to_string(),
        }
    }
}
