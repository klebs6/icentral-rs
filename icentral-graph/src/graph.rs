crate::ix!();

//-------------------------------------------[icentral/src/graph_t.cc]
//-------------------------------------------[icentral/src/bcc_scratch.cc]

/// TODO: remove this
pub fn print_edgelist_and_mucs<GH>(graph: &Graph<GH>)  
where Graph<GH>
: GetEdgeListDebugger 
+ Debug
+ GetMucDebuggerWithoutNodes
+ SetPrintNodes
+ GetEdges
+ SetMucDebug
+ GetMucs<GH>,
GH
: ExtendWith<GH,Error=BetweennessCentralityError>
+ Debug
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
+ SetPrintNodes
{
    debug!("{:?}", graph.edgelist_debugger());
    debug!("{:?}", graph.muc_debugger_without_nodes());
}

pub fn extract_graph_name(path: &str) -> String {
    
    let pos: usize = match path.rfind("/") { 
        Some(x) => x + 1, 
        None    => 0 
    };

    let length: usize = path.len() - pos + 1;

    let name = path[pos .. pos + length - 1].to_string();

    debug!("extracting graph_name {} from path {}", name, path);

    name
}

/**
  | Simple undirected unweighted graph
  | data structure no checks whatsoever
  | should call init_size(..) first then
  | insert_edge(..) to populate
  | 
  | IMP: nodes have indexes from 0 to n-1
  | 
  | IMP: graph is assumed to be connected
  |
  */
pub struct Graph<GH> {
    pub(crate) nodes_map:       NeighborsMap,
    pub(crate) nodes_to_mucs:   MucIdMap,
    pub(crate) edges:           Edges,
    pub(crate) mucs:            Vec<MinimumUnionCycle<GH>>,
    pub(crate) mcb:             MinimumCycleBasis,
    pub(crate) visit_markers:   VisitMarkers,
    pub(crate) bc_computed:     bool,
    pub(crate) scores:          BetweennessScores,
    pub(crate) graph_name:      String,
}

impl<GH> NumNodes for Graph<GH> {

    fn num_nodes(&self) -> usize {
        self.nodes_map.len()
    }
}

impl<GH> Graph<GH> 
where GH: BccGraphHashInterface
{
    delegate_to_neighbors_map!{}
    delegate_to_edges!{}

    #[inline] pub fn debug_all(&self) -> bool {
        true
    }

    pub fn adjacency_list_for_dachshund(&self) -> Vec<(usize,usize)> {
        self.edges.adjacency_list_for_dachshund()
    }
}

impl<GH> NodeIdToMucId for Graph<GH> 
//where GH: BccGraphHashInterface
{
    fn nodeid_to_mucid(&self, idx: NodeId) -> MinimumUnionCycleId {
        self.nodes_to_mucs.mucid_for_node(idx)
    }
}

impl<GH> GetMuc<GH> for Graph<GH> {

    fn muc(&self, idx: MinimumUnionCycleId) -> &MinimumUnionCycle<GH> {
        &self.mucs[idx.val()]
    }

    fn muc_mut(&mut self, idx: MinimumUnionCycleId) -> &mut MinimumUnionCycle<GH> {
        &mut self.mucs[idx.val()]
    }
}

impl<GH> SetGraphName for Graph<GH> {

    fn set_graph_name(&mut self, name: &str) {

        debug!("setting graph_name to {}", name);

        self.graph_name = name.to_string();
    }
}

impl<GH> GetEdges for Graph<GH> {

    fn edges(&self) -> &Edges {
        &self.edges
    }
}

impl<GH> HasEdge for Graph<GH> {

    fn has_edge(&self, e: &Edge) -> bool {
        self.edges.has_edge(e)
    }
}

impl<GH> NumEdges for Graph<GH> {

    fn num_edges(&self) -> usize {
        self.edges.len()
    }
}

impl<GH> Named for Graph<GH> {

    fn name(&self) -> Cow<'_,str> {
        Cow::Borrowed(&self.graph_name)
    }
}

impl<GH> CreateNamedEmpty for Graph<GH> {

    fn empty(name: &str) -> Self {

        debug!("creating new empty graph named {}", name);

        let nodes_map_name     = name![name, "nodes_map"];
        let edges_name         = name![name, "edges"];
        let visit_markers_name = name![name, "visit_markers"];
        let scores_name        = name![name, "scores"];
        let nodes_to_mucs_name = name![name, "nodes_to_mucs"];

        Self {
            nodes_map:       NeighborsMap::empty_mapped(nodes_map_name),
            nodes_to_mucs:   MucIdMap::empty_mapped(nodes_to_mucs_name),
            edges:           Edges::empty(edges_name),
            mucs:            vec![],
            mcb:             MinimumCycleBasis::empty(),
            visit_markers:   VisitMarkers::empty_mapped(visit_markers_name),
            bc_computed:     false,
            scores:          BetweennessScores::empty_mapped(scores_name),
            graph_name:      name.to_string(),
        }
    }
}

impl<GH> GetNodesToMucs for Graph<GH> {

    fn get_nodes_to_mucs<'a>(&'a self) -> &'a MucIdMap {
        &self.nodes_to_mucs
    }
}

impl<GH> GetMucs<GH> for Graph<GH> {

    fn get_mucs<'a>(&'a self) -> &Vec<MinimumUnionCycle<GH>> {
        &self.mucs
    }
}

impl<GH> fmt::Debug for Graph<GH> 
where GH: BccGraphHashInterface 
{
    //TODO: how should we handle the unwrapping
    //done here?
    //
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let binding = f.debug_struct("Graph");

        let mut builder = binding;

        builder.field("graph_name",          &self.graph_name);
        builder.field("nodes_map_len",       &self.nodes_map.len());
        builder.field("nodes_to_mucs_len",   &self.nodes_to_mucs.len());
        builder.field("edges_len",           &self.edges.len());
        builder.field("visit_markers_len",   &self.visit_markers.len());

        if self.debug_all() {
            builder.field("nodes_map",       &self.nodes_map);
            builder.field("nodes_to_mucs",   &self.nodes_to_mucs);
            builder.field("edges",           &self.edges);
            builder.field("visit_markers",   &self.visit_markers);
        }

        builder.field("mucs",                &self.mucs.len());
        builder.field("mcb",                 &self.mcb);
        builder.field("bc_computed",         &self.bc_computed);
        builder.field("scores",              &self.scores.len());
        builder.field("scores",              &self.scores.len());;

        builder.finish_non_exhaustive()
    }
}

impl<GH> From<GraphMock> for Graph<GH> {

    fn from(mock: GraphMock) -> Self {

        debug!("creating Graph from mock: {:?}", mock);

        let name = mock.name();

        let mut g = Graph::empty(&name);

        mock.fill(&mut g);

        g
    }
}

impl<GH> VisitMarkersHandle for Graph<GH> {

    fn visit_markers_handle(&mut self) -> &mut VisitMarkers {
        &mut self.visit_markers
    }
}
