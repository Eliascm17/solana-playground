use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
struct Error {
    code: i32,
    message: String,
    data: Option<ErrorData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorData {
    logs: Vec<String>,
}

impl Default for Error {
    fn default() -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as i32,
            message: StatusCode::INTERNAL_SERVER_ERROR.as_str().to_owned(),
            data: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientError {
    id: u16,
    jsonrpc: String,
    error: Error,
}

impl std::error::Error for ClientError {}

impl Default for ClientError {
    fn default() -> Self {
        Self {
            id: 0,
            jsonrpc: String::from("2.0"),
            error: Error::default(),
        }
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(error: reqwest::Error) -> Self {
        ClientError {
            error: Error {
                code: error
                    .status()
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
                    .as_u16() as i32,
                message: error.to_string(),
                ..Error::default()
            },
            ..Default::default()
        }
    }
}

impl ClientError {
    pub fn new(error_msg: &str) -> Self {
        ClientError {
            error: Error {
                code: StatusCode::SEE_OTHER.as_u16() as i32,
                message: error_msg.to_string(),
                ..Error::default()
            },
            ..Default::default()
        }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Client error: {}", self.error.message)
    }
}
