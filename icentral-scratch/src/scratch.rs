crate::ix!();

pub trait FindEdgeBccWithScratch<GH> {

    fn find_edge_bcc_with_scratch(&mut self, 
        bcc:  &mut BiconnectedComponentsScratch<GH>,
        edge: &Edge)
    -> Result<(),BetweennessCentralityError>;

    fn find_edge_bcc_with_scratch_step(&mut self, 
        v:    NodeId,
        bcc:  &mut BiconnectedComponentsScratch<GH>,
        edge: &Edge)
    -> Result<(),BetweennessCentralityError>;
}

pub struct BiconnectedComponentsScratch<GH> {
    name:                   String,
    articulation_point_map: ArticulationPointMap,
    bcc_subgraph:           GH,
    bcc_fast_subgraph:      SubGraph,
}

impl<GH> fmt::Debug for BiconnectedComponentsScratch<GH> 

where GH: GetPrintNodes + SetPrintNodes + Debug {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let old = self.bcc_subgraph.get_print_nodes();

        self.bcc_subgraph.set_print_nodes(false);

        let res = f.debug_struct("BiconnectedComponentsScratch")
            .field("bcc_subgraph",&self.bcc_subgraph)
            .field("articulation_point_map", &self.articulation_point_map)
            .finish();

        self.bcc_subgraph.set_print_nodes(old);

        res
    }
}

impl<GH> CreateNamedEmpty for BiconnectedComponentsScratch<GH> 

where GH: CreateNamedEmpty {

    fn empty(name: &str) -> Self {

        let bcc_fast_subgraph_name      = name![name, "bcc_fast_subgraph"];
        let bcc_subgraph_name           = name![name, "bcc_subgraph"];
        let articulation_point_map_name = name![name, "articulation_point_map"];

        Self {
            name:                   name.to_owned(),
            articulation_point_map: ArticulationPointMap::empty_mapped(articulation_point_map_name),
            bcc_subgraph:           GH::empty(bcc_subgraph_name),
            bcc_fast_subgraph:      SubGraph::empty(bcc_fast_subgraph_name),
        }
    }
}

impl<GH> BiconnectedComponentsScratch<GH> 

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
{

    delegate_to_subgraph!{bcc_fast_subgraph}

    delegate_to_graphhash!{bcc_subgraph}

    delegate_to_articulation_point_map!{}

    pub fn bcc_fast_subgraph_reset_with_graphhash(&mut self) {
        self.bcc_fast_subgraph.reset_with(&mut self.bcc_subgraph);
    }

    pub fn set_articulation_point_map(
        &mut self, 
        other: ArticulationPointMap) 
    {
        self.articulation_point_map = other;
    }

    pub fn bcc_subgraph(&self) -> &GH {
        &self.bcc_subgraph
    }

    pub fn bcc_subgraph_mut(&mut self) -> &mut GH {
        &mut self.bcc_subgraph
    }
    
    // This compute_bc DOES NOT return correct bc
    // values for articulation points
    //
    pub fn compute_bc(&mut self, 
        mut scores: &mut BetweennessScores,
        max_iter:   Option<usize>) 
    -> Result<(),BetweennessCentralityError> 
    {
        // scores will have for each vertex in the
        // muc it's betweenness centrality init bc
        // of all nodes to zero
        //
        // XXX why fill this scores? Should have
        // all vertices in the graph, here we
        // increment/decrement
        //
        for node in self.bcc_subgraph_mapped_nodes() {
            scores.set_score_for_node(node, 0.0);
        }

        // do BFS's from the nodes in the bcc
        match max_iter {

            Some(max_iter) => {


                for node in self.bcc_fast_subgraph.limited_nodeid_range(Some(max_iter)) {

                    self.bc_iter(node, scores);
                }
            }

            None => {

                for node in self.bcc_fast_subgraph.nodeid_range() {

                    self.bc_iter(node, scores);
                }

                for (k,size_vec) in self.articulation_point_map.iter() {

                    if size_vec.len() > 1 {

                        let mut sub: usize = 0;

                        for i in 0..size_vec.len() {
                            sub += size_vec[i] * size_vec[i];
                        }

                        let vg_i: f64 = size_vec.iter().sum::<usize>() as f64;

                        scores.set_score_for_node(
                            *k, 
                            scores.score_for_node(*k) + vg_i * vg_i - sub as f64
                        );
                    }
                }

                scores.halve();
            }
        }

        Ok(())
    }
    
    pub fn bc_iter(&mut self, 
        s:          NodeId,
        mut scores: &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError> 
    {
        let mut stack: Stack<NodeId> = default!();

        self.bcc_fast_subgraph.reinit_maps();

        self.bcc_fast_subgraph.set_path_count_for_node(s,1);
        self.bcc_fast_subgraph.set_distance_for_node(s,0.0);

        self.bcc_fast_subgraph.enqueue(s);

        while let Some(v_i) = self.bcc_fast_subgraph.dequeue() {

            stack.push(v_i);

            let neighbors = self.bcc_fast_subgraph.neighbors(v_i);

            for &v_n in neighbors.iter() {

                if self.bcc_fast_subgraph.distance_is_infinite(v_n) {

                    self.bcc_fast_subgraph.enqueue(v_n);

                    self.bcc_fast_subgraph.set_distance_one_step_away(v_n,v_i);
                }

                if self.bcc_fast_subgraph.distance_is_one_step_away(v_n, v_i) {

                    self.bcc_fast_subgraph.increment_path_count_for_node_from(
                        v_n, 
                        v_i
                    );

                    self.bcc_fast_subgraph.parents_for_node(v_n).push(v_i);
                }
            }
        }

        while let Some(v_n) = stack.pop() {

            if self.has_both_articulation_points(s, v_n) 
            && s != v_n 
            {
                let c_t: f64 = self.subgraphs_product_through_articulation_points(s, v_n);

                self.bcc_fast_subgraph.increment_sigma_value_for_node(v_n, c_t);

                let new_v_n: NodeId = self.bcc_fast_subgraph.label_map_inout(v_n);

                scores.set_score_for_node(
                    new_v_n, 
                    scores.score_for_node(new_v_n) + c_t
                );
            }

            for i in 0..self.bcc_fast_subgraph.parents_for_node(v_n).len() {

                let v_p: NodeId = self.bcc_fast_subgraph.parents_for_node(v_n)[i];

                let sp_sn = self.bcc_fast_subgraph.path_count_ratio(v_p, v_n);

                self.bcc_fast_subgraph.increment_pair_dependency_for_node(
                    v_p,
                    {
                        let pdn = self.bcc_fast_subgraph.pair_dependency_for_node(v_n);
                        sp_sn * (1.0 + pdn)
                    }
                );

                if self.has_articulation_point(s) {

                    self.bcc_fast_subgraph.set_sigma_value_for_node(
                        v_p, 
                        {
                            let t0 = self.bcc_fast_subgraph.sigma_value_for_node(v_p);
                            let t1 = self.bcc_fast_subgraph.sigma_value_for_node(v_n);
                            t0 + t1 * sp_sn
                        }
                    );

                    let new_v_p: NodeId = self.bcc_fast_subgraph.label_map_inout(v_p);

                    scores.set_score_for_node(
                        new_v_p, 
                        scores.score_for_node(new_v_p) + self.bcc_fast_subgraph.sigma_value_for_node(v_n) * sp_sn
                    );
                }
            }

            if s != v_n {

                let new_v_n: NodeId = self.bcc_fast_subgraph.label_map_inout(v_n);

                scores.set_score_for_node(
                    new_v_n, 
                    scores.score_for_node(new_v_n) + self.bcc_fast_subgraph.pair_dependency_for_node(v_n)
                );
            }

            if self.has_articulation_point(s) {

                let vg_s: f64 = self.subgraph_micentraltude_through_articulation_point(s);

                let new_v_n: NodeId = self.bcc_fast_subgraph.label_map_inout(v_n);

                scores.set_score_for_node(
                    new_v_n, 
                    scores.score_for_node(new_v_n) + self.bcc_fast_subgraph.pair_dependency_for_node(v_n) * (vg_s as f64) * 2.0
                );
            }
        }

        Ok(())
    }
}
