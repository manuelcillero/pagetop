use crate::response::{page::Page, ResponseError};
use crate::server::http::{header::ContentType, StatusCode};
use crate::server::HttpResponse;

use std::fmt;

#[derive(Debug)]
pub enum FatalError {
    NotModified,
    BadRequest,
    AccessDenied,
    NotFound,
    PreconditionFailed,
    InternalError,
    Timeout,
}

impl fmt::Display for FatalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            // Error 304.
            FatalError::NotModified => write!(f, "Not Modified"),
            // Error 400.
            FatalError::BadRequest => write!(f, "Bad Client Data"),
            // Error 403.
            FatalError::AccessDenied => {
                let mut error_page = Page::new();
                let error_content = error_page.context().theme().error_403_access_denied();
                if let Ok(page) = error_page
                    .with_title("Error FORBIDDEN")
                    .using_template("error")
                    .add_to("region-content", error_content)
                    .render()
                {
                    write!(f, "{}", page.into_string())
                } else {
                    write!(f, "Access Denied")
                }
            }
            // Error 404.
            FatalError::NotFound => {
                let mut error_page = Page::new();
                let error_content = error_page.context().theme().error_404_not_found();
                if let Ok(page) = error_page
                    .with_title("Error RESOURCE NOT FOUND")
                    .using_template("error")
                    .add_to("region-content", error_content)
                    .render()
                {
                    write!(f, "{}", page.into_string())
                } else {
                    write!(f, "Not Found")
                }
            }
            // Error 412.
            FatalError::PreconditionFailed => write!(f, "Precondition Failed"),
            // Error 500.
            FatalError::InternalError => write!(f, "Internal Error"),
            // Error 504.
            FatalError::Timeout => write!(f, "Timeout"),
        }
    }
}

impl ResponseError for FatalError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    #[rustfmt::skip]
    fn status_code(&self) -> StatusCode {
        match *self {
            FatalError::NotModified        => StatusCode::NOT_MODIFIED,
            FatalError::BadRequest         => StatusCode::BAD_REQUEST,
            FatalError::AccessDenied       => StatusCode::FORBIDDEN,
            FatalError::NotFound           => StatusCode::NOT_FOUND,
            FatalError::PreconditionFailed => StatusCode::PRECONDITION_FAILED,
            FatalError::InternalError      => StatusCode::INTERNAL_SERVER_ERROR,
            FatalError::Timeout            => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
