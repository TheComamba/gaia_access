use strum::{Display, EnumIter, EnumString};

#[allow(non_camel_case_types)]
#[derive(EnumIter, EnumString, Display)]
pub enum GaiaSchema {
    gaiadr3,
}
