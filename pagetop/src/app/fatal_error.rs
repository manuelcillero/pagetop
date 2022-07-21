use crate::app::HttpResponse;
use crate::app::http::{header::ContentType, StatusCode};
use crate::response::ResponseError;

use std::fmt;

#[derive(Debug)]
pub enum FatalError {
    InternalError,
    BadClientData,
    Timeout,
}

impl fmt::Display for FatalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            FatalError::InternalError => write!(f, "Internal Error"),
            FatalError::BadClientData => write!(f, "Bad Client Data"),
            FatalError::Timeout       => write!(f, "Timeout"),
        }
    }
}

impl ResponseError for FatalError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            FatalError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            FatalError::BadClientData => StatusCode::BAD_REQUEST,
            FatalError::Timeout       => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
