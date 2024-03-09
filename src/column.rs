use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[allow(non_camel_case_types)]
#[derive(
    Debug, Clone, Copy, EnumIter, EnumString, Display, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum GaiaColumn {
    designation,
    ecl_lon,
    ecl_lat,
    phot_g_mean_mag,
    teff_gspphot,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        query::GaiaQueryBuilder,
        result::{get_float, get_string},
        schema::GaiaSchema,
        table::GaiaTable,
    };

    #[test]
    fn test_some_enum_variants_to_string() {
        assert_eq!(GaiaColumn::designation.to_string(), "designation");
        assert_eq!(GaiaColumn::ecl_lon.to_string(), "ecl_lon");
        assert_eq!(GaiaColumn::ecl_lat.to_string(), "ecl_lat");
        assert_eq!(GaiaColumn::phot_g_mean_mag.to_string(), "phot_g_mean_mag");
        assert_eq!(GaiaColumn::teff_gspphot.to_string(), "teff_gspphot");
    }

    #[test]
    fn request_a_string() {
        let col = GaiaColumn::designation;
        let result = GaiaQueryBuilder::new(GaiaSchema::gaiadr3, GaiaTable::gaia_source)
            .top(1)
            .select(vec![col])
            .do_query()
            .unwrap();
        assert_eq!(result.columns[0], col);
        assert!(result.data[0].contains_key(&col));
        assert!(get_string(result.data[0].get(&col).unwrap()).is_some());
    }

    #[test]
    fn request_a_float() {
        let col = GaiaColumn::ecl_lon;
        let result = GaiaQueryBuilder::new(GaiaSchema::gaiadr3, GaiaTable::gaia_source)
            .top(1)
            .select(vec![col])
            .do_query()
            .unwrap();
        assert_eq!(result.columns[0], col);
        assert!(result.data[0].contains_key(&col));
        assert!(get_float(result.data[0].get(&col).unwrap()).is_some());
    }
}
