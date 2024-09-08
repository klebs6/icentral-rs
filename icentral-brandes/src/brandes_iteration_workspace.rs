crate::ix!();

pub struct BrandesIterationWorkspace {
    name:              String,
    stack:             NodeIdStack,
    queue:             NodeIdQueue,
    parents:           ParentsMap,
    path_counts:       PathCounts,
    distances:         DistanceMap,
    pair_dependencies: PairDependencies,
}

delegate_to_parents![BrandesIterationWorkspace];
delegate_to_bfs_queue![BrandesIterationWorkspace];
delegate_to_bfs_stack![BrandesIterationWorkspace];

impl BrandesIterationWorkspace {

    pub fn new(len: usize, id: NodeId, name: &str) -> Self {

        let stack_name             = name![name, "stack"];
        let queue_name             = name![name, "queue"];
        let parents_name           = name![name, "parents"];
        let path_counts_name       = name![name, "path_counts"];
        let distances_name         = name![name, "distances"];
        let pair_dependencies_name = name![name, "pair_dependencies"];

        let mut x = Self {
            name:               name.to_owned(),
            stack:              NodeIdStack::empty(stack_name),
            queue:              NodeIdQueue::empty(queue_name),
            parents:            ParentsMap::new(len, parents_name),
            path_counts:        PathCounts::new(len, path_counts_name),
            distances:          DistanceMap::new(len, distances_name),
            pair_dependencies:  PairDependencies::new(len, pair_dependencies_name),
        };

        x.path_counts.set_path_count_to_one(id);

        x.distances.set_zero_distance(id);

        x.enqueue(id);

        x
    }

    pub fn search_neighborhood_and_update_path_counts<G: GetNeighborsForNode>(
        &mut self,
        graph: &G) 
    -> Result<(),BetweennessCentralityError> 
    {
        while let Some(v) = self.queue.dequeue() {

            self.stack.push(v);

            let nbr_vec = graph.neighbors(v);

            for nbr_idx in 0..nbr_vec.len() {

                let w = nbr_vec[nbr_idx];

                if self.distances.is_infinite(w) {

                    self.enqueue(w);

                    self.distances.set_one_step_away(w, v);
                }

                if self.distances.is_one_step_away(w, v) {

                    self.path_counts.increment_path_count_for_node(
                        w,
                        self.path_counts.path_count_for_node(v)
                    );

                    self.add_parent(w,v);
                }
            }
        }

        Ok(())
    }

    pub fn update_pair_dependencies_and_scores(&mut self, 
        id:     NodeId, 
        scores: &mut BetweennessScores)
    -> Result<(),BetweennessCentralityError> 
    {
        while let Some(w) = self.stack.pop() {

            for &v in self.parents_for_node(w).iter() {

                self.pair_dependencies.increment_pair_dependency_for_node(
                    v, 
                    {
                        let t0 = {
                            let num   = self.path_counts.path_count_for_node(v) as f64;
                            let denom = self.path_counts.path_count_for_node(w) as f64;
                            num / denom
                        };

                        let t1 = 1.0 + self.pair_dependencies.pair_dependency_for_node(w);

                        t0 * t1
                    }
                );
            }

            if w != id {

                scores.increase_score_for_node(
                    w,  
                    self.pair_dependencies.pair_dependency_for_node(w)
                );
            }
        }

        Ok(())
    }
}
