crate::ix!();

impl<GH> FindEdgeBccWithComponent<GH> for Graph<GH> 
where GH
: HasEdge 
+ ConnectedComponentSize 
+ RemoveEdge
+ CreateNamedEmpty
+ MappedNodes
+ NumNodes
+ for<'a> Named<'a>
+ GetNeighborsForNode
+ GetNodeIdRange
+ GetEdges
+ BccGraphHashInterface
{
    /// the node is an articulation point and it
    /// belongs to the bcc of interest, so it must
    /// be added to the art point of this bcc
    /// along with the sizes of the subgraphs it
    /// connects the bcc to
    ///
    fn find_edge_bcc_with_component_step(
        &mut self, 
        gh:        &GH,
        v:         NodeId,
        component: &mut Component,
        e:         &Edge) 
    -> Result<(),BetweennessCentralityError>
    {
        let mut subgraph_sz_vec: Vec<usize>  = vec![];

        let visit_markers_name = name![
            self.name(), 
            "find_edge_bcc_with_component_step::visit_markers"
        ];

        let mut visit_markers = VisitMarkers::new_with_single_node_visited(
            self.num_nodes(),
            v,
            visit_markers_name
        );

        let v_nbr_vec = self.neighbors(v);

        for &u in v_nbr_vec.iter() {

            if !gh.has_edge(&Edge::new(v, u)) && visit_markers.unvisited(u) {

                let cnt = self.connected_component_size_through_v_and_this_edge(
                    &mut visit_markers, 
                    u
                );

                subgraph_sz_vec.push(cnt);

                // debug!("---{}, {}", v, cnt);
            }
        }

        component.map_articulation_point(v,&subgraph_sz_vec);

        Ok(())
    }
    
    /// a. find the bcc subgraph
    ///
    /// b. find the articulation points in bcc
    ///
    /// c. find the sizes of the subgraphs
    /// connected to each art point
    ///
    /// IMP: assumes @e is not in the graph
    /// already
    ///
    fn find_edge_bcc_with_component(
        &mut self, 
        component: &mut Component,
        e:         &Edge) 
    -> Result<(),BetweennessCentralityError>
    {
        if self.has_edge(e) {
            return Err(BetweennessCentralityError::DuplicateEdgeInsertion { 
                edge: e.clone() 
            });
        }

        self.insert_edge(e);

        let gh_name = name![self.name(), "find_edge_bcc_with_component::gh"];

        let mut gh: GH = GH::empty(gh_name);

        self.find_edge_bcc_subgraph(&mut gh, e);

        // g.remove_edge(e.src, e.dst); 
        //
        // TODO: make sure it's okay! [NO, needed
        // below]
        //
        let mut art_pt_vec: Vec<NodeId> = vec![];

        self.find_articulation_points(&mut art_pt_vec);

        // to make art points unique
        let mut art_pt_set: HashSet<NodeId> = default!();

        for i in 0..art_pt_vec.len() {
            art_pt_set.insert(art_pt_vec[i]);
        }

        for v in gh.mapped_nodes() {

            if art_pt_set.get(&v).is_none() {

                self.find_edge_bcc_with_component_step(
                    &gh, 
                    v, 
                    component, 
                    e
                )?;
            }
        }

        // fix articulation_point_map
        gh.remove_edge(e);

        component.reset_subgraph_with(&mut gh);

        component.remap_articulation_point_map();

        self.remove_edge(e);

        Ok(())
    }
}
