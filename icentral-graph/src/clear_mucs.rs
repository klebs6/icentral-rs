crate::ix!();

impl<GH> ClearMucs for Graph<GH> 
where GH
: IsValid 
+ ExtendWith<GH>
+ GetConnectedComponentSizes
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
    
    fn clear_mucs(&mut self) 
    -> Result<(),BetweennessCentralityError> 
    {
        for m in 0..self.mucs.len() {

            if self.mucs[m].is_valid() {

                self.mucs[m].clear_conn_vertex_map();
                self.mucs[m].clear_subgraph_map();
            }
        }

        Ok(())
    }
}
