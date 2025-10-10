use crate::components::MiniMap;
use crate::components::Node;
use crate::data::Store;
use dioxus::prelude::*;

#[component]
pub fn Mindmap() -> Element {
    let mut store = Store::new();
    let mut pane = store.pane;
    let mut graph = store.graph;

    let t = *store.pane.transform.read();
    let mut size = use_signal(|| (0f32, 0f32));
    rsx! {
        svg {
            width: "100%",
            style: "height: calc(100vh - 2em); background:#fafafa;cursor:grab; user-select: none; z-index: 999",
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
                pane.dragging.set(None);
                pane.minimap_dragging.set(false);
                store.pane.panning.set(false);

                        pane.drop_target.set(None)
            },
            onmouseleave: move |_| {
                pane.dragging.set(None);
                pane.minimap_dragging.set(false);
                pane.panning.set(false);
                        pane.drop_target.set(None)
            },
            onmousemove: move |evt| {
                let coords = evt.client_coordinates();
                let (mx, my) = pane.transform(coords.x as f32, coords.y as f32);
                if let Some(node_id) = *pane.dragging.read() {
                    let (ox, oy) = *pane.drag_offset.read();
                    let (x, y) = (mx - ox, my - oy);
                    graph.move_node(node_id, x, y);

                    for node in graph.nodes.read().values() {
                      if let Some(location) = node.on(x,y) {
                        pane.drop_target.set(Some((node_id, location)))
                      }
                    }
                }
                if *pane.panning.read() {
                    let (start_x, start_y) = *pane.pan_offset.read();
                    pane.transform.write().pan_x += coords.x as f32 - start_x;
                    pane.transform.write().pan_y += coords.y as f32 - start_y;
                    pane.pan_offset.set((coords.x as f32, coords.y as f32));
                }
            },
            onmousedown: move |evt| {
                evt.prevent_default();
                let coords = evt.client_coordinates();
                pane.panning.set(true);
                pane.pan_offset.set((coords.x as f32, coords.y as f32));
            },

            ondoubleclick: move |evt| {
                let coords = evt.client_coordinates();
                let (mx, my) = pane.transform(coords.x as f32, coords.y as f32);
                graph.add_node(mx, my)
            },
            g { transform: format!("translate({},{}) scale({})", t.pan_x, t.pan_y, t.scale),
                for node in graph.nodes.read().values() {
                    Node { id: node.id, store: store.clone() }
                }
            }
            MiniMap { store: store.clone(), svg_size: size.clone() }
        }
    }
}
