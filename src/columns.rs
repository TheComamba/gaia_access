use strum_macros::{Display, EnumString};

#[allow(non_camel_case_types)]
#[derive(Debug, EnumString, Display)]
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

    #[test]
    fn test_enum_to_string() {
        assert_eq!(GaiaColumn::designation.to_string(), "designation");
        assert_eq!(GaiaColumn::ecl_lon.to_string(), "ecl_lon");
        assert_eq!(GaiaColumn::ecl_lat.to_string(), "ecl_lat");
        assert_eq!(GaiaColumn::phot_g_mean_mag.to_string(), "phot_g_mean_mag");
        assert_eq!(GaiaColumn::teff_gspphot.to_string(), "teff_gspphot");
    }
}
