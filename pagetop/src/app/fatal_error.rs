use crate::app::HttpResponse;
use crate::app::http::{header::ContentType, StatusCode};
use crate::response::{page::Page, ResponseError};

use std::fmt;

#[derive(Debug)]
pub enum FatalError {
    NotFound,
    AccessDenied,
    InternalError,
    BadClientData,
    Timeout,
}

impl fmt::Display for FatalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            FatalError::NotFound => {
                let mut error_page = Page::new();
                let error_content = error_page.context().theme().error_404_not_found();
                if let Ok(page) = error_page
                    .with_title("Error RESOURCE NOT FOUND")
                    .using_template("error")
                    .add_to("content", error_content)
                    .render()
                {
                    write!(f, "{}", page.into_string())
                } else {
                    write!(f, "Not Found")
                }
            },
            FatalError::AccessDenied => {
                let mut error_page = Page::new();
                let error_content = error_page.context().theme().error_403_access_denied();
                if let Ok(page) = error_page
                    .with_title("Error FORBIDDEN")
                    .using_template("error")
                    .add_to("content", error_content)
                    .render()
                {
                    write!(f, "{}", page.into_string())
                } else {
                    write!(f, "Access Denied")
                }
            },
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
            FatalError::NotFound      => StatusCode::NOT_FOUND,
            FatalError::AccessDenied  => StatusCode::FORBIDDEN,
            FatalError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            FatalError::BadClientData => StatusCode::BAD_REQUEST,
            FatalError::Timeout       => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
