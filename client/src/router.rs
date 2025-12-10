use components::Mindmap;
use dioxus::prelude::*;

use crate::components;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/css/main.css");

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home,

    #[route("/channels/:channel_id")]
    Channel { channel_id: String },

    #[route("/:any")]
    NotFound { any: String },
}

#[component]
fn Home() -> Element {
    rsx! {
        default_style {}
        h1 {
            "Welcome! Create a mindmap by navigating to /channels/YOUR_CHANNEL_NAME"
        }
    }
}

#[component]
fn Channel(channel_id: String) -> Element {
    rsx! {
        default_style {}
        Mindmap { channel_id: "root" }
    }
}

#[component]
fn NotFound(any: String) -> Element {
    // TODO: Return 404 status code
    rsx! {
        default_style {}
        p { "Not found :(" }
    }
}

fn default_style() -> Element {
    rsx! {
            document::Style {
            {
                format!(
                    " @font-face {{ font-family: 'Roboto Light'; src: url({}) format('truetype');}} ",
                    asset!("/assets/fonts/Roboto-Light.ttf"),
                )
            }
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
    }
}
