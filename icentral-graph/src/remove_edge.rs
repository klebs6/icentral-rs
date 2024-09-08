crate::ix!();

impl<GH> RemoveEdge for Graph<GH> {

    fn remove_edge(&mut self, edge: &Edge) 
    -> Result<(),BetweennessCentralityError> 
    {
        debug!("in {}, removing edge {}", self.name(), edge);

        let rev = edge.reversed();

        self.edges.remove_edge(&edge);
        self.edges.remove_edge(&rev);

        debug!("src nbrs {:#?}", self.nodes_map.neighbors(edge.src));
        debug!("dst nbrs {:#?}", self.nodes_map.neighbors(edge.dst));

        self.nodes_map.unlink_edge(edge);

        Ok(())
    }
}
