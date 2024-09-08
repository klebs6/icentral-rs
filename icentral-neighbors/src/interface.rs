crate::ix!();

pub trait GetNodes {

    fn nodes(&self) -> &NeighborsMap;
}

pub trait UnlinkAll {

    fn unlink_all(&mut self, src: NodeId, dst: NodeId);
}

pub trait AddEdge {

    fn add_edge(&mut self, e: &Edge);
}

pub trait UnlinkEdge {

    fn unlink_edge(&mut self, e: &Edge);
}

pub trait ReinitWithLen {

    fn reinit(&mut self, len: usize);
}

pub trait AddIsolatedNode {

    fn add_isolated_node(
        &mut self, 
        node: NodeId
    );
}

pub trait SetNeighbors {

    fn set_neighbors(
        &mut self, 
        node: NodeId, 
        nbrs: Vec<NodeId>
    );
}

pub trait AddNeighbor {

    fn add_neighbor(
        &mut self, 
        node: NodeId, 
        nbr:  NodeId
    );
}

pub trait RemoveNodeAndNeighbors {

    fn remove_node_and_neighbors(&mut self, node: NodeId);
}
