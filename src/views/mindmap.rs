use crate::components::LocationIndicator;
use crate::components::MiniMap;
use crate::components::Node;
use crate::components::NodeLink;
use crate::components::RawNode;
use crate::data::Store;
use dioxus::prelude::*;

#[component]
pub fn Mindmap() -> Element {
    let store = Store::new();
    let mut pane = store.pane;
    let mut graph = store.graph;

    let t = *store.pane.transform.read();
    let mut size = use_signal(|| (0f32, 0f32));

    let mut links = Vec::new();
    let mut nodes = Vec::new();
    graph.for_each_node(|node| {
        // Links first
        if let Some(parent_id) = node.parent_id {
            links.push(rsx! {
                NodeLink { id: node.id, parent_id, graph: graph.clone() }
            });
        }

        nodes.push(rsx! {
            Node { id: node.id, store: store.clone(), key: node.id }
        });
    });
    rsx! {
        svg {
            width: "100%",
            style: "height: calc(100vh - 2em); background:#fafafa;cursor:grab; user-select: none; z-index: 999",
            tabindex: "0",
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
                if let Some(dragging_node) = *pane.dragging_node.read() {
                    if let Some(target) = dragging_node.target {
                        graph.move_node_into(dragging_node.id, target);
                    } else {
                        graph.try_move_node(dragging_node.id, dragging_node.coords);
                    }
                }
                pane.dragging_node.set(None);
            },
            onmouseleave: move |_| {
                pane.dragging_node.set(None);
                pane.minimap_dragging.set(false);
                pane.panning.set(false);
            },
            onmousemove: move |evt| {
                let coords = evt.element_coordinates();
                let svg_coords = pane.transform(coords.x as f32, coords.y as f32);
                pane.update_drag(svg_coords, graph.on(svg_coords));
                if *pane.panning.read() {
                    let (start_x, start_y) = *pane.pan_offset.read();
                    pane.transform.write().pan_x += coords.x as f32 - start_x;
                    pane.transform.write().pan_y += coords.y as f32 - start_y;
                    pane.pan_offset.set((coords.x as f32, coords.y as f32));
                }
            },
            onmousedown: move |evt| {
                let coords = evt.element_coordinates();
                let svg_coords = pane.transform(coords.x as f32, coords.y as f32);
                if let Some((target_id, _)) = graph.on(svg_coords) {
                    let node = graph.get_node(target_id).unwrap();
                    pane.start_drag(&node, svg_coords);
                } else {
                    pane.panning.set(true);
                    pane.pan_offset.set((coords.x as f32, coords.y as f32));
                    pane.editing.set(None);
                }
                evt.prevent_default();
            },
            ondoubleclick: move |evt| {
                let coords = evt.element_coordinates();
                let svg_coords = pane.transform(coords.x as f32, coords.y as f32);
                if let Some((node_id, _)) = graph.on(svg_coords) {
                    pane.editing.set(Some(node_id));
                } else {
                    let node_id = graph.add_node(svg_coords);
                    pane.editing.set(Some(node_id));
                }
            },
            onkeydown: move |evt| {
                let editing = *store.pane.editing.read();
                match evt.key() {
                    Key::Enter => {
                        if !evt.modifiers().shift() {
                            if let Some(id) = editing {
                                let id = graph.add_sibling(id);
                                pane.editing.set(Some(id));
                            }
                        }
                    }
                    Key::Tab => {
                        if let Some(id) = editing {
                            let dir = if evt.modifiers().shift() { -1.0 } else { 1.0 };
                            let id = graph.add_child(id, dir);
                            pane.editing.set(Some(id));
                        }
                    }
                    _ => {}
                }
            },

            g { transform: format!("translate({},{}) scale({})", t.pan_x, t.pan_y, t.scale),
                for link in links {
                    {link}
                }
                for node in nodes {
                    {node}
                }

                if let Some(dragging_node) = *pane.dragging_node.read() {
                    if let Some(node) = graph.get_node(dragging_node.id) {
                        g { transform: format!("translate({},{})", dragging_node.coords.0, dragging_node.coords.1),
                            RawNode { node }
                        }
                        if let Some((_, location)) = dragging_node.target {
                            g {
                                transform: format!(
                                    "translate({},{})",
                                    dragging_node.coords.0 + 8.0,
                                    dragging_node.coords.1 - 8.0,
                                ),
                                LocationIndicator { location }
                            }
                        }
                    }
                }
            }

            MiniMap { store: store.clone(), svg_size: size.clone() }
        }
    }
}
