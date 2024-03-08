#[derive(Debug)]
pub enum GaiaError {
    QueryError(reqwest::Error),
    ParseError(serde_json::Error),
}

impl From<reqwest::Error> for GaiaError {
    fn from(error: reqwest::Error) -> Self {
        GaiaError::QueryError(error)
    }
}

impl From<serde_json::Error> for GaiaError {
    fn from(error: serde_json::Error) -> Self {
        GaiaError::ParseError(error)
    }
}
