use dioxus::prelude::*;

use crate::layouts::FrontendLayout;
use crate::views::HomePage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(FrontendLayout)]
    #[route("/")]
    HomePage {},
   
}
