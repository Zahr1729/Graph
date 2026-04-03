use std::fmt;

use crate::node::NodeId;


#[derive(Clone, Copy, Eq, PartialEq, Hash, Default, PartialOrd, Ord)]
pub struct EdgeId(pub usize);


impl fmt::Debug for EdgeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

impl EdgeId {
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}


/// An Edge at minimum needs a reference to two nodes.
#[derive(Clone, Copy, Hash, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Edge {
    pub first: NodeId,
    pub second: NodeId,
}

impl Edge {
    pub fn contains_node(&self, node_id: &NodeId) -> bool {
        return node_id == &self.first || node_id == &self.second
    }
}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}, {:?}]", self.first, self.second)?;
        Ok(())
    }
}