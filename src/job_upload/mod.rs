use crate::schema::Schema;

pub struct JobUpload;

impl Schema for JobUpload {
    fn string(&self) -> String {
        "job_upload".to_string()
    }
}
