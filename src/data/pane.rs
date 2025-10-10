use dioxus::prelude::*;
use uuid::Uuid;

use crate::data::RelativeLocation;

#[derive(Clone, Copy, PartialEq)]
pub struct Transform {
    pub pan_x: f32,
    pub pan_y: f32,
    pub scale: f32, // leave scale = 1 for now
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pane {
    pub dragging: Signal<Option<Uuid>>,
    pub drag_offset: Signal<(f32, f32)>,
    pub panning: Signal<bool>,
    pub pan_offset: Signal<(f32, f32)>,
    pub transform: Signal<Transform>,
    pub minimap_dragging: Signal<bool>,
    pub minimap_drag_offset: Signal<(f32, f32)>,
    pub drop_target: Signal<Option<(Uuid, RelativeLocation)>>,
}

impl Pane {
    pub fn new() -> Self {
        Self {
            dragging: use_signal(|| None),
            drag_offset: use_signal(|| (0f32, 0f32)),
            panning: use_signal(|| false),
            pan_offset: use_signal(|| (0f32, 0f32)),
            transform: use_signal(|| Transform {
                pan_x: 0.0,
                pan_y: 0.0,
                scale: 1.0,
            }),
            minimap_dragging: use_signal(|| false),
            minimap_drag_offset: use_signal(|| (0f32, 0f32)),
            drop_target: use_signal(|| None),
        }
    }

    pub fn transform(&mut self, x: f32, y: f32) -> (f32, f32) {
        let t = *self.transform.read();
        (x - t.pan_x, y - t.pan_y)
    }
}
