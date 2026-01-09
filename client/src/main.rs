use dioxus::prelude::*;

mod components;
mod data;
mod router;

use router::Route;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
