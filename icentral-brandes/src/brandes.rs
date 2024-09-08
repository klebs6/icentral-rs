crate::ix!();

/**
  | One iteration of Brandes algorithm
  | from source @s
  |
  */
#[tracing::instrument]
pub fn brandes_iter(
    scores:    &mut BetweennessScores,
    component: Arc<Mutex<Component>>,
    source:    NodeId,
    workspace: &mut ICentralWorkspace) 
-> Result<(),BetweennessCentralityError>  
{
    let component_len = component.lock()?.num_nodes();

    debug!("running Brandes iteration from source={}, component_len={}", source, component_len);

    workspace.init_all(component_len);

    if let mut component = component.lock()? {

        debug!("locked component");

        debug!("brandes_iter, initiating bbfs from source={}", source);

        bbfs(
            None, 
            workspace, 
            &component,
            source
        );

        debug!("brandes_iter, initiating rbfs from source={}", source);

        rbfs(
            scores, 
            &mut component, 
            source, 
            workspace, 
            Some(RbfsOperation::Addition),
        );

        debug!("unlocking component");
    }

    debug!(
        "ending brandes iteration from source {} with scores: {:#?}, workspace: {:#?}", 
        source, 
        scores, 
        workspace
    );

    Ok(())
}

pub fn brandes_block(
    delta_bc_of_vertices: Arc<Mutex<Vec<f64>>>,
    component:                 Arc<Mutex<Component>>,
    source_vec:           Arc<Mutex<Vec<NodeId>>>)
{
    todo!();
        /*
        //    workspace_t workspace;
        //    for(node_id_t s = 0; s < component.subgraph.len(); ++s) {
        //        brandes_iter(scores, component, s, workspace);
        //    }
        */
}

pub fn brandes_betweenness_centrality_iteration<G: NumNodes + GetNeighborsForNode>(
    graph:  &G, 
    id:     NodeId, 
    scores: &mut BetweennessScores) 
-> Result<(),BetweennessCentralityError> 
{
    debug!("initiating brandes_betweenness_centrality_iteration from nodeid: {}", id);

    let num_nodes = graph.num_nodes();

    let workspace_name = format!("brandes_betweenness_centrality_iteration_for_{}_workspace", id);

    let mut workspace  = BrandesIterationWorkspace::new(num_nodes,id, &workspace_name);

    workspace.search_neighborhood_and_update_path_counts(graph)?;

    workspace.update_pair_dependencies_and_scores(id, scores)?;

    Ok(())
}

pub fn brandes_betweenness_centrality<G: GetNeighborsForNode + NumNodes + CreateScoresVector>(graph: &G) 
-> Result<BetweennessScores,BetweennessCentralityError> 
{
    debug!("initiating brandes_betweenness_centrality for graph of len {}", graph.num_nodes());

    let mut scores = graph.create_scores_vector();

    for nodeid in NodeIdRange::new(0,graph.num_nodes()) {

        brandes_betweenness_centrality_iteration(graph, nodeid, &mut scores);
    }

    scores.halve();

    Ok(scores)
}

pub fn brandes_bc<G: NumNodes + GetNeighborsForNode + CreateScoresVector + GetLimitedNodeIdRange>(
    graph:    &mut G,
    max_iter: Option<usize>) 
-> Result<BetweennessScores,BetweennessCentralityError> 
{
    debug!("initiating Brandes BC for graph of len {}", graph.num_nodes());

    let mut scores = graph.create_scores_vector();

    let nodes   = graph.limited_nodeid_range(max_iter);

    let n_nodes = nodes.len();

    for node in nodes {

        if node.val() % 100 == 0 {
            debug!("running brandes_iter_with_graph for iter {} / {}", node, n_nodes);
        }

        brandes_iter_with_graph(
            graph, 
            node, 
            &mut scores
        );
    }

    if max_iter.is_none() {
        scores.halve();
    }

    Ok(scores)
}

// @s_out_vec must have the same size as the graph
//
// increments bc values in @scores, with the pair
// dependency of @s
//
pub fn brandes_iter_with_graph<G: NumNodes + GetNeighborsForNode>(
    graph:  &mut G,
    source: NodeId,
    scores: &mut BetweennessScores) 
-> Result<(), BetweennessCentralityError> 
{
    debug!("initiating Brandes iteration from source: {}", source);

    let num_nodes = graph.num_nodes();

    let mut workspace = BrandesIterWithGraphWorkspace::new(
        num_nodes,
        source,
        "brandes_iter_with_graph_workspace"
    );

    workspace.search_neighborhood_and_update_path_counts(graph)?;

    workspace.update_scores(source, scores)?;

    Ok(())
}

pub fn brandes_bc_hash_out<G: NumNodes + GetNeighborsForNode + NumNodes + MappedNodes + GetLimitedNodeIdRange>(
    graph:    &G,
    max_iter: Option<usize>) 
-> Result<BetweennessScores,BetweennessCentralityError> 
{
    debug!("initiating brandes_bc_hash_out for graph of len {}", graph.num_nodes());

    let mut scores = BetweennessScores::new_from_graph_ref(graph, "brandes_bc_hash_out::scores");

    for node in graph.limited_nodeid_range(max_iter) {

        brandes_iter_hash_out(
            graph, 
            node, 
            &mut scores
        );
    }

    if max_iter.is_none() {

        debug!("max_iter.is_none() == true, so we will halve the scores!");

        scores.halve();
    }

    Ok(scores)
}

// increments bc values in @scores, with the pair
// dependency of @s
//
pub fn brandes_iter_hash_out<G: NumNodes + GetNeighborsForNode>(
    graph:  &G,
    source: NodeId,
    scores: &mut BetweennessScores) 
-> Result<(),BetweennessCentralityError> 
{
    debug!("initiating brandes_iter_hash_out for source node: {}", source);

    let num_nodes = graph.num_nodes();

    let mut workspace = BrandesIterHashWorkspace::new(
        num_nodes,
        source,
        &format!["brandes_iter_hash_out::source{}_workspace", source]
    );

    workspace.search_neighborhood_and_update_path_counts(graph)?;

    workspace.update_pair_dependencies_and_scores(source, scores)?;

    Ok(())
}
