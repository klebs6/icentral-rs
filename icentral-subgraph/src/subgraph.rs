crate::ix!();

/**
  | all functions here assume the given
  | node ids are proper ones from 0 to n-1
  | so, the caller must use
  |
  */
pub struct SubGraph {

    pub(crate) name:                   String,
    pub(crate) nodes_map:              NeighborsMap,
    pub(crate) edges:                  Edges,

    pub(crate) label_map:              LabelMap,

    /**
      | structures to be used by algorithms
      | they are here so they are built once
      |
      */
    pub(crate) parents:                ParentsMap,

    pub(crate) path_counts:            PathCounts,
    pub(crate) new_path_counts:        PathCounts,
    pub(crate) inc_path_counts:        PathCounts,

    pub(crate) distances:              DistanceMap,

    pub(crate) pair_dependencies:      PairDependencies,
    pub(crate) new_pair_dependencies:  PairDependencies,

    pub(crate) sigmas:                 SigmaMap,
    pub(crate) new_sigmas:             SigmaMap,

    pub(crate) visit_markers:          VisitMarkers,
    pub(crate) stack:                  NodeIdStack,
    pub(crate) queue:                  NodeIdQueue,
}

delegate_to_bfs_stack![SubGraph];
delegate_to_bfs_queue![SubGraph];
delegate_to_parents![SubGraph];
delegate_to_visit_markers![SubGraph];
delegate_to_label_map![SubGraph];
delegate_to_distances![SubGraph];

delegate_to_sigmas![SubGraph];
delegate_to_sigmas![SubGraph; new_sigmas];

delegate_to_pair_dependencies![SubGraph];
delegate_to_pair_dependencies![SubGraph; new_pair_dependencies];

delegate_to_path_counts![SubGraph];
delegate_to_path_counts![SubGraph; new_path_counts];
delegate_to_path_counts![SubGraph; inc_path_counts];

//-------------------------------------------[icentral/src/subgraph_t.cc]
impl SubGraph {

    delegate_to_neighbors_map!{}
    delegate_to_edges!{}

    pub fn iteration1_build_stack(
        &mut self,
        src:   NodeId,
        dst:   NodeId,
        stack: &mut Stack<NodeId>)
    -> Result<(),BetweennessCentralityError>
    {
        while let Some(v_i) = self.queue.dequeue() {

            stack.push(v_i);

            if v_i == dst {

                self.new_path_counts_increment_path_count_for_node(
                    v_i,
                    self.path_count_for_node(src)
                );
            }

            let nbrs = self.nodes_map.neighbors(v_i);

            for &v_n in nbrs.iter() {

                if self.distances.is_infinite(v_n) {

                    self.enqueue(v_n);

                    self.distances.set_distance_for_node(
                        v_n, 
                        self.distance(v_i) + 1.0
                    );
                }

                if self.distance(v_n) == self.distance(v_i) + 1.0 {

                    self.increment_path_count_for_node_from(
                        v_n, 
                        v_i
                    );

                    self.new_path_counts_increment_path_count_for_node_from(
                        v_n,
                        v_i,
                    );

                    self.add_parent(v_n, v_i);
                }
            }
        }

        Ok(())
    }

    pub fn iteration2_build_stack(&mut self, 
        s:     NodeId, 
        stack: &mut Vec<NodeId>)
    -> Result<(),BetweennessCentralityError> 
    {
        while let Some(v_i) = self.queue.dequeue() {

            stack.push(v_i);

            let neighbors = self.nodes_map.neighbors(v_i);

            for &v_n in neighbors.iter() {

                if self.distances.is_infinite(v_n) {

                    self.enqueue(v_n);

                    self.distances.set_distance_for_node(
                        v_n, 
                        self.distance(v_i) + 1.0
                    );
                }

                if self.distance_is_one_step_away(v_n, v_i) {

                    self.increment_path_count_for_node_from(
                        v_n, 
                        v_i
                    );

                    self.add_parent(v_n, v_i);
                }
            }
        }

        Ok(())
    }

    pub fn init_iteration1(&mut self, source: NodeId)
    {
        self.reinit_maps();

        self.set_path_count_to_one(source);
        self.new_path_counts.set_path_count_to_one(source);

        self.distances.set_zero_distance(source);

        self.enqueue(source);
    }

    pub fn iteration1_step1_process_nodes(
        &mut self, 
        v_i:   NodeId,
        stack: &mut Stack<NodeId>, 
        src:   NodeId, 
        dst:   NodeId)
    {
        let neighbors = self.nodes_map.neighbors(v_i);

        for &node in neighbors.iter() {

            if self.distances.is_infinite(node) {

                self.enqueue(node);

                self.distances.set_distance_for_node(
                    node,
                    self.distance(v_i) + 1.0
                );
            }

            if self.distances.is_one_step_away(node,v_i) {

                self.increment_path_count_for_node_from(
                    node, 
                    v_i
                );

                self.new_path_counts_increment_path_count_for_node_from(
                    node, 
                    v_i
                );

                self.parents.add_parent(node,v_i);
            }
        }
    }

    pub fn iteration1_fill_stack(
        &mut self, 
        stack: &mut Stack<NodeId>, 
        src:   NodeId, 
        dst:   NodeId)
    -> Result<(),BetweennessCentralityError> 
    {
        while let Some(v_i) = self.queue.dequeue() {

            stack.push(v_i);

            if v_i == dst {

                let src_paths = self.path_count_for_node(src);

                self.new_path_counts_increment_path_count_for_node(
                    v_i, 
                    src_paths
                );
            }

            self.iteration1_step1_process_nodes(
                v_i, 
                stack, 
                src, 
                dst
            );
        }

        Ok(())
    }

    pub fn init_iteration2(&mut self, source: NodeId)
    {
        self.reinit_maps();

        self.set_path_count_to_one(source);

        self.set_distance_zero(source);

        self.enqueue(source);
    }

    pub fn adjust_stack(&self, stack: &mut Vec<NodeId>) {

        // fix order of stack
        // IMP::THIS CAN BE MADE much BETTER!
        // HEAP FOR EXAMPLE
        // EVEN THE SWAPPING CAN BE DONE MORE EFFICIENTLY
        // for now it's not a bottleneck
        for i in 1..stack.len() {

            if self.distance_is_farther_away(stack[i - 1], stack[i]) {

                let mut j: usize = i;

                while self.distance_is_farther_away(stack[j - 1], stack[j]) {

                    let tmp: NodeId = stack[j - 1];

                    stack[j - 1] = stack[j];

                    stack[j] = tmp;

                    j -= 1;
                }
            }
        }
    }

    pub fn resize_pair_dependencies_and_sigma_based_on_map_len(&mut self) {

        let map_len = self.nodes_map.len();

        self.pair_dependencies.reinit(map_len);
        self.reinit_sigmas(map_len);
    }

    pub fn iteration2_update1(&mut self, stack: &mut Vec<NodeId>) 
    -> Result<(),BetweennessCentralityError>
    {
        while let Some(v_i) = self.dequeue() {

            stack.push(v_i);

            let neighbors = self.nodes_map.neighbors(v_i);

            for &node in neighbors.iter() {

                if self.distance_is_infinite(node) {

                    self.enqueue(node);

                    self.set_distance_one_step_away(node,v_i);
                }

                if self.distance_is_one_step_away(node,v_i) {

                    self.increment_path_count_for_node_from(node,v_i);

                    self.add_parent(node,v_i);
                }
            }
        }

        Ok(())
    }

    pub fn iteration2_update2_process_neighbor(&mut self, v: NodeId, nbr: NodeId) 
    {
        if self.distance_is_farther_than_one_away(nbr, v) {

            self.set_distance_one_step_away(nbr, v);

            self.set_single_parent(nbr, v);

            self.set_path_count_to_zero(nbr);

            self.inc_path_counts_update_path_counts(nbr,v);

            let nbr_inc_paths = self.inc_path_counts_path_count_for_node(nbr);

            self.increment_path_count_for_node(
                nbr, 
                nbr_inc_paths 
            );

            self.bfs_maybe_visit(nbr);

        } else if self.distance_is_one_step_away(nbr,v) {

            self.inc_path_counts_increment_path_count_for_node_from(nbr,v);

            self.increment_path_count_for_node(
                nbr, 
                self.inc_path_counts_path_count_for_node(v)
            );

            self.add_parent(nbr,v);

            self.bfs_maybe_visit(nbr);
        }
    }

    pub fn iteration2_update2(&mut self) 
    -> Result<(),BetweennessCentralityError>
    {
        while let Some(v) = self.dequeue() {

            let neighbors = self.neighbors(v);

            for &nbr in neighbors.iter() {

                self.iteration2_update2_process_neighbor(
                    v, 
                    nbr
                );
            }
        }

        Ok(())
    }

    pub fn bfs_maybe_visit(&mut self, node: NodeId) {

        if self.unvisited(node) {

            self.visit(node);

            self.enqueue(node);
        }
    }

    pub fn maybe_augment_bc_value_for_node(&self, 
        source: NodeId, 
        v_n:    NodeId, 
        scores: &mut BetweennessScores)
    {
        if source != v_n {

            let remapped: NodeId = self.label_map_inout(v_n);

            let val = scores.score_for_node(remapped) + self.pair_dependency_for_node(v_n) / 2.0;

            scores.set_score_for_node(
                remapped, 
                val
            );
        }
    }

    pub fn maybe_attenuate_bc_value_for_node(&self, 
        source: NodeId, 
        v_n:    NodeId, 
        scores: &mut BetweennessScores)
    {
        if source != v_n {

            let remapped: NodeId = self.label_map_inout(v_n);

            let val = scores.score_for_node(remapped) - self.pair_dependency_for_node(v_n) / 2.0;

            scores.set_score_for_node(
                remapped, 
                val
            );
        }
    }

    pub fn maybe_attenuate_bc_value_for_node_using_vertex_map<GH: NumNodes>(
        &self, 
        source:              NodeId, 
        v_n:                 NodeId, 
        scores:              &mut BetweennessScores,
        tmp_subgraph_map:    &SubGraphMap<GH>,
        tmp_conn_vertex_map: &ConnVertexMap)
    {
        if tmp_conn_vertex_map.has_mapping_for_node(source) {

            let vg_s: f64 = tmp_subgraph_map.subgraph_for_node(source).num_nodes() as f64;

            let new_v_n: NodeId = self.label_map_inout(v_n);

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) - self.pair_dependency_for_node(v_n) * vg_s
            );
        }
    }

    pub fn maybe_augment_bc_value_for_node_using_vertex_map<GH: NumNodes>(
        &self, 
        source:              NodeId, 
        v_n:                 NodeId, 
        scores:              &mut BetweennessScores,
        tmp_subgraph_map:    &SubGraphMap<GH>,
        tmp_conn_vertex_map: &ConnVertexMap)
    {
        if tmp_conn_vertex_map.has_mapping_for_node(source) {

            let vg_s: f64 = tmp_subgraph_map.subgraph_for_node(source).num_nodes() as f64;

            let new_v_n: NodeId = self.label_map_inout(v_n);

            let val = scores.score_for_node(new_v_n) + self.pair_dependency_for_node(v_n) * vg_s;

            scores.set_score_for_node(
                new_v_n, 
                val
            );
        }
    }

    pub fn rbfs1_to_add_the_new_pair_dependencies_step<GH: NumNodes>(
        &mut self,
        v_n:                 NodeId,
        source:              NodeId,
        stack:               &Vec<NodeId>,
        tmp_subgraph_map:    &SubGraphMap<GH>,
        tmp_conn_vertex_map: &ConnVertexMap,
        scores:              &mut BetweennessScores) 
    {
        self.maybe_update_all_sigmas(
            v_n,
            source,
            tmp_subgraph_map,
            tmp_conn_vertex_map
        );

        self.muc_attenuate_no_new(
            v_n,
            source,
            tmp_conn_vertex_map, 
            scores
        );

        self.maybe_attenuate_bc_value_for_node(
            source,
            v_n,
            scores
        );

        self.maybe_attenuate_bc_value_for_node_using_vertex_map(
            source,
            v_n,
            scores,
            tmp_subgraph_map,
            tmp_conn_vertex_map,
        );
    }

    pub fn rbfs1_to_add_the_new_pair_dependencies<GH: NumNodes>(
        &mut self,
        source:              NodeId,
        stack:               &Vec<NodeId>,
        tmp_subgraph_map:    &SubGraphMap<GH>,
        tmp_conn_vertex_map: &ConnVertexMap,
        scores:              &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError>
    {
        for i in (0..=stack.len() - 1).rev() {

            // RBFS to subtract old pair dependency
            let v_n: NodeId = stack[i];;

            self.rbfs1_to_add_the_new_pair_dependencies_step(
                v_n, 
                source, 
                stack, 
                tmp_subgraph_map, 
                tmp_conn_vertex_map, 
                scores
            );
        }

        Ok(())
    }

    pub fn rbfs2_to_add_the_new_pair_dependencies_step<GH: NumNodes>(
        &mut self,
        v_n:                 NodeId,
        source:              NodeId,
        stack:               &Vec<NodeId>,
        tmp_subgraph_map:    &SubGraphMap<GH>,
        tmp_conn_vertex_map: &ConnVertexMap,
        scores:              &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError>
    {
        self.maybe_update_all_sigmas(
            v_n,
            source,
            tmp_subgraph_map,
            tmp_conn_vertex_map
        )?;

        self.muc_augment_no_new(
            v_n,
            source,
            tmp_conn_vertex_map, 
            scores
        );

        self.maybe_augment_bc_value_for_node(
            source,
            v_n,
            scores
        );

        self.maybe_augment_bc_value_for_node_using_vertex_map(
            source,
            v_n,
            scores,
            tmp_subgraph_map,
            tmp_conn_vertex_map
        );

        Ok(())
    }

    pub fn rbfs2_to_add_the_new_pair_dependencies<GH: NumNodes>(
        &mut self,
        source:              NodeId,
        stack:               &Vec<NodeId>,
        tmp_subgraph_map:    &SubGraphMap<GH>,
        tmp_conn_vertex_map: &ConnVertexMap,
        scores:              &mut BetweennessScores) 
    {
        // RBFS to add the new pair dependencies
        for i in (0..=stack.len() - 1).rev() {

            let v_n: NodeId = stack[i];

            self.rbfs2_to_add_the_new_pair_dependencies_step(
                v_n, 
                source, 
                stack, 
                tmp_subgraph_map, 
                tmp_conn_vertex_map, 
                scores
            );
        }
    }

    pub fn src_dist_is_greater_than_dst_plus_one(
        &self, 
        src: NodeId, 
        dst: NodeId) -> bool
    {
        self.distance(src) > (self.distance(dst) + 1.0)
    }

    pub fn src_dist_equals_dst_plus_one(
        &self, 
        src: NodeId, 
        dst: NodeId) -> bool
    {
        self.distance(src) == (self.distance(dst) + 1.0)
    }
    
    // |src_distance-dst_distance| >= 2 (the difficult case)
    //
    pub fn iteration_2<GH: NumNodes>(
        &mut self, 
        tmp_subgraph_map:    &SubGraphMap<GH>,
        tmp_conn_vertex_map: &ConnVertexMap,
        source:              NodeId,
        src:                 NodeId,
        dst:                 NodeId,
        src_distance:        f64,
        dst_distance:        f64,
        scores:              &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError> 
    {
        let mut stack: Vec<NodeId> = default!();

        self.init_iteration2(source);

        self.iteration2_update1(&mut stack)?;

        /*
        * steps:
        * 1. do the reverse BFS and subtract the olds
        * 2. compute the new counts
        * 3. fix the order of stack
        * 4. add the new increments
        */
        self.rbfs1_to_add_the_new_pair_dependencies(
            source,
            &stack,
            tmp_subgraph_map,
            tmp_conn_vertex_map,
            scores,
        );

        self.compute_new_path_counts_and_paths(src,dst);

        self.iteration2_update2();

        self.adjust_stack(&mut stack);

        self.resize_pair_dependencies_and_sigma_based_on_map_len();

        self.rbfs2_to_add_the_new_pair_dependencies(
            source,
            &stack,
            tmp_subgraph_map,
            tmp_conn_vertex_map,
            scores,
        );

        Ok(())
    }

    pub fn iteration2_step3_when_src_dist_is_greater_than_dst_plus_one(
        &mut self, 
        w: NodeId, 
        v: NodeId)
    -> Result<(),BetweennessCentralityError>
    {
        self.distances.set_distance_for_node(
            w, 
            self.distance(v) + 1.0
        );

        self.set_single_parent(w,v);

        self.set_path_count_for_node(w,0);

        let v_inc_paths = self.inc_path_counts_path_count_for_node(v);

        self.inc_path_counts_set_path_count_for_node(
            w, 
            v_inc_paths 
        );

        let w_inc_paths = self.inc_path_counts_path_count_for_node(w);

        self.increment_path_count_for_node(
            w, 
            w_inc_paths 
        );

        self.bfs_maybe_visit(w);

        Ok(())
    }

    pub fn iteration2_step3_when_src_dist_equals_dst_plus_one(
        &mut self, 
        w: NodeId, 
        v: NodeId)
    -> Result<(),BetweennessCentralityError>
    {
        self.inc_path_counts_increment_path_count_for_node(
            w, 
            self.inc_path_counts_path_count_for_node(v)
        );

        self.increment_path_count_for_node(
            w, 
            self.inc_path_counts_path_count_for_node(v)
        );

        if !self.has_parent(w,v) {

            self.add_parent(w,v);
        }

        self.bfs_maybe_visit(w);

        Ok(())
    }

    pub fn iteration2_step3_process_neighbor(&mut self, v: NodeId, nbr: NodeId)
    -> Result<(),BetweennessCentralityError>
    {
        if self.src_dist_is_greater_than_dst_plus_one(nbr,v) {

            self.iteration2_step3_when_src_dist_is_greater_than_dst_plus_one(nbr,v)?;

        } else if self.src_dist_equals_dst_plus_one(nbr, v) {

            self.iteration2_step3_when_src_dist_equals_dst_plus_one(nbr, v)?;
        }

        Ok(())
    }

    pub fn iteration2_step3_process(&mut self, v: NodeId)
    {
        let nbrs = self.neighbors(v);

        for &nbr in nbrs.iter() {

            self.iteration2_step3_process_neighbor(v,nbr);
        }
    }

    pub fn iteration2_step3(&mut self)
    {
        while let Some(v) = self.queue.dequeue() {

            self.iteration2_step3_process(v);
        }
    }
}
