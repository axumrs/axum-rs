use dioxus::prelude::*;

use crate::{LOGO_URL, route};

#[component]
pub fn FrontendLayout() -> Element {
    rsx! {
        header {
            nav {
                Link { to: route::Route::HomePage {},
                    Logo {}
                    h2 { "AXUM中文网" }
                }
            }
        }
        Outlet::<route::Route> {}
    }
}

#[component]
fn Logo(class: Option<String>) -> Element {
    rsx! {
        img {
            src: "{LOGO_URL}",
            alt: "AXUM中文网",
            class: "size-8 object-cover",
        }
    }
}
