crate::ix!();

impl<G: NumNodes + GetNodeIdRange + GetNeighborsForNode + GetEdges> ResetWith<G> for SubGraph {

    fn reset_with(&mut self, g: &G) 
    {
        self.nodes_map.reinit(g.num_nodes());

        for node in g.nodeid_range() {

            self.nodes_map.set_neighbors(
                node, 
                g.neighbors(node)
            );
        }

        self.edges = g.edges().clone();

        self.label_map.resize_inout(
            self.num_nodes(),
            NodeId::bad()
        );

        for node in self.nodeid_range() {

            self.label_map.insert_outin(node,node);
        }
    }
}
