crate::ix!();

pub trait GetNodeIdRange {

    fn nodeid_range(&self) -> Vec<NodeId>;
}

pub trait GetLimitedNodeIdRange {

    fn limited_nodeid_range(&self, cap: Option<usize>) -> Vec<NodeId>;
}

pub trait HasMapForNode {

    fn has_map_for_node(&self, node: NodeId) -> bool;
}

pub trait MappedNodes {

    fn mapped_nodes(&self) -> Vec<NodeId>;
}

pub trait NumNodes {

    fn num_nodes(&self) -> usize;
}
