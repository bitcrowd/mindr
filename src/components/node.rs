use crate::data::Store;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn Node(id: Uuid, store: Store) -> Element {
    let node = store.graph.get_node(id).unwrap();
    let width = node.width();
    let height = node.height();
    let font_size = node.font_size();
    // let mut input_value = use_signal(|| node.text.clone());
    rsx! {
            g {
                transform: format!("translate({},{})", node.x, node.y),
                style: "pointer-events: none;",
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
                        fill: "lightblue",
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
                        fill: "lightblue",
                        stroke: "black",
                        "stroke-width": "1.5",
                    }
                }
                text {
                    x: "0",
                    y: "5",
                    "text-anchor": "middle",
                    "font-size": "{font_size}",
    style: "user-select:none; -webkit-user-select:none; -moz-user-select:none; -ms-user-select:none; pointer-events:none;",
                    "{node.text}"
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
            }
        }
}
