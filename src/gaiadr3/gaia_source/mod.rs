use strum::Display;

use crate::{column::Column, table::Table};

pub struct GaiaSource;

impl Table for GaiaSource {
    fn string(&self) -> String {
        "gaia_source".to_string()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Display, PartialEq, Eq, Hash)]
pub enum Col {
    designation,
    ecl_lon,
}

impl Column for Col {}
