crate::ix!();

impl<GH> InsertEdge for Graph<GH> {

    fn insert_edge(&mut self, edge: &Edge) 
    -> Result<(),BetweennessCentralityError> 
    {
        debug!("in {}, inserting edge {} into graph", self.name(), edge);

        if self.edges.has_edge(&edge) {
            return Ok(());
        }

        let rev = edge.reversed();

        if self.edges.has_edge(&rev) {
            return Ok(());
        }

        self.edges.insert_edge(rev);

        self.nodes_map.add_edge(edge);

        Ok(())
    }
}
