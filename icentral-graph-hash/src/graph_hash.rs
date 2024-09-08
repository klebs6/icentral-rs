crate::ix!();

//-------------------------------------------[icentral/src/graph_hash_t.cc]

pub type BcMap = Arc<Mutex<BetweennessScores>>;

/**
  | graph with nodes having integral indices
  | that shouldn't be between 0 and n-1 will
  | be used to store MinimumUnionCycles and subgraphs
  |
  */
pub struct GraphHash {

    name:                   String,
    nodes_map:              NeighborsMap,
    edges:                  Edges,

    /**
      | structures to be used by algorithms
      | they are here so they are built once
      |
      */
    parents:                ParentsMap,
    path_counts:            PathCounts,
    new_path_counts:        PathCounts,
    inc_path_counts:        PathCounts,
    distances:              DistanceMap,
    pair_dependencies:      PairDependencies,
    new_pair_dependencies:  PairDependencies,
    sigmas:                 SigmaMap,
    new_sigmas:             SigmaMap,
    visit_markers:          VisitMarkers,
    stack:                  NodeIdStack,
    queue:                  NodeIdQueue,
    print_nodes:            AtomicBool,
}

delegate_to_parents!{GraphHash}
delegate_to_distances!{GraphHash}
delegate_to_sigmas!{GraphHash}
delegate_to_sigmas!{GraphHash; new_sigmas}
delegate_to_visit_markers!{GraphHash}
delegate_to_bfs_queue!{GraphHash}
delegate_to_bfs_stack!{GraphHash}
delegate_to_pair_dependencies!{GraphHash}
delegate_to_pair_dependencies!{GraphHash; new_pair_dependencies}
delegate_to_path_counts!{GraphHash}
delegate_to_path_counts!{GraphHash; new_path_counts}
delegate_to_path_counts!{GraphHash; inc_path_counts}

impl HasMapForNode for GraphHash {

    delegate!{
        to self.nodes_map {
            fn has_map_for_node(&self, id: NodeId) -> bool;
        }
    }
}

impl GetNodeIdRange for GraphHash {

    delegate!{
        to self.nodes_map {
            fn nodeid_range(&self) -> Vec<NodeId>;
        }
    }
}

impl PairDependencyForNode for GraphHash {

    delegate!{ 
        to self.pair_dependencies {
            fn pair_dependency_for_node(&self, node: NodeId) -> f64;
        }
    }
}

impl ResetWith<GraphHash> for GraphHash {
    fn reset_with(&mut self, g: &GraphHash) {
        todo!();
    }
}

impl SetPairDependencyForNode for GraphHash {

    delegate!{ 
        to self.pair_dependencies {
            fn set_pair_dependency_for_node(&mut self, node: NodeId, val: f64);
        }
    }
}

impl PathCountForNode for GraphHash {

    delegate!{

        to self.path_counts {

            fn path_count_for_node(&self, node: NodeId) -> usize;

            fn path_count_for_node_ref(&self, node: NodeId) -> &usize;

            fn path_count_for_node_mut(&mut self, node: NodeId) -> &mut usize;
        }
    }
}

impl HasEdge for GraphHash {

    fn has_edge(&self, e: &Edge) -> bool {
        self.edges.has_edge(e)
    }
}

impl ParentsForNode for GraphHash {

    fn parents_for_node(&self, v_n: NodeId) -> Vec<NodeId> {
        self.parents.parents_for_node(v_n)
    }
}

impl NumNodes for GraphHash {
    fn num_nodes(&self) -> usize {
        self.nodes_map.len()
    }
}

impl NumEdges for GraphHash {
    fn num_edges(&self) -> usize {
        self.edges.len()
    }
}

impl GetEdges for GraphHash {
    fn edges(&self) -> &Edges {
        &self.edges
    }
}

impl MappedNodes for GraphHash {
    fn mapped_nodes(&self) -> Vec<NodeId> {
        self.nodes_map.mapped_nodes()
    }
}

impl Named for GraphHash {

    fn name(&self) -> Cow<'_,str> {
        Cow::Borrowed(&self.name)
    }
}

impl GraphHash {

    delegate_to_neighbors_map!{}
    delegate_to_edges!{}
}

impl<G> NewFromGraphRef<G> for GraphHash 
where G: GetEdges
{

    fn new_from_graph_ref(graph: &G, name: &str) -> Self {

        let mut x = GraphHash::empty(name);

        for edge in graph.edges().iter() {
            x.insert_edge(&edge);
        }

        x
    }
}

impl NewFromCycleVec for GraphHash {

    fn new_from_cycle_vec(cycle_vec: &Vec<Cycle>, name: &str) -> Self {

        let mut x = GraphHash::empty(name);

        for i in NodeIdRange::new(0,cycle_vec.len()) {
            x.insert_node(i);
        }

        for i in 0..cycle_vec.len() {

            for j in 0..cycle_vec.len() {

                if i != j && shared_vertex(&cycle_vec[i], &cycle_vec[j]) 
                {
                    x.insert_edge(&Edge::new_with_ids(i, j));
                }
            }
        }

        x
    }
}

impl fmt::Debug for GraphHash {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let binding = f.debug_struct("GraphHash");

        let mut builder = binding;

        builder.field("edges",  &self.edges);

        if self.print_nodes.load(atomic::Ordering::SeqCst) {

            builder.field("nodes_map", &self.nodes_map);

        } 

        builder.finish()
    }
}

impl SpawnScores for GraphHash {

    fn spawn_scores(&self) -> BetweennessScores
    {
        let scores_name = name![self.name(), "scores"];

        BetweennessScores::new_from_graph_ref(self, scores_name) 
    }
}

impl BrandesIterInit for GraphHash {

    fn brandes_iter_init(&mut self, s: NodeId)
    {
        self.reinit_maps();

        self.stack.clear();

        self.path_counts.set_path_count_for_node(s,1);

        self.distances.set_distance_for_node(s,0.0);

        self.enqueue(s);
    }
}

impl BrandesIterUpdateDistancesAndPathForNeighbors for GraphHash {

    fn brandes_iter_update_dist_and_path_for_neighbors(&mut self, s: NodeId)
    {
        while let Some(v) = self.dequeue() {

            self.stack.push(v);

            self.update_dist_and_path_for_neighbors(v);
        }
    }
}

impl BrandesIterUpdatePairDependenciesAndFill for GraphHash {

    fn brandes_iter_update_pair_dependencies_and_fill(
        &mut self, 
        s:      NodeId, 
        scores: &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError> 
    {
        for i in (0..=self.stack.len() - 1).rev() {

            let w: NodeId = self.stack_node_at_index(i);

            self.update_parent_pair_dependencies_and_increment_score_for_node(w, s, scores);
        }

        Ok(())
    }
}

impl SetPrintNodes for GraphHash {

    fn set_print_nodes(&self, val: bool) {
        self.print_nodes.store(val, atomic::Ordering::SeqCst);
    }
}

impl GetPrintNodes for GraphHash {

    fn get_print_nodes(&self) -> bool {
        self.print_nodes.load(atomic::Ordering::SeqCst)
    }
}

impl GetNeighborsForNode for GraphHash {

    fn neighbors(&self, id: NodeId) 
    -> Vec<NodeId>
    {
        self.nodes_map.neighbors(id)
    }
}

impl ReinitMapsForNode for GraphHash {

    fn reinit_maps_for_node(&mut self, k: NodeId)
    {
        self.parents.set_parents_for_node(k, vec![]);

        self.path_counts.set_path_count_for_node(k, 0);
        self.new_path_counts.set_path_count_for_node(k, 0);

        self.distances.set_distance_for_node(k, -1.0);

        self.pair_dependencies.set_pair_dependency_for_node(k, 0.0);
        self.new_pair_dependencies.set_pair_dependency_for_node(k, 0.0);

        self.set_sigma_value_for_node(k, 0.0);
        self.new_sigmas_set_sigma_value_for_node(k, 0.0);

        self.inc_path_counts.set_path_count_for_node(k, 0);

        self.visit_markers.unvisit(k);
    }
}

impl ReinitMaps for GraphHash {
    
    fn reinit_maps(&mut self)
    {
        for k in self.nodes_map.mapped_nodes() {
            self.reinit_maps_for_node(k);
        }
    }
}

impl InsertNode for GraphHash {
    
    fn insert_node(&mut self, id: NodeId) 
    {
        self.nodes_map.add_isolated_node(id);
    }
}

impl InsertEdge for GraphHash {
    
    fn insert_edge(&mut self, edge: &Edge)
    -> Result<(),BetweennessCentralityError> 
    {
        let redge = edge.reversed();

        if self.edges.has_edge(&edge)
        || self.edges.has_edge(&redge)
        {
            return Ok(());

        } else {

            self.edges.insert_edge(*edge);
        }

        self.insert_node(edge.src);
        self.insert_node(edge.dst);

        self.nodes_map.add_edge(&edge);

        Ok(())
    }
}

impl RemoveEdge for GraphHash {

    fn remove_edge(&mut self, edge: &Edge)
    -> Result<(),BetweennessCentralityError> 
    {
        self.edges.remove_edge(&edge);

        let redge = edge.reversed();

        self.edges.remove_edge(&redge);

        self.nodes_map.unlink_edge(edge);

        Ok(())
    }
}

impl GetConnectedComponentSizes for GraphHash {

    /**
      | the vector @out_vec will have sizes
      | of the connected components in the graph
      |
      */
    fn conn_comp_sizes(&self) 
    -> Result<Vec<i32>,BetweennessCentralityError>
    {
        let mut out_vec = vec![];

        let visited_map_name = name![
            self.name(), 
            "conn_comp_sizes::visited_map"
        ];

        let mut visited_map = VisitMarkers::new_from_nodes(
            self.nodes_map.mapped_nodes(),
            &visited_map_name
        );

        for id in visited_map.iter_unvisited() {

            self.conn_comp_sizes_step(
                &mut visited_map, 
                id, 
                &mut out_vec
            );
        }

        Ok(out_vec)
    }
}
    
impl ExtendWith<GraphHash> for GraphHash {

    type Error = BetweennessCentralityError;

    fn extend_with(&mut self, other: &GraphHash) 
        -> Result<(),Self::Error>
    {
        self.nodes_map.extend_with(
            &other.nodes_map
        );

        self.edges.extend(
            &other.edges
        );

        Ok(())
    }
}
    
impl InitDebugIteration for GraphHash {

    fn init_dbg_iteration(&mut self, source: NodeId) {

        self.reinit_maps();
        self.path_counts.set_path_count_for_node(source,1);
        self.distances.set_distance_for_node(source,0.0);
        self.enqueue(source);
    }
}

impl DebugIterationStep for GraphHash {

    fn dbg_iteration_step(&mut self, v_s: &mut Vec<NodeId>) 
    -> Result<(),BetweennessCentralityError> 
    {
        while let Some(v_i) = self.dequeue() {

            v_s.push(v_i);

            self.update_dist_and_path_for_neighbors(v_i);
        }

        Ok(())
    }
}

impl RemoveBridges for GraphHash {

     fn remove_bridges(&mut self, bridge_vec: Vec<Edge>) {

        debug!("removing bridges...");

        for bridge in bridge_vec.iter() {

            self.remove_edge(&bridge);
        }
    }
}

impl FindConnectedComponents<GraphHash> for GraphHash {

    type Error = BetweennessCentralityError;

    fn find_conn_comp(&mut self) 
    -> Result<Vec<GraphHash>,Self::Error> 
    {
        debug!("finding connected components...");

        let mut out_vec = vec![];

        let visited_map_name = name![self.name(), "find_conn_comp::visited_map"];

        let mut visited_map = VisitMarkers::new_from_nodes(
            self.nodes_map.mapped_nodes(),
            visited_map_name
        );

        for id in self.nodes_map.mapped_nodes() {

            if visited_map.unvisited(id) {

                let gh_name = name![self.name(), format!("graphhash_for_id{}", id)];

                let gh = GraphHash::new_via_bfs_from_id(
                    gh_name,
                    self,
                    id,
                    &mut visited_map
                );

                out_vec.push(gh);
            }
        }

        Ok(out_vec)
    }
}

impl CreateNamedEmpty for GraphHash {

    fn empty(name: &str) -> Self {

        let nodes_map_name             = name![name, "nodes_map"];
        let edges_name                 = name![name, "edges"];
        let parents_name               = name![name, "parents"];
        let path_counts_name           = name![name, "path_counts"];
        let new_path_counts_name       = name![name, "new_path_counts"];
        let inc_path_counts_name       = name![name, "inc_path_counts"];
        let distances_name             = name![name, "distances"];
        let pair_dependencies_name     = name![name, "pair_dependencies"];
        let new_pair_dependencies_name = name![name, "new_pair_dependencies"];
        let sigmas_name                = name![name, "sigmas"];
        let new_sigmas_name            = name![name, "new_sigmas"];
        let visit_markers_name         = name![name, "visit_markers"];
        let stack_name                 = name![name, "stack"];
        let queue_name                 = name![name, "queue"];

        Self {
            name:                   name.to_owned(),
            nodes_map:              NeighborsMap::empty_mapped(nodes_map_name),
            edges:                  Edges::empty(edges_name),
            parents:                ParentsMap::empty_mapped(parents_name),
            path_counts:            PathCounts::empty_mapped(path_counts_name),
            new_path_counts:        PathCounts::empty_mapped(new_path_counts_name),
            inc_path_counts:        PathCounts::empty_mapped(inc_path_counts_name),
            distances:              DistanceMap::empty_mapped(distances_name),
            pair_dependencies:      PairDependencies::empty_mapped(pair_dependencies_name),
            new_pair_dependencies:  PairDependencies::empty_mapped(new_pair_dependencies_name),
            sigmas:                 SigmaMap::empty_mapped(sigmas_name),
            new_sigmas:             SigmaMap::empty_mapped(new_sigmas_name),
            visit_markers:          VisitMarkers::empty_mapped(visit_markers_name),
            stack:                  NodeIdStack::empty(stack_name),
            queue:                  NodeIdQueue::empty(queue_name),
            print_nodes:            AtomicBool::new(false),
        }
    }
}

impl GraphHash {

    /// do a BFS from id, count the number of
    /// vertices, and mark the visited
    ///
    pub fn fill_edges_from_bfs(
        &mut self,
        parent:      &GraphHash, 
        id:          NodeId, 
        visited_map: &mut VisitMarkers)
    {
        let mut q = NodeIdQueue::empty("fill_edges_from_bfs::queue");

        q.enqueue(id);

        while let Some(node) = q.dequeue() {

            let nbrs_vec = parent.nodes_map.neighbors(node);

            for &nbr in nbrs_vec.iter() {

                self.insert_edge(&Edge::new(node, nbr));

                if visited_map.unvisited(nbr) {

                    visited_map.visit(nbr);

                    q.enqueue(nbr);
                }
            }
        }
    }

    pub fn new_via_bfs_from_id(
        name:        &str,
        parent:      &GraphHash, 
        id:          NodeId, 
        visited_map: &mut VisitMarkers) -> Self {

        let mut gh: GraphHash = GraphHash::empty(name);

        visited_map.visit(id);

        gh.insert_node(id);

        gh.fill_edges_from_bfs(
            parent,
            id,
            visited_map
        );

        gh
    }

    pub fn maybe_set_distance_for_neighbor(
        &mut self,
        w: NodeId,
        v: NodeId)
    {
        if self.distances.is_infinite(w) {

            self.enqueue(w);

            self.distances.set_distance_for_node(
                w, 
                self.distance(v) + 1.0
            );
        }
    }

    pub fn maybe_increment_path_count_and_add_parent_for_neighbor(
        &mut self,
        w: NodeId,
        v: NodeId)
    {
        if self.distances.is_one_step_away(w, v) {

            self.path_counts.increment_path_count_for_node_from(w,v);

            self.parents.add_parent(w, v);
        }
    }

    pub fn update_dist_and_path_for_neighbors(
        &mut self,
        v: NodeId)
    {
        let nbr_vec = self.neighbors(v);

        for &w in nbr_vec.iter() {

            self.maybe_set_distance_for_neighbor(w, v);

            self.maybe_increment_path_count_and_add_parent_for_neighbor(w, v);
        }
    }

    pub fn update_pair_dependencies(&mut self, w: NodeId, v: NodeId) {

        let v_paths = self.path_counts.path_count_for_node(v) as f64;
        let w_paths = self.path_counts.path_count_for_node(w) as f64;

        let paths_ratio = v_paths / w_paths;

        let w_pair_dependencies = self.pair_dependencies.pair_dependency_for_node(w);

        self.pair_dependencies.increment_pair_dependency_for_node(
            v,
            paths_ratio * (1.0 + w_pair_dependencies)
        );
    }

    pub fn update_parent_pair_dependencies_and_increment_score_for_node(
        &mut self, 
        w:      NodeId,
        s:      NodeId, 
        scores: &mut BetweennessScores) 
    {
        let parents = self.parents.parents_for_node(w);

        for v in parents {

            self.update_pair_dependencies(w,v);
        }

        if w != s {

            scores.increase_score_for_node(
                w, 
                self.pair_dependencies.pair_dependency_for_node(w)
            );
        }
    }

    pub fn conn_comp_sizes_step(&self, 
        visited_map: &mut VisitMarkers, 
        id:          NodeId, 
        out_vec:     &mut Vec<i32>) 
    {
        visited_map.visit(id);

        out_vec.push(1);

        let queue_name = name![self.name(), "conn_comp_sizes_step::queue"];

        // do a BFS from s, count the
        // number of vertices, and mark
        // the visited
        //
        let mut q = NodeIdQueue::empty(queue_name);

        q.enqueue(id);

        while let Some(node) = q.dequeue() {

            let nbrs_vec = self.nodes_map.neighbors(node);

            for &nbr in nbrs_vec.iter() {

                if visited_map.unvisited(nbr) {

                    visited_map.visit(nbr);

                    q.enqueue(nbr);

                    *out_vec.last_mut().unwrap() += 1;
                }
            }
        }
    }
    
    pub fn find_single_source_shortest_paths_step(
        &self, 
        v:                            NodeId, 
        id:                           NodeId, 
        single_source_shortest_paths: &mut DistanceMap,
        queue:                        &mut NodeIdQueue) 
    {
        let nbr_vec = self.neighbors(v);

        for &nbr in nbr_vec.iter() {

            if single_source_shortest_paths.is_infinite(nbr) {

                single_source_shortest_paths.set_distance_for_node(
                    nbr, 
                    single_source_shortest_paths.distance(v) + 1.0
                );

                queue.enqueue(nbr);
            }
        }
    }
}

impl FindSingleSourceShortestPaths for GraphHash {

    fn find_single_source_shortest_paths(&self, id: NodeId) 
    -> Result<DistanceMap,BetweennessCentralityError>
    {
        let queue_name        = format!("sssp_queue_for_{}", id);
        let distance_map_name = format!("sssp_distance_map_for_{}", id);

        let mut single_source_shortest_paths = DistanceMap::new_from_nodes(
            self.nodes_map.mapped_nodes(), 
            &distance_map_name
        );

        let mut queue  = NodeIdQueue::new(id, &queue_name);

        single_source_shortest_paths.set_distance_for_node(id,0.0);

        while let Some(v) = queue.dequeue() {

            self.find_single_source_shortest_paths_step(
                v,
                id,
                &mut single_source_shortest_paths,
                &mut queue
            );
        }

        Ok(single_source_shortest_paths)
    }
}

impl FindPruningCounts for GraphHash {

    fn find_pruning_counts_exp(&mut self, 
        src:    NodeId,
        dst:    NodeId) 
    -> Result<(i32,i32,i32),BetweennessCentralityError> 
    {
        let edge = Edge::new(src,dst);

        // IMP: the edge (src, dst) must not be in
        // the graph else doesn't make sense to
        // count pruned BFS's
        //
        let insert: bool = self.has_edge(&edge);

        self.remove_edge(&edge);

        let src_distances = self.find_single_source_shortest_paths(edge.src)?;
        let dst_distances = self.find_single_source_shortest_paths(edge.dst)?;

        let mut d0 = 0;
        let mut d1 = 0;
        let mut d2 = 0;

        for node in self.nodes_map.mapped_nodes() {

            let src_dist = src_distances.distance(node);
            let dst_dist = dst_distances.distance(node);

            let diff:     f64 = src_dist - dst_dist;
            let abs_diff: f64 = diff.abs();

            match abs_diff {
                0.0  => d0 += 1,
                1.0  => d1 += 1,
                _  => d2 += 1,
            }
        }

        if insert {
            self.insert_edge(&edge);
        }

        Ok((d0,d1,d2))
    }
}
