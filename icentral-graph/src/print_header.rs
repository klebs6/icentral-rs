crate::ix!();

impl<GH> PrintHeader for Graph<GH> {

    #[inline] fn print_header(&self) {

        debug!("==========================================================");
        debug!("Graph:         {}", self.graph_name);
        debug!("Graph # nodes: {}", self.num_nodes());
        debug!("Graph # edges: {}", self.edges.len());
    }
}
