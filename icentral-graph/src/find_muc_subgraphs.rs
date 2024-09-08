crate::ix!();

impl<GH> FindMucSubgraphs for Graph<GH> 

where GH
: GetConnectedComponentSizes
+ ClearMucs 
+ CreateNamedEmpty
+ ExtendWith<GH,Error=BetweennessCentralityError>
+ GetEdges
+ GetNeighborsForNode
+ GetNodeIdRange
+ GraphHashMucInterface
+ HasMapForNode
+ InsertEdge
+ InsertNode
+ IsValid 
+ MappedNodes
+ NumEdges
+ NumNodes

{

    /**
      | TODO not sure what to do with single node
      | muc's, will not do anything for them
      | now
      |
      */
    fn find_muc_subgraphs(&mut self, muc_id: MinimumUnionCycleId) 
    -> Result<(),BetweennessCentralityError> 
    {
        debug!("finding muc_subgraphs for muc_id={}", muc_id);

        // THIS IS SLOW, and not needed for the
        // QUBE ideal speedup experiemtn so just
        // return if you want to do this
        // experiment..
        //
        // return;
        //
        if self.muc(muc_id).num_nodes() == 1 {
            return Ok(());
        }

        if !self.muc(muc_id).is_valid() {
            return Ok(());
        }

        let mut to_insert = vec![];

        let muc = self.muc(muc_id);

        if let conn_vertex_map = muc.vertex_map()? {

            for item in conn_vertex_map.iter() {

                let mapping = self.build_graphhash_mapping_for_conn_vertex(
                    item, 
                    muc_id
                )?;

                to_insert.push(mapping);
            }
        }

        for (id,g) in to_insert.into_iter() {
            self.mucs[muc_id.val()].insert_subgraph(id,g);
        }

        Ok(())
    }
}
