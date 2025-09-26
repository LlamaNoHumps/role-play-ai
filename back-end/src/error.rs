use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type HttpResult<T> = std::result::Result<T, HttpError>;

pub enum HttpError {
    Internal(anyhow::Error),
    Unauthorized(anyhow::Error),
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        match &self {
            HttpError::Unauthorized(e) => (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
            HttpError::Internal(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}

impl<E> From<E> for HttpError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        HttpError::Internal(err.into())
    }
}
