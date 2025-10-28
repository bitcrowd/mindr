use crate::data::Graph;
use crate::data::Pane;

#[derive(Clone, PartialEq)]
pub struct Store {
    pub graph: Graph,
    pub pane: Pane,
}

impl Store {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            pane: Pane::new(),
        }
    }
}
