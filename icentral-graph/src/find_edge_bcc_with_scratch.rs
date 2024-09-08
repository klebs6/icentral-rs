crate::ix!();

impl<GH> FindEdgeBccWithScratch<GH> for Graph<GH> 

where GH
: DebugIterationStep
+ FindPruningCounts
+ GetEdges
+ GetNeighborsForNode
+ GetNodeIdRange
+ GetSigmaValueForNode
+ HasEdge
+ InitDebugIteration
+ InsertEdge
+ MappedNodes
+ NumEdges
+ NumNodes
+ PairDependencyForNode
+ ParentsForNode
+ RemoveEdge
+ BccGraphHashInterface
{
    /// a. find the bcc subgraph
    ///
    /// b. find the articulation points in bcc
    ///
    /// c. find the sizes of the subgraphs
    /// connected to each art point
    ///
    fn find_edge_bcc_with_scratch(&mut self, 
        bcc:  &mut BiconnectedComponentsScratch<GH>,
        edge: &Edge)
    -> Result<(),BetweennessCentralityError>
    {
        self.find_edge_bcc_subgraph(bcc.bcc_subgraph_mut(), &edge);

        bcc.bcc_fast_subgraph_reset_with_graphhash();

        let mut art_pt_vec: Vec<NodeId> = vec![];

        self.find_articulation_points(&mut art_pt_vec);

        let mut art_pt_set: HashSet<NodeId> = default!();

        for i in 0..art_pt_vec.len() {
            art_pt_set.insert(art_pt_vec[i]);
        }

        for v in bcc.bcc_subgraph_mapped_nodes() {

            if art_pt_set.contains(&v) {

                self.find_edge_bcc_with_scratch_step(v, bcc, edge)?;
            }
        }

        let new_delta_articulation_point_map_name = name![
            self.name(), 
            "find_edge_bcc_with_scratch::new_delta_articulation_point_map"
        ];

        // fix articulation_point_map
        let mut new_delta_articulation_point_map = ArticulationPointMap::empty_mapped(new_delta_articulation_point_map_name);

        for item in bcc.articulation_point_map_iter() {

            let new_v: NodeId = bcc.bcc_fast_subgraph_label_map_outin(*item.0);

            new_delta_articulation_point_map.map_articulation_point(
                new_v,
                &item.1.to_vec()
            );
        }

        bcc.set_articulation_point_map(new_delta_articulation_point_map);

        Ok(())
    }

    /// the node is an articulation point and it
    /// belongs to the bcc of interest, so it must
    /// be added to the art point of this bcc
    /// along with the sizes of the subgraphs it
    /// connects the bcc to
    ///
    fn find_edge_bcc_with_scratch_step(&mut self, 
        v:    NodeId,
        bcc:  &mut BiconnectedComponentsScratch<GH>,
        edge: &Edge)
    -> Result<(),BetweennessCentralityError>
    {
        let mut subgraph_sz_vec: Vec<usize>  = vec![];

        let visit_markers_name = name![
            self.name(), 
            "find_edge_bcc_With_scratch_step::name"
        ];

        let mut visit_markers = VisitMarkers::new_with_single_node_visited(
            self.num_nodes(), 
            v,
            visit_markers_name
        );

        let v_nbr_vec = self.neighbors(v);

        for u in NodeIdRange::new(0,v_nbr_vec.len()) {

            if !bcc.bcc_subgraph_has_edge(&Edge::new(v, u)) && visit_markers.unvisited(u) {

                let cnt = self.connected_component_size_through_v_and_this_edge(
                    &mut visit_markers, 
                    u
                );

                subgraph_sz_vec.push(cnt);
            }
        }

        bcc.map_articulation_point(
            v,
            &subgraph_sz_vec
        );

        Ok(())
    }
}
