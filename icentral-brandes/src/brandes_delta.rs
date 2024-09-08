crate::ix!();

/// case 1: dst_distance - src_distance = 1 
///
/// (the sort of easier case)
///
pub fn brandes_delta_step_adjacent<G: GetNeighborsForNode>(
    graph:     Arc<Mutex<G>>,
    source:    NodeId,
    mut src:   NodeId,
    mut dst:   NodeId,
    scores:    &mut BetweennessScores) 
-> Result<(),BetweennessCentralityError> 
where G: NumNodes + Debug + GetNeighborsForNode
{
    debug!("initiating brandes_delta_step_adjacent for source: {}", source);

    let num_nodes = graph.lock()?.num_nodes();

    let mut workspace = BrandesDeltaIterWorkspace::new(
        num_nodes,
        source, 
        "brandes_delta_step_adjacent.workspace"
    );

    match graph.lock() {

        Ok(mut graph_guard) => {
            let graph = &*graph_guard;
            workspace.search_neighborhood_and_update_path_counts(graph, src, dst)?;
        }

        // Handle the locking error.
        Err(e) => {

            return Err(BetweennessCentralityError::LockError {
                msg: format!("Failed to acquire lock on graph during update: {:?}", e)
            });
        }
    }

    workspace.update_pair_dependencies_and_scores(
        source, 
        src, 
        dst, 
        scores
    )?;

    Ok(())
}

pub struct BrandesDeltaStepAdjacentWorkspace {
    name:               String,
    parents:            ParentsMap,
    path_counts:        PathCounts,
    inc_path_counts:    PathCounts,
    distances:          DistanceMap,
    pair_dependencies:  PairDependencies,
    queue:              NodeIdQueue,
    stack:              NodeIdStack,
}

delegate_to_parents![BrandesDeltaStepAdjacentWorkspace];
delegate_to_bfs_queue![BrandesDeltaStepAdjacentWorkspace];
delegate_to_bfs_stack![BrandesDeltaStepAdjacentWorkspace];
delegate_to_pair_dependencies![BrandesDeltaStepAdjacentWorkspace];

impl BrandesDeltaStepAdjacentWorkspace {

    pub fn new(num_nodes: usize, source: NodeId, name: &str) -> Self {

        let mut x = Self {
            name:              name.to_owned(),
            parents:           ParentsMap::new(num_nodes,            &format!{"{}.parents",           name }),
            path_counts:       PathCounts::new(num_nodes,            &format!{"{}.path_counts",       name }),
            inc_path_counts:   PathCounts::new(num_nodes,            &format!{"{}.inc_path_counts",   name }),
            distances:         DistanceMap::new(num_nodes,           &format!{"{}.distances",         name }),
            pair_dependencies: PairDependencies::new(num_nodes,      &format!{"{}.pair_dependencies", name }),
            queue:             NodeIdQueue::empty(                   &format!{"{}.queue",             name }),
            stack:             NodeIdStack::empty(                   &format!{"{}.stack",             name }),
        };

        x.path_counts.set_path_count_to_one(source);

        x.distances.set_zero_distance(source);

        x.enqueue(source);

        x
    }

    pub fn maybe_update_distances(&mut self, w: NodeId, v: NodeId) {

        if self.distances.is_infinite(w) {

            self.enqueue(w);

            self.distances.set_one_step_away(w, v);
        }
    }

    pub fn maybe_update_path_counts(&mut self, w: NodeId, v: NodeId) {

        if self.distances.is_one_step_away(w, v) {

            self.path_counts.increment_path_count_for_node_from(w,v);

            self.add_parent(w,v);
        }
    }

    pub fn maybe_update_counts_for_neighbors<G: GetNeighborsForNode>(&mut self, graph: &G, v: NodeId) {

        for &w in graph.neighbors(v).iter() {

            self.maybe_update_distances(w,v);
            self.maybe_update_path_counts(w,v);
        }
    }

    pub fn build_stack_and_update_neighbors<G: GetNeighborsForNode>(&mut self, graph: &G) 
    {
        while let Some(v) = self.queue.dequeue() {

            self.stack.push(v);

            self.maybe_update_counts_for_neighbors(graph,v);
        }
    }

    pub fn update_pair_dependencies(&mut self, w: NodeId, v: NodeId) {

        let path_ratio = self.path_counts.path_count_ratio(v,w);

        let w_pair_dependencies = self.pair_dependency_for_node(w);

        self.pair_dependencies.increment_pair_dependency_for_node(
            v,
            path_ratio * (1.0 + w_pair_dependencies)
        );
    }

    pub fn update_pair_dependencies_for_parents(
        &mut self, 
        w:      NodeId,
        source: NodeId, 
        scores: &mut BetweennessScores) 
    {
        let parents = self.parents_for_node(w);

        for &v in parents.iter() {

            self.update_pair_dependencies(
                w,
                v
            );
        }
    }

    pub fn attenuate_based_on_pair_dependencies_step(
        &mut self, 
        w:      NodeId,
        source:      NodeId, 
        scores: &mut BetweennessScores) 
    {
        self.update_pair_dependencies_for_parents(w, source, scores);

        if w != source {

            scores.decrease_score_for_node(
                w, 
                self.pair_dependency_for_node(w) / 2.0
            );
        }
    }

    /// TODO: is this a good name?
    ///
    pub fn attenuate_based_on_pair_dependencies(
        &mut self, 
        source:      NodeId, 
        scores: &mut BetweennessScores) 
    {
        for i in (0..=self.stack.len() - 1).rev() {

            let w: NodeId = self.stack_node_at_index(i);

            self.attenuate_based_on_pair_dependencies_step(w, source, scores);
        }
    }

    pub fn update_dst_based_on_src(
        &mut self, 
        src: NodeId, 
        dst: NodeId) 
    -> Result<(),BetweennessCentralityError> 
    {
        // vector<int> old_distances = distances;    
        self.enqueue(dst);

        self.distances.set_one_step_away(dst, src);

        self.clear_node_parents(dst);

        self.add_parent(dst,src);

        self.inc_path_counts.set_path_count_for_node(
            dst, 
            self.path_counts.path_count_for_node(src)
        );

        self.path_counts.set_path_count_for_node(
            dst,
            self.inc_path_counts.path_count_for_node(dst)
        );

        Ok(())
    }

    pub fn update_path_counts_for_neighbors_and_mark_visited_step_far<G>(
        &mut self, 
        w:     NodeId,
        graph: &mut G, 
        v:     NodeId) 
    {
        self.distances.set_one_step_away(w, v);

        self.clear_node_parents(w);

        self.add_parent(w,v);

        self.path_counts.set_path_count_to_zero(w);

        self.inc_path_counts.set_path_count_for_node(
            w, 
            self.inc_path_counts.path_count_for_node(v)
        );

        self.path_counts.increment_path_count_for_node(
            w, 
            self.inc_path_counts.path_count_for_node(w)
        );
    }

    pub fn update_path_counts_for_neighbors_and_mark_visited_step_adjacent<G>(
        &mut self, 
        w:     NodeId,
        graph: &mut G,
        v:     NodeId) 
    {
        self.inc_path_counts.increment_path_count_for_node_from(w, v);

        self.path_counts.increment_path_count_for_node(
            w, 
            self.inc_path_counts.path_count_for_node(v)
        );

        // if(old_distance(&w) == old_distance(&v) || v == dst) {
        if !self.has_parent(w,v) {

            self.add_parent(w,v);
        }
    }

    pub fn update_path_counts_for_neighbors_and_mark_visited_step<G>(
        &mut self, 
        w:     NodeId,
        graph: &mut G,
        v:     NodeId

    ) {

        let w_dist    = self.distances.distance(w);
        let v_dist_p1 = self.distances.distance(v) + 1.0;

        match w_dist {

            dist if dist > v_dist_p1 => {

                self.update_path_counts_for_neighbors_and_mark_visited_step_far(w, graph, v);
            }

            dist if dist == v_dist_p1 => {

                self.update_path_counts_for_neighbors_and_mark_visited_step_adjacent(w, graph, v);
            }

            _ => { }
        }
    }

    pub fn maybe_visit<G: VisitMarkersHandle>(&mut self, w: NodeId, graph: &mut G) {

        if graph.visit_markers_handle().unvisited(w) {

            graph.visit_markers_handle().visit(w);

            self.enqueue(w);
        }
    }

    pub fn update_path_counts_for_neighbors_and_mark_visited<G: GetNeighborsForNode + VisitMarkersHandle>(
        &mut self, 
        graph: &mut G,
        v:     NodeId) 
    {
        // S.push(v);
        for &w in graph.neighbors(v).iter() {

            self.update_path_counts_for_neighbors_and_mark_visited_step(w, graph, v);

            self.maybe_visit(w,graph);
        }
    }

    pub fn update_path_counts<G: VisitMarkersHandle + GetNeighborsForNode>(&mut self, graph: &mut G) 
    -> Result<(),BetweennessCentralityError> 
    {
        while let Some(v) = self.queue.dequeue() {

            self.update_path_counts_for_neighbors_and_mark_visited(
                graph, 
                v, 
            );
        }

        Ok(())
    }

    pub fn fix_stack_step(&mut self, i: usize) 
    {
        let mut j: usize = i;

        let j0 = self.stack_node_at_index(j - 1);
        let j1 = self.stack_node_at_index(j);

        while self.distance(j0) > self.distance(j1) {

            let tmp: NodeId = self.stack_node_at_index(j - 1);

            self.stack_set_node_at_index(
                j - 1, 
                self.stack_node_at_index(j)
            );

            self.stack_set_node_at_index(j, tmp);

            j -= 1;
        }
    }

    pub fn fix_stack(&mut self) 
    {
        // TODO:
        //
        // IMP::THIS CAN BE MADE much BETTER!
        //
        // HEAP FOR EXAMPLE
        //
        // EVEN THE SWAPPING CAN BE DONE MORE
        // EFFICIENTLY
        for i in 1..self.stack.len() {

            let i0 = self.stack_node_at_index(i - 1);
            let i1 = self.stack_node_at_index(i);

            if self.distance(i0) > self.distance(i1) {
                self.fix_stack_step(i);
            }
        }
    }

    pub fn distance(&self, n: NodeId) -> f64 {
        self.distances.distance(n)
    }

    pub fn augment_with_pair_dependencies_step(
        &mut self, 
        w:      NodeId,
        source: NodeId, 
        scores: &mut BetweennessScores) 
    {
        self.update_pair_dependencies_for_parents(
            w, 
            source, 
            scores
        );

        if w != source {

            scores.increase_score_for_node(
                w,
                self.pair_dependencies.pair_dependency_for_node(w) / 2.0
            )
        }
    }

    pub fn augment_with_pair_dependencies(
        &mut self, 
        source: NodeId, 
        scores: &mut BetweennessScores) 
    {
        self.pair_dependencies.fill(0.0);

        for i in (0..=self.stack_len() - 1).rev() {

            let w: NodeId = self.stack_node_at_index(i);

            self.augment_with_pair_dependencies_step(w, source, scores);
        }
    }
}

pub fn brandes_delta_step_far<G: NumNodes + VisitMarkersHandle + GetNeighborsForNode + ResetVisitMarkersAndVisitNode>(
    graph:   &mut G,
    source:  NodeId,
    mut src: NodeId,
    mut dst: NodeId,
    scores:  &mut BetweennessScores) 
-> Result<(),BetweennessCentralityError> 
{
    let num_nodes = graph.num_nodes();

    let mut workspace = BrandesDeltaStepAdjacentWorkspace::new(num_nodes, source, "workspace");

    workspace.build_stack_and_update_neighbors(&*graph);

    workspace.attenuate_based_on_pair_dependencies(source, scores);

    workspace.update_dst_based_on_src(src, dst)?;

    graph.reset_visit_markers_and_visit_node(dst);

    workspace.update_path_counts(graph)?;

    workspace.fix_stack();

    workspace.augment_with_pair_dependencies(source, scores);

    Ok(())
}

pub fn make_sure_src_is_the_closer_to_source_node(
    src:          &mut NodeId,
    dst:          &mut NodeId,
    src_distance: &mut f64,
    dst_distance: &mut f64) 
{
    if src_distance > dst_distance {

        let mut tmp_v: NodeId = NodeId::default();

        let mut tmp_i: f64 = 0.0;

        tmp_v = *src;

        *src = *dst;

        *dst = tmp_v;

        tmp_i = *src_distance;

        *src_distance = *dst_distance;

        *dst_distance = tmp_i;
    }
}

/**
  | increments bc values in @scores with the change
  | of pair dependency of @source after adding edge
  | (src, dst) edge (src, dst) is assumed not to be
  | in the graph
  |
  | the two cases, when d(source, dst) - d(source, src) is
  | 1 and > 2 are handled here
  |
  */
pub fn brandes_delta_iter<G>(
    graph:         Arc<Mutex<G>>,
    source:        NodeId,
    src:           &mut NodeId,
    dst:           &mut NodeId,
    src_distance:  &mut f64,
    dst_distance:  &mut f64,
    scores:        &mut BetweennessScores) 
-> Result<(),BetweennessCentralityError> 

where G
: NumNodes 
+ VisitMarkersHandle 
+ Debug 
+ ResetVisitMarkersAndVisitNode 
+ GetNeighborsForNode

{
    // make sure that @src is the closer to source node
    make_sure_src_is_the_closer_to_source_node(
        src,
        dst,
        src_distance,
        dst_distance
    );

    match *dst_distance - *src_distance {

        1.0 => Ok(
            brandes_delta_step_adjacent(
                graph,
                source,
                *src,
                *dst,
                scores
            )?
        ),
        _ => {

            match graph.lock() {

                Ok(mut graph_guard) => {

                    let graph = &mut *graph_guard;

                    Ok(
                        brandes_delta_step_far(
                            graph,
                            source,
                            *src,
                            *dst,
                            scores
                        )?
                    )
                },

                Err(e) => Err(BetweennessCentralityError::LockError {
                    msg: format!("Failed to acquire lock on graph during update: {:?}", e)
                })
            }
        },
    }
}
