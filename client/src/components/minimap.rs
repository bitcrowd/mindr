use crate::data::Store;
use dioxus::prelude::*;

const MINIMAP_WIDTH: f32 = 150.0;
const MINIMAP_HEIGHT: f32 = 100.0;
const MINIMAP_MARGIN: f32 = 100.0;
const MINIMAP_PADDING: f32 = 10.0;

#[component]
pub fn MiniMap(store: Store, svg_size: Signal<(f32, f32)>) -> Element {
    let t = *store.pane.transform.read();

    // Compute bounds of nodes
    let (min_x, max_x, min_y, max_y) = store.graph.bounds();

    // Ensure mini-map scale includes nodes and viewport
    let (svg_w, svg_h) = *svg_size.read();
    let viewport_width = svg_w / t.scale;
    let viewport_height = svg_h / t.scale;

    let map_width = (max_x - min_x).max(viewport_width).max(1.0);
    let map_height = (max_y - min_y).max(viewport_height).max(1.0);

    let scale_x = MINIMAP_WIDTH / map_width;
    let scale_y = MINIMAP_HEIGHT / map_height;
    let scale = scale_x.min(scale_y);

    // Mindmap coordinates of viewport
    let viewport_left = -t.pan_x / t.scale;
    let viewport_top = -t.pan_y / t.scale;
    let viewport_right = viewport_left + viewport_width;
    let viewport_bottom = viewport_top + viewport_height;

    // Precompute node positions

    // Precompute viewport rectangle in mini-map coordinates
    let mini_view_x = (viewport_left - min_x) * scale + MINIMAP_MARGIN;
    let mini_view_y = (viewport_top - min_y) * scale + MINIMAP_MARGIN;
    let mini_view_w = (viewport_right - viewport_left) * scale;
    let mini_view_h = (viewport_bottom - viewport_top) * scale;
    let g_translate_x = svg_w - MINIMAP_WIDTH - 150.0;
    let g_translate_y = svg_h - MINIMAP_HEIGHT - 150.0;

    let mini_to_world =
        move |dx: f32, dy: f32| -> (f32, f32) { (dx / scale * t.scale, dy / scale * t.scale) };
    let mut nodes = Vec::new();
    store.graph.for_each_node(|node| {
        nodes.push(rsx! {
            rect {
                x: "{(node.x - min_x) * scale + MINIMAP_MARGIN}",
                y: "{(node.y - min_y) * scale + MINIMAP_MARGIN}",
                width: "{node.width() * scale}",
                height: "{node.height() * scale}",
                fill: node.color,
            }
        });
    });

    rsx! {
        g {
            transform: "translate({g_translate_x},{g_translate_y})",

            onmousedown: move |evt| {
                store.pane.minimap_dragging.set(true);
                let coords = evt.element_coordinates();
                store.pane.minimap_drag_offset.set((coords.x as f32, coords.y as f32));
                evt.stop_propagation();
                let x = (coords.x as f32 - MINIMAP_MARGIN - g_translate_x) / scale + min_x;
                let y = (coords.y as f32 - MINIMAP_MARGIN - g_translate_y) / scale + min_y;
                let mut t_new = *store.pane.transform.read();
                t_new.pan_x = -x * t.scale + svg_w / 2.0;
                t_new.pan_y = -y * t.scale + svg_h / 2.0;
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
                x: "{MINIMAP_MARGIN - MINIMAP_PADDING}",
                y: "{MINIMAP_MARGIN - MINIMAP_PADDING}",
                width: "{MINIMAP_WIDTH + 2.0 * MINIMAP_PADDING}",
                height: "{MINIMAP_HEIGHT + 2.0 * MINIMAP_PADDING}",
                fill: "white",
                stroke: "black",

                "stroke-width": "1.0",
            }
            for node in nodes {
                {node}
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
