use crate::data::node::measure_line_height;
use crate::data::{NodeProperty, Store, FONT_SIZE, TEXT_PADDING};
use dioxus::prelude::*;
use std::rc::Rc;
use uuid::Uuid;

const SELECTED_PADDING: f32 = 5.0;

#[component]
fn RawChildNode(width: f32, height: f32, color: String) -> Element {
    rsx! {
        rect {
            x: format!("{}", -width / 2.0),
            y: format!("{}", -height / 2.0),
            width: format!("{}", width),
            height: format!("{}", height),
            rx: "12",
            ry: "12",
            fill: "{color}",
            stroke: "black",
            "stroke-width": "1.5",
        }
    }
}

#[component]
fn RawRootNode(width: f32, height: f32, color: String) -> Element {
    rsx! {
        rect {
            x: format!("{}", -width / 2.0),
            y: format!("{}", -height / 2.0),
            width: format!("{}", width + 4.0),
            height: format!("{}", height + 4.0),
            rx: "20",
            ry: "20",
            fill: "rgba(0,0,0,0.3)",
        }

        rect {
            x: format!("{}", -width / 2.0),
            y: format!("{}", -height / 2.0),
            width: format!("{}", width),
            height: format!("{}", height),
            rx: "20",
            ry: "20",
            fill: "{color}",
            stroke: "black",
            "stroke-width": "2",
        }
    }
}

#[component]
fn NodeLabel(label: String) -> Element {
    rsx! {
        for (index , line) in label.lines().enumerate() {
            text {
                y: index as f32 * measure_line_height() + TEXT_PADDING,
                x: TEXT_PADDING,
                text_anchor: "start",
                dominant_baseline: "text-before-edge",
                font_size: FONT_SIZE,
                "{line}"
            }
        }
    }
}

#[component]
fn RawNode(node: crate::data::RenderedNode) -> Element {
    let width = node.width();
    let height = node.height();
    let color = node.rendered_color.clone();
    rsx! {
        if node.parent_id.is_none() {
            RawRootNode { width, height, color }
        } else {
            RawChildNode { width, height, color }
        }
    }
}

const ESTIMATE_FONT_SIZE: f32 = 11f32;
const ESTIMATE_PADDING: f32 = 4f32;
const ESTIMATE_ICON_SIZE: f32 = 12.0f32;
const ESTIMATE_ICON_SPACING: f32 = 2.0f32;
#[component]
pub fn Estimate(estimate: f64) -> Element {
    let approx_char_width = ESTIMATE_FONT_SIZE * 0.6;
    let text = format!("{:.2}", estimate)
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string();
    let text_width = text.len() as f32 * approx_char_width;

    let width = text_width + ESTIMATE_PADDING * 2.0 + ESTIMATE_ICON_SIZE + ESTIMATE_ICON_SPACING;
    let height = ESTIMATE_FONT_SIZE + ESTIMATE_PADDING * 2.0;
    let radius = height / 2.0;

    rsx! {
        rect {
            x: "-{width / 2.0}",
            y: "-{height / 2.0}",
            width: "{width}",
            height: "{height}",
            rx: "{radius}",
            ry: "{radius}",
            fill: "#2c3e50",
            stroke: "#444",
            stroke_width: "0",
        }

        text {
            x: "{(ESTIMATE_ICON_SIZE + ESTIMATE_ICON_SPACING) / 2.0}",
            y: "1",
            fill: "#fff",
            font_size: "{ESTIMATE_FONT_SIZE}",
            text_anchor: "middle",
            dominant_baseline: "middle",
            font_family: "sans-serif",
            "{text}"
        }
        g {
            transform: format!(
                "translate({},{}), scale(0.5)",
                -(width / 2.0) + ESTIMATE_ICON_SPACING + 2.0,
                -ESTIMATE_ICON_SIZE / 2.0,
            ),
            fill: "none",
            stroke: "#fff",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d: "M12 6v6l4 2" }
            circle { cx: "12", cy: "12", r: "10" }
        }
    }
}

#[component]
pub fn Node(id: Uuid, store: Store) -> Element {
    let node = store.graph.get_node(id).unwrap();
    let width = node.width();
    let height = node.height();
    let font_size = node.font_size();
    let is_editing = *store.pane.editing.read() == Some(id);
    let (node_x, node_y) = store.pane.coords(&node);
    let mut input_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        if let Some(input) = &*input_element.read() {
            let _ = input.set_focus(is_editing);
        }
    });

    rsx! {
        g { transform: format!("translate({},{})", node_x, node_y),
            g {
                onmousedown: move |evt| {
                    if is_editing {
                        evt.stop_propagation();
                    }
                },
                ondoubleclick: move |evt| {
                    if is_editing {
                        evt.stop_propagation();
                    }
                },
                style: if is_editing { "" } else { "pointer-events: none;" },
                id: "{node.id}",
                RawNode { node: node.clone() }

                if *store.pane.selected.read() == Some(id) {
                    rect {
                        x: format!("{}", (-width / 2.0) - SELECTED_PADDING),
                        y: format!("{}", (-height / 2.0) - SELECTED_PADDING),
                        width: format!("{}", width + SELECTED_PADDING * 2.0),
                        height: format!("{}", height + SELECTED_PADDING * 2.0),
                        rx: "12",
                        ry: "12",
                        stroke: "red",
                        fill: "none",
                        "stroke-width": "1",
                        "stroke-dasharray": "4",
                    }
                }

                if node.estimate_rollup > 0.0 {
                    g { transform: format!("translate({},{})", 0, node.height() / 2.0 + 3.0),
                        Estimate { estimate: node.estimate_rollup }
                    }
                }

                if *store.pane.editing.read() == Some(node.id) {
                    foreignObject {
                        x: format!("{}", -width / 2.0),
                        y: format!("{}", -height / 2.0),
                        width: format!("{}", width),
                        height: format!("{}", height),
                        textarea {
                            key: "{id}-textarea",
                            onmounted: move |element| input_element.set(Some(element.data())),
                            value: "{node.text}",
                            autofocus: true,
                            autocomplete: "off",
                            autocapitalize: "off",
                            spellcheck: "false",
                            tabindex: -1,
                            style: "
                              user-select: none;
                              margin: 0px;
                              padding: {TEXT_PADDING}px {TEXT_PADDING}px;
                              font-family: inherit;
                              width: {width}px;
                              height: {height}px;
                              outline:none;
                              background: transparent;
                              border: none;
                              resize:none;
                              overflow:hidden;
                              font-size: {font_size}px;
                              display: block;
                              line-height: {measure_line_height() + 0.3}px",
                            oninput: move |evt| {
                                store.graph.update_node(id, NodeProperty::Text(evt.value().clone()));
                            },
                        }
                    }
                } else {
                    g { transform: format!("translate({},{})", -width / 2.0, -height / 2.0),

                        NodeLabel { label: node.text }
                    }
                }
            }
        }
    }
}
