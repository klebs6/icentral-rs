crate::ix!();

impl<GH> CreateAndPushNewMuc for Graph<GH> 

where GH
: ExtendWith<GH,Error=BetweennessCentralityError> 
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
    type Error = BetweennessCentralityError;

    fn create_and_push_new_muc(
        &mut self, 
        shortest_path: &Vec<NodeId>,
        edge:          &Edge, 
        src_muc_id:    MinimumUnionCycleId,
        dst_muc_id:    MinimumUnionCycleId)
    -> Result<MinimumUnionCycleId,Self::Error>
    {
        debug!("we will create and push a new MUC");

        let mut new_muc: MinimumUnionCycle<GH> = default!();

        new_muc.set_id(self.mucs.len());

        merge_mucs(
            &mut self.nodes_to_mucs,
            &mut new_muc, 
            &mut self.mucs[dst_muc_id.val()]
        );

        merge_mucs(
            &mut self.nodes_to_mucs,
            &mut new_muc, 
            &mut self.mucs[src_muc_id.val()]
        );

        for &node in shortest_path.iter() {

            let node_muc_id: MinimumUnionCycleId = self.nodes_to_mucs.mucid_for_node(node);

            if node_muc_id != new_muc.id() {

                merge_mucs(
                    &mut self.nodes_to_mucs,
                    &mut new_muc, 
                    &mut self.mucs[node_muc_id.val()]
                );
            }
        }

        new_muc.insert_edge(&edge);

        let id = new_muc.id();

        // cout << endl << endl;
        // new_muc.muc_subgraph.print_graph();
        //
        self.mucs.push(new_muc);

        Ok(id)
    }
}
