use crate::components::MiniMap;
use crate::components::NodeBox;
use crate::data::Store;
use dioxus::prelude::*;

#[component]
pub fn Mindmap() -> Element {
    let mut store = Store::new();
    let panning = *store.pane.panning.read();
    let dragging = *store.pane.dragging.read();
    let dragging_str = dragging
        .map(|v| format!("{}", v))
        .unwrap_or('-'.to_string());

    let t = *store.pane.transform.read();
    let mut size = use_signal(|| (0f32, 0f32));
    rsx! {
        div { "panning {panning} {dragging_str}" }
        svg {
            width: "100%",
            style: "height: calc(100vh - 3em); background:#fafafa;cursor:grab;",
            onresize: move |evt| {
                match evt.data.get_border_box_size() {
                    Ok(sz) => {
                        size.set((sz.width as f32, sz.height as f32));
                    }
                    Err(_) => {
                        size.set((800f32, 600f32));
                    }
                }
            },
            onmouseup: move |_| {
                store.pane.dragging.set(None);
                store.pane.minimap_dragging.set(false);
                store.pane.panning.set(false);
            },
            onmouseleave: move |_| {
                store.pane.dragging.set(None);
                store.pane.minimap_dragging.set(false);
                store.pane.panning.set(false);
            },
            onmousemove: move |evt| {
                let coords = evt.client_coordinates();
                let (mx, my) = store.pane.transform(coords.x as f32, coords.y as f32);
                if let Some(node_id) = *store.pane.dragging.read() {
                    let (ox, oy) = *store.pane.drag_offset.read();
                    store.graph.move_node(node_id, mx - ox, my - oy)
                }
                if *store.pane.panning.read() {
                    let (start_x, start_y) = *store.pane.pan_offset.read();
                    store.pane.transform.write().pan_x += coords.x as f32 - start_x;
                    store.pane.transform.write().pan_y += coords.y as f32 - start_y;
                    store.pane.pan_offset.set((coords.x as f32, coords.y as f32));
                }
            },
            onmousedown: move |evt| {
                let coords = evt.client_coordinates();
                store.pane.panning.set(true);
                store.pane.pan_offset.set((coords.x as f32, coords.y as f32));
            },

            ondblclick: move |evt| {
                let coords = evt.client_coordinates();
                let (mx, my) = store.pane.transform(coords.x as f32, coords.y as f32);
                store.graph.add_node(mx, my)
            },
            g { transform: format!("translate({},{}) scale({})", t.pan_x, t.pan_y, t.scale),
                for node in store.graph.nodes.read().values() {
                    NodeBox { id: node.id, store: store.clone() }
                }
            }
            MiniMap { store: store.clone(), svg_size: size.clone() }
        }
    }
}
