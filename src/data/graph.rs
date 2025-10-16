use crate::data::Node;
use dioxus::prelude::*;
use std::collections::HashMap;
use uuid::Uuid;

use super::RelativeLocation;

const SPACING_X: f32 = 50.0; // horizontal gap between parent and child
const SPACING_Y: f32 = 10.0; // vertical gap between siblings

const COLORS: [&'static str; 8] = [
    "#ffc6ff", "#ffadad", "#ffd6a5", "#fdffb6", "#caffbf", "#9bf6ff", "#a0c4ff", "#bdb2ff",
];

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

    pub fn add_node(&mut self, x: f32, y: f32) -> Uuid {
        let mut nodes = self.nodes.write();
        let id = Uuid::new_v4();
        nodes.insert(
            id,
            Node {
                id,
                x,
                y,
                text: "".to_string(),
                parent_id: None,
                color: COLORS.last().unwrap(),
            },
        );
        id
    }

    pub fn move_node(&mut self, id: Uuid, x: f32, y: f32) {
        let mut nodes = self.nodes.write();
        if let Some(node) = nodes.get_mut(&id) {
            node.x = x;
            node.y = y;
        }
    }

    pub fn update_node_text(&mut self, id: Uuid, new_text: String) {
        let mut nodes = self.nodes.write();
        if let Some(node) = nodes.get_mut(&id) {
            node.text = new_text;
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

    pub fn on(&self, x: f32, y: f32) -> Option<Node> {
        let mut target = None;
        for node in self.nodes.read().values() {
            if let Some(_) = node.on(x, y) {
                target = Some(node.clone())
            }
        }
        target
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

    fn colorize_with(&mut self, node_id: Uuid, color: &'static str) {
        for child_id in self.direct_children(node_id) {
            self.colorize_with(child_id, color);
        }
        let mut nodes = self.nodes.write();
        if let Some(node) = nodes.get_mut(&node_id) {
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
        let node = self.get_node(node_id).unwrap();

        let children = self.direct_children(node_id);

        let total_height: f32 = children
            .iter()
            .map(|&child_id| self.compute_subtree_heights(child_id, heights))
            .sum::<f32>()
            + SPACING_Y * (children.len().saturating_sub(1)) as f32;

        heights.insert(node_id, total_height.max(node.height()));
        heights[&node_id]
    }

    fn direct_children(&self, node_id: Uuid) -> Vec<Uuid> {
        self.nodes
            .read()
            .values()
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

        let parent = self.get_node(parent_id).unwrap();

        let total_height: f32 = children.iter().map(|id| heights[id]).sum::<f32>()
            + SPACING_Y * (children.len() as f32 - 1.0);
        let mut y = parent.y - total_height / 2.0;

        for &child_id in children {
            let child = self.get_node(child_id).unwrap();
            let child_x =
                parent.x + direction * ((parent.width() + child.width()) / 2.0 + SPACING_X);
            let child_height = heights[&child_id];
            let target_y = y + child_height / 2.0;
            self.move_node(child_id, child_x, target_y);
            y += child_height + SPACING_Y;

            let grandchildren = self.direct_children(child_id);
            self.spread_children_vertically(child_id, &grandchildren, heights, direction);
        }
    }

    fn assign_positions(&mut self, root_id: Uuid, heights: &HashMap<Uuid, f32>) {
        let root = self.get_node(root_id).unwrap();
        let root_x = root.x;

        let children = self.direct_children(root_id);

        let (left, right): (Vec<_>, Vec<_>) = children
            .into_iter()
            .partition(|&id| self.get_node(id).unwrap().x < root_x);

        self.spread_children_vertically(root_id, &left, heights, -1.0);
        self.spread_children_vertically(root_id, &right, heights, 1.0);
    }
}
