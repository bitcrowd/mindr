use crate::data::RelativeLocation;
use crate::data::Store;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn LocationIndicator(location: RelativeLocation) -> Element {
    use RelativeLocation::*;
    let rotation = match location {
        Top => -90.0,
        Bottom => 90.0,
        Left | Right | Center => 0.0,
    };

    rsx! {
        path {
            // fill_rule: "evenodd",
            d: "M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8",
            transform: format!("rotate({} 8 8)", rotation),
        }
    }
}

#[component]
pub fn Node(id: Uuid, store: Store) -> Element {
    let node = store.graph.get_node(id).unwrap();
    let width = node.width();
    let height = node.height();
    let font_size = node.font_size();
    let drop_target = *store.pane.drop_target.read();
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
            if let Some((target_id, location)) = drop_target {
                if target_id == id {
                    g { transform: format!("translate({},{})", width / 2.0 + 8.0, -8),
                        LocationIndicator { location }
                    }
                }
            }
        }
    }
}
