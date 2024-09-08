crate::ix!();

impl<GH> CreateSingleVertexMucs for Graph<GH> 

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

    fn create_single_vertex_muc(
        &mut self, 
        idx: NodeId) 
    {
        let mut muc = MinimumUnionCycle::<GH>::default();

        muc.set_id(self.mucs.len());

        muc.insert_node(idx);

        self.nodes_to_mucs.set_mucid_for_node(idx, muc.id());

        self.mucs.push(muc);
    }

    fn maybe_create_single_vertex_muc(
        &mut self, 
        idx: NodeId) 
    {
        if self.nodes_to_mucs.mucid_for_node(idx) == MinimumUnionCycleId::inf() {

            self.create_single_vertex_muc(idx);
        }
    }

    fn create_single_vertex_mucs(&mut self) {

        // print_mucs();
        // create single vertex muc's
        for idx in NodeIdRange::new(0,self.nodes_to_mucs.len()) {

            self.maybe_create_single_vertex_muc(idx);
        }
    }
}
