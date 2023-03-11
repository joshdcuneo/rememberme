use actix_web::{body::BoxBody, http, HttpResponse};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    DatabaseError(sqlx::Error),
    #[error("not found")]
    NotFound,
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::error::Error::RowNotFound => Error::NotFound,
            _ => Error::DatabaseError(err),
        }
    }
}

impl actix_web::error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Error::DatabaseError(_) => {
                eprintln!("Database error: {}", self);
                HttpResponse::with_body(http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error")
            }
            Error::NotFound => HttpResponse::with_body(http::StatusCode::NOT_FOUND, "Not Found"),
        }
        .map_into_boxed_body()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
