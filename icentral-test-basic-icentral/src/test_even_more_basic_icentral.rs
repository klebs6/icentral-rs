crate::ix!();

#[test] fn test_even_more_basic_icentral() -> Result<(), BetweennessCentralityError> {

    let start = Instant::now();

    setup_test_logger![];

    info!("Testing [iCentral] ... ");

    let mut graph = Graph::from(GraphMock::Paper); debug!("graph: {:#?}", graph);

    let e: Edge = Edge::new_with_ids(3,4);

    let mut component = Component::new_from_graph_ref(
        &graph, 
        "test_basic_icentral::component"
    );

    debug!("component: {:#?}", component);

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

    debug!("delta_bc_of_vertices: {:?}",delta_bc_of_vertices);

    let elapsed = Instant::now() - start;

    info!("elapsed: {:?}", elapsed);

    Ok(())

}
