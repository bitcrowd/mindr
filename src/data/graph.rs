use crate::data::Node;
use dioxus::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq)]
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

    pub fn get_node(&self, id: Uuid) -> Option<Node> {
        self.nodes.read().get(&id).cloned()
    }
}
