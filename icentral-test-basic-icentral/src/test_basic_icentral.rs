crate::ix!();

pub const USE_ERDOS: bool = false;

#[test] fn test_basic_icentral() -> Result<(), BetweennessCentralityError> {

    setup_test_logger![];
    
    let start = Instant::now();

    tracing::info!("Testing [iCentral] ... ");

    let e: Edge = Edge::new_with_ids(3,7);

    let mut graph: Graph = match USE_ERDOS {
        true  => Graph::from_filename("icentral/Erdos02.lcc.net"),
        false => Graph::from(GraphMock::Paper),
    };

    debug!("graph: {:#?}", graph);

    // fill_test_graph(graph);
    let mut scores = fast_brandes_bc(&graph)?;

    info!("scores: {:?}", scores);

    let mut component = Component::new_from_graph_ref(
        &graph, 
        "test_basic_icentral::component"
    );

    info!("component: {:#?}", component);

    let mut delta_bc_of_vertices = BetweennessScores::new(
        component.num_nodes(), 
        "test_basic_icentral::delta_bc_of_vertices"
    );

    let num_threads = 1;

    icentral(
        num_threads, 
        &mut delta_bc_of_vertices, 
        arcmut![component], 
        e, 
        None
    );

    info!("delta_bc_of_vertices: {:?}",delta_bc_of_vertices);

    for node in delta_bc_of_vertices.nodeid_range() {

        scores.increase_score_for_node(
            node, 
            delta_bc_of_vertices.score_for_node(node)
        );
    }

    info!("scores: {:?}", scores);

    let mut mismatches = vec![];

    debug!("if we insert this edge, it looks like we get mismatches...");

    //graph.insert_edge(&e);

    let ref_scores = brandes_bc(&mut graph, None)?;

    info!("ref_scores: {:?}", ref_scores);

    info!("checking for mismatches");

    for node in scores.nodeid_range() {

        let diff = (scores.score_for_node(node) - ref_scores.score_for_node(node)).abs();

        if diff > EPS.into() {

            mismatches.push(diff);
        }

        // debug!("[%f]  [%f]", scores.score_for_node(&v), ref_scores.score_for_node(&v));
    }

    for item in mismatches.iter() {

        warn!("data mismatch! diff={}", item);
    }

    if mismatches.len() > 0 {

        warn!("got {} mismatches!", mismatches.len());

        return Err(
            BCError::DataMismatches { mismatches }
        );
    }

    info!("no mismatches -- success!");

    let elapsed = Instant::now() - start;

    info!("elapsed: {:?}", elapsed);

    Ok(())
}
