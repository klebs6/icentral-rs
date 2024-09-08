crate::ix!();

pub trait GraphHashMucInterface
: ExtendWith<Self> 
+ Sized
+ NumNodes 
+ NumEdges 
+ GetEdges 
+ HasMapForNode 
+ MappedNodes 
+ InsertEdge 
+ InsertNode {}

impl<T> GraphHashMucInterface for T where
T: ExtendWith<Self> 
+ Sized
+ NumNodes 
+ NumEdges 
+ GetEdges 
+ HasMapForNode 
+ MappedNodes 
+ InsertEdge 
+ InsertNode {}

pub trait GetMucs<GH> {

    fn get_mucs<'a>(&'a self) -> &Vec<MinimumUnionCycle<GH>>;
}

pub trait GetMuc<GH> {
    fn muc(&self, idx: MinimumUnionCycleId) -> &MinimumUnionCycle<GH>;
    fn muc_mut(&mut self, idx: MinimumUnionCycleId) -> &mut MinimumUnionCycle<GH>;
}

pub trait FindMucs {

    fn find_mucs_fast(&mut self) 
    -> Result<(),BetweennessCentralityError>;

    fn find_mucs(&mut self);
}

pub trait FindMucMcb<GH> {

    fn find_muc_mcb(&mut self) 
    -> Result<(),BetweennessCentralityError>;

    fn find_muc_mcb_for_cycle(
        &mut self, 
        src:           NodeId, 
        cycle_graph:   &GH, 
        visit_markers: &mut VisitMarkers) 
    -> Result<(),BetweennessCentralityError>;
}

pub trait CollectAllMucEdges {

    fn collect_all_edges_in_mucs_in_one_set(&mut self) 
    -> Result<Edges,BetweennessCentralityError>;
}

pub trait ClearMucs {

    fn clear_mucs(&mut self) 
    -> Result<(),BetweennessCentralityError>;
}

pub trait MergeMucCycle<GH> {

    fn merge_muc_cycle(
        &mut self, 
        muc:   &mut MinimumUnionCycle<GH>,
        cycle: &Cycle
    );
}

pub trait CreateSingleVertexMucs {

    fn create_single_vertex_muc(
        &mut self, 
        idx: NodeId
    );

    fn maybe_create_single_vertex_muc(
        &mut self, 
        idx: NodeId
    );

    fn create_single_vertex_mucs(&mut self);
}

pub trait FindAllMucSubgraphs {

    fn find_all_muc_subgraphs(&mut self);
}

pub trait ConstructSingleNodeMucs {

    fn maybe_construct_single_node_muc(&mut self, idx: NodeId);
    fn construct_single_node_mucs(&mut self);
}

pub trait ConstructMucsForConnectedComponent<GH> {

    fn maybe_construct_mucs_for_connected_component(
        &mut self, 
        component: GH
    );

    fn construct_mucs_for_connected_component(
        &mut self,
        component: GH
    );
}

pub trait ConstructMucs<GH> {

    fn construct_mucs(&mut self, conn_comp_vec: Vec<GH>);
}

pub trait GetNumMucs {

    fn get_num_mucs(&mut self) -> usize;
}
