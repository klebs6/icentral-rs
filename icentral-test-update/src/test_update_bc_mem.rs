crate::ix!();
   

#[test] fn test_update_bc_mem() -> Result<(),BetweennessCentralityError> {

    setup_test_logger![];

    info!("Testing [Update_BC_mem] ... ");

    let mut graph: Graph = Graph::from_filename("icentral/Erdos02.lcc.net");

    let mut e: Edge = Edge::new_with_ids(0,2);

    // fill_test_graph(graph);

    e = *graph.edges().iter().nth(0).unwrap();

    graph.remove_edge(&e);

    info!("({}, {})", e.src, e.dst);

    let graph_len = graph.len();

    let mut dummy_scores  = BetweennessScores::new(graph_len, "test_update_bc_mem::dummy_scores");

    let mut workspaces    = WorkspaceMap::new(graph_len, "test_update_bc_mem::workspaces");

    let mut component     = Component::new_from_graph_ref(&graph, "test_update_bc_mem::component");

    workspaces.bbfs_rbfs(
        &mut dummy_scores, 
        None, 
        &mut component
    );

    let mut scores = fast_brandes_bc(&graph)?;

    let mut bc_mem_workspace = BcMemWorkspace::empty("test_update_bc_mem::bc_mem_workspace");

    update_bc_mem(
        &mut bc_mem_workspace,
        &mut scores, 
        &graph, 
        e, 
        &mut workspaces
    )?;

    graph.insert_edge(&e);

    let ref_scores = brandes_bc(&mut graph, None)?;

    for node in scores.nodeid_range() {

        let diff = (scores.score_for_node(node) - ref_scores.score_for_node(node)).abs();

        if diff > EPS.into() {

            let msg = format!(
                "({}) -- [{}]  [{}]", 
                node, 
                scores.score_for_node(node), 
                ref_scores.score_for_node(node)
            );

            return Err(BCError::mismatch_diff(diff,Some(&msg)));
        }

        // printf("(%d) -- [%f]  [%f]", node, scores.score_for_node(&node), ref_scores.score_for_node(&node));
    }

    Ok(())
}
