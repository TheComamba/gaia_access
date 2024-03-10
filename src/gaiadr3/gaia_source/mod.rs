use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

use crate::{column::Column, table::Table};

pub struct GaiaSource;

impl Table for GaiaSource {
    fn string(&self) -> String {
        "gaia_source".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(
    Debug, Clone, Copy, EnumIter, EnumString, Display, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum Col {
    designation,
    ecl_lon,
}

impl Column for Col {
    fn string(&self) -> String {
        self.to_string()
    }
}
