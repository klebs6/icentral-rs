crate::ix!();


//-------------------------------------------[brandes_iter_hash]
pub struct BrandesIterHashWorkspace {
    parents:           ParentsMap,
    path_counts:       PathCounts,
    distances:         DistanceMap,
    pair_dependencies: PairDependencies,
    stack:             NodeIdStack,
    queue:             NodeIdQueue,
}

delegate_to_parents![BrandesIterHashWorkspace];
delegate_to_bfs_queue![BrandesIterHashWorkspace];
delegate_to_bfs_stack![BrandesIterHashWorkspace];
delegate_to_path_counts![BrandesIterHashWorkspace];
delegate_to_pair_dependencies![BrandesIterHashWorkspace];

impl BrandesIterHashWorkspace {

    pub fn new(len: usize, id: NodeId, name: &str) -> Self {

        debug!("creating new BrandesIterHashWorkspace of len {}, named {}, for nodeid: {}", len, name, id);

        let mut x = Self {
            parents:           ParentsMap::new(len,       name![name, "parents"]),
            path_counts:       PathCounts::new(len,       name![name, "path_counts"]),
            distances:         DistanceMap::new(len,      name![name, "distances"]),
            pair_dependencies: PairDependencies::new(len, name![name, "pair_dependencies"]),
            stack:             NodeIdStack::empty(        name![name, "stack"]),
            queue:             NodeIdQueue::empty(        name![name, "queue"]),
        };

        x.path_counts.set_path_count_to_one(id);

        x.distances.set_zero_distance(id);

        x.enqueue(id);

        x
    }

    pub fn search_neighborhood_and_update_path_counts<G: GetNeighborsForNode>(
        &mut self,
        graph: &G

    ) -> Result<(),BetweennessCentralityError> {

        while let Some(v) = self.queue.dequeue() {

            debug!("searching neighborhood and updating path counts for node: {}", v);

            self.stack_push(v);

            let nbr_vec = graph.neighbors(v);

            for &w in nbr_vec.iter() {

                if self.distances.is_infinite(w) {

                    debug!("during neighborhood search, we discovered a neighbor {} marked (infinite distance away from {})!", w, v);

                    debug!("adding {} to our work queue", w);

                    self.enqueue(w);

                    debug!("setting distance for {} to *one* away from {}", w, v);

                    self.distances.set_one_step_away(w, v);

                } else {

                    let w_distance = self.distances.distance(w);

                    debug!("during neighborhood search, we discovered a neighbor {} marked as {} units away from {}", w, w_distance, v);
                }

                if self.distances.is_one_step_away(w, v) {

                    debug!("w={} is found to be one step away from v={}", w, v);

                    debug!("will increment path_count_for_node w={} from v={}", w, v);

                    self.increment_path_count_for_node_from(w, v);

                    debug!("will add parent v={} to node w={}", v, w);

                    self.add_parent(w, v);
                }
            }
        }

        Ok(())
    }

    pub fn calculate_and_set_pair_dependencies(
        &mut self, 
        parent: NodeId, 
        node:   NodeId)
    {
        debug!("calculating pair dependences between node={}, and parent={}", node, parent);

        let sp_sn = self.path_count_ratio(parent,node);

        let node_pair_dependencies   = self.pair_dependency_for_node(node);
        let parent_pair_dependencies = self.pair_dependency_for_node(parent);

        self.set_pair_dependency_for_node(
            parent, 
            parent_pair_dependencies + sp_sn * (1.0 + node_pair_dependencies)
        );
    }

    pub fn update_pair_dependencies_and_scores(
        &mut self, 
        source:     NodeId, 
        scores: &mut BetweennessScores)
    -> Result<(),BetweennessCentralityError> 
    {
        debug!("will now pop the brandes stack for source={}", source);

        while let Some(node) = self.stack.pop() {

            debug!("popped node={} off the brandes stack -- will calculate pair dependencies for its parents", node);

            let parents = self.parents_for_node(node);

            debug!("parents={:?}", parents);

            for parent in parents {

                self.calculate_and_set_pair_dependencies(parent,node);
            }

            if node != source {

                //TODO: possibly want to
                //auto-vivify
                let pair_dep_for_node = self.pair_dependency_for_node(node);

                debug!("node={} != source={}, updating betweenness scores for node with pair_dependency value={}", node, source, pair_dep_for_node);

                scores.increase_score_for_node(
                    node, 
                    pair_dep_for_node
                );
            }
        }

        debug!("updated pair_dependencies and scores for source={}", source);

        Ok(())
    }
}
