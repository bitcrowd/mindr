use crate::data::{NodeProperty, Store};
use dioxus::prelude::*;

#[component]
pub fn Sidebar(store: Store) -> Element {
    let selected_node = store
        .pane
        .selected
        .read()
        .and_then(|id| store.graph.get_node(id));
    rsx! {
        if let Some(node) = selected_node {
            div { class: "sidebar",
                textarea {
                    class: "sidebar-textarea",
                    value: node.text,
                    oninput: move |evt| {
                        store.graph.update_node(id, NodeProperty::Text(evt.value().clone()));
                    },
                }
            }
        }
    }
}
