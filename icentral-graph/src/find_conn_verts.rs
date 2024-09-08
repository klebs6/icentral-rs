crate::ix!();

impl<GH> FindConnVerts for Graph<GH> 
where GH
: GetEdges
+ ClearMucs
+ ExtendWith<GH>
+ GetConnectedComponentSizes
+ GetNeighborsForNode
+ GetNodeIdRange
+ HasMapForNode
+ InsertEdge
+ InsertNode
+ IsValid
+ MappedNodes
+ NumEdges
+ NumNodes
{
    /// find the connection vertices for each muc
    ///
    /// edges that belong to the graph and don't
    /// belong to any muc connect two connection
    /// vertices
    ///
    /// 1. collect all edges in mucs in one set
    ///
    /// 2. do simple subtraction of edges and
    ///    update muc's
    ///
    fn find_conn_verts(&mut self) 
    -> Result<(),BetweennessCentralityError> 
    {
        debug!("finding connection vertices...");

        self.clear_mucs()?;

        let all_muc_edges = self.collect_all_edges_in_mucs_in_one_set()?;

        self.do_simple_subtraction_of_edges_and_update_mucs(&all_muc_edges);

        Ok(())
    }
}
