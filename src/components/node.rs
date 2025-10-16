use crate::data::Store;
use dioxus::prelude::*;
use std::rc::Rc;
use uuid::Uuid;

#[component]
pub fn Node(id: Uuid, store: Store) -> Element {
    let node = store.graph.get_node(id).unwrap();
    let width = node.width();
    let height = node.height();
    let font_size = node.font_size();
    let is_editing = *store.pane.editing.read() == Some(id);
    let mut input_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    if let Some(input) = &*input_element.read() {
        let _ = input.set_focus(is_editing);
    }
    rsx! {
        g {
            transform: format!("translate({},{})", node.x, node.y),
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
            if None == node.parent_id {
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
                    fill: "{node.color}",
                    stroke: "black",
                    "stroke-width": "2",
                }
            } else {
                rect {
                    x: format!("{}", -width / 2.0),
                    y: format!("{}", -height / 2.0),
                    width: format!("{}", width),
                    height: format!("{}", height),
                    rx: "12",
                    ry: "12",
                    fill: "{node.color}",
                    stroke: "black",
                    "stroke-width": "1.5",
                }
            }


            if *store.pane.editing.read() == Some(id) {
                circle {
                    cx: format!("{}", width / 2.0),
                    cy: format!("{}", height / 2.0),
                    r: "10",
                    fill: "red",
                    stroke: "black",
                }
            }
            foreignObject {
                x: format!("{}", -width / 2.0),
                y: format!("{}", -height / 2.0),
                width: format!("{}", width),
                height: format!("{}", height),
                style: "user-select: none;",
                textarea {

                    style: if store.pane.dragging.read().is_some() { "pointer-events: none;" } else { "" } ,
                    key: "{id}-textarea",
                    onmounted: move |element| input_element.set(Some(element.data())),
                    value: "{node.text}",
                    autofocus: "true",
                    autocomplete: "off",
                    autocapitalize: "off",
                    spellcheck: "false",
                    style: "user-select: none; padding-top: 7px; padding-bottom: 10px; width: 100%; height: 100%; outline:none; background: transparent; border: none; resize:none; overflow:hidden; text-align: center; font-size: {font_size}px; display: block",
                    oninput: move |evt| {
                        store.graph.update_node_text(id, evt.value().clone());
                        store.graph.layout_all();
                    },
                    onkeypress: move |evt| {
                        if evt.key() == Key::Enter {
                            if !evt.modifiers().shift() {
                                store.pane.editing.set(None);
                                evt.prevent_default();
                                if let Some(input) = &*input_element.read() {
                                    let _ = input.set_focus(false);
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}
