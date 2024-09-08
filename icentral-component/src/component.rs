crate::ix!();

pub trait FindEdgeBccWithComponent<GH> {

    fn find_edge_bcc_with_component_step(
        &mut self, 
        gh:        &GH,
        v:         NodeId,
        component: &mut Component,
        e:         &Edge) 
    -> Result<(),BetweennessCentralityError>;

    fn find_edge_bcc_with_component(
        &mut self, 
        component: &mut Component,
        e:         &Edge) 
    -> Result<(),BetweennessCentralityError>;
}

//-------------------------------------------[icentral/src/component_t.h]

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum CompType {
    BiconnectedComponent, 
    MinimumUnionCycle, 
    Graph
}

impl Default for CompType {

    fn default() -> Self {
        Self::BiconnectedComponent
    }
}

/**
 | The component (subgraph along with other
 | needed information) that bc computation blocks
 | operate on.
 | 
 | Could be a BiconnectedComponents, an
 |  MinimumUnionCycle, or just a graph.
 |
 */
pub struct Component {
    name:                   String,
    subgraph:               SubGraph,
    articulation_point_map: ArticulationPointMap,
    comp_type:              CompType,
}

impl Component {

    delegate_to_articulation_point_map!{}

    pub fn new_from_graph_ref<G>(g: &G, name: &str) -> Self 
        where G: NumNodes + MappedNodes + GetNodeIdRange + GetNeighborsForNode + GetEdges
    {
        debug!("creating new Component named {} with CompType::Graph from Graph of num_nodes: {}", name, g.num_nodes());

        let subgraph_name               = name![name, "subgraph"];
        let articulation_point_map_name = name![name, "articulation_point_map"];

        Self {
            name:                   name.to_owned(),
            subgraph:               SubGraph::new_from_graph_ref(g, subgraph_name),
            articulation_point_map: ArticulationPointMap::empty_mapped(articulation_point_map_name),
            comp_type:              CompType::Graph,
        }
    }

    pub fn new_biconnected(name: &str) -> Self {

        let subgraph_name               = name![name, "subgraph"];
        let articulation_point_map_name = name![name, "articulation_point_map"];

        Component {
            name:                   name.to_owned(),
            subgraph:               SubGraph::empty(subgraph_name),
            articulation_point_map: ArticulationPointMap::empty_mapped(articulation_point_map_name),
            comp_type:              CompType::BiconnectedComponent,
        }
    }

    pub fn subgraph(&self) -> &SubGraph {
        &self.subgraph
    }

    pub fn ty(&self) -> CompType {
        self.comp_type.clone()
    }

    pub fn create_distance_maps(&self, edge: &Edge)
    -> Result<(DistanceMap, DistanceMap),BetweennessCentralityError>
    {
        Ok(self.subgraph.create_distance_maps(edge)?)
    }

    delegate! {
        to self.subgraph {

            #[call(reset_with)]
            pub fn reset_subgraph_with<GH>(&mut self, gh: &mut GH)
                where GH: NumNodes + GetNodeIdRange + GetNeighborsForNode + GetEdges;

            pub fn label_map_outin(&self, out: NodeId) -> NodeId;

            pub fn label_map_inout(&self, out: NodeId) -> NodeId;

            pub fn neighbors(&self, n: NodeId) -> Vec<NodeId>;

            #[call(num_nodes)]
            pub fn num_nodes(&self) -> usize;

            pub fn insert_edge_between_nodes(&mut self, src: NodeId, dst: NodeId);

            pub fn insert_edge(&mut self, edge: &Edge);

            pub fn find_single_source_shortest_paths(
                &self, 
                s: NodeId) 
            -> Result<DistanceMap,BetweennessCentralityError>;
        }
    }

    pub fn remap_articulation_point_map(&mut self) {

        let mut new_articulation_point_map 
            = ArticulationPointMap::empty_mapped(&self.articulation_point_map.name());

        for (k,v) in self.articulation_point_map.iter() {

            let new_v: NodeId = self.label_map_outin(*k);

            new_articulation_point_map.map_articulation_point(new_v,v);
        }

        self.articulation_point_map = new_articulation_point_map;
    }
}

impl fmt::Debug for Component {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let binding     = f.debug_struct("Component");
        let mut builder = binding;

        builder
            .field("subgraph",  &self.subgraph.debug_without_nodes())
            .field("comp_type", &self.comp_type);

        for (k,v) in self.articulation_point_map.iter() {

            let tag = format!{"articulation_point [{}]",k};

            let point: String = v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");

            builder.field(
                tag.as_str(), 
                &point
            );
        }

        builder.finish()
    }
}
