use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum MattermostError {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Api(String),
}

impl Display for MattermostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MattermostError::Http(e) => write!(f, "HTTP error: {}", e),
            MattermostError::Json(e) => write!(f, "JSON error: {}", e),
            MattermostError::Api(msg) => write!(f, "API error: {}", msg),
        }
    }
}

impl From<reqwest::Error> for MattermostError {
    fn from(err: reqwest::Error) -> Self {
        MattermostError::Http(err)
    }
}

impl From<serde_json::Error> for MattermostError {
    fn from(err: serde_json::Error) -> Self {
        MattermostError::Json(err)
    }
}

impl From<String> for MattermostError {
    fn from(err: String) -> Self {
        MattermostError::Api(err)
    }
}

impl Error for MattermostError {}
