#[derive(Debug)]
pub enum GaiaError {
    General(String),
    Query(reqwest::Error),
    Parse(serde_json::Error),
}

impl From<reqwest::Error> for GaiaError {
    fn from(error: reqwest::Error) -> Self {
        GaiaError::Query(error)
    }
}

impl From<serde_json::Error> for GaiaError {
    fn from(error: serde_json::Error) -> Self {
        GaiaError::Parse(error)
    }
}
