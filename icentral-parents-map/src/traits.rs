crate::ix!();

pub trait NumParentsForNode {

    fn num_parents_for_node(&self, node: NodeId) -> usize;
}

pub trait ParentsForNode {

    fn parents_for_node(&self, v_n: NodeId) 
    -> Vec<NodeId>;
}

pub trait ClearParents {

    fn clear_parents(&mut self);
}

pub trait ClearNodeParents {

    fn clear_node_parents(&mut self, node: NodeId);
}

pub trait SetParentsForNode {

    fn set_parents_for_node(
        &mut self, 
        node:    NodeId, 
        parents: Vec<NodeId>
    );
}

pub trait SetSingleParent {

    fn set_single_parent(
        &mut self, 
        node:   NodeId, 
        parent: NodeId
    );
}

pub trait HasParent {

    fn has_parent(&self, 
        node:      NodeId, 
        candidate: NodeId) -> bool;
}

pub trait AddParent {

    fn add_parent(
        &mut self,
        node:   NodeId,
        parent: NodeId
    );
}
