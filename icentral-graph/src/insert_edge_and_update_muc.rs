/// His disciples asked him: "What should we do for our work to be perfect?"
///
/// The master said to them: "be ready in every circumstance. Blessed are they
/// who have found the strife, and who have seen the struggle with their eyes. 
///
/// They have not killed, nor have they been killed, but they have emerged
/// victorious."
///
/// Judas asked "Tell me, master, what is the beginning of the way?"
///
/// He said to them "Love, and goodness. If one of these had existed among the
/// rulers, wickedness would never have come to be."
///
/// Matthew said "you have spoken of the end of the universe with no difficulty"
///
/// The master said "you have understood all things I have said to you, and you
/// have accepted them in faith.  If you know them, they are yours. If not, they
/// are not yours.
///
/// They asked him "to what place are we going?"
///
/// He said "stand in the place you can reach."
///
/// If you have understood everything I have told you, you will become immortal.
/// For you... everything.
///
/// - the dialogue of the savior.
///
crate::ix!();

impl<GH> InsertEdgeUpdateMuc for Graph<GH>
where GH
: GraphHashMucInterface 
+ ClearMucs 
+ CreateNamedEmpty 
+ ExtendWith<GH,Error=BetweennessCentralityError>
+ GetConnectedComponentSizes 
+ GetNeighborsForNode 
+ GetNodeIdRange
+ IsValid 
{
    /// case 1: both @src and @dst are in the same
    /// muc
    ///
    ///         then just update the graph and the
    ///         muc graph
    ///
    /// just update the muc and the graph
    ///
    fn insert_edge_and_update_muc_when_full_edge_contained_in_muc(&mut self, 
        edge:       &Edge, 
        src_muc_id: MinimumUnionCycleId) 
    {
        debug!("full edge is contained in muc");

        self.mucs[src_muc_id.val()].insert_edge(&edge);

        self.insert_edge(&edge);
    }

    /// case 2: @src and @dst are not in the same
    /// muc
    ///
    fn insert_edge_and_update_muc_when_we_are_not_in_the_same_muc(
        &mut self, 
        edge:       &Edge, 
        src_muc_id: MinimumUnionCycleId,
        dst_muc_id: MinimumUnionCycleId) 
    -> Result<(),BetweennessCentralityError>
    {
        debug!("we will insert_edge and update_muc, but we are not in the same muc");

        let mut shortest_path = self.get_shortest_path(
            edge.src, 
            edge.dst
        )?;

        let id = self.create_and_push_new_muc(
            &shortest_path, 
            edge,
            src_muc_id,
            dst_muc_id
        )?;

        self.insert_edge(&edge);

        // TODO updating the connection
        // vertices must be done in a much
        // faster way!
        //
        self.find_conn_verts();;

        self.find_muc_subgraphs(id);

        Ok(())
    }

    fn insert_edge_update_muc(&mut self, edge: &Edge) {

        let src_muc_id: MinimumUnionCycleId = self.nodes_to_mucs.mucid_for_node(edge.src);
        let dst_muc_id: MinimumUnionCycleId = self.nodes_to_mucs.mucid_for_node(edge.dst);

        debug!("insert_edge_update_muc with src_muc_id={}, dst_muc_id={}", src_muc_id, dst_muc_id);

        if src_muc_id == dst_muc_id {

            self.insert_edge_and_update_muc_when_full_edge_contained_in_muc(
                edge,
                src_muc_id
            );

        } else {

            self.insert_edge_and_update_muc_when_we_are_not_in_the_same_muc(
                edge,
                src_muc_id,
                dst_muc_id
            );
        }
    }
}
