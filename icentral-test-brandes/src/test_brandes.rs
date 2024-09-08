crate::ix!();

#[test] fn test_brandes() 
-> Result<(),BetweennessCentralityError> 
{
    setup_test_logger![];

    debug!("initiating test_brandes");

    let mut graph = Graph::from(GraphMock::Paper);

    debug!("graph: {:#?}", graph);

    let brandes_time = exp_brandes_p(
        &graph,
        None,
        None
    )?;

    info!("Brandes runtime:  {:?}", brandes_time);

    Ok(())
}

/// TODO: implement max_time
///
pub fn exp_brandes_p<GH>(
    graph:    &Graph<GH>,
    num_iter: Option<usize>, 
    max_time: Option<Duration>) 
-> Result<Duration,BetweennessCentralityError> 
where Graph<GH>: MappedNodes + GetLimitedNodeIdRange
{
    warn!("TODO: still need to implement max_time on this function!");

    graph.print_header();

    let brandes_time = match num_iter {
        Some(num_iter) => {
            exp_brandes_p_bounded_iterations(graph,num_iter)?
        }
        None => {
            exp_brandes_p_all_iterations(graph)?
        }
    };

    Ok(brandes_time)
}

pub fn exp_brandes_p_all_iterations<GH>(graph: &Graph<GH>) 
-> Result<Duration,BetweennessCentralityError> 
where Graph<GH>: MappedNodes + GetLimitedNodeIdRange + NumNodes
{
    debug!("exp_brandes_p will do all iterations");

    let mut tm: Timer = Timer::default();

    tm.start();

    let scores: BetweennessScores = brandes_bc_hash_out(
        graph,
        None,
    )?;

    tm.stop();

    let interval     = tm.interval();
    let graph_len    = graph.num_nodes();
    let brandes_time = interval;

    let brandes_avg_iter_time = interval / graph_len as u32;

    info!(
        "interval={:?}, graph_len={}, brandes_avg_iter_time={:?}, brandes_time={:?} -- DID ALL ITERATIONS", 
        interval,
        graph_len,
        brandes_avg_iter_time,
        brandes_time
    );

    Ok(brandes_time)
}

pub fn exp_brandes_p_bounded_iterations<GH>(
    graph:    &Graph<GH>, 
    num_iter: usize) 
-> Result<Duration,BetweennessCentralityError> 
where Graph<GH>: MappedNodes + GetLimitedNodeIdRange + NumNodes
{
    debug!("exp_brandes_p will do {} iterations", num_iter);

    let mut tm: Timer = Timer::default();

    tm.start();

    let scores: BetweennessScores = brandes_bc_hash_out(
        graph,
        Some(num_iter)
    )?;

    tm.stop();

    let interval = tm.interval();

    let brandes_time = {
        interval * graph.num_nodes() as u32 / num_iter as u32
    };

    let brandes_avg_iter_time = interval / num_iter as u32;

    info!(
        "brandes_avg_iter_time={:?}, interval={:?}, brandes_time={:?}, num_iter={}", 
        brandes_avg_iter_time,
        interval,
        brandes_time,
        num_iter
    );

    Ok(brandes_time)
}
