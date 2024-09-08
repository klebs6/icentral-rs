crate::ix!();
   

#[test] fn test_fast_brandes_bc() -> Result<(),BetweennessCentralityError> {

    setup_test_logger![];

    let start = Instant::now();

    info!("Testing [brandes_BC] ... ");

    //let mut graph = Graph::from_filename(".cpp/icentral/Erdos02.lcc.net");
    let mut graph = Graph::from(GraphMock::Paper);

    info!("running fast_brandes_bc...");

    let scores = fast_brandes_bc(&graph)?;

    info!("finished fast_brandes_bc, running brandes_bc...");

    let ref_scores = brandes_bc(&mut graph, None)?;

    info!("finished brandes_bc");

    for node in scores.nodeid_range() {

        trace!("scores iter {}",node);

        let diff = scores.score_for_node(node) - ref_scores.score_for_node(node);

        if diff > EPS.into() {
            return Err(BCError::mismatch_diff(diff, None));
        }

        // printf("[%f]  [%f]", scores.score_for_node(&v), ref_scores.score_for_node(&v));
    }

    let elapsed = Instant::now() - start;

    info!("elapsed: {:?}", elapsed);

    info!("scores: {:#?}", scores);

    info!("ref_scores: {:#?}", ref_scores);

    Ok(())
}
