use thiserror::Error;
use actix_web::{http, HttpResponse, body::BoxBody};

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl actix_web::error::ResponseError for Error {
  fn error_response(&self) -> HttpResponse<BoxBody> {
    match self {
      Error::DatabaseError(_) =>  {
        eprintln!("Database error: {}", self);
        HttpResponse::with_body(http::StatusCode::INTERNAL_SERVER_ERROR, "Internal Error")
      }
    }.map_into_boxed_body()
}
}

pub type Result<T> = std::result::Result<T, Error>;