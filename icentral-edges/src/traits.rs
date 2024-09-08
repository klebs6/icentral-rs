crate::ix!();

pub trait NumEdges {

    fn num_edges(&self) -> usize;
}

pub trait GetEdges {

    fn edges(&self) -> &Edges;
}

pub trait HasEdge {

    fn has_edge(&self, edge: &Edge) -> bool;
}

pub trait InsertEdge {

    fn insert_edge(&mut self, edge: &Edge) 
    -> Result<(),BetweennessCentralityError>;
}

pub trait InsertEdgeBetweenNodes {

    fn insert_edge_between_nodes(&mut self, 
        src: NodeId,
        dst: NodeId) 
    -> Result<(),BetweennessCentralityError>;
}

pub trait RemoveEdge {

    fn remove_edge(&mut self, edge: &Edge) 
    -> Result<(),BetweennessCentralityError>;
}

pub trait RemoveEdgeBetweenNodes {

    fn remove_edge_between_nodes(
        &mut self, 
        src: NodeId,
        dst: NodeId) 
    -> Result<(),BetweennessCentralityError>;
}
