use dioxus::prelude::*;

use components::Mindmap;

mod components;
mod data;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/css/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
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
        Mindmap {}
    }
}
