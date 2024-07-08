use crate::database::errors::DbError;

use axum::{body::Body, http::Response, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiServerError {
    #[error("Bad Request")]
    BadRequest,
    #[error("Internal Server Error")]
    InternalServerError,
}

impl IntoResponse for ApiServerError {
    fn into_response(self) -> Response<Body> {
        let status = match self {
            ApiServerError::BadRequest => 400,
            ApiServerError::InternalServerError => 500,
        };

        Response::builder()
            .status(status)
            .body(self.to_string().into())
            .expect("Error building response")
    }
}

impl From<DbError> for ApiServerError {
    fn from(error: DbError) -> Self {
        match error {
            DbError::BlockNotFound | DbError::AccountNotFound | DbError::TransactionNotFound => {
                ApiServerError::InternalServerError
            }
            _ => ApiServerError::BadRequest,
        }
    }
}
