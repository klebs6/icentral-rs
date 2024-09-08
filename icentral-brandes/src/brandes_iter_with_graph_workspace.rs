crate::ix!();


pub struct BrandesIterWithGraphWorkspace {
    stack:             NodeIdStack,
    queue:             NodeIdQueue,
    parents:           ParentsMap,
    path_counts:       PathCounts,
    distances:         DistanceMap,
    pair_dependencies: PairDependencies,
}

delegate_to_parents![BrandesIterWithGraphWorkspace];
delegate_to_bfs_queue![BrandesIterWithGraphWorkspace];
delegate_to_bfs_stack![BrandesIterWithGraphWorkspace];

impl BrandesIterWithGraphWorkspace {

    pub fn new(len: usize, s: NodeId, name: &str) -> Self {

        let mut x = Self {
            stack:             NodeIdStack::empty(        &format!{"{}.stack",             name}),
            queue:             NodeIdQueue::empty(        &format!{"{}.queue",             name}),
            parents:           ParentsMap::new(len,       &format!{"{}.parents",           name}), 
            path_counts:       PathCounts::new(len,       &format!{"{}.path_counts",       name}),
            distances:         DistanceMap::new(len,      &format!{"{}.distances",         name}),
            pair_dependencies: PairDependencies::new(len, &format!{"{}.pair_dependencies", name}),
        };

        x.path_counts.set_path_count_to_one(s);

        x.distances.set_zero_distance(s);

        x.enqueue(s);

        x
    }

    pub fn maybe_update_distances(&mut self, v: NodeId, w: NodeId)
    {
        if self.distances.is_infinite(w) {

            self.enqueue(w);

            self.distances.set_one_step_away(w, v);
        }
    }

    pub fn maybe_update_path_counts_and_parents(&mut self, v: NodeId, w: NodeId)
    {
        if self.distances.is_one_step_away(w, v) {

            self.path_counts.increment_path_count_for_node(
                w,
                self.path_counts.path_count_for_node(v)
            );

            self.add_parent(w,v);
        }
    }

    pub fn search_neighborhood_and_update_path_counts<G: GetNeighborsForNode>(&mut self, graph: &mut G) 
    -> Result<(),BetweennessCentralityError>
    {
        while let Some(v) = self.queue.dequeue() {

            self.stack.push(v);

            let nbr_vec = graph.neighbors(v);

            trace!("v {} nbr_vec len {}", v, nbr_vec.len());

            for &w in nbr_vec.iter() {

                self.maybe_update_distances(v,w);

                self.maybe_update_path_counts_and_parents(v,w);
            }
        }

        Ok(())
    }

    pub fn augment_pair_dependencies(&mut self, v: NodeId, w: NodeId) {

        let v_paths = self.path_counts.path_count_for_node(v) as f64;
        let w_paths = self.path_counts.path_count_for_node(w) as f64;

        let paths_ratio = v_paths / w_paths;

        let update = paths_ratio * (1.0 + self.pair_dependencies.pair_dependency_for_node(w));

        self.pair_dependencies.increment_pair_dependency_for_node(v, update);
    }

    pub fn update_scores(
        &mut self, 
        s:      NodeId, 
        scores: &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError> 
    {
        while let Some(w) = self.stack.pop() {

            for &v in self.parents_for_node(w).iter() {

                self.augment_pair_dependencies(v,w);
            }

            if w != s {

                scores.increase_score_for_node(
                    w,  
                    self.pair_dependencies.pair_dependency_for_node(w)
                );
            }
        }

        Ok(())
    }
}
