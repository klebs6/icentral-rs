crate::ix!();

pub trait NodeIdToMucId {

    fn nodeid_to_mucid(&self, idx: NodeId) -> MinimumUnionCycleId;
}

#[derive(Debug)]
pub struct ConnectionVertexDescriptor {
    id:                 NodeId,
    subgraph_micentraltude: usize,
}

/**
  | not sure yet TODO
  |
  */
pub struct MinimumUnionCycle<GH> {
    
    conn_vertex_map:     Arc<Mutex<ConnVertexMap>>,
    subgraph_map:        SubGraphMap<GH>,
    muc_subgraph:        GH,
    id:                  MinimumUnionCycleId,

    /**
      | flag to tell if the muc was deleted or
      | not
      |
      */
    valid:               bool,


    /**
      | these are used to facilitate fast computation
      | where iterations are done in a fast graph,
      | not hash based one
      |
      */
    muc_fast_subgraph:   SubGraph,

    tmp_conn_vertex_map: ConnVertexMap,
    tmp_subgraph_map:    SubGraphMap<GH>,
    print_nodes:         AtomicBool,
}

impl<GH: SetPrintNodes + Debug> fmt::Debug for MinimumUnionCycle<GH> 
where GH: ExtendWith<GH> 
        + GetConnectedComponentSizes
        + GetEdges 
        + GetNeighborsForNode
        + GetNodeIdRange
        + HasMapForNode 
        + InsertEdge 
        + InsertNode 
        + MappedNodes 
        + NumEdges 
        + NumNodes
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let binding = f.debug_struct("MinimumUnionCycle");

        let mut builder = binding;

        if !self.valid {

            builder.field("invalid_muc", &true);

        } else {

            builder.field("connection_vertices", &self.connection_vertices());
            builder.field("bridges",             &self.bridges());

            self.muc_subgraph.set_print_nodes(self.get_print_nodes());

            builder.field("subgraph",            &self.muc_subgraph);
        }

        builder.finish()
    }
}
    
impl<GH> Default for MinimumUnionCycle<GH> {

    fn default() -> Self {
        Self {
            valid: true,
            ..Default::default()
        }
    }
}

impl<GH> Named<'_> for MinimumUnionCycle<GH> {

    type Name = String;

    fn name(&self) -> Self::Name {
        format!("muc_with_id{}", self.id)
    }
}

impl<GH> GetPrintNodes for MinimumUnionCycle<GH> 
where GH: GraphHashMucInterface
{
    fn get_print_nodes(&self) -> bool 
    {
        self.print_nodes.load(atomic::Ordering::SeqCst)
    }
}

impl<GH> SetPrintNodes for MinimumUnionCycle<GH> 
where GH: GraphHashMucInterface
{
    fn set_print_nodes(&self, val: bool) 
    {
        debug!("in muc with id={}, toggling print functionality for nodes, {}", self.id, val);

        self.print_nodes.store(val, atomic::Ordering::SeqCst);
    }
}

impl<GH> NumNodes for MinimumUnionCycle<GH> 
where GH: GraphHashMucInterface
{
    delegate!{
        to self.muc_subgraph {
            fn num_nodes(&self) -> usize;
        }
    }
}

impl<GH> NumEdges for MinimumUnionCycle<GH> 
where GH: GraphHashMucInterface
{
    delegate!{
        to self.muc_subgraph {
            fn num_edges(&self) -> usize;
        }
    }
}

impl<GH> InsertNode for MinimumUnionCycle<GH> 
where GH: GraphHashMucInterface
{
    delegate!{
        to self.muc_subgraph {
            fn insert_node(&mut self, n: NodeId);
        }
    }
}

impl<GH> InsertEdge for MinimumUnionCycle<GH> 
where GH: GraphHashMucInterface
{
    delegate!{
        to self.muc_subgraph {
            fn insert_edge(&mut self, edge: &Edge)
                -> Result<(),BetweennessCentralityError>;
        }
    }
}

impl<GH> HasMapForNode for MinimumUnionCycle<GH> 
where GH: GraphHashMucInterface
{
    delegate!{
        to self.muc_subgraph {
            fn has_map_for_node(&self, n: NodeId) -> bool;
        }
    }
}

impl<GH> GetEdges for MinimumUnionCycle<GH> 
where GH: GraphHashMucInterface
{
    delegate!{
        to self.muc_subgraph {
            fn edges(&self) -> &Edges;
        }
    }
}

impl<GH> MappedNodes for MinimumUnionCycle<GH> 
where GH: GraphHashMucInterface
{
    delegate!{
        to self.muc_subgraph {
            fn mapped_nodes(&self) -> Vec<NodeId>;
        }
    }
}

impl<GH> ExtendWith<MinimumUnionCycle<GH>> for MinimumUnionCycle<GH> 
where GH: GetConnectedComponentSizes 
        + ExtendWith<GH, Error=BetweennessCentralityError> 
        + GetEdges 
        + GetNeighborsForNode 
        + GetNodeIdRange 
        + HasMapForNode 
        + InsertEdge 
        + InsertNode
        + MappedNodes 
        + NumEdges 
        + NumNodes
{
    type Error = BetweennessCentralityError;

    /// muc1 = muc1 U muc2
    ///
    /// 1. copy the graph of muc2 into muc1
    ///
    /// 2. for each bridge edge, if the other end
    ///    becomes in the graph, add the edge
    ///
    fn extend_with(&mut self, other: &MinimumUnionCycle<GH>)
        -> Result<(),Self::Error>
    {
        debug!("extending muc with id={}, via muc with id={}", self.id, other.id);

        debug!("copying graph of muc2 into muc1");

        self.muc_subgraph.extend_with(
            &other.muc_subgraph
        )?;

        debug!("for each bridge edge, if the other end becomes in the graph, add the edge");

        let mut to_insert = vec![];

        if let conn_vertex_map = other.conn_vertex_map.lock()? {

            for (conn_vert,v) in conn_vertex_map.iter() {

                for i in 0..v.len() {

                    if self.muc_subgraph.has_map_for_node(v[i]) {

                        to_insert.push((conn_vert,v[i]));
                    }
                }
            }
        }

        for (src,dst) in to_insert.iter() {
            self.muc_subgraph.insert_edge(&Edge::new(*src, *dst));
        }

        Ok(())
    }
}

impl<GH> IsValid for MinimumUnionCycle<GH> 
{
    fn is_valid(&self) -> bool 
    {
        self.valid
    }
}
    
impl<GH> MinimumUnionCycle<GH> 
where GH: GetConnectedComponentSizes 
        + ExtendWith<GH> 
        + GetEdges 
        + GetNeighborsForNode 
        + GetNodeIdRange 
        + HasMapForNode 
        + InsertEdge 
        + InsertNode
        + MappedNodes 
        + NumEdges 
        + NumNodes 
{
    pub fn vertex_map(&self) -> LockResult<MutexGuard<'_,ConnVertexMap>> {
        self.conn_vertex_map.lock()
    }

    pub fn set_muc_subgraph(&mut self, gh: GH) 
    {
        debug!("in muc with id={}, setting muc_subgraph", self.id);

        self.muc_subgraph = gh;
    }

    pub fn insert_subgraph(&mut self, idx: NodeId, g: Arc<GH>) 
    {
        debug!("in muc with id={}, inserting subgraph at index={}", self.id, idx);

        self.subgraph_map.set_subgraph_map_for_node(idx,g);
    }

    pub fn clear_conn_vertex_map(&mut self) 
    {
        debug!("in muc with id={}, clearing conn_vertex_map", self.id);

        self.conn_vertex_map.lock().unwrap().clear();
    }

    pub fn id(&self) -> MinimumUnionCycleId 
    {
        self.id
    }

    pub fn set_id(&mut self, new: usize) 
    {
        debug!("setting muc id -- old={}, new={}", self.id, new);

        self.id = mucid![new];
    }

    pub fn clear_subgraph_map(&mut self) 
    {
        debug!("in muc with id={}, clearing subgraph_map", self.id);

        self.subgraph_map.clear();
    }

    pub fn invalidate(&mut self) 
    {
        debug!("invalidating muc, id={}", self.id);

        self.valid = false;
    }

    pub fn connection_vertices(&self)
    -> Result<Vec<ConnectionVertexDescriptor>,BetweennessCentralityError> 
    {
        debug!("in muc with id={}, scanning conn_vertex_map for connection_vertices", self.id); 

        let mut result = vec![];

        if let conn_vertex_map = self.conn_vertex_map.lock()? {

            for (id,ids) in conn_vertex_map.iter() {

                result.push(ConnectionVertexDescriptor {
                    id,
                    subgraph_micentraltude: self.subgraph_map.subgraph_for_node(id).num_nodes(),
                });
            }
        }

        Ok(result)
    }

    pub fn bridges(&self) 
    -> Result<Vec<Edge>,BetweennessCentralityError> 
    {
        debug!("in muc with id={}, scanning conn_vertex_map for bridges", self.id); 

        let mut res = vec![];

        if let conn_vertex_map = self.conn_vertex_map.lock()? {

            for (id1,ids) in conn_vertex_map.iter() {

                for id2 in ids.iter() {

                    res.push(Edge::new(id1,*id2));
                }
            }
        }

        Ok(res)
    }

    /**
      | make sure the node with @id exists in
      | the muc! this function will blindly
      | insert it
      |
      */
    pub fn insert_conn_vertex(&mut self, 
        id:  NodeId,
        nbr: NodeId) 
    -> Result<(),BetweennessCentralityError> 
    {
        warn!("make sure the node with id={} exists in the muc with id={}! this function will blindly insert it!", id, self.id);

        if let mut conn_vertex_map = self.conn_vertex_map.lock()? {

            if let Some(v) = conn_vertex_map.vertices_for_node_mut(id) {

                v.push(nbr);

            } else {

                let mut vec: Vec<NodeId> = vec![];

                vec.push(nbr);

                conn_vertex_map.set_vertex_map_for_node(id,vec);
            }
        }

        Ok(())
    }

    pub fn compute_bc_initialize(&mut self) 
    {
        debug!("muc with id={}, initializing betweenness centrality computation", self.id);

        self.muc_fast_subgraph.reset_with(&self.muc_subgraph);

        self.tmp_conn_vertex_map.clear();

        self.tmp_subgraph_map.clear();
    }
    
    pub fn compute_bc_initialize_tmp_conn_vertex_map(&mut self) 
    -> Result<(),BetweennessCentralityError> 
    {
        if let conn_vertex_map = self.conn_vertex_map.lock()? {

            for (k,v) in conn_vertex_map.iter() {

                let new_v: NodeId = self.muc_fast_subgraph.label_map_outin(k);

                self.tmp_conn_vertex_map.set_vertex_map_for_node(new_v,v.to_vec());
            }
        }

        Ok(())
    }

    pub fn compute_bc_initialize_tmp_subgraph_map(&mut self) 
    -> Result<(),BetweennessCentralityError> 
    {
        for (k,v) in self.subgraph_map.iter() {

            let new_v: NodeId = self.muc_fast_subgraph.label_map_outin(k);

            self.tmp_subgraph_map.set_subgraph_map_for_node(new_v,v.clone());
        }

        Ok(())
    }

    pub fn initialize_scores_for_muc_subgraph(&self, scores: &mut BetweennessScores) 
    {
        for node in self.muc_subgraph.mapped_nodes() {
            scores.set_score_for_node(node, 0.0);
        }
    }

    pub fn adjust_max_iter(&self, max_iter: Option<usize>) -> usize {

        let max_iter = match max_iter {
            Some(max_iter) => {
                min(
                    max_iter,
                    self.muc_fast_subgraph.num_nodes()
                )
            }
            None => self.muc_fast_subgraph.num_nodes(),
        };

        max_iter
    }

    pub fn initialize_bfs_for_node_in_the_muc(&mut self, source: NodeId)
    {
        self.muc_fast_subgraph.reinit_maps();

        self.muc_fast_subgraph.increment_path_count_for_node(source,1);

        self.muc_fast_subgraph.set_distance_for_node(source, 0.0);

        self.muc_fast_subgraph.enqueue(source);
    }

    pub fn build_stack_for_bfs(&mut self) -> NodeIdStack {

        let stack_name = name![self.name(), "build_stack_for_bfs::stack"];

        let mut stack = NodeIdStack::empty(stack_name);

        while let Some(v_i) = self.muc_fast_subgraph.dequeue() {

            stack.push(v_i);

            let nbrs = self.muc_fast_subgraph.neighbors(v_i);

            for &v_n in nbrs.iter() {

                if self.muc_fast_subgraph.distance(v_n) < 0.0 {

                    self.muc_fast_subgraph.enqueue(v_n);

                    self.muc_fast_subgraph.set_distance_one_step_away(
                        v_n, 
                        v_i,
                    );
                }

                if self.muc_fast_subgraph.distance_is_one_step_away(v_n, v_i) {

                    self.muc_fast_subgraph.increment_path_count_for_node(
                        v_n,
                        self.muc_fast_subgraph.path_count_for_node(v_i)
                    );

                    self.muc_fast_subgraph.parents_for_node(v_n).push(v_i);
                }
            }
        }

        stack
    }

    pub fn bfs_process_stack_item(
        &mut self, 
        v_n:    NodeId, 
        source: NodeId, 
        scores: &mut BetweennessScores)
    {
        if self.tmp_conn_vertex_map.has_mapping_for_node(source)
        && self.tmp_conn_vertex_map.has_mapping_for_node(v_n)
        && source != v_n 
        {
            let mut vg_s: usize = 0;
            let mut vg_n: usize = 0;

            vg_s = self.tmp_subgraph_map.subgraph_for_node(source).num_nodes();
            vg_n = self.tmp_subgraph_map.subgraph_for_node(v_n).num_nodes();

            let c_t: usize = vg_s * vg_n;

            self.muc_fast_subgraph.increment_sigma_value_for_node(v_n, c_t as f64);

            let new_v_n: NodeId = self.muc_fast_subgraph.label_map_inout(v_n);

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) + c_t as f64
            );
        }

        for i in 0..self.muc_fast_subgraph.parents_for_node(v_n).len() {

            let v_p: NodeId = self.muc_fast_subgraph.parents_for_node(v_n)[i];

            let sp_sn = self.muc_fast_subgraph.path_count_ratio(v_p, v_n);

            self.muc_fast_subgraph.increment_pair_dependency_for_node(
                v_p, 
                sp_sn * (1.0 + self.muc_fast_subgraph.pair_dependency_for_node(v_n))
            );

            if self.tmp_conn_vertex_map.has_mapping_for_node(source) {

                self.muc_fast_subgraph.increment_sigma_value_for_node(
                    v_p, 
                    self.muc_fast_subgraph.sigma_value_for_node(v_n) * sp_sn
                );

                let new_v_p: NodeId = self.muc_fast_subgraph.label_map_inout(v_p);

                scores.set_score_for_node(
                    new_v_p,
                    scores.score_for_node(new_v_p) + self.muc_fast_subgraph.sigma_value_for_node(v_n) * sp_sn
                );
            }
        }

        if source != v_n {

            let new_v_n: NodeId = self.muc_fast_subgraph.label_map_inout(v_n);

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) + self.muc_fast_subgraph.pair_dependency_for_node(v_n)
            );
        }

        if self.tmp_conn_vertex_map.has_mapping_for_node(source) {

            let vg_s: f64 = self.tmp_subgraph_map.subgraph_for_node(source).num_nodes() as f64;

            let new_v_n: NodeId = self.muc_fast_subgraph.label_map_inout(v_n);

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) + self.muc_fast_subgraph.pair_dependency_for_node(v_n) * vg_s * 2.0
            );
        }
    }

    pub fn do_bfs_for_node_in_the_muc(
        &mut self, 
        source: NodeId, 
        scores: &mut BetweennessScores)
    {
        self.initialize_bfs_for_node_in_the_muc(source);

        let mut stack = self.build_stack_for_bfs();

        while let Some(v_n) = stack.pop() {
            self.bfs_process_stack_item(v_n, source, scores);
        }
    }

    pub fn do_bf_searches_from_the_nodes_in_the_muc(
        &mut self, 
        max_iter: usize, 
        scores:   &mut BetweennessScores)
    {
        for source in NodeIdRange::new(0,max_iter) {
            self.do_bfs_for_node_in_the_muc(source,scores);
        }
    }

    pub fn update_scores_after_bfs(&self, scores: &mut BetweennessScores) 
        -> Result<(),BetweennessCentralityError> 
    {
        for (k,v) in self.tmp_subgraph_map.iter() {

            let size_vec = v.conn_comp_sizes()?;

            if size_vec.len() > 1 {

                let mut sub: i32 = 0;

                for i in 0..size_vec.len() {
                    sub += size_vec[i] * size_vec[i];
                }

                let vg_i: f64 = v.num_nodes() as f64;

                let new_node_id: NodeId = self.muc_fast_subgraph.label_map_inout(k);

                scores.set_score_for_node(
                    new_node_id, 
                    scores.score_for_node(new_node_id) + vg_i * vg_i - (sub as f64)
                );
            }
        }

        scores.halve();

        Ok(())
    }

    pub fn compute_bc(&mut self, 
        scores:   &mut BetweennessScores,
        max_iter: Option<usize>) 
    -> Result<(),BetweennessCentralityError> 
    {
        debug!("computing betweenness centrality for muc with id={}", self.id);

        // scores will have for each vertex in the
        // muc it's betweenness centrality init bc
        // of all nodes to zero
        //
        scores.clear();

        if !self.valid {

            warn!("tried to compute betweenness centrality on an invalid muc! may want to investigate this!");

            return Ok(());
        }

        self.compute_bc_initialize();

        self.compute_bc_initialize_tmp_conn_vertex_map()?;

        self.compute_bc_initialize_tmp_subgraph_map()?;

        self.initialize_scores_for_muc_subgraph(scores);

        // do BFS's from the nodes in the muc

        let max_iter = self.adjust_max_iter(max_iter);

        self.do_bf_searches_from_the_nodes_in_the_muc(max_iter, scores);

        self.update_scores_after_bfs(scores);

        Ok(())
    }

    /* ------- Incremental algorithm on top of MinimumUnionCycle  ------- */
    
    //////////////////////////////////////////////////
    //IMP:: FOR NOW THIS GUY DOESN'T USE fast_subgraph
    //////////////////////////////////////////////////
    //void muc_t::compute_bc_inc(tr1_map_t(double)& scores,
    //                        node_id_t src,
    //                        node_id_t dst)
    //{
    //    if(!valid)
    //        return;
    //
    //    //scores will have for each vertex in the muc it's betweenness centrality
    //    muc_subgraph.remove_edge(src, dst);
    //    tr1_map_t(int) src_distances, dst_distances;
    //    muc_subgraph.find_single_source_shortest_paths(src, src_distances);
    //    muc_subgraph.find_single_source_shortest_paths(dst, dst_distances);
    //    
    //    //this must be commented in general.. if it's there it makes scores contain deltas
    //    //bcc_subgraph.i_fill_map<double>(scores, 0);
    //    
    //    for(graph_hash_t::nodes_map_t::iterator
    //                        it =  muc_subgraph.nodes_map.begin();
    //                        it != muc_subgraph.nodes_map.end();
    //                        ++it) {
    //        node_id_t source = it->first;
    //        if(src_distances[source] != dst_distances[source]) {
    //            i_iteration(source, 
    //                    src, 
    //                    dst, 
    //                    src_distances[source], 
    //                    dst_distances[source], 
    //                    scores);
    //        }
    //    }
    //    muc_subgraph.insert_edge(src, dst);
    //}
    pub fn compute_bc_inc(&mut self, 
        scores:   &mut BetweennessScores,
        mut src:  NodeId,
        mut dst:  NodeId,
        max_iter: Option<usize>) 
    -> Result<(),BetweennessCentralityError> 
    {
        if !self.valid {
            return Ok(());
        }

        self.muc_fast_subgraph.reset_with(&self.muc_subgraph);

        self.tmp_conn_vertex_map.clear();

        self.tmp_subgraph_map.clear();

        if let conn_vertex_map = self.conn_vertex_map.lock()? {

            for (k,v) in conn_vertex_map.iter() {

                let new_v: NodeId = self.muc_fast_subgraph.label_map_outin(k);

                self.tmp_conn_vertex_map.set_vertex_map_for_node(new_v,v.clone());
            }
        }

        for (k,v) in self.subgraph_map.iter() {

            let new_v: NodeId = self.muc_fast_subgraph.label_map_outin(k);

            self.tmp_subgraph_map.set_subgraph_map_for_node(new_v,v.clone());
        }

        src = self.muc_fast_subgraph.label_map_outin(src);
        dst = self.muc_fast_subgraph.label_map_outin(dst);

        // scores will have for each vertex in the
        // muc it's betweenness centrality
        //
        self.muc_fast_subgraph.remove_edge_between_nodes(src, dst);

        let src_distances = self.muc_fast_subgraph.find_single_source_shortest_paths(src)?;
        let dst_distances = self.muc_fast_subgraph.find_single_source_shortest_paths(dst)?;

        // this must be commented in general.. if
        // it's there it makes scores contain
        // deltas
        //
        // bcc_subgraph.i_fill_map<double>(scores, 0);

        let max_iter = match max_iter {
            Some(max_iter) => min(max_iter,self.muc_fast_subgraph.num_nodes()),
            None           => self.muc_fast_subgraph.num_nodes(),
        };

        for source in NodeIdRange::new(0,max_iter) {

            if src_distances.distance(source) != dst_distances.distance(source) {

                self.iteration(
                    source, 
                    src, 
                    dst, 
                    src_distances.distance(source), 
                    dst_distances.distance(source), 
                    scores
                );
            }
        }

        self.muc_fast_subgraph.insert_edge_between_nodes(src, dst);

        Ok(())
    }
    
    pub fn iteration(&mut self, 
        source:      NodeId,
        mut src:    NodeId,
        mut dst:    NodeId,
        mut src_distance:  f64,
        mut dst_distance:  f64,
        scores: &mut BetweennessScores)  
    {
        make_sure_src_is_the_closer_to_source_node(
            &mut src, 
            &mut dst, 
            &mut src_distance, 
            &mut dst_distance
        );

        if dst_distance - src_distance == 1.0 {

            // dbg_iteration(source, src, dst, src_distance, dst_distance, scores);
            self.iteration_1(source, src, dst, src_distance, dst_distance, scores);;

        } else {

            // dbg_iteration(source, src, dst, src_distance, dst_distance, scores);
            self.iteration_2(source, src, dst, src_distance, dst_distance, scores);;
        }
    }

    pub fn iteration_1_process(&mut self, 
        v_n:    NodeId,
        source: NodeId,
        src:    NodeId,
        dst:    NodeId,
        src_distance:  f64,
        dst_distance:  f64,
        scores: &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError> 
    {
        self.muc_fast_subgraph.maybe_update_all_sigmas_and_do_new(
            v_n,
            source,
            &self.tmp_subgraph_map,
            &self.tmp_conn_vertex_map,
        )?;

        self.muc_fast_subgraph.muc_update(
            v_n,
            source,
            &self.tmp_conn_vertex_map, 
            scores
        );

        // IMP: this is the only change that
        // happens to
        // self.muc_fast_subgraph.parents,
        // @src should be added as parent for
        // dst
        //
        if v_n == dst {

            let v_p: NodeId = src;

            let new_sp_sn = self.muc_fast_subgraph.new_path_counts_path_count_ratio(
                v_p, 
                v_n
            );

            self.muc_fast_subgraph.new_pair_dependencies_increment_pair_dependency_for_node(
                v_p,
                new_sp_sn * (1.0 + self.muc_fast_subgraph.new_pair_dependencies_pair_dependency_for_node(v_n))
            );

            if self.tmp_conn_vertex_map.has_mapping_for_node(source) {

                self.muc_fast_subgraph.new_sigmas_increment_sigma_value_for_node(
                    v_p,
                    self.muc_fast_subgraph.new_sigmas_sigma_value_for_node(v_n) * new_sp_sn
                );

                let new_v_p: NodeId = self.muc_fast_subgraph.label_map_inout(v_p);

                scores.set_score_for_node(
                    new_v_p, 
                    scores.score_for_node(new_v_p) + self.muc_fast_subgraph.new_sigmas_sigma_value_for_node(v_n) * new_sp_sn / 2.0
                );
            }
        }

        if source != v_n {

            let new_v_n: NodeId = self.muc_fast_subgraph.label_map_inout(v_n);

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) - self.muc_fast_subgraph.pair_dependency_for_node(v_n) / 2.0
            );

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) + self.muc_fast_subgraph.new_pair_dependencies_pair_dependency_for_node(v_n) / 2.0
            );
        }

        if self.tmp_conn_vertex_map.has_mapping_for_node(source) {

            let vg_s: i32 = self.tmp_subgraph_map.subgraph_for_node(source).num_nodes().try_into()?;

            let new_v_n: NodeId = self.muc_fast_subgraph.label_map_inout(v_n);

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) - self.muc_fast_subgraph.pair_dependency_for_node(v_n) * vg_s as f64
            );

            scores.set_score_for_node(
                new_v_n, 
                scores.score_for_node(new_v_n) + self.muc_fast_subgraph.new_pair_dependencies_pair_dependency_for_node(v_n) * vg_s as f64
            );
        }

        Ok(())
    }
    
    // |src_distance-dst_distance| = 1 (the easy case) 
    //
    pub fn iteration_1(&mut self, 
        source:      NodeId,
        src:    NodeId,
        dst:    NodeId,
        src_distance:  f64,
        dst_distance:  f64,
        scores: &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError> 
    {
        let mut stack: Stack<NodeId> = default!();

        self.muc_fast_subgraph.init_iteration1(source);

        self.muc_fast_subgraph.iteration1_fill_stack(
            &mut stack, 
            src, 
            dst
        )?;

        while let Some(v_n) = stack.pop() {

            self.iteration_1_process(
                v_n,
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
    
    // |src_distance-dst_distance| >= 2 (the difficult case)
    //
    pub fn iteration_2(&mut self, 
        source:   NodeId,
        src:      NodeId,
        dst:      NodeId,
        src_distance:    f64,
        dst_distance:    f64,
        scores:   &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError> 
    {
        self.muc_fast_subgraph.iteration_2(
            &self.tmp_subgraph_map, 
            &self.tmp_conn_vertex_map, 
            source, 
            src, 
            dst, 
            src_distance, 
            dst_distance, 
            scores
        );

        Ok(())
    }
}
