crate::ix!();

impl<GH> SubtractEdge for Graph<GH> 
where GH
: GetConnectedComponentSizes
+ ExtendWith<GH>
+ GetEdges
+ GetNeighborsForNode
+ GetNodeIdRange
+ HasMapForNode
+ InsertEdge
+ InsertNode
+ MappedNodes
+ NumEdges
+ NumNodes
{
    fn simple_subtract_edge_and_update_mucs(&mut self, edge: &Edge) 
    {
        // edge not in muc's, connects two
        // connection vertices
        //
        let bridge_edge: Edge = *edge;

        let muc1_id: MinimumUnionCycleId = self.nodes_to_mucs.mucid_for_node(bridge_edge.src);
        let muc2_id: MinimumUnionCycleId = self.nodes_to_mucs.mucid_for_node(bridge_edge.dst);

        self.mucs[muc1_id.val()].insert_conn_vertex(bridge_edge.src, bridge_edge.dst);
        self.mucs[muc2_id.val()].insert_conn_vertex(bridge_edge.dst, bridge_edge.src);
    }

    fn maybe_simple_subtract_edge_and_update_mucs(
        &mut self, 
        edge:          &Edge,
        all_muc_edges: &Edges) 
    {
        let edge_r = edge.reversed();

        if !all_muc_edges.has_edge(&edge)
        && !all_muc_edges.has_edge(&edge_r)
        {
            self.simple_subtract_edge_and_update_mucs(edge);
        }
    }

    fn do_simple_subtraction_of_edges_and_update_mucs(
        &mut self, 
        all_muc_edges: &Edges) 
    {
        // TODO: can we avoid this clone?
        let edges = self.edges.clone();

        for &edge in edges.iter() {

            self.maybe_simple_subtract_edge_and_update_mucs(
                &edge,
                all_muc_edges
            );
        }
    }
}
