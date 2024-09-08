crate::ix!();

pub trait IsValid {

    fn is_valid(&self) -> bool;
}

pub trait McbFind {

    fn mcb_find(&self);
}

pub trait PrintHeader {

    fn print_header(&self);
}

pub trait InitDebugIteration {

    fn init_dbg_iteration(&mut self, source: NodeId);
}

pub trait DebugIterationStep {

    fn dbg_iteration_step(&mut self, v_s: &mut Vec<NodeId>) 
    -> Result<(),BetweennessCentralityError>;
}

pub trait UpdateWithSrcDst {

    fn update(&mut self, src: NodeId, dst: NodeId);
}

pub trait UpdatePairDependencies {

    fn update_pair_dependencies(&mut self, v_p: NodeId, v_n: NodeId);

    fn update_new_pair_dependencies(&mut self, v_p: NodeId, v_n: NodeId);
}

pub trait GetConnectedComponentSizes {

    fn conn_comp_sizes(&self) 
    -> Result<Vec<i32>,BetweennessCentralityError>;
}

pub trait FindSingleSourceShortestPaths {

    fn find_single_source_shortest_paths(&self, s: NodeId) 
    -> Result<DistanceMap,BetweennessCentralityError>;
}

pub trait CreateDistanceMaps {

    fn create_distance_maps(&self, edge: &Edge)
    -> Result<(DistanceMap, DistanceMap),BetweennessCentralityError>;
}

pub trait ComputeNewPathCountsAndPaths {

    fn compute_new_path_counts_and_paths(
        &mut self, 
        src: NodeId, 
        dst: NodeId);
}

pub trait BfsFromSource {

    fn do_bfs_from_source_count_vertices_and_mark_visited(
        &self, 
        source:        NodeId, 
        visit_markers: &mut VisitMarkers,
        out_vec:       &mut Vec<i32>)
    -> Result<(),BetweennessCentralityError>;
}

pub trait FindPruningCounts {

    fn find_pruning_counts_exp(&mut self, 
        src:    NodeId,
        dst:    NodeId) 
    -> Result<(i32,i32,i32),BetweennessCentralityError>;
}

pub trait FindConnVerts {

    fn find_conn_verts(&mut self) 
    -> Result<(),BetweennessCentralityError>;
}

pub trait BuildGraphHashMappingForConnVertex<GH> {

    fn build_graphhash_mapping_for_conn_vertex_step(
        &self, 
        gh:         &mut GH,
        bfs_source: NodeId,
        conn_vert:  NodeId,
        item:       (NodeId, &Vec<NodeId>),
        muc_id:     MinimumUnionCycleId) 
    -> Result<(),BetweennessCentralityError>;

    fn build_graphhash_mapping_for_conn_vertex(
        &self, 
        item:   (NodeId, &Vec<NodeId>),
        muc_id: MinimumUnionCycleId) 
    -> Result<(NodeId, Arc<GH>),BetweennessCentralityError>;
}

pub trait FindMucSubgraphs {

    fn find_muc_subgraphs(&mut self, muc_id: MinimumUnionCycleId) 
    -> Result<(),BetweennessCentralityError>;
}

pub trait ConnectNodeIds {

    fn connect_nodeids(&mut self, src: NodeId, dst: NodeId) 
    -> Result<(),BetweennessCentralityError>;
}

pub trait ResetVisitMarkersAndVisitNode {

    fn reset_visit_markers_and_visit_node(
        &mut self, 
        node: NodeId);
}

pub trait VisitMarkersHandle {

    fn visit_markers_handle(&mut self) -> &mut VisitMarkers;
}

pub trait ReadGraph {
    type Error;

    fn read_graph(&mut self, path: &str) 
    -> Result<(),Self::Error>;
}

pub trait GetShortestPath {

    fn get_shortest_path(&mut self, 
        src:      NodeId,
        dst:      NodeId)  
    -> Result<Vec<NodeId>,BetweennessCentralityError>;
}

pub trait BrandesIterInit {

    fn brandes_iter_init(&mut self, s: NodeId);
}

pub trait BrandesIterUpdateDistancesAndPathForNeighbors {

    fn brandes_iter_update_dist_and_path_for_neighbors(&mut self, s: NodeId);
}

pub trait GetPrintNodes {

    fn get_print_nodes(&self) -> bool;
}

pub trait SetPrintNodes {

    fn set_print_nodes(&self, val: bool);
}

pub trait InsertNode {

    fn insert_node(&mut self, id: NodeId);
}

pub trait ReinitMaps {

    fn reinit_maps(&mut self);
}

pub trait ReinitMapsForNode {

    fn reinit_maps_for_node(&mut self, k: NodeId);
}

pub trait CreateAndPushNewMuc {

    type Error;

    fn create_and_push_new_muc(
        &mut self, 
        shortest_path: &Vec<NodeId>,
        edge:          &Edge, 
        src_muc_id:    MinimumUnionCycleId,
        dst_muc_id:    MinimumUnionCycleId)
    -> Result<MinimumUnionCycleId,Self::Error>;
}

pub trait ConnectedComponentSize {

    fn connected_component_size_through_v_and_this_edge(
        &self, 
        visit_markers: &mut VisitMarkers, 
        u:             NodeId) -> usize;
}

pub trait InitBetweennessCentrality {

    fn init_bc(&mut self) 
    -> Result<(),BetweennessCentralityError>;
}

pub trait ApproxBrandesIterationRuntimeOnBccSubgraph {

    fn approx_bcc_iter_tm(&mut self, 
        src:           NodeId,
        dst:           NodeId,
        avg_iter_time: &mut Duration,
        num_iter:      Option<usize>) 
    -> Result<(),BetweennessCentralityError>;
}

pub trait FromFilename {

    fn from_filename(filename: &str) -> Self;
}

pub trait InsertEdgeUpdateMuc {

    fn insert_edge_and_update_muc_when_full_edge_contained_in_muc(&mut self, 
        edge:       &Edge, 
        src_muc_id: MinimumUnionCycleId
    );

    fn insert_edge_and_update_muc_when_we_are_not_in_the_same_muc(
        &mut self, 
        edge:       &Edge, 
        src_muc_id: MinimumUnionCycleId,
        dst_muc_id: MinimumUnionCycleId) 
    -> Result<(),BetweennessCentralityError>;

    fn insert_edge_update_muc(&mut self, edge: &Edge);
}

pub trait InsertEdgeUpdateBc {

    fn insert_edge_update_bc_experimental(&mut self, 
        edge:        &Edge,
        bcc_stat:    &mut BiconnectedComponentsStat,
        max_iter_d1: Option<usize>,
        max_iter_d2: Option<usize>) 
    -> Result<(),BetweennessCentralityError>;
}

pub trait SubtractEdge {

    fn simple_subtract_edge_and_update_mucs(&mut self, edge: &Edge);

    fn maybe_simple_subtract_edge_and_update_mucs(
        &mut self, 
        edge:          &Edge,
        all_muc_edges: &Edges
    );

    fn do_simple_subtraction_of_edges_and_update_mucs(
        &mut self, 
        all_muc_edges: &Edges
    );
}

pub trait CreateRandomConnected {

    fn random_connected(n_vertices: usize, n_edges: usize) -> Self;
}

pub trait SetGraphName {

    fn set_graph_name(&mut self, name: &str);
}

pub trait NewFromCycleVec {

    fn new_from_cycle_vec(cycle_vec: &Vec<Cycle>, name: &str) -> Self;
}

pub trait NewFromGraphRef<G> {

    fn new_from_graph_ref(graph: &G, name: &str) -> Self;
}

pub trait RemoveBridges {

     fn remove_bridges(&mut self, bridge_vec: Vec<Edge>);
}

pub trait FindConnectedComponents<GH> {

    type Error;

    fn find_conn_comp(&mut self) 
    -> Result<Vec<GH>,Self::Error>;
}
