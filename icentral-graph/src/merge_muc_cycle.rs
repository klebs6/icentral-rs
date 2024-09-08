crate::ix!();

impl<GH> MergeMucCycle<GH> for Graph<GH> 

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
    /**
      | add all the edges in cycle to muc
      |
      */
    fn merge_muc_cycle(
        &mut self, 
        muc:   &mut MinimumUnionCycle<GH>,
        cycle: &Cycle)  {
        
        for i in 0..cycle.num_edges() {

            let mut src: NodeId = cycle[i].src;
            let mut dst: NodeId = cycle[i].dst;

            muc.insert_edge(&Edge::new(src, dst));

            self.nodes_to_mucs.set_mucid_for_node(src, muc.id());
            self.nodes_to_mucs.set_mucid_for_node(dst, muc.id());
        }
    }
}
