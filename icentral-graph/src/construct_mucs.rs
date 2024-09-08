crate::ix!();

impl<GH> ConstructMucs<GH> for Graph<GH> 
where GH
: GetEdges
+ InsertEdge
+ NumEdges
+ GetConnectedComponentSizes
+ InsertNode
+ GetNeighborsForNode
+ GetNodeIdRange
+ HasMapForNode
+ MappedNodes
+ NumNodes
+ ExtendWith<GH>
{
    fn construct_mucs(&mut self, conn_comp_vec: Vec<GH>)
    {
        debug!("constructing MinimumUnionCycles");

        for component in conn_comp_vec.into_iter() {
            self.maybe_construct_mucs_for_connected_component(component);
        }
    }
}
