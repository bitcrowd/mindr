use crate::data::Store;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn Node(id: Uuid, store: Store) -> Element {
    let node = store.graph.get_node(id).unwrap();
    let width = node.width();
    let height = node.height();
    let font_size = node.font_size();
    rsx! {
        g {

            transform: format!("translate({},{})", node.x, node.y),
            onmousedown: move |evt| {
                let t = *store.pane.transform.read();
                evt.stop_propagation();
                store.pane.dragging.set(Some(id));
                let coords = evt.client_coordinates();
                let ox = (coords.x as f32 - t.pan_x) - node.x;
                let oy = (coords.y as f32 - t.pan_y) - node.y;
                store.pane.drag_offset.set((ox, oy));
            },

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
            text {
                x: "0",
                y: "5",
                "text-anchor": "middle",
                "font-size": "{font_size}",
                "{node.text}"
            }
        }
    }
}
