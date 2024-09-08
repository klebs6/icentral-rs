crate::ix!();

impl InsertEdge for SubGraph {

    fn insert_edge(&mut self, edge: &Edge) 
    -> Result<(),BetweennessCentralityError> 
    {
        Ok(self.insert_edge_between_nodes(edge.src, edge.dst)?)
    }
}

impl InsertEdgeBetweenNodes for SubGraph {

    fn insert_edge_between_nodes(&mut self, 
        src: NodeId,
        dst: NodeId) 
    -> Result<(),BetweennessCentralityError> 
    {
        let mut e1 = Edge::new(src,dst);
        let mut e2 = e1.reversed();

        if !self.edges.connects(src,dst)
        {
            self.edges.insert_edge(e1);

            self.nodes_map.add_neighbor(src,dst);
            self.nodes_map.add_neighbor(dst,src);
        }

        Ok(())
    }
}
