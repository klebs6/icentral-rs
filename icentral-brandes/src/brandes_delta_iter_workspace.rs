crate::ix!();


pub struct BrandesDeltaIterWorkspace {
    stack:                 NodeIdStack, 
    queue:                 NodeIdQueue,
    parents:               ParentsMap,
    old_path_counts:       PathCounts,
    new_path_counts:       PathCounts,
    distances:             DistanceMap,
    old_pair_dependencies: PairDependencies,
    new_pair_dependencies: PairDependencies,
}

delegate_to_parents![BrandesDeltaIterWorkspace];
delegate_to_distances![BrandesDeltaIterWorkspace];
delegate_to_bfs_queue![BrandesDeltaIterWorkspace];
delegate_to_bfs_stack![BrandesDeltaIterWorkspace];

impl BrandesDeltaIterWorkspace {

    pub fn new(len: usize, s: NodeId, name: &str) -> Self {

        let mut x = Self {
            stack:                 NodeIdStack::empty(        &format!{"{}.stack",                   name}), 
            queue:                 NodeIdQueue::empty(        &format!{"{}.queue",                   name}),
            parents:               ParentsMap::new(len,       &format!{"{}.parents",                 name}),
            old_path_counts:       PathCounts::new(len,       &format!{"{}.old_path_counts",         name}),
            new_path_counts:       PathCounts::new(len,       &format!{"{}.new_path_counts",         name}),
            distances:             DistanceMap::new(len,      &format!{"{}.distances",               name}),
            old_pair_dependencies: PairDependencies::new(len, &format!{"{}.old_pair_dependencies",   name}),
            new_pair_dependencies: PairDependencies::new(len, &format!{"{}.new_pair_dependencies",   name}),
        };

        x.old_path_counts.set_path_count_to_one(s);
        x.new_path_counts.set_path_count_to_one(s);

        x.distances.set_zero_distance(s);

        x.enqueue(s);

        x
    }

    pub fn search_neighborhood_and_update_path_counts<G: GetNeighborsForNode>(
        &mut self,
        graph: &G,
        src:   NodeId,
        dst:   NodeId) 
    -> Result<(),BetweennessCentralityError> 
    {
        while let Some(id) = self.queue.dequeue() {

            self.stack.push(id);

            if id == dst {

                self.new_path_counts.increment_path_count_for_node(
                    id, 
                    self.old_path_counts.path_count_for_node(src)
                );
            }

            let nbr_vec = graph.neighbors(id);

            for &w in nbr_vec.iter() {

                if self.distance(w) < 0.0 {

                    self.enqueue(w);

                    self.set_distance_one_step_away(w,id);
                }

                if self.distance(w) == self.distance(id) + 1.0 {

                    self.old_path_counts.increment_path_count_for_node(
                        w, 
                        self.old_path_counts.path_count_for_node(id)
                    );

                    self.new_path_counts.increment_path_count_for_node(
                        w, 
                        self.new_path_counts.path_count_for_node(id)
                    );

                    self.add_parent(w, id);
                }
            }
        }

        Ok(())
    }

    pub fn update_pair_dependencies_and_scores(
        &mut self, 
        s:      NodeId, 
        src:    NodeId, 
        dst:    NodeId, 
        scores: &mut BetweennessScores)
    -> Result<(),BetweennessCentralityError> 
    {
        while let Some(w) = self.stack.pop() {

            for &v in self.parents_for_node(w).iter() {

                self.old_pair_dependencies.increment_pair_dependency_for_node(
                    v, 
                    {
                        let t0 = self.old_path_counts.path_count_for_node(v) as f64 / self.old_path_counts.path_count_for_node(w) as f64;

                        let t1 = 1.0 + self.old_pair_dependencies.pair_dependency_for_node(w);

                        t0 * t1
                    }
                );

                self.new_pair_dependencies.increment_pair_dependency_for_node(
                    v, 
                    {
                        let t0 = self.new_path_counts.path_count_for_node(v) as f64 / self.new_path_counts.path_count_for_node(w) as f64;

                        let t1 = 1.0 + self.new_pair_dependencies.pair_dependency_for_node(w);

                        t0 * t1
                    }
                );
            }

            // IMP: this is the only change
            // that happens to parents, @src should
            // be added as parent for dst
            if w == dst {

                let v: NodeId = src;

                self.new_pair_dependencies.increment_pair_dependency_for_node(
                    v, 
                    {
                        let t0 = self.new_path_counts.path_count_for_node(v) as f64 / self.new_path_counts.path_count_for_node(w) as f64;
                        let t1 = 1.0 + self.new_pair_dependencies.pair_dependency_for_node(w);
                        t0 * t1
                    }
                );
            }

            if w != s {

                scores.increase_score_for_node(
                    w,  
                    (self.new_pair_dependencies.pair_dependency_for_node(w) - self.old_pair_dependencies.pair_dependency_for_node(w)) / 2.0
                );

                // scores.increase_score_for_node(&w,  (0 - old_pair_dependencies.pair_dependency_for_node(&w))/2.0);
            }
        }

        Ok(())
    }
}
