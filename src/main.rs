use dioxus::prelude::*;

mod adapter;
mod backend;
mod components;
mod image_processing;
mod models;
mod repositories;
mod utils;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::logger::init(dioxus::logger::tracing::Level::DEBUG)
        .expect("Failed to initialize logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    components::common::toast::use_toast_context();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        components::common::toast::ToastPortal {  }
        components::home::Home {}
    }
}
