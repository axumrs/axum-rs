use pulldown_cmark::{html, Options, Parser};

fn get_parser(md: &str) -> Parser<'_> {
    Parser::new_ext(md, Options::all())
}
pub fn to_html(md: &str) -> String {
    let mut out_html = String::new();
    html::push_html(&mut out_html, get_parser(md));
    out_html
}
