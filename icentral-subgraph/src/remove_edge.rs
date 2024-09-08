crate::ix!();

impl RemoveEdgeBetweenNodes for SubGraph {

    fn remove_edge_between_nodes(
        &mut self, 
        src: NodeId,
        dst: NodeId) 
    -> Result<(),BetweennessCentralityError> 
    {
        self.edges.unlink_all(src,dst);
        self.nodes_map.unlink_all(src,dst);

        Ok(())
    }
}
