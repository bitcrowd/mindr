use crate::data::Store;
use dioxus::prelude::*;
use uuid::Uuid;

#[component]
pub fn RedCross(x: f32, y: f32) -> Element {
    rsx! {
        g {
           transform: "translate({x - 20.0}, {y})",
            stroke: "#FF0000",
            stroke_width: 2,
            stroke_linecap: "round",

            line { x1: -6, y1: -6, x2: 6, y2: 6 }
            line { x1: 6, y1: -6, x2: -6, y2: 6 }
            line { x1: 5, y1: 0, x2: 20, y2: 0, stroke: "black"}
        }
    }
}
#[component]
pub fn NodeLink(id: Uuid, parent_id: Uuid, store: Store) -> Element {
    let graph = store.graph;
    let Some(child) = graph.get_node(id) else {
        return rsx! {};
    };

    let Some(parent) = graph.get_node(parent_id) else {
        let x = child.x - child.width() / 2.0;
        let y = child.y;
        return rsx! {
          RedCross { x, y }
        };
    };

    let is_right = child.x > parent.x;
    let side_mult = if is_right { 1.0 } else { -1.0 };

    // Parent/child edges
    let start_x = parent.x + side_mult * parent.width() / 2.0;
    let start_y = parent.y;
    let end_x = child.x - side_mult * child.width() / 2.0;
    let end_y = child.y;

    // Horizontal offsets
    let start_offset = 20.0 * side_mult;
    let end_offset = -20.0 * side_mult;

    // Thickness
    let parent_thickness = 8.0;
    let child_thickness = 4.0;

    // Unit perpendicular
    let dx = end_x - start_x;
    let dy = end_y - start_y;
    let len = (dx * dx + dy * dy).sqrt().max(1.0);
    let nx = -dy / len;
    let ny = dx / len;

    // Compute path points
    let p1 = (
        start_x + nx * parent_thickness / 2.0,
        start_y + ny * parent_thickness / 2.0,
    );
    let p2 = (
        start_x - nx * parent_thickness / 2.0,
        start_y - ny * parent_thickness / 2.0,
    );
    let e1 = (
        end_x + nx * child_thickness / 2.0,
        end_y + ny * child_thickness / 2.0,
    );
    let e2 = (
        end_x - nx * child_thickness / 2.0,
        end_y - ny * child_thickness / 2.0,
    );
    let c1 = (start_x + start_offset, start_y);
    let c2 = (end_x + end_offset, end_y);

    let path_data = format!(
        "M {} {} C {} {}, {} {}, {} {} L {} {} C {} {}, {} {}, {} {} Z",
        p1.0,
        p1.1,
        c1.0,
        c1.1,
        c2.0,
        c2.1,
        e1.0,
        e1.1,
        e2.0,
        e2.1,
        c2.0,
        c2.1,
        c1.0,
        c1.1,
        p2.0,
        p2.1
    );

    rsx! {
        path { d: "{path_data}", fill: "rgba(80,80,80,0.9)" }
    }
}
