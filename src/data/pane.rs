use dioxus::prelude::*;
use uuid::Uuid;

use crate::data::RelativeLocation;
use crate::data::RenderedNode;

#[derive(Clone, Copy, PartialEq)]
pub struct Transform {
    pub pan_x: f32,
    pub pan_y: f32,
    pub scale: f32, // leave scale = 1 for now
}

#[derive(Clone, Copy, PartialEq)]
pub struct DraggingNode {
    pub id: Uuid,
    pub offset: (f32, f32),
    pub coords: (f32, f32),
    pub target: Option<(Uuid, RelativeLocation)>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pane {
    pub dragging_node: Signal<Option<DraggingNode>>,
    pub panning: Signal<bool>,
    pub pan_offset: Signal<(f32, f32)>,
    pub transform: Signal<Transform>,
    pub minimap_dragging: Signal<bool>,
    pub minimap_drag_offset: Signal<(f32, f32)>,
    pub editing: Signal<Option<Uuid>>,
}

impl Pane {
    pub fn new() -> Self {
        Self {
            dragging_node: use_signal(|| None),
            panning: use_signal(|| false),
            pan_offset: use_signal(|| (0f32, 0f32)),
            transform: use_signal(|| Transform {
                pan_x: 0.0,
                pan_y: 0.0,
                scale: 1.0,
            }),
            minimap_dragging: use_signal(|| false),
            minimap_drag_offset: use_signal(|| (0f32, 0f32)),
            editing: use_signal(|| None),
        }
    }

    pub fn transform(&self, x: f32, y: f32) -> (f32, f32) {
        let t = *self.transform.read();
        (x - t.pan_x, y - t.pan_y)
    }

    pub fn start_drag(&mut self, node: &RenderedNode, (x, y): (f32, f32)) {
        let ox = x - node.x;
        let oy = y - node.y;
        self.dragging_node.set(Some(DraggingNode {
            id: node.id,
            offset: (ox, oy),
            coords: (node.x, node.y),
            target: None,
        }))
    }

    pub fn update_drag(&mut self, (x, y): (f32, f32), target: Option<(Uuid, RelativeLocation)>) {
        self.dragging_node.with_mut(|opt| {
            if let Some(node) = opt {
                let (ox, oy) = node.offset;
                node.coords = (x - ox, y - oy);
                node.target = target;
            }
        });
    }

    pub fn is_dragging(&self, id: Uuid) -> bool {
        if let Some(node) = *self.dragging_node.read() {
            return node.id == id;
        }
        return false;
    }
}
