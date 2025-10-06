use crate::data::Store;
use crate::data::Transform;
use dioxus::prelude::*;
#[component]
pub fn MiniMap(store: Store, svg_size: Signal<(f32, f32)>) -> Element {
    let t = *store.pane.transform.read();
    let nodes = &store.graph.nodes.read();

    // Compute bounds of nodes
    let (min_x, max_x, min_y, max_y) = if !nodes.is_empty() {
        let min_x = nodes.values().map(|n| n.x).fold(f32::INFINITY, f32::min);
        let max_x = nodes
            .values()
            .map(|n| n.x)
            .fold(f32::NEG_INFINITY, f32::max);
        let min_y = nodes.values().map(|n| n.y).fold(f32::INFINITY, f32::min);
        let max_y = nodes
            .values()
            .map(|n| n.y)
            .fold(f32::NEG_INFINITY, f32::max);
        (min_x, max_x, min_y, max_y)
    } else {
        (0.0, 1.0, 0.0, 1.0)
    };
    // Mini-map size and margin
    let mini_width = 100.0;
    let mini_height = 100.0;
    let margin = 100.0;

    // Ensure mini-map scale includes nodes and viewport
    let (svg_w, svg_h) = *svg_size.read();
    let viewport_width = svg_w / t.scale;
    let viewport_height = svg_h / t.scale;

    let map_width = (max_x - min_x).max(viewport_width).max(1.0);
    let map_height = (max_y - min_y).max(viewport_height).max(1.0);

    let scale_x = mini_width / map_width;
    let scale_y = mini_height / map_height;
    let scale = scale_x.min(scale_y);

    // Mindmap coordinates of viewport
    let viewport_left = -t.pan_x / t.scale;
    let viewport_top = -t.pan_y / t.scale;
    let viewport_right = viewport_left + viewport_width;
    let viewport_bottom = viewport_top + viewport_height;

    // Precompute node positions
    let node_positions: Vec<(f32, f32)> = nodes
        .values()
        .map(|node| {
            (
                (node.x - min_x) * scale + margin,
                (node.y - min_y) * scale + margin,
            )
        })
        .collect();

    // Precompute viewport rectangle in mini-map coordinates
    let mini_view_x = (viewport_left - min_x) * scale + margin;
    let mini_view_y = (viewport_top - min_y) * scale + margin;
    let mini_view_w = (viewport_right - viewport_left) * scale;
    let mini_view_h = (viewport_bottom - viewport_top) * scale;
    let g_translate_x = svg_w - mini_width - 150.0;
    let g_translate_y = svg_h - mini_height - 150.0;
    let mini_to_world =
        move |dx: f32, dy: f32| -> (f32, f32) { (dx / scale * t.scale, dy / scale * t.scale) };
    rsx! {
        g {
            transform: "translate({g_translate_x},{g_translate_y})",

            onmousedown: move |evt| {
                store.pane.minimap_dragging.set(true);
                let coords = evt.element_coordinates();
                store.pane.minimap_drag_offset.set((coords.x as f32, coords.y as f32));
                evt.stop_propagation();
                  let offset_x = svg_w - mini_width - margin;
                  let offset_y = svg_h - mini_height - margin;
                  let x = (coords.x as f32 - margin - offset_x) / scale + min_x;
                  let y = (coords.y as f32 - margin - offset_y) / scale + min_y;
                    let mut t_new = *store.pane.transform.read();
                    t_new.pan_x = -x * t.scale;
                    t_new.pan_y = -y * t.scale;
                    store.pane.transform.set(t_new);

            },
            onmousemove: move |evt| {
                if *store.pane.minimap_dragging.read() {
                    let coords = evt.element_coordinates();
                    let (last_x, last_y) = *store.pane.minimap_drag_offset.read();
                    let dx = coords.x as f32 - last_x;
                    let dy = coords.y as f32 - last_y;
                    store.pane.minimap_drag_offset.set((coords.x as f32, coords.y as f32));
                    let (world_dx, world_dy) = mini_to_world(dx as f32, dy as f32);
                    let mut t_new = *store.pane.transform.read();
                    t_new.pan_x -= world_dx;
                    t_new.pan_y -= world_dy;
                    store.pane.transform.set(t_new);
                }
            },
            // Mini-map background
            rect {
                x: "{margin}",
                y: "{margin}",
                width: "{mini_width}",
                height: "{mini_height}",
                fill: "white",
                stroke: "black",

                "stroke-width": "1.0",
            }

            // Nodes
            for (cx , cy) in node_positions.iter() {
                circle {
                    cx: "{cx}",
                    cy: "{cy}",
                    r: "3",
                    fill: "lightblue",
                    stroke: "black",
                }
            }

            // Viewport rectangle
            rect {
                x: "{mini_view_x}",
                y: "{mini_view_y}",
                width: "{mini_view_w}",
                height: "{mini_view_h}",
                fill: "none",
                stroke: "red",
                "stroke-width": "1.5",
            }
        }
    }
}
