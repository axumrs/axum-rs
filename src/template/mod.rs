use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}
