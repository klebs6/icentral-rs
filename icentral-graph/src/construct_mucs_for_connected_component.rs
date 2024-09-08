crate::ix!();

impl<GH> ConstructMucsForConnectedComponent<GH> for Graph<GH> 

where GH
: NumNodes
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
{

    fn maybe_construct_mucs_for_connected_component(
        &mut self, 
        component: GH)
    {
        if component.num_nodes() >= 3 {
            self.construct_mucs_for_connected_component(component);
        }
    }

    fn construct_mucs_for_connected_component(
        &mut self,
        component: GH)
    {
        let mut muc = MinimumUnionCycle::<GH>::default();

        muc.set_id(self.mucs.len());

        muc.set_muc_subgraph(component);

        for nodeid in muc.mapped_nodes() {

            self.nodes_to_mucs.set_mucid_for_node(
                nodeid, 
                muc.id()
            );
        }

        self.mucs.push(muc);
    }
}
