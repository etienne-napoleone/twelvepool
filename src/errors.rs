use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum RequestError {
    HttpError(reqwest::Error),
    JsonDeserializationError(serde_json::Error),
}

impl From<reqwest::Error> for RequestError {
    fn from(err: reqwest::Error) -> Self {
        RequestError::HttpError(err)
    }
}

impl From<serde_json::Error> for RequestError {
    fn from(err: serde_json::Error) -> Self {
        RequestError::JsonDeserializationError(err)
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            RequestError::HttpError(err) => write!(f, "{}", err),
            RequestError::JsonDeserializationError(err) => write!(f, "{}", err),
        }
    }
}

#[derive(Debug)]
pub enum TxHashDecodeError {
    B64DecodeError(base64::DecodeError),
    StringFromUtf8Error(std::string::FromUtf8Error),
}

impl From<base64::DecodeError> for TxHashDecodeError {
    fn from(err: base64::DecodeError) -> Self {
        TxHashDecodeError::B64DecodeError(err)
    }
}

impl From<std::string::FromUtf8Error> for TxHashDecodeError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        TxHashDecodeError::StringFromUtf8Error(err)
    }
}

impl Display for TxHashDecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            TxHashDecodeError::B64DecodeError(err) => write!(f, "{}", err),
            TxHashDecodeError::StringFromUtf8Error(err) => write!(f, "{}", err),
        }
    }
}
