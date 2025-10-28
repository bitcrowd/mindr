use crate::data::Connection;
use crate::data::Graph;
use crate::data::Pane;

#[derive(Clone, PartialEq)]
pub struct Store {
    pub graph: Graph,
    pub pane: Pane,
    pub connection: Connection,
}

impl Store {
    pub fn new() -> Self {
        let graph = Graph::new();
        Self {
            graph: graph.clone(),
            pane: Pane::new(),
            connection: Connection::new(graph),
        }
    }
}
