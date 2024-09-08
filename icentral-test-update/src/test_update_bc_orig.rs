crate::ix!();
   

#[test] fn test_update_bc_orig() -> Result<(),BetweennessCentralityError> {

    setup_test_logger![];
    
    tracing::info!("Testing [Update_BC] ... ");

    let mut start = Instant::now();

    //let mut graph = Graph::from_filename("icentral/Erdos02.lcc.net");
    let mut graph = Graph::from(GraphMock::Paper);

    let mut e: Edge = Edge::new_with_ids(100,3455);

    // fill_test_graph(graph);
    e = *graph.edges().iter().nth(0).unwrap();

    graph.remove_edge(&e);

    // printf("(%d, %d)", e.first, e.second);
    let mut scores = fast_brandes_bc(&graph)?;

    update_bc(
        &mut scores, 
        &mut graph, 
        CompType::BiconnectedComponent, 
        e, 
        None, 
        None
    );

    graph.insert_edge(&e);

    let ref_scores = brandes_bc(&mut graph, None)?;

    for node in scores.nodeid_range() {

        if (scores.score_for_node(node) - ref_scores.score_for_node(node)).abs() > EPS.into() {

            let msg = format!{
                "({}) -- [{}]  [{}]", 
                node, 
                scores.score_for_node(node), 
                ref_scores.score_for_node(node)
            };

            return Err(BCError::DataMismatch { msg  });
        }

        // printf("(%d) -- [%f]  [%f]", node, scores.score_for_node(&node), ref_scores.score_for_node(&node));
    }

    let elapsed = Instant::now() - start;

    info!("elapsed: {:?}", elapsed);

    Ok(())
}
