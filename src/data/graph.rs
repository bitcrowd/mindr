use crate::data::Node;
use dioxus::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use super::RelativeLocation;

#[derive(Copy, Clone, PartialEq)]
pub struct Graph {
    pub nodes: Signal<HashMap<Uuid, Node>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: use_signal(|| HashMap::new()),
        }
    }

    pub fn add_node(&mut self, x: f32, y: f32) {
        let mut nodes = self.nodes.write();
        let id = Uuid::new_v4();
        nodes.insert(
            id,
            Node {
                id,
                x,
                y,
                text: format!("Node {}", id),
                parent_id: None,
            },
        );
    }

    pub fn move_node(&mut self, id: Uuid, x: f32, y: f32) {
        let mut nodes = self.nodes.write();
        if let Some(node) = nodes.get_mut(&id) {
            node.x = x;
            node.y = y;
        }
    }

    pub fn move_node_into(&mut self, id: Uuid, target: (Uuid, RelativeLocation)) {
        let mut nodes = self.nodes.write();
        let (target_id, _) = target;
        if let Some(node) = nodes.get_mut(&id) {
            node.parent_id = Some(target_id);
        }

        if let Some(node) = nodes.get_mut(&target_id) {
            if node.parent_id == Some(id) {
                node.parent_id = None;
            }
        }
    }

    pub fn get_node(&self, id: Uuid) -> Option<Node> {
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
}
