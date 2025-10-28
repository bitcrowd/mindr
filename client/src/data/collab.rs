use std::sync::{Arc, Mutex};
use uuid::Uuid;
use yrs::updates::decoder::Decode;
use yrs::updates::encoder::Encode;
use yrs::{
    types::{EntryChange, Event, PathSegment},
    Any, Array, ArrayRef, DeepObservable, Doc, Map, MapPrelim, MapRef, Observable, Out,
    Subscription, Transact, TransactionMut, Update,
};

pub struct CollabGraph {
    pub doc: Doc,
    pub y_nodes: MapRef,
    pub y_order: ArrayRef,
}

#[derive(Copy, Clone)]
pub enum Side {
    Left,
    Right,
}
impl From<Side> for Any {
    fn from(side: Side) -> Self {
        match side {
            Side::Left => Any::String("Left".into()),
            Side::Right => Any::String("Right".into()),
        }
    }
}
impl From<String> for Side {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Left" => Side::Left,
            "Right" => Side::Right,
            _ => unreachable!("Invalid value for Side: {}", value),
        }
    }
}

#[derive(Copy, Clone)]
pub enum NodeKind {
    Root { coords: (f32, f32) },
    Child { parent_id: Uuid, side: Side },
}

#[derive(Clone)]
pub struct Node {
    pub text: String,
    pub kind: NodeKind,
}

impl Node {
    pub fn new_root(coords: (f32, f32)) -> Self {
        Node {
            text: "".to_string(),
            kind: NodeKind::Root { coords },
        }
    }

    pub fn new_child(parent_id: Uuid, side: Side) -> Self {
        Node {
            text: "".to_string(),
            kind: NodeKind::Child { parent_id, side },
        }
    }
    fn from_txn(txn: &TransactionMut, map: &MapRef) -> Self {
        let text = String::try_from(map.get(txn, "text").unwrap()).unwrap();
        if let Some(parent_id) = map.get(txn, "parent_id") {
            let parent_id = Uuid::parse_str(&String::try_from(parent_id).unwrap()).unwrap();
            let side = String::try_from(map.get(txn, "side").unwrap())
                .map(|s| Side::from(s))
                .unwrap();
            Node {
                text: text,
                kind: NodeKind::Child {
                    parent_id: parent_id.into(),
                    side: side,
                },
            }
        } else {
            let x = extract_f64(&map.get(txn, "x").unwrap()).unwrap() as f32;
            let y = extract_f64(&map.get(txn, "y").unwrap()).unwrap() as f32;
            Node {
                text: text,
                kind: NodeKind::Root { coords: (x, y) },
            }
        }
    }
}

fn extract_f64(out: &Out) -> Option<f64> {
    match out {
        Out::Any(Any::Number(n)) => Some(*n),
        _ => None,
    }
}

fn update_coords(txn: &mut TransactionMut, ymap: MapRef, (x, y): (f32, f32)) {
    ymap.insert::<&str, Any>(txn, "x", (x as f64).into());
    ymap.insert::<&str, Any>(txn, "y", (y as f64).into());
}

fn update_parent(txn: &mut TransactionMut, ymap: MapRef, parent_id: Uuid, side: Side) {
    ymap.insert::<&str, Any>(txn, "parent_id", parent_id.to_string().into());
    ymap.insert::<&str, Any>(txn, "side", side.into());
}

impl CollabGraph {
    pub fn new() -> Self {
        let doc = Doc::new();
        let map = doc.get_or_insert_map("nodes");
        let list = doc.get_or_insert_array("order");

        CollabGraph {
            doc: doc,
            y_nodes: map,
            y_order: list,
        }
    }

    pub fn update(&mut self, update: Vec<u8>) {
        self.doc
            .transact_mut()
            .apply_update(Update::decode_v2(&update).unwrap());
    }

    pub fn add_node(&mut self, node: Node) -> Uuid {
        let mut txn = self.doc.transact_mut();

        let id = Uuid::new_v4();
        let ymap = self.y_nodes.insert(
            &mut txn,
            id.to_string().clone(),
            MapPrelim::from([("text", node.text)]),
        );
        match node.kind {
            NodeKind::Root { coords } => {
                update_coords(&mut txn, ymap, coords);
            }
            NodeKind::Child { parent_id, side } => {
                update_parent(&mut txn, ymap, parent_id, side);
            }
        }
        self.y_order
            .push_back::<Any>(&mut txn, id.to_string().clone().into());
        id
    }

    pub fn update_node_coords(&mut self, id: Uuid, coords: (f32, f32)) {
        let mut txn = self.doc.transact_mut();
        if let Some(Out::YMap(ymap)) = self.y_nodes.get(&mut txn, &id.to_string()) {
            update_coords(&mut txn, ymap, coords);
        }
    }

    pub fn update_node_parent(&mut self, id: Uuid, parent_id: Uuid, side: Side) {
        let mut txn = self.doc.transact_mut();
        if let Some(Out::YMap(ymap)) = self.y_nodes.get(&mut txn, &id.to_string()) {
            update_parent(&mut txn, ymap, parent_id, side);
        }
    }

    pub fn update_node_text(&mut self, id: Uuid, text: String) {
        let mut txn = self.doc.transact_mut();
        if let Some(Out::YMap(ymap)) = self.y_nodes.get(&mut txn, &id.to_string()) {
            ymap.insert::<&str, Any>(&mut txn, "text", text.into());
        }
    }

    pub fn observe_nodes<F>(&mut self, callback: F) -> Subscription
    where
        F: FnMut(Uuid, Option<Node>) + 'static,
    {
        let cb = Arc::new(Mutex::new(callback));
        self.y_nodes.observe_deep(move |txn, events| {
            for event in events.iter() {
                if let Event::Map(map_event) = event {
                    let path = map_event.path();
                    if path.is_empty() {
                        for (key, change) in map_event.keys(txn) {
                            let id = Uuid::parse_str(key).expect("Expected Node ID");
                            let node = match change {
                                EntryChange::Inserted(Out::YMap(node)) => {
                                    Some(Node::from_txn(txn, node))
                                }
                                EntryChange::Updated(_, Out::YMap(node)) => {
                                    Some(Node::from_txn(txn, node))
                                }
                                EntryChange::Removed(_) => None,
                                _ => None,
                            };
                            if let Ok(mut f) = cb.lock() {
                                (f)(id, node);
                            }
                        }
                    } else {
                        if let PathSegment::Key(key) = &path[0] {
                            let id = Uuid::parse_str(key).expect("Expected Node ID");
                            if let Ok(mut f) = cb.lock() {
                                (f)(id, Some(Node::from_txn(txn, map_event.target())));
                            }
                        }
                    }
                }
            }
        })
    }

    pub fn observe_order<F>(&mut self, callback: F) -> Subscription
    where
        F: FnMut(Vec<Uuid>) + 'static,
    {
        let cb = Arc::new(Mutex::new(callback));
        self.y_order.observe(move |txn, event| {
            let new_order = event
                .target()
                .iter(txn)
                .map(|id| {
                    let id_str = String::try_from(id).unwrap();
                    Uuid::parse_str(&id_str).unwrap()
                })
                .collect();

            if let Ok(mut f) = cb.lock() {
                (f)(new_order);
            }
        })
    }

    pub fn observe_doc<F>(&self, callback: F) -> Subscription
    where
        F: FnMut(Vec<u8>) + 'static,
    {
        let cb = Arc::new(Mutex::new(callback));
        self.doc
            .observe_update_v2(move |_txn, event| {
                if let Ok(mut f) = cb.lock() {
                    (f)(event.update.clone());
                }
            })
            .unwrap()
    }

    // observe doc
    //
    // self.subscriptions.push(
    //     self.doc
    //         .observe_update_v2(move |txn, event| {
    //             dbg!(&event.update);
    //         })
    //         .expect("PANIC"),
    // );
}
