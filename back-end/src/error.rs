use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type HttpResult<T> = std::result::Result<T, HttpError>;

pub struct HttpError(anyhow::Error);

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

impl<E> From<E> for HttpError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
