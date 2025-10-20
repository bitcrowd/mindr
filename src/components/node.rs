use crate::data::Store;
use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use std::rc::Rc;
use uuid::Uuid;

#[component]
pub fn DraggedNode(id: Uuid, coords: (f32, f32)) -> Element {
    // Create motion values for scale and opacity
    let mut scale = use_motion(1.0f32);
    let mut opacity = use_motion(1.0f32);
    let animation = AnimationConfig::new(AnimationMode::Tween(Tween {
        duration: Duration::from_millis(100),
        ..Default::default()
    }))
    .with_delay(Duration::from_millis(50));
    // Animate scale and opacity
    use_effect(move || {
        scale.animate_to(0.5, animation.clone());
        opacity.animate_to(0.5, animation.clone());
    });

    rsx! {
        g {
            transform: format!("translate({}, {})", coords.0, coords.1),
            use {
                href: format!("#{id}"),
                style: format!("transform: scale({}); opacity: {};", scale.get_value(), opacity.get_value()),
            }
        }
    }
}

#[component]
fn RawChildNode(width: f32, height: f32, color: &'static str) -> Element {
    rsx! {
        rect {
            x: format!("{}", -width / 2.0),
            y: format!("{}", -height / 2.0),
            width: format!("{}", width),
            height: format!("{}", height),
            rx: "12",
            ry: "12",
            fill: "{color}",
            stroke: "black",
            "stroke-width": "1.5",
        }
    }
}

#[component]
fn RawRootNode(width: f32, height: f32, color: &'static str) -> Element {
    rsx! {
        rect {
            x: format!("{}", -width / 2.0),
            y: format!("{}", -height / 2.0),
            width: format!("{}", width + 4.0),
            height: format!("{}", height + 4.0),
            rx: "20",
            ry: "20",
            fill: "rgba(0,0,0,0.3)",
        }

        rect {
            x: format!("{}", -width / 2.0),
            y: format!("{}", -height / 2.0),
            width: format!("{}", width),
            height: format!("{}", height),
            rx: "20",
            ry: "20",
            fill: "{color}",
            stroke: "black",
            "stroke-width": "2",
        }
    }
}

#[component]
fn RawNode(node: crate::data::RenderedNode) -> Element {
    let width = node.width();
    let height = node.height();
    rsx! {
        if node.parent_id == None {
            RawRootNode { width, height, color: node.color }
        } else {
            RawChildNode { width, height, color: node.color }
        }
    }
}

#[component]
pub fn Node(id: Uuid, store: Store) -> Element {
    let node = store.graph.get_node(id).unwrap();
    let width = node.width();
    let height = node.height();
    let font_size = node.font_size();
    let is_editing = *store.pane.editing.read() == Some(id);
    let mut input_element: Signal<Option<Rc<MountedData>>> = use_signal(|| None);
    if let Some(input) = &*input_element.read() {
        let _ = input.set_focus(is_editing);
    }

    rsx! {
        g {
            transform: format!("translate({},{})", node.x, node.y),
            g {
              onmousedown: move |evt| {
                  if is_editing {
                      evt.stop_propagation();
                  }
              },
              ondoubleclick: move |evt| {
                  if is_editing {
                      evt.stop_propagation();
                  }
              },
              style: if is_editing { "" } else { "pointer-events: none;" },
              id:  "{node.id}",
              RawNode { node: node.clone() }

              if *store.pane.editing.read() == Some(id) {
                  circle {
                      cx: format!("{}", width / 2.0),
                      cy: format!("{}", height / 2.0),
                      r: "10",
                      fill: "red",
                      stroke: "black",
                  }
              }
              foreignObject {
                  x: format!("{}", -width / 2.0),
                  y: format!("{}", -height / 2.0),
                  width: format!("{}", width),
                  height: format!("{}", height),
                  style: "user-select: none;",
                  textarea {
                      style: if store.pane.dragging_node.read().is_some() { "pointer-events: none;" } else { "" },
                      key: "{id}-textarea",
                      onmounted: move |element| input_element.set(Some(element.data())),
                      value: "{node.text}",
                      autofocus: "true",
                      autocomplete: "off",
                      autocapitalize: "off",
                      spellcheck: "false",
                      tabindex: "-1",
                      style: "user-select: none; padding-top: 7px; padding-bottom: 10px; width: 100%; height: 100%; outline:none; background: transparent; border: none; resize:none; overflow:hidden; text-align: center; font-size: {font_size}px; display: block",
                      oninput: move |evt| {
                          store.graph.update_node_text(id, evt.value().clone());
                      },
                  }
              }
            }
        }
    }
}
