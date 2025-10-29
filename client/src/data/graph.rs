use crate::data::{CollabGraph, RelativeLocation, RenderedNode};
use crate::data::{Node, NodeKind};
use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use super::collab::Side;

const SPACING_X: f32 = 50.0; // horizontal gap between parent and child
const SPACING_Y: f32 = 10.0; // vertical gap between siblings

const COLORS: [&'static str; 8] = [
    "#ffc6ff", "#ffadad", "#ffd6a5", "#fdffb6", "#caffbf", "#9bf6ff", "#a0c4ff", "#bdb2ff",
];

#[derive(Copy, Clone, PartialEq)]
pub struct Graph {
    nodes: Signal<HashMap<Uuid, RenderedNode>, SyncStorage>,
    order: Signal<Vec<Uuid>, SyncStorage>,
    doc: Signal<CollabGraph, SyncStorage>,
    subscriptions: Signal<Vec<yrs::Subscription>>,
}

impl Graph {
    pub fn new() -> Self {
        let nodes = use_signal_sync(|| HashMap::new());
        let order = use_signal_sync(|| Vec::new());
        let doc = use_signal_sync(|| CollabGraph::new());
        let subscriptions = use_signal(|| Vec::new());

        let mut graph = Self {
            nodes,
            order,
            doc,
            subscriptions,
        };
        graph.subscribe();
        graph
    }

    fn subscribe(&mut self) {
        let mut nodes = self.nodes.clone();
        let mut order = self.order.clone();
        use_hook(|| {
            let layout_lock = Arc::new(Mutex::new(()));
            {
                let layout_lock = layout_lock.clone();
                let sub = self.doc.write().observe_nodes(move |id, node| {
                    if let Some(node) = node {
                        let node = match node.kind {
                            NodeKind::Root { coords } => {
                                RenderedNode::new(id, coords, None, node.text)
                            }
                            NodeKind::Child { parent_id, side } => {
                                let offset = match side {
                                    Side::Left => -1f32,
                                    _ => 1f32,
                                };
                                let x = nodes
                                    .read()
                                    .get(&parent_id)
                                    .map(|p| p.x + offset)
                                    .unwrap_or(0f32);
                                RenderedNode::new(id, (x, 0f32), Some(parent_id), node.text)
                            }
                        };
                        nodes.write().insert(id, node);
                    } else {
                        nodes.write().remove(&id);
                    }
                    if let Ok(_guard) = layout_lock.try_lock() {
                        UpdatedGraph {
                            nodes: nodes,
                            order: order,
                        }
                        .layout_all();
                    }
                });
                self.subscriptions.write().push(sub);
            }

            let layout_lock = layout_lock.clone();
            {
                let sub = self.doc.write().observe_order(move |new_order| {
                    order.set(new_order);
                    if let Ok(_guard) = layout_lock.try_lock() {
                        UpdatedGraph {
                            nodes: nodes,
                            order: order,
                        }
                        .layout_all();
                    }
                });

                self.subscriptions.write().push(sub);
            }
        });
    }

    pub fn get_doc(&self) -> Signal<CollabGraph, SyncStorage> {
        self.doc
    }

    pub fn add_root_node(&mut self, coords: (f32, f32)) -> Uuid {
        let node = Node::new_root(coords);
        self.doc.write().add_node(node)
    }

    pub fn add_child(&mut self, parent_id: Uuid, location: RelativeLocation) -> Uuid {
        let side = match location {
            RelativeLocation::Left => Side::Left,
            _ => Side::Right,
        };

        let node = Node::new_child(parent_id, side);
        self.doc.write().add_node(node)
    }

    pub fn add_sibling(&mut self, node_id: Uuid) -> Uuid {
        let node = if let Some(sibling) = self.get_node(node_id) {
            if let Some(parent_id) = sibling.parent_id {
                if let Some(parent) = self.get_node(parent_id) {
                    let side = if parent.x >= sibling.x {
                        Side::Left
                    } else {
                        Side::Right
                    };
                    Node::new_child(parent_id, side)
                } else {
                    Node::new_root((0f32, 0f32))
                }
            } else {
                Node::new_root((sibling.x, sibling.y + 10.0 * SPACING_Y))
            }
        } else {
            Node::new_root((0f32, 0f32))
        };
        self.doc.write().add_node(node)
    }

    pub fn update_node_text(&mut self, id: Uuid, new_text: String) {
        self.doc.write().update_node_text(id, new_text)
    }

    pub fn move_node(&mut self, id: Uuid, coords: (f32, f32)) {
        if let Some(node) = self.get_node(id) {
            if let Some(parent_id) = node.parent_id {
                if let Some(parent) = self.get_node(parent_id) {
                    let side = if parent.x > coords.0 {
                        Side::Left
                    } else {
                        Side::Right
                    };
                    self.doc.write().update_node_parent(id, parent_id, side)
                }
            } else {
                self.doc.write().update_node_coords(id, coords);
            }
        }
    }

    pub fn move_node_into(
        &mut self,
        id: Uuid,
        coords: (f32, f32),
        target: Option<(Uuid, RelativeLocation)>,
    ) {
        if let Some((target_id, location)) = target {
            if self.get_root(target_id) == Some(id) {
                self.move_node(id, coords);
            } else {
                let side = match location {
                    RelativeLocation::Left => Side::Left,
                    _ => Side::Right,
                };
                self.doc.write().update_node_parent(id, target_id, side);
            }
        } else {
            self.move_node(id, coords);
        }
    }

    pub fn get_root(&self, id: Uuid) -> Option<Uuid> {
        self.get_node(id).map(|n| {
            if let Some(parent_id) = n.parent_id {
                return self.get_root(parent_id).unwrap();
            } else {
                return id;
            }
        })
    }

    pub fn on(&self, coords: (f32, f32)) -> Option<(Uuid, RelativeLocation)> {
        let mut target = None;
        for node in self.nodes.read().values() {
            if let Some(relative_location) = node.on(coords) {
                target = Some((node.id, relative_location))
            }
        }
        target
    }

    pub fn get_node(&self, id: Uuid) -> Option<RenderedNode> {
        self.nodes.read().get(&id).cloned()
    }

    pub fn bounds(&self) -> (f32, f32, f32, f32) {
        if self.nodes.read().is_empty() {
            return (0.0, 1.0, 0.0, 1.0);
        }

        self.nodes.read().values().fold(
            (
                f32::INFINITY,
                f32::NEG_INFINITY,
                f32::INFINITY,
                f32::NEG_INFINITY,
            ),
            |(min_x, max_x, min_y, max_y), n| {
                (
                    min_x.min(n.x),
                    max_x.max(n.x),
                    min_y.min(n.y),
                    max_y.max(n.y),
                )
            },
        )
    }

    pub fn for_each_node<F>(&self, mut f: F)
    where
        F: FnMut(&RenderedNode),
    {
        for node in self.nodes.read().values() {
            f(node);
        }
    }
}

struct UpdatedGraph {
    nodes: Signal<HashMap<Uuid, RenderedNode>, SyncStorage>,
    order: Signal<Vec<Uuid>, SyncStorage>,
}

impl UpdatedGraph {
    pub fn layout_all(&mut self) {
        let root_ids: Vec<Uuid> = self
            .nodes
            .read()
            .values()
            .filter(|n| n.parent_id == None)
            .map(|n| n.id)
            .collect();

        for root_id in root_ids {
            self.layout_subtree(root_id);
            self.colorize(root_id);
        }
    }

    pub fn get_node(&self, id: Uuid) -> Option<RenderedNode> {
        self.nodes.read().get(&id).cloned()
    }

    fn colorize_with(&mut self, node_id: Uuid, color: &'static str) {
        for child_id in self.direct_children(node_id) {
            self.colorize_with(child_id, color);
        }
        if let Some(node) = self.nodes.write().get_mut(&node_id) {
            node.color = color;
        }
    }

    fn colorize(&mut self, root_id: Uuid) {
        for (i, child_id) in self.direct_children(root_id).iter().enumerate() {
            let color = COLORS[i % COLORS.len()];
            self.colorize_with(*child_id, color);
        }
    }

    fn layout_subtree(&mut self, root_id: Uuid) {
        let mut heights = std::collections::HashMap::<Uuid, f32>::new();
        self.compute_subtree_heights(root_id, &mut heights);

        self.assign_positions(root_id, &heights);
    }

    fn compute_subtree_heights(
        &self,
        node_id: Uuid,
        heights: &mut std::collections::HashMap<Uuid, f32>,
    ) -> f32 {
        let node_height = self.get_node(node_id).map(|n| n.height()).unwrap_or(0f32);

        let children = self.direct_children(node_id);

        let total_height: f32 = children
            .iter()
            .map(|&child_id| self.compute_subtree_heights(child_id, heights))
            .sum::<f32>()
            + SPACING_Y * (children.len().saturating_sub(1)) as f32;

        heights.insert(node_id, total_height.max(node_height));
        heights[&node_id]
    }

    fn direct_children(&self, node_id: Uuid) -> Vec<Uuid> {
        self.order
            .iter()
            .filter_map(|n| self.get_node(*n))
            .filter(|n| n.parent_id == Some(node_id))
            .map(|n| n.id)
            .collect()
    }

    fn spread_children_vertically(
        &mut self,
        parent_id: Uuid,
        children: &[Uuid],
        heights: &HashMap<Uuid, f32>,
        direction: f32, // +1.0 = right, -1.0 = left
    ) {
        if children.is_empty() {
            return;
        }

        if let Some(parent) = self.get_node(parent_id) {
            let total_height: f32 = children.iter().map(|id| heights[id]).sum::<f32>()
                + SPACING_Y * (children.len() as f32 - 1.0);
            let mut y = parent.y - total_height / 2.0;

            for &child_id in children {
                if let Some(child) = self.get_node(child_id) {
                    let child_x =
                        parent.x + direction * ((parent.width() + child.width()) / 2.0 + SPACING_X);
                    let child_height = heights[&child_id];
                    let target_y = y + child_height / 2.0;
                    if let Some(node) = self.nodes.write().get_mut(&child_id) {
                        node.x = child_x;
                        node.y = target_y;
                    }

                    y += child_height + SPACING_Y;

                    let grandchildren = self.direct_children(child_id);
                    self.spread_children_vertically(child_id, &grandchildren, heights, direction);
                }
            }
        }
    }

    fn assign_positions(&mut self, root_id: Uuid, heights: &HashMap<Uuid, f32>) {
        if let Some(root) = self.get_node(root_id) {
            let root_x = root.x;

            let children = self.direct_children(root_id);

            let (left, right): (Vec<_>, Vec<_>) = children
                .into_iter()
                .partition(|&id| self.get_node(id).unwrap().x < root_x);

            self.spread_children_vertically(root_id, &left, heights, -1.0);
            self.spread_children_vertically(root_id, &right, heights, 1.0);
        }
    }
}
