//! Define the error type for the Gaia API.

/// The error type for the Gaia API.
#[derive(Debug)]
pub enum GaiaError {
    /// General error with a string message.
    General(String),
    /// Error from the query.
    Query(reqwest::Error),
    /// Error from parsing the response.
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
