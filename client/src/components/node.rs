use crate::data::{NodeProperty, Store};
use dioxus::prelude::*;
use std::rc::Rc;
use uuid::Uuid;

const SELECTED_PADDING: f32 = 5.0;

#[component]
pub fn DraggedNode(id: Uuid, coords: (f32, f32)) -> Element {
    rsx! {
        g { transform: format!("translate({}, {})", coords.0, coords.1),
            r#use {
                href: format!("#{id}"),
                style: "transform: scale(0.5); opacity: 0.3;",
            }
        }
    }
}

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
    let mut input_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    use_effect(move || {
        if let Some(input) = &*input_element.read() {
            let _ = input.set_focus(is_editing);
        }
    });

    rsx! {
        g { transform: format!("translate({},{})", node.x, node.y),
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

                foreignObject {
                    x: format!("{}", -width / 2.0),
                    y: format!("{}", -height / 2.0),
                    width: format!("{}", width),
                    height: format!("{}", height),
                    if *store.pane.editing.read() == Some(node.id) {
                        textarea {
                            style: if store.pane.dragging_node.read().is_some() { "pointer-events: none;" } else { "" },
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
                              margin: 0px 10px;
                              padding: 9px 0px;
                              font-family: inherit;
                              width: 100%;
                              height: 100%;
                              outline:none;
                              background: transparent;
                              border: none;
                              resize:none;
                              overflow:hidden;
                              font-size: {font_size}px;
                              display: block;
                              line-height: 1.2",
                            oninput: move |evt| {
                                store.graph.update_node(id, NodeProperty::Text(evt.value().clone()));
                            },
                        }
                    } else {
                        div {
                            style: "
                                vertical-align: top;
                                line-height: 1.2;
                                padding-left: 2px;
                                display: flex;
                                justify-content: center;
                                align-items: center;
                                width: 100%;
                                height: 100%;
                                overflow: hidden;
                                white-space: pre-wrap;
                                word-wrap: break-word;
                                text-align: center;
                                font-size: {font_size}px;
                                background: transparent;
                                color: black;
                                pointer-events: none;
                                -webkit-user-select: none;
                                -moz-user-select: none;
                                -ms-user-select: none;
                                user-select: none;
                                font-family: inherit;",
                            {node.text}
                        }
                    }
                }
            }
        }
    }
}
