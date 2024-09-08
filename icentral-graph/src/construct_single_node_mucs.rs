crate::ix!();

impl<GH> ConstructSingleNodeMucs for Graph<GH> 

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

    fn maybe_construct_single_node_muc(&mut self, idx: NodeId) {

        if self.nodes_to_mucs.mucid_for_node(idx) == MinimumUnionCycleId::inf() {

            let mut muc = MinimumUnionCycle::<GH>::default();

            muc.set_id(self.mucs.len());

            muc.insert_node(idx);

            self.nodes_to_mucs.set_mucid_for_node(
                idx, 
                muc.id()
            );

            self.mucs.push(muc);
        }
    }

    fn construct_single_node_mucs(&mut self) {

        debug!("constructing single node MinimumUnionCycles...");

        // print_mucs();
        //
        // create single vertex muc's
        //
        for idx in NodeIdRange::new(0,self.nodes_to_mucs.len()) {

            self.maybe_construct_single_node_muc(idx);
        }
    }
}
