use askama::Template;
use axum::response::Html;

use crate::{Result, template};

pub async fn index() -> Result<Html<String>> {
    let tpl = template::Index {};
    let html = tpl.render()?;
    Ok(Html(html))
}
