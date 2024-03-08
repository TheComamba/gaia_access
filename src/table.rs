use strum::EnumIter;

#[derive(EnumIter)]
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

#[cfg(test)]
mod tests {
    use crate::{column::GaiaColumn, query::GaiaQueryBuilder};
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn request_top_entry_from_every_table() {
        for table in GaiaTable::iter() {
            let result = GaiaQueryBuilder::new(table)
                .top(1)
                .select(vec![GaiaColumn::designation])
                .do_query();
            assert!(result.is_ok());
        }
    }
}
