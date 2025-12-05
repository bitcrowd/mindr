use crate::data::{NodeProperty, Store};
use dioxus::prelude::*;
use regex::Regex;

use dioxus_free_icons::icons::ld_icons::{LdClock, LdPalette, LdPercent};
use dioxus_free_icons::Icon;

const FLOAT_PATTERN: &str = r"^\d+([.,]\d+)?$";

#[component]
pub fn Sidebar(store: Store) -> Element {
    let selected_node = store
        .pane
        .selected
        .read()
        .and_then(|id| store.graph.get_node(id));

    if let Some(node) = selected_node {
        let float_regex = Regex::new(FLOAT_PATTERN).unwrap();
        rsx! {

            div { class: "sidebar",
                div { class: "sidebar__section",

                    Icon { icon: LdClock, class: "sidebar__icon" }
                    input {
                        r#type: "text",
                        pattern: FLOAT_PATTERN,
                        inputmode: "decimal",
                        class: "sidebar__estimate-input",
                        value: "{node.estimate.map(|e| e.to_string()).unwrap_or_default()}",
                        oninput: move |evt| {
                            if evt.value().is_empty() {
                                store.graph.update_node(node.id, NodeProperty::NoEstimate)
                            } else {
                                let normalized = evt.value().replace(',', ".");
                                if float_regex.is_match(&normalized) {
                                    if let Ok(num) = normalized.parse::<f64>() {
                                        store
                                            .graph
                                            .update_node(
                                                node.id,
                                                NodeProperty::Estimate(num.clamp(0.0, 10000.0)),
                                            )
                                    }
                                }
                            }
                        },
                    }
                    div { class: "sidebar__estimate-buttons",
                        for v in [0.5f64, 1f64, 2f64, 3f64, 5f64, 8f64] {
                            button {
                                onclick: move |_| {
                                    store.graph.update_node(node.id, NodeProperty::Estimate(v));
                                },
                                "{v}"
                            }
                        }
                    }
                }
                div { class: "sidebar__section",
                    Icon { icon: LdPercent, class: "sidebar__icon" }
                    input {
                        r#type: "number",
                        value: "{node.progress}",
                        min: "0",
                        max: "100",
                        class: "sidebar__progress-input",
                        oninput: move |evt| {
                            if let Ok(num) = evt.value().parse::<i64>() {
                                if (0..=100).contains(&num) {
                                    store.graph.update_node(node.id, NodeProperty::Progress(num))
                                }
                            } else if evt.value().is_empty() {
                                store.graph.update_node(node.id, NodeProperty::Progress(0))
                            }
                        },
                    }

                    input {
                        r#type: "range",
                        class: "sidebar__progress-slider",
                        value: node.progress,
                        oninput: move |evt| {
                            if let Ok(num) = evt.value().parse::<i64>() {
                                store.graph.update_node(node.id, NodeProperty::Progress(num))
                            }
                        },
                        min: "0",
                        max: "100",
                    }
                }

                div { class: "sidebar__section",
                    Icon { icon: LdPalette, class: "sidebar__icon" }
                    for c in crate::data::graph::COLORS {
                        div {
                            class: "sidebar__color",
                            style: "background: {c};",
                            onclick: move |_| { store.graph.update_node(node.id, NodeProperty::Color(c.to_string())) },
                        }
                    }
                }
            }
        }
    } else {
        rsx! {}
    }
}
