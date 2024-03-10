use crate::schema::Schema;

pub struct TapUpload;

impl Schema for TapUpload {
    fn string(&self) -> String {
        "tap_upload".to_string()
    }
}
