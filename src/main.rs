use axum_rs::{LOGO_URL, route};
use dioxus::prelude::*;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: "{LOGO_URL}", r#type: "image/png" }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<route::Route> {}
    }
}
