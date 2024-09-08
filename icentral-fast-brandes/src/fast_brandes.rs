crate::ix!();

/**
  | Computes betweenness centrality using
  | Brandes algorithm
  | 
  | Will construct @scores
  |
  */
#[tracing::instrument]
pub fn fast_brandes_bc<G>(graph:  &G) 
-> Result<BetweennessScores,BetweennessCentralityError> 
where 
G
: CreateScoresVector 
+ Debug 
+ NumNodes 
+ MappedNodes 
+ GetNodeIdRange 
+ GetNeighborsForNode 
+ GetEdges
{
    let num_nodes = graph.num_nodes();

    debug!("num_nodes: {}", num_nodes);

    let mut scores = graph.create_scores_vector();

    // 1. make a component (fill a subgraph)
    let mut component = Component::new_from_graph_ref(graph, "fast_brandes_bc::component");

    info!("created component: {:#?}", component);

    let component_num_nodes = component.num_nodes();

    debug!("comp_subgraph_len: {}", component_num_nodes);

    let mut workspace = ICentralWorkspace::new_init_all(component_num_nodes,"fast_brandes_bc::workspace");

    debug!("created workspace for iCentral {:#?}", workspace);

    let component = arcmut![component];

    // 2. loop through all sources
    for source in NodeIdRange::new(0,component_num_nodes) {

        brandes_iter(
            &mut scores, 
            component.clone(), 
            source, 
            &mut workspace
        );
    }

    debug!("finished with fast_brandes_bc");

    Ok(scores)
}
