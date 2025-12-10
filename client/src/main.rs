#[cfg(feature = "desktop")]
use dioxus::desktop::{muda::*, use_muda_event_handler};

use dioxus::prelude::*;

use components::Mindmap;

mod components;
mod data;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/css/main.css");

fn main() {
    #[cfg(feature = "desktop")]
    {
        let menu = Menu::new();
        let edit_menu = Submenu::new("Edit", true);

        edit_menu
            .append_items(&[&MenuItem::with_id("save", "Save", true, None)])
            .unwrap();

        menu.append_items(&[&edit_menu]).unwrap();

        // Create a desktop config that overrides the default menu with the custom menu
        let config = dioxus::desktop::Config::new().with_menu(menu);

        // Launch the app with the custom menu
        dioxus::LaunchBuilder::new().with_cfg(config).launch(App);
        return;
    }

    #[cfg(not(feature = "desktop"))]
    {
        dioxus::launch(App);
    }
}

#[component]
fn App() -> Element {
    #[cfg(feature = "desktop")]
    {
        use_muda_event_handler(move |muda_event| {
            if muda_event.id() == "save" {
                save_to_file();
            }
        });
    }

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

#[cfg(feature = "desktop")]
fn save_to_file() {
    println!("{}", "Still to be implemented");
}
