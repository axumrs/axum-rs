use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("模板错误")]
    Template(#[from] askama::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{self:?}");
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
