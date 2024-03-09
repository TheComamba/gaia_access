use strum::{Display, EnumIter, EnumString};

#[allow(non_camel_case_types)]
#[derive(EnumIter, EnumString, Display)]
pub enum GaiaSchema {
    external,
    gaiadr1,
    gaiadr2,
    gaiadr3,
    gaiaedr3,
    gaiafpr,
    job_upload,
    public,
    tap_config,
    tap_schema,
    tap_upload,
}
