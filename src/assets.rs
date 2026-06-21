use axum::{
    http::{StatusCode, Uri, header},
    response::{Html, IntoResponse, Response},
};

#[derive(rust_embed::Embed)]
#[folder = "assets"]
pub struct Asset;

const INDEX_HTML: &'static str = "index.html";

impl Asset {
    pub async fn static_handler(uri: Uri) -> impl IntoResponse {
        let path = uri.path().trim_start_matches("/");

        if path.is_empty() || path == INDEX_HTML {
            return Self::index_html().await;
        }

        match Self::get(path) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();

                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => {
                if path.contains('.') {
                    return Self::not_found().await;
                }
                Self::index_html().await
            }
        }
    }

    pub async fn index_html() -> Response {
        match Self::get(INDEX_HTML) {
            Some(content) => Html(content.data).into_response(),
            None => Self::not_found().await,
        }
    }

    pub async fn not_found() -> Response {
        (StatusCode::NOT_FOUND, "404").into_response()
    }
}
