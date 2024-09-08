crate::ix!();

pub trait FindEdgeBccWithDelta<GH> {

    fn find_edge_bcc_with_delta_step(&mut self, 
        v:    NodeId,
        bcc:  &mut BiconnectedComponentsDelta<GH>,
        edge: &Edge)
    -> Result<(),BetweennessCentralityError>;

    fn find_edge_bcc_with_delta(&mut self, 
        bcc:  &mut BiconnectedComponentsDelta<GH>,
        edge: &Edge)
    -> Result<(),BetweennessCentralityError>;
}

pub struct BiconnectedComponentsDelta<GH> {
    delta_articulation_point_map: ArticulationPointMap,
    bcc_subgraph:                 GH,
    bcc_fast_subgraph:            SubGraph,
}

impl<GH> CreateNamedEmpty for BiconnectedComponentsDelta<GH> 
where GH: CreateNamedEmpty
{
    fn empty(name: &str) -> Self {

        let bcc_fast_subgraph_name            = name![name, "bcc_fast_subgraph"];
        let bcc_subgraph_name                 = name![name, "bcc_subgraph"];
        let delta_articulation_point_map_name = name![name, "delta_articulation_point_map"];

        Self {
            delta_articulation_point_map: ArticulationPointMap::empty_mapped(delta_articulation_point_map_name),
            bcc_subgraph:                 GH::empty(bcc_subgraph_name),
            bcc_fast_subgraph:            SubGraph::empty(bcc_fast_subgraph_name),
        }
    }
}

impl<GH> fmt::Debug for BiconnectedComponentsDelta<GH> 
where GH: GetPrintNodes + SetPrintNodes + Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let old = self.bcc_subgraph.get_print_nodes();

        self.bcc_subgraph.set_print_nodes(false);

        let res = f.debug_struct("BiconnectedComponentsDelta")
            .field("bcc_subgraph",                 &self.bcc_subgraph)
            .field("delta_articulation_point_map", &self.delta_articulation_point_map)
            .finish();

        self.bcc_subgraph.set_print_nodes(old);

        res
    }
}

//-------------------------------------------[icentral/src/bcc_delta.cc]

impl<GH> BiconnectedComponentsDelta<GH> 
where GH: ResetWith<GH> 
        + RemoveEdge 
        + FindSingleSourceShortestPaths 
        + MappedNodes 
        + NumNodes 
        + GetNodeIdRange 
        + GetNeighborsForNode 
        + ParentsForNode
        + InitDebugIteration
        + DebugIterationStep
        + NumEdges
        + FindPruningCounts
        + HasEdge
        + GetEdges 
        + InsertEdge 
        + RemoveEdge 
        + PathCountForNode 
        + PairDependencyForNode 
        + SetPairDependencyForNode 
        + SetSigmaValueForNode 
        + GetSigmaValueForNode
{
    delegate_to_subgraph!{bcc_fast_subgraph}

    delegate_to_graphhash!{bcc_subgraph}

    delegate_to_articulation_point_map!{delta}

    pub fn bcc_fast_subgraph_reset_with_graphhash(&mut self) {
        self.bcc_fast_subgraph.reset_with(&mut self.bcc_subgraph);
    }

    pub fn set_delta_articulation_point_map(
        &mut self, 
        other: ArticulationPointMap) 
    {
        self.delta_articulation_point_map = other;
    }

    pub fn bcc_subgraph(&self) -> &GH {
        &self.bcc_subgraph
    }

    pub fn bcc_subgraph_mut(&mut self) -> &mut GH {
        &mut self.bcc_subgraph
    }
    
    //////////////////////////////////////////////////
    //IMP:: FOR NOW THIS GUY DOESN'T USE fast_subgraph
    //////////////////////////////////////////////////
    pub fn compute_bc(
        &mut self, 
        scores: &mut BetweennessScores,
        src: NodeId,
        dst: NodeId)
    -> Result<(),BetweennessCentralityError> 
    {
        // scores will have for each vertex in the
        // muc it's betweenness centrality
        //
        self.bcc_subgraph.remove_edge(&Edge::new(src,dst));

        let src_distances = self.bcc_subgraph.find_single_source_shortest_paths(src)?;
        let dst_distances = self.bcc_subgraph.find_single_source_shortest_paths(dst)?;

        struct IterationJob {
            s:     NodeId, 
            src:   NodeId,
            dst:   NodeId,
            src_distance: f64, 
            dst_distance: f64,
        }

        let mut jobs = vec![];

        // this must be commented in general.. if
        // it's there it makes scores contain
        // deltas
        //
        // bcc_subgraph.i_fill_map<double>(scores, 0);
        //
        for s in self.bcc_subgraph.mapped_nodes() {

            if src_distances.distance(s) != dst_distances.distance(s) {

                jobs.push(
                    IterationJob {
                        s,
                        src,
                        dst,
                        src_distance: src_distances.distance(s),
                        dst_distance: dst_distances.distance(s),
                    }
                );
            }
        }

        for job in jobs {

            self.iteration(
                job.s,
                job.src,
                job.dst,
                job.src_distance,
                job.dst_distance,
                scores
            );
        }

        self.bcc_subgraph.insert_edge(
            &Edge::new(src,dst)
        );

        Ok(())
    }

    fn build_counts(&mut self, 
        scores:      &mut BetweennessScores,
        max_iter_d1: usize,
        max_iter_d2: usize,
        src:         NodeId, 
        dst:         NodeId, 
        src_distances:   &DistanceMap, 
        dst_distances:   &DistanceMap) 
    -> (Vec<i32>, Vec<Duration>) 
    {
        let mut tm: Timer = Timer::default();

        let mut cnt_arr: Vec<i32>      = vec!{0,0,0};
        let mut tot_arr: Vec<Duration> = vec![Duration::default(); 3];

        for i in 0..self.bcc_fast_subgraph.num_nodes() {

            let s: NodeId = i.try_into().unwrap();

            let diff: f64 = src_distances.distance(s) - dst_distances.distance(s);

            let abs_diff: f64 = diff.abs();

            tm.start();

            if src_distances.distance(s) != dst_distances.distance(s) {

                self.iteration(
                    s, 
                    src, 
                    dst, 
                    src_distances.distance(s), 
                    dst_distances.distance(s), 
                    scores
                );
            }

            tm.stop();

            match abs_diff {
                0.0  => {
                    cnt_arr[0] += 1;
                    tot_arr[0] += tm.interval();
                },

                1.0  => {
                    cnt_arr[1] += 1;
                    tot_arr[1] += tm.interval();
                },

                _  => {
                    cnt_arr[2] += 1;
                    tot_arr[2] += tm.interval();
                },
            }

            let eps = 0.001;

            let stop1: bool = (tot_arr[1].as_secs_f64() - max_iter_d1 as f64) < eps;
            let stop2: bool = (tot_arr[2].as_secs_f64() - max_iter_d2 as f64) < eps;

            if stop1 && stop2 {
                break;
            }
        }

        (cnt_arr,tot_arr)
    }
    
    pub fn compute_bc_exp(&mut self, 
        scores:      &mut BetweennessScores,
        mut src:     NodeId,
        mut dst:     NodeId,
        bcc_stat:    &mut BiconnectedComponentsStat,
        max_iter_d1: Option<usize>,
        max_iter_d2: Option<usize>)  
    -> Result<(),BetweennessCentralityError>
    {
        let max_iter_d1: usize = max_iter_d1.unwrap_or(usize::MAX);
        let max_iter_d2: usize = max_iter_d2.unwrap_or(usize::MAX);

        src = self.bcc_fast_subgraph.label_map_outin(src);
        dst = self.bcc_fast_subgraph.label_map_outin(dst);

        let mut tm: Timer = Timer::default();

        // scores will have for each vertex in the
        // muc it's betweenness centrality
        //
        self.bcc_fast_subgraph.remove_edge_between_nodes(src, dst);

        tm.start();

        let src_distances = self.bcc_fast_subgraph.find_single_source_shortest_paths(src)?;
        let dst_distances = self.bcc_fast_subgraph.find_single_source_shortest_paths(dst)?;

        tm.stop();

        bcc_stat.single_source_shortest_paths_tm = tm.interval();
        
        // this must be commented in general.. if
        // it's there it makes scores contain
        // deltas
        //
        // bcc_subgraph.i_fill_map<double>(scores, 0);

        let (cnt_arr,tot_arr) = self.build_counts(
            scores,
            max_iter_d1,
            max_iter_d2,
            src,
            dst,
            &src_distances,
            &dst_distances
        );

        self.bcc_fast_subgraph.insert_edge_between_nodes(src, dst);

        bcc_stat.update(&cnt_arr,&tot_arr);

        Ok(())
    }

    pub fn do_d1_iter(&mut self, 
        scores:        &mut BetweennessScores,
        max_iter_d1:   Option<usize>,
        d1_s_vec:      &Vec<NodeId>,
        cnt_arr:       &mut Vec<i32>,
        tot_arr:       &mut Vec<Duration>,
        src:           NodeId, 
        dst:           NodeId, 
        src_distances: &DistanceMap, 
        dst_distances: &DistanceMap) 
    {
        let mut tm: Timer = Timer::default();

        let d1_num_iter: usize = min(
            max_iter_d1.unwrap_or(usize::MAX),
            d1_s_vec.len()
        );

        for i in 0..d1_num_iter {

            let s: NodeId = d1_s_vec[i];

            tm.start();

            self.iteration(
                s, 
                src, 
                dst, 
                src_distances.distance(s), 
                dst_distances.distance(s), 
                scores
            );

            tm.stop();

            cnt_arr[1] += 1;

            tot_arr[1] += tm.interval();
        }
    }

    pub fn do_d2_iter(&mut self, 
        scores:      &mut BetweennessScores,
        max_iter_d2: Option<usize>,
        d2_s_vec:    &Vec<NodeId>,
        cnt_arr:     &mut Vec<i32>,
        tot_arr:     &mut Vec<Duration>,
        src:         NodeId, 
        dst:         NodeId, 
        src_distances:   &DistanceMap, 
        dst_distances:   &DistanceMap) 
    {
        let mut tm: Timer = Timer::default();

        let d2_num_iter: usize = min(
            max_iter_d2.unwrap_or(usize::MAX),
            d2_s_vec.len()
        );

        for i in 0..d2_num_iter {

            let s: NodeId = d2_s_vec[i];

            tm.start();

            self.iteration(
                s, 
                src, 
                dst, 
                src_distances.distance(s), 
                dst_distances.distance(s), 
                scores
            );

            tm.stop();

            cnt_arr[2] += 1;

            tot_arr[2] += tm.interval();
        }
    }
    
    pub fn build_arrays(
        &self, 
        src_distances: &DistanceMap,
        dst_distances: &DistanceMap,
        d1_s_vec:  &mut Vec<NodeId>,
        d2_s_vec:  &mut Vec<NodeId>,
        cnt_arr:   &mut Vec<i32>,
        tot_arr:   &mut Vec<Duration>)
    {
        for i in 0..self.bcc_fast_subgraph.num_nodes() {

            let s: NodeId = i.try_into().unwrap();

            let diff: f64 = src_distances.distance(s) - dst_distances.distance(s);

            let abs_diff: f64 = diff.abs();

            match abs_diff {
                0.0  => {
                    cnt_arr[0] += 1;
                },

                1.0  => {
                    d1_s_vec.push(s);
                },

                _  => {
                    d2_s_vec.push(s);
                },
            }
        }
    }

    pub fn compute_bc_maxiter_exp(&mut self, 
        scores:      &mut BetweennessScores,
        mut src:     NodeId,
        mut dst:     NodeId,
        bcc_stat:    &mut BiconnectedComponentsStat,
        max_iter_d1: Option<usize>,
        max_iter_d2: Option<usize>) 
    -> Result<(),BetweennessCentralityError>
    {
        src = self.bcc_fast_subgraph.label_map_outin(src);
        dst = self.bcc_fast_subgraph.label_map_outin(dst);

        let mut tm: Timer = Timer::default();

        // scores will have for each vertex in the
        // muc it's betweenness centrality
        //
        self.bcc_fast_subgraph.remove_edge_between_nodes(src, dst);

        tm.start();

        let src_distances = self.bcc_fast_subgraph.find_single_source_shortest_paths(src)?;
        let dst_distances = self.bcc_fast_subgraph.find_single_source_shortest_paths(dst)?;

        tm.stop();

        bcc_stat.single_source_shortest_paths_tm = tm.interval();

        // this must be commented in general.. if
        // it's there it makes scores contain
        // deltas
        //
        // bcc_subgraph.i_fill_map<double>(scores, 0);
        //
        let mut d1_s_vec: Vec<NodeId> = vec![];
        let mut d2_s_vec: Vec<NodeId> = vec![];

        let mut cnt_arr: Vec<i32>      = vec!{0,0,0};
        let mut tot_arr: Vec<Duration> = vec![Duration::from_secs(0); 3];

        self.build_arrays(
            &src_distances,
            &dst_distances,
            &mut d1_s_vec, 
            &mut d2_s_vec, 
            &mut cnt_arr, 
            &mut tot_arr
        );

        self.do_d1_iter(
            scores,
            max_iter_d1,
            &d1_s_vec,
            &mut cnt_arr,
            &mut tot_arr,
            src,
            dst,
            &src_distances,
            &dst_distances,
        );

        self.do_d2_iter(
            scores,
            max_iter_d2,
            &d2_s_vec,
            &mut cnt_arr,
            &mut tot_arr,
            src,
            dst,
            &src_distances,
            &dst_distances,
        );

        self.bcc_fast_subgraph.insert_edge_between_nodes(src, dst);

        bcc_stat.update(&cnt_arr,&tot_arr);

        Ok(())
    }
    
    pub fn iteration(&mut self, 
        source:    NodeId,
        mut src:   NodeId,
        mut dst:   NodeId,
        mut src_distance: f64,
        mut dst_distance: f64,
        scores:    &mut BetweennessScores)
    {
        make_sure_src_is_the_closer_to_source_node(
            &mut src, 
            &mut dst, 
            &mut src_distance, 
            &mut dst_distance
        );

        match dst_distance - src_distance {

            1.0 => {

                // dbg_iteration(s, src, dst, src_distance, dst_distance, scores);
                self.iteration_1(
                    source, 
                    src,
                    dst,
                    scores
                );

                // i_iteration_2(s, edge, src_distance, dst_distance, scores);
            }

            _ => {

                // dbg_iteration(s, edge, src_distance, dst_distance, scores);
                self.iteration_2(
                    source, 
                    src,
                    dst,
                    scores
                );
            }
        }
    }

    pub fn compute_path_cnt_ratio(&self, 
        v_p: NodeId, 
        v_n: NodeId) -> f64 
    {
        let p_paths = self.bcc_subgraph.path_count_for_node(v_p) as f64;
        let n_paths = self.bcc_subgraph.path_count_for_node(v_n) as f64;

        p_paths / n_paths
    }

    pub fn sigmas_insert_balanced(
        &mut self, 
        v_p:   NodeId, 
        v_n:   NodeId, 
        sp_sn: f64)
    {
        self.bcc_subgraph.set_sigma_value_for_node(
            v_p, 
            {
                let t0 = self.bcc_subgraph.sigma_value_for_node(v_p);
                let t1 = self.bcc_subgraph.sigma_value_for_node(v_n);
                t0 +  t1 * sp_sn
            }
        );
    }

    pub fn sigmas_insert_from_articulation_point_map(
        &mut self,
        source: NodeId,
        v_n:    NodeId)
    {
        let c_t: f64 = self.subgraphs_product_through_delta_articulation_points(source, v_n);

        self.bcc_subgraph.set_sigma_value_for_node(
            v_n, 
            {
                let t0 = self.bcc_subgraph.sigma_value_for_node(v_n);
                t0 + c_t
            }
        );
    }

    pub fn insert_pairwise_balanced(
        &mut self, 
        v_p:   NodeId, 
        v_n:   NodeId, 
        sp_sn: f64)
    {
        let p_pair_dependencies = self.bcc_subgraph.pair_dependency_for_node(v_p);
        let n_pair_dependencies = self.bcc_subgraph.pair_dependency_for_node(v_n);

        let val = p_pair_dependencies + sp_sn * (1.0 + n_pair_dependencies);

        self.bcc_subgraph.set_pair_dependency_for_node(
            v_p, 
            val
        );
    }

    pub fn maybe_update_all_sigmas(
        &mut self, 
        source: NodeId, 
        v_n:    NodeId)

    -> Result<(),BetweennessCentralityError> 
    {
        if self.has_both_delta_articulation_points(source, v_n)
        && source != v_n 
        {
            let c_t: f64 = self.subgraphs_product_through_delta_articulation_points(source, v_n);

            self.bcc_fast_subgraph.increment_sigma_value_for_node(
                v_n, 
                c_t
            );

            self.bcc_fast_subgraph.new_sigmas_increment_sigma_value_for_node(
                v_n, 
                c_t
            );

            /*this guy must not change!*/

            // scores.score_for_node(&v_n) = scores.score_for_node(&v_n) - c_t;
            // scores.score_for_node(&v_n) = scores.score_for_node(&v_n) + c_t;
        }

        Ok(())
    }

    pub fn update_parent_maps(
        &mut self, 
        source: NodeId, 
        v_p:    NodeId, 
        v_n:    NodeId, 
        scores: &mut BetweennessScores) 
    {
        let sp_sn = self.bcc_fast_subgraph.path_count_ratio(v_p, v_n);

        self.bcc_fast_subgraph.set_pair_dependency_for_node(
            v_p, 
            {
                self.bcc_fast_subgraph.pair_dependency_for_node(v_p) 
                + sp_sn 
                * (1.0 + self.bcc_fast_subgraph.pair_dependency_for_node(v_n))
            }
        );

        let new_sp_sn = self.bcc_fast_subgraph.new_path_counts_path_count_ratio(v_p, v_n);

        self.bcc_fast_subgraph.new_pair_dependencies_set_pair_dependency_for_node(
            v_p, 
            {
                let pdp = self.bcc_fast_subgraph.new_pair_dependencies_pair_dependency_for_node(v_p);
                let pdn = self.bcc_fast_subgraph.new_pair_dependencies_pair_dependency_for_node(v_n);
                pdp + new_sp_sn * (1.0 + pdn)
            }
        );

        if self.has_delta_articulation_point(source) {

            self.bcc_fast_subgraph.increment_sigma_value_for_node(
                v_p, 
                self.bcc_fast_subgraph.sigma_value_for_node(v_n) * sp_sn
            );

            self.bcc_fast_subgraph.new_sigmas_increment_sigma_value_for_node(
                v_p, 
                self.bcc_fast_subgraph.new_sigmas_sigma_value_for_node(v_n) * new_sp_sn
            );

            let new_v_p: NodeId = self.bcc_fast_subgraph.label_map_inout(v_p);

            scores.set_score_for_node(
                new_v_p, 
                scores.score_for_node(new_v_p) - self.bcc_fast_subgraph.sigma_value_for_node(v_n) * sp_sn / 2.0
            );

            scores.set_score_for_node(
                new_v_p, 
                scores.score_for_node(new_v_p) + self.bcc_fast_subgraph.new_sigmas_sigma_value_for_node(v_n) * new_sp_sn / 2.0
            );
        }
    }

    pub fn maybe_update_maps(
        &mut self, 
        source: NodeId, 
        v_n:    NodeId, 
        src:    NodeId,
        dst:    NodeId,
        scores: &mut BetweennessScores)
    {
        // IMP: this is the only change that
        // happens to self.bcc_fast_subgraph.P, @src should be added
        // as parent for dst
        //
        if v_n == dst {

            let v_p: NodeId = src;

            let new_sp_sn: f64 = {
                self.bcc_fast_subgraph.new_path_counts_path_count_for_node(v_p) as f64 
                / self.bcc_fast_subgraph.new_path_counts_path_count_for_node(v_n) as f64
            };

            self.bcc_fast_subgraph.new_pair_dependencies_set_pair_dependency_for_node(
                v_p, 
                {
                    let pdp = self.bcc_fast_subgraph.new_pair_dependencies_pair_dependency_for_node(v_p);
                    let pdn = self.bcc_fast_subgraph.new_pair_dependencies_pair_dependency_for_node(v_n);
                    pdp + new_sp_sn * (1.0 + pdn)
                }
            );

            if self.delta_articulation_point_map.has_articulation_point(source) {

                self.bcc_fast_subgraph.new_sigmas_increment_sigma_value_for_node(
                    v_p, 
                    self.bcc_fast_subgraph.new_sigmas_sigma_value_for_node(v_n) * new_sp_sn
                );

                let new_v_p: NodeId = self.bcc_fast_subgraph.label_map_inout(v_p);

                scores.set_score_for_node(
                    new_v_p, 
                    scores.score_for_node(new_v_p) + self.bcc_fast_subgraph.new_sigmas_sigma_value_for_node(v_n) * new_sp_sn / 2.0
                );
            }
        }
    }

    pub fn update_scores(&self, 
        source: NodeId, 
        v_n:    NodeId, 
        scores: &mut BetweennessScores)
    {
        if source != v_n {

            let new_v_n: NodeId = self.bcc_fast_subgraph.label_map_inout(v_n);

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) - self.bcc_fast_subgraph.pair_dependency_for_node(v_n) / 2.0
            );

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) + self.bcc_fast_subgraph.new_pair_dependencies_pair_dependency_for_node(v_n) / 2.0
            );
        }

        if self.has_delta_articulation_point(source) {

            let vg_s: f64 = self.subgraph_micentraltude_through_delta_articulation_point(source);

            let new_v_n: NodeId = self.bcc_fast_subgraph.label_map_inout(v_n);

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) - self.bcc_fast_subgraph.pair_dependency_for_node(v_n) * vg_s
            );

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) + self.bcc_fast_subgraph.new_pair_dependencies_pair_dependency_for_node(v_n) * vg_s
            );
        }
    }
    
    /**
      | src_distance-dst_distance| = 1 (the easy case)
      |
      */
    pub fn iteration_1(&mut self, 
        source: NodeId,
        src:    NodeId,
        dst:    NodeId,
        scores: &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError> 
    {
        let mut stack: Stack<NodeId> = default!();

        self.bcc_fast_subgraph.init_iteration1(source);

        self.bcc_fast_subgraph.iteration1_build_stack(src,dst,&mut stack)?;

        while let Some(v_n) = stack.pop() {

            self.maybe_update_all_sigmas(source,v_n);

            let parents = self.bcc_fast_subgraph.parents_for_node(v_n);

            for &v_p in parents.iter() {

                self.update_parent_maps(
                    source, 
                    v_p, 
                    v_n, 
                    scores
                );
            }

            self.maybe_update_maps(
                source, 
                v_n, 
                src, 
                dst, 
                scores
            );

            self.update_scores(source, v_n, scores);
        }

        Ok(())
    }

    pub fn iteration2_init_fast_subgraph(
        &mut self, 
        source: NodeId)
    {
        self.bcc_fast_subgraph.init_iteration2(source);
    }

    pub fn iteration2_build_stack(&mut self, s: NodeId) 
    -> Result<Vec<NodeId>,BetweennessCentralityError>
    {
        let mut stack: Vec<NodeId> = vec![];

        self.bcc_fast_subgraph.iteration2_build_stack(s,&mut stack)?;

        Ok(stack)
    }

    pub fn update_all_sigmas(
        &mut self, 
        s:   NodeId, 
        v_n: NodeId) 
    {
        if self.has_delta_articulation_point(s)
        && self.has_delta_articulation_point(v_n)
        {
            // && s != v_n)
            let c_t: f64 = self.subgraphs_product_through_delta_articulation_points(s, v_n);

            self.bcc_fast_subgraph.increment_sigma_value_for_node(v_n, c_t);

            /*this guy must not change!*/

            // scores.score_for_node(&v_n) = scores.score_for_node(&v_n) - c_t;
        }
    }

    pub fn update_parent_maps2(
        &mut self, 
        source: NodeId, 
        v_p:    NodeId, 
        v_n:    NodeId, 
        scores: &mut BetweennessScores)
    {
        let sp_sn = self.bcc_fast_subgraph.path_count_ratio(v_p, v_n);

        self.bcc_fast_subgraph.increment_pair_dependency_for_node(
            v_p,
            sp_sn * (1.0 + self.bcc_fast_subgraph.pair_dependency_for_node(v_n))
        );

        if self.has_delta_articulation_point(source) {

            self.bcc_fast_subgraph.increment_sigma_value_for_node(
                v_p, 
                self.bcc_fast_subgraph.sigma_value_for_node(v_n) * sp_sn
            );

            //    node_id_t new_v_p = self.bcc_fast_subgraph.label_map_inout_label_map[v_p];
            //    scores.score_for_node(&new_v_p) = scores.score_for_node(&new_v_p) - self.bcc_fast_subgraph.sigma_value_for_node(&v_n)*sp_sn/2.0;
        }
    }

    pub fn iteration2_step2(
        &mut self, 
        source: NodeId, 
        stack:  &Vec<NodeId>,
        scores: &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError>
    {
        /*
         | steps:
         | 1. do the reverse BFS and subtract the olds
         | 2. compute the new counts
         | 3. fix the order of stack 
         | 4. add the new increments
         */
        for i in (0..=stack.len() - 1).rev() {

            // RBFS to subtract old pair dependency
            let v_n: NodeId = stack[i];;

            self.update_all_sigmas(source,v_n);

            let parents = self.bcc_fast_subgraph.parents_for_node(v_n);

            for &v_p in parents.iter() {

                self.update_parent_maps2(
                    source, 
                    v_p, 
                    v_n, 
                    scores
                );
            }

            self.update_scores2(source, v_n, scores)
        }

        Ok(())
    }

    pub fn update_scores2(
        &self, 
        source: NodeId, 
        v_n:    NodeId, 
        scores: &mut BetweennessScores)
    {
        if source != v_n {

            let new_v_n: NodeId = self.bcc_fast_subgraph.label_map_inout(v_n);

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) - self.bcc_fast_subgraph.pair_dependency_for_node(v_n) / 2.0
            );
        }

        if self.has_delta_articulation_point(source) {

            let vg_s: f64 = self.subgraph_micentraltude_through_delta_articulation_point(source);

            let new_v_n: NodeId = self.bcc_fast_subgraph.label_map_inout(v_n);

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) - self.bcc_fast_subgraph.pair_dependency_for_node(v_n) * vg_s
            );

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) - self.bcc_fast_subgraph.sigma_value_for_node(v_n) / 2.0
            );
        }
    }

    pub fn compute_new_path_counts_and_paths(&mut self, src: NodeId, dst:NodeId)
    {
        self.bcc_fast_subgraph.compute_new_path_counts_and_paths(src, dst);
    }

    pub fn update_fast_subgraph(&mut self, src: NodeId, dst: NodeId)
    {
        self.bcc_fast_subgraph.update(src,dst);
    }

    pub fn iteration2_adjust_stack(&self, stack: &mut Vec<NodeId>)
    {
        // fix order of stack 
        // IMP::THIS CAN BE MADE much BETTER!
        // HEAP FOR EXAMPLE
        // EVEN THE SWAPPING CAN BE DONE MORE EFFICIENTLY
        // for now it's not a bottleneck
        for i in 1..stack.len() {

            if self.bcc_fast_subgraph.distance(stack[i - 1]) > self.bcc_fast_subgraph.distance(stack[i]) {

                let mut j: usize = i;

                while self.bcc_fast_subgraph.distance(stack[j - 1]) > self.bcc_fast_subgraph.distance(stack[j]) {

                    let tmp: NodeId = stack[j - 1];

                    stack[j - 1] = stack[j];

                    stack[j] = tmp;

                    j -= 1;
                }
            }
        }
    }

    pub fn iteration2_resize_pair_dependencies_and_sigma(&mut self) {

        let nodes_map_len = self.bcc_fast_subgraph.num_nodes();

        self.bcc_fast_subgraph.reinit_pair_dependencies(nodes_map_len);
        self.bcc_fast_subgraph.reinit_sigmas(nodes_map_len);
    }

    pub fn maybe_update_all_sigmas2(
        &mut self, 
        source: NodeId, 
        v_n:    NodeId)

    -> Result<(),BetweennessCentralityError> 
    {
        if self.has_both_delta_articulation_points(source,v_n)
        {
            // && s != v_n) 

            let c_t: f64 = self.subgraphs_product_through_delta_articulation_points(source, v_n);

            self.bcc_fast_subgraph.increment_sigma_value_for_node(v_n, c_t);

            /*this guy must not change!*/

            // scores.score_for_node(&v_n) = scores.score_for_node(&v_n) + c_t;
        }

        Ok(())
    }

    pub fn update_parent_maps3(
        &mut self, 
        source: NodeId, 
        v_p:    NodeId, 
        v_n:    NodeId, 
        scores: &mut BetweennessScores)
    {
        let sp_sn = self.compute_path_cnt_ratio(v_p,v_n);

        self.insert_pairwise_balanced(v_p,v_n,sp_sn);

        if self.has_delta_articulation_point(source) {

            self.bcc_fast_subgraph.increment_sigma_value_for_node(
                v_p, 
                self.bcc_fast_subgraph.sigma_value_for_node(v_n) * sp_sn
            );

            //   node_id_t new_v_p = self.bcc_fast_subgraph.label_map_inout_label_map[v_p];
            //   scores.score_for_node(&new_v_p) = scores.score_for_node(&new_v_p) + self.bcc_fast_subgraph.sigma_value_for_node(&v_n)*sp_sn/2.0;
        }
    }

    pub fn update_scores3(
        &self, 
        source: NodeId, 
        v_n:    NodeId, 
        scores: &mut BetweennessScores)
    {
        if source != v_n {

            let new_v_n: NodeId = self.bcc_fast_subgraph.label_map_inout(v_n);

            let new_val = scores.score_for_node(new_v_n) + self.bcc_fast_subgraph.new_pair_dependencies_pair_dependency_for_node(v_n) / 2.0;

            scores.set_score_for_node(
                new_v_n, 
                new_val
            );
        }

        if self.has_delta_articulation_point(source) {

            let vg_s: f64 = self.subgraph_micentraltude_through_delta_articulation_point(source);

            let new_v_n: NodeId = self.bcc_fast_subgraph.label_map_inout(v_n);

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) + self.bcc_fast_subgraph.pair_dependency_for_node(v_n) * vg_s as f64
            );

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) + self.bcc_fast_subgraph.sigma_value_for_node(v_n) / 2.0
            );
        }
    }

    pub fn iteration2_rbfs_to_add_the_new_pair_dependencies(
        &mut self,
        stack:  &Vec<NodeId>,
        source: NodeId,
        scores: &mut BetweennessScores) 
    {
        // RBFS to add the new pair dependencies
        for i in (0..=stack.len() - 1).rev() {

            let v_n: NodeId = stack[i];

            self.maybe_update_all_sigmas2(source, v_n);

            for i in 0..self.bcc_fast_subgraph.parents_for_node(v_n).len() {

                let v_p: NodeId = self.bcc_fast_subgraph.parents_for_node(v_n)[i];

                self.update_parent_maps3(
                    source, 
                    v_p, 
                    v_n, 
                    scores
                );
            }

            self.update_scores3(source, v_n, scores);
        }
    }

    // |src_distance-dst_distance| >= 2 (the difficult case)
    //
    pub fn iteration_2(&mut self, 
        s:      NodeId,
        src:    NodeId,
        dst:    NodeId,
        scores: &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError> 
    {
        self.iteration2_init_fast_subgraph(s);

        let mut stack: Vec<NodeId> = self.iteration2_build_stack(s)?;

        self.iteration2_step2(s,&stack, scores)?;

        self.compute_new_path_counts_and_paths(src,dst);

        self.update_fast_subgraph(src,dst);

        self.bcc_fast_subgraph.iteration2_step3();

        self.iteration2_adjust_stack(&mut stack);

        self.iteration2_debug_step();

        self.iteration2_resize_pair_dependencies_and_sigma();

        self.iteration2_rbfs_to_add_the_new_pair_dependencies(&stack, s, scores);

        Ok(())
    }

    pub fn maybe_insert_into_sigmas(
        &mut self, 
        v_n:    NodeId, 
        source: NodeId)
    {
        if self.has_delta_articulation_point(source)
        && self.has_delta_articulation_point(v_n)
        && source != v_n 
        {
            self.sigmas_insert_from_articulation_point_map(source,v_n);

            // this guy must not change!
            // scores.score_for_node(&v_n) = scores.score_for_node(&v_n) - c_t;
        }
    }
    pub fn rbfs_build_maps(&mut self, 
        v_n:          NodeId,
        v_s:          &Vec<NodeId>,
        source:       NodeId,
        src:          NodeId,
        dst:          NodeId,
        src_distance: f64,
        dst_distance: f64,
        scores:       &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError> 
    {
        let parents = self.bcc_subgraph_parents_for_node(v_n);

        for &v_p in parents.iter() {

            let sp_sn = self.compute_path_cnt_ratio(v_p,v_n);

            self.insert_pairwise_balanced(v_p,v_n,sp_sn);

            if self.has_delta_articulation_point(source) {

                self.sigmas_insert_balanced(v_p,v_n,sp_sn);

                scores.set_score_for_node(
                    v_p, 
                    scores.score_for_node(v_p) - self.bcc_subgraph_sigma_value_for_node(v_n) * sp_sn / 2.0
                );
            }
        }
        Ok(())
    }

    pub fn rbfs_maybe_insert(&mut self, 
        source: NodeId, 
        v_n:    NodeId, 
        scores: &mut BetweennessScores) 
    {
        if source != v_n {

            scores.set_score_for_node(
                v_n, 
                scores.score_for_node(v_n) - self.bcc_subgraph_pair_dependency_for_node(v_n) / 2.0
            );
        }
    }

    pub fn rbfs_to_subtract_old_pair_dependency_step(&mut self, 
        v_n:          NodeId,
        v_s:          &Vec<NodeId>,
        source:       NodeId,
        src:          NodeId,
        dst:          NodeId,
        src_distance: f64,
        dst_distance: f64,
        scores:       &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError> 
    {
        self.maybe_insert_into_sigmas(v_n,source);

        self.rbfs_build_maps(
            v_n, 
            v_s, 
            source, 
            src, 
            dst, 
            src_distance, 
            dst_distance, 
            scores
        )?;

        self.rbfs_maybe_insert(source,v_n,scores);

        if self.has_delta_articulation_point(source) {

            let vg_s: f64 = self.subgraph_micentraltude_through_delta_articulation_point(source);

            scores.set_score_for_node(
                v_n, 
                scores.score_for_node(v_n) - self.bcc_subgraph_pair_dependency_for_node(v_n) * vg_s
            );
        }

        Ok(())
    }

    pub fn rbfs_to_subtract_old_pair_dependency(&mut self, 
        v_s:          &Vec<NodeId>,
        source:       NodeId,
        src:          NodeId,
        dst:          NodeId,
        src_distance: f64,
        dst_distance: f64,
        scores:       &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError> 
    {
        for idx in (0..=v_s.len() - 1).rev() {

            let v_n: NodeId = v_s[idx];

            self.rbfs_to_subtract_old_pair_dependency_step(
                v_n,
                v_s, 
                source, 
                src, 
                dst, 
                src_distance, 
                dst_distance, 
                scores
            )?;
        }

        Ok(())
    }
    
    pub fn dbg_iteration(&mut self, 
        source:       NodeId,
        src:          NodeId,
        dst:          NodeId,
        src_distance: f64,
        dst_distance: f64,
        scores:       &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError> 
    {
        let mut v_s: Vec<NodeId> = vec![];

        self.bcc_subgraph_init_dbg_iteration(source);

        self.bcc_subgraph_dbg_iteration_step(&mut v_s)?;

        self.rbfs_to_subtract_old_pair_dependency(
            &v_s, 
            source, 
            src, 
            dst, 
            src_distance, 
            dst_distance, 
            scores
        )?;

        self.bcc_subgraph_insert_edge(&Edge::new(src, dst));

        self.bcc_subgraph_init_dbg_iteration(source);

        v_s.clear();

        self.bcc_subgraph_dbg_iteration_step(&mut v_s)?;

        self.rbfs_to_subtract_old_pair_dependency(
            &v_s, 
            source, 
            src, 
            dst, 
            src_distance, 
            dst_distance, 
            scores)?;

        self.bcc_subgraph_remove_edge(&Edge::new(src, dst));

        Ok(())
    }

    pub fn iteration2_debug_step(&self) {

        //DBG
        //        printf("===========================Source: %d", source);
        //        for(tr1_map_t<int>::iterator it = self.bcc_fast_subgraph.path_counts.begin();
        //                it != self.bcc_fast_subgraph.path_counts.end();
        //                ++it) {
        //            printf("cnt[%d]: %d           ", it->first, it->second);
        //            printf("# parents: %d", self.bcc_fast_subgraph.parents[it->first].len());
        //        }
        /////        
        //DBG
        //        printf("========================Source: %d", source);
        //        for(int i = stack.len()-1; i >= 0; --i) {
        //            printf("%d", stack[i]);
        //        }
        //////

        //DBG print all:
        //        printf("===========================Source: %d", source);
        //        for(tr1_map_t<int>::iterator it = self.bcc_fast_subgraph.path_counts.begin();
        //                it != self.bcc_fast_subgraph.path_counts.end();
        //                ++it) {
        //            printf("cnt[%d]: %d", it->first, it->second);
        //            for(int i = 0; i < self.bcc_fast_subgraph.parents[it->first].len(); ++i) {
        //                printf("        self.bcc_fast_subgraph.parents[%d][%d]: %d", it->first, i, self.bcc_fast_subgraph.parents[it->first][i]);
        //            }
        //        }
        //        printf("stack:  ");
        //        for(int i = stack.len()-1; i >= 0; --i) {
        //            printf("%d  ", stack[i]);
        //        }
        //        printf("");
        ///////////

        //DBG
        //        printf("===========================Source: %d", source);
        //        for(tr1_map_t<double>::iterator it = self.bcc_fast_subgraph.pair_dependencies.begin();
        //                it != self.bcc_fast_subgraph.pair_dependencies.end();
        //                ++it) {
        //            printf("pd[%d]: %f           ", it->first, it->second);
        //            printf("# parents: %d", self.bcc_fast_subgraph.parents[it->first].len());
        //        }
        //////
    }
}
