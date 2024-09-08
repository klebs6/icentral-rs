crate::ix!();

pub trait FindEdgeBccSubgraph {

    fn find_edge_bcc_subgraph<GH: BccGraphHashInterface>(
        &mut self, 
        bcc:  &mut GH,
        edge: &Edge
    ) -> Result<(),BetweennessCentralityError>;
}

impl<T> FindEdgeBccSubgraph for T 
where T: Sized + for<'a> Named<'a> + NumNodes + GetNeighborsForNode
{
    /**
      | the edge (@src, @dst) must be in the graph
      | the biconnected component subgraph that
      | contains the passed edge will be returned
      | in @bcc
      |
      | IMP assumption:
      |
      | the input graph is connected, and the edge
      | (src, dst) is inserted, so the edge (src,
      | dst) is part of a cycle, so both ends
      | belong to the same bcc
      |
      |IMP: (src,dst) must exist!
      */
    fn find_edge_bcc_subgraph<GH: BccGraphHashInterface>(
        &mut self, 
        bcc:  &mut GH,
        edge: &Edge

    ) -> Result<(),BetweennessCentralityError> {

        debug!("initiating Graph::find_edge_bcc_subgraph");

        let mut u: NodeId = NodeId::default();

        let num_nodes = self.num_nodes();

        let color_vec_name = name![self.name(), "find_edge_bcc_subgraph::color_vec"];
        let pred_vec_name  = name![self.name(), "find_edge_bcc_subgraph::pred_vec"];
        let distances_name = name![self.name(), "find_edge_bcc_subgraph::distances"];
        let low_vec_name   = name![self.name(), "find_edge_bcc_subgraph::low_vec"];

        let mut color_vec = ColorMap::new(num_nodes, color_vec_name);
        let mut pred_vec  = PredecessorMap::new(num_nodes, pred_vec_name);

        let mut distances = DistanceMap::new(num_nodes, distances_name);
        let mut low_vec   = DistanceMap::new(num_nodes, low_vec_name);

        let mut edge_stack: Stack<Edge> = default!();

        let mut time: f64 = 0.0;

        let mut nbrs_vec = self.neighbors(edge.src);

        move_destination_edge_to_front(
            &mut nbrs_vec, 
            edge.dst
        );

        let mut ctx = BccDfsVisitorContext {
            color_vec:  &mut color_vec, 
            low_vec:    &mut low_vec, 
            distances:  &mut distances, 
            pred_vec:   &mut pred_vec, 
            edge_stack: &mut edge_stack, 
            time:       &mut time, 
        };

        edge_bcc_dfs_visitor(
            self,
            edge.src, 
            &mut ctx,
        );

        while let Some(e) = edge_stack.pop() {
            bcc.insert_edge(&e);
        }

        Ok(())
    }
}
