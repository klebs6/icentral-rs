crate::ix!();

pub trait NodeAtIndex {

    fn node_at_index(&self, idx: usize) -> NodeId;
}

pub trait SetNodeAtIndex {

    fn set_node_at_index(&mut self, idx: usize, n: NodeId);
}

pub trait Push {

    type Item;

    fn push(&mut self, n: Self::Item);
}

pub trait Pop {

    type Item;

    fn pop(&mut self) -> Option<Self::Item>;
}
