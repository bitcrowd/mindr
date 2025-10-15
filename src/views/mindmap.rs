use crate::components::LocationIndicator;
use crate::components::MiniMap;
use crate::components::Node;
use crate::components::NodeLink;
use crate::data::Store;
use dioxus::prelude::*;

#[component]
pub fn Mindmap() -> Element {
    let store = Store::new();
    let mut pane = store.pane;
    let mut graph = store.graph;

    let t = *store.pane.transform.read();
    let mut size = use_signal(|| (0f32, 0f32));

    let drop_target = match *store.pane.drop_target.read() {
        Some((target_id, location)) => Some((graph.get_node(target_id), location)),
        None => None,
    };

    let mut links = Vec::new();
    let mut nodes = Vec::new();
    let mut dragging_node = rsx! {};
    for node in graph.nodes.read().values() {
        // Links first
        if let Some(parent_id) = node.parent_id {
            links.push(rsx! {
                NodeLink { id: node.id, parent_id, store: store.clone() }
            });
        }

        if Some(node.id) == *pane.dragging.read() {
            dragging_node = rsx! {
                Node { id: node.id, store: store.clone() }
            };
        } else {
            nodes.push(rsx! {
                Node { id: node.id, store: store.clone() }
            });
        }
    }
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
                pane.minimap_dragging.set(false);
                pane.panning.set(false);
                if let (Some(id), Some(target)) = (
                    *pane.dragging.read(),
                    *pane.drop_target.read(),
                ) {
                    graph.move_node_into(id, target);
                }
                pane.dragging.set(None);
                pane.drop_target.set(None);
            },
            onmouseleave: move |_| {
                pane.dragging.set(None);
                pane.minimap_dragging.set(false);
                pane.panning.set(false);
                pane.drop_target.set(None)
            },
            onmousemove: move |evt| {
                let coords = evt.element_coordinates();
                let (mx, my) = pane.transform(coords.x as f32, coords.y as f32);
                if let Some(node_id) = *pane.dragging.read() {
                    let (ox, oy) = *pane.drag_offset.read();
                    let (x, y) = (mx - ox, my - oy);
                    graph.move_node(node_id, x, y);
                    let mut drop_target = None;
                    for node in graph.nodes.read().values() {
                        if let Some(location) = node.on(x, y) {
                            if node.id != node_id {
                                drop_target = Some((node.id, location));
                            }
                        }
                    }
                    pane.drop_target.set(drop_target);
                }
                if *pane.panning.read() {
                    let (start_x, start_y) = *pane.pan_offset.read();
                    pane.transform.write().pan_x += coords.x as f32 - start_x;
                    pane.transform.write().pan_y += coords.y as f32 - start_y;
                    pane.pan_offset.set((coords.x as f32, coords.y as f32));
                }
            },
            onmousedown: move |evt| {
                let coords = evt.element_coordinates();
                if None == *pane.editing.read() {
                    let (x, y) = pane.transform(coords.x as f32, coords.y as f32);
                    if let Some(node) = graph.on(x, y) {
                        pane.dragging.set(Some(node.id));
                        let ox = x - node.x;
                        let oy = y - node.y;
                        pane.drag_offset.set((ox, oy));
                    } else {
                        pane.panning.set(true);
                        pane.pan_offset.set((coords.x as f32, coords.y as f32));
                    }
                } else {
                    pane.editing.set(None)
                }
            },
            ondoubleclick: move |evt| {
                let coords = evt.element_coordinates();
                let (x, y) = pane.transform(coords.x as f32, coords.y as f32);
                dbg!(graph.on(x, y));
                if let Some(node) = graph.on(x, y) {
                    pane.editing.set(Some(node.id));
                } else {
                    graph.add_node(x, y)
                }
            },
            g { transform: format!("translate({},{}) scale({})", t.pan_x, t.pan_y, t.scale),
                for link in links {
                    {link}
                }
                for node in nodes {
                    {node}
                }

                {dragging_node}

                if let Some((Some(target), location)) = drop_target {
                    g { transform: format!("translate({},{})", target.x + target.width() / 2.0 + 8.0, target.y - 8.0),
                        LocationIndicator { location }
                    }
                }
            }

            MiniMap { store: store.clone(), svg_size: size.clone() }
        }
    }
}
