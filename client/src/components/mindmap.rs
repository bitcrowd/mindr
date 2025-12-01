use crate::components::DraggedNode;
use crate::components::LocationIndicator;
use crate::components::MiniMap;
use crate::components::Node;
use crate::components::NodeLink;
use crate::components::Sidebar;
use crate::data::RelativeLocation;
use crate::data::Store;
use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn Mindmap() -> Element {
    let store = Store::new();
    let mut pane = store.pane;
    let mut graph = store.graph;

    let t = *store.pane.transform.read();
    let mut size = use_signal(|| (0f32, 0f32));

    let mut container: Signal<Option<Rc<MountedData>>> = use_signal(|| None);

    let mut nodes = Vec::new();
    let dragging_id = (*pane.dragging_node.read()).map(|n| n.id);
    graph.for_each_node(|node| {
        nodes.push((
            node.id,
            rsx! {
                if let Some(parent_id) = node.parent_id {
                    NodeLink { id: node.id, parent_id, store: store.clone() }
                }
                Node { id: node.id, store: store.clone(), key: node.id }
            },
        ));
    });
    nodes.sort_by_key(|(id, _)| {
        let root_id = graph.get_root(*id);
        (dragging_id != Some(root_id), root_id)
    });
    rsx! {
        div {
            div {
                class: "wrapper",
                tabindex: 0,
                // autofocus: pane.editing.read().is_none(),
                onmounted: move |element| container.set(Some(element.data())),
                onkeydown: move |evt| {
                    let selected = *store.pane.selected.read();
                    let editing = *store.pane.editing.read();
                    let shift = evt.modifiers().shift();
                    match evt.key() {
                        Key::Enter => {
                            if let Some(id) = editing {
                                if !shift {
                                    let id = graph.add_sibling(id);
                                    pane.editing.set(Some(id));
                                    pane.selected.set(Some(id));
                                    evt.prevent_default();
                                }
                            } else if let Some(id) = selected {
                                pane.editing.set(Some(id));
                            }
                        }
                        Key::Tab => {
                            if let Some(id) = selected {
                                let dir = if shift {
                                    RelativeLocation::Left
                                } else {
                                    RelativeLocation::Right
                                };
                                let id = graph.add_child(id, dir);
                                pane.editing.set(Some(id));
                                pane.selected.set(Some(id));
                            }
                            evt.prevent_default();
                        }
                        Key::Escape => {
                            if pane.editing.read().is_none() {
                                pane.selected.set(None);
                            } else {
                                pane.editing.set(None);
                            }
                            if let Some(div) = &*container.read() {
                                let _ = div.set_focus(true);
                            }
                            evt.prevent_default();
                            evt.stop_propagation();
                        }
                        Key::Backspace => {
                            if pane.editing.read().is_none() {
                                if let Some(id) = *pane.selected.read() {
                                    if shift {
                                        graph.delete_node(id);
                                    } else {
                                        graph.delete_branch(id);
                                    }
                                }
                                pane.dragging_node.set(None);
                                pane.selected.set(None);
                            }
                        }
                        _ => {}
                    }
                },
                svg {
                    class: "mindmap",
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
                            graph
                                .move_node_into(
                                    dragging_node.id,
                                    dragging_node.coords,
                                    dragging_node.target,
                                );
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
                        let dragging_node = *pane.dragging_node.read();
                        if let Some(dragging_node) = dragging_node {
                            let target = graph.on_other(dragging_node.id, svg_coords);
                            pane.update_drag(svg_coords, target);
                            graph.move_node(dragging_node.id, dragging_node.coords);
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
                        let svg_coords = pane.transform(coords.x as f32, coords.y as f32);
                        if let Some((target_id, _)) = graph.on(svg_coords) {
                            if let Some(node) = graph.get_node(target_id) {
                                pane.start_drag(&node, svg_coords);
                                pane.selected.set(Some(node.id));
                                pane.editing.set(None);
                            }
                        } else {
                            pane.panning.set(true);
                            pane.pan_offset.set((coords.x as f32, coords.y as f32));
                            pane.editing.set(None);
                            pane.selected.set(None);
                        }
                    },
                    ondoubleclick: move |evt| {
                        let coords = evt.element_coordinates();
                        let svg_coords = pane.transform(coords.x as f32, coords.y as f32);
                        if let Some((node_id, _)) = graph.on(svg_coords) {
                            pane.editing.set(Some(node_id));
                        } else {
                            let node_id = graph.add_root_node(svg_coords);
                            pane.editing.set(Some(node_id));
                            pane.selected.set(Some(node_id));
                        }
                    },

                    g { transform: format!("translate({},{}) scale({})", t.pan_x, t.pan_y, t.scale),
                        for (_ , rendered) in nodes {
                            {rendered}
                        }

                        if let Some(dragging_node) = *pane.dragging_node.read() {
                            if let Some(node) = graph.get_node(dragging_node.id) {
                                if node.parent_id.is_some() {
                                    DraggedNode {
                                        id: node.id,
                                        coords: dragging_node.coords,
                                    }
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

                    MiniMap { store: store.clone(), svg_size: size }
                }
            }
            Sidebar { store: store.clone() }
        }
    }
}
