crate::ix!();

/**
  | estimates speedups WITHOUT assuming
  | that all edges are going to be picked
  | from the same biconnected component
  |
  */
pub fn speedup_info<GH>(
        graph:               &mut Graph<GH>,
        num_edges_to_insert: i32) 
-> Result<(),BetweennessCentralityError>
where GH: BccGraphHashInterface
{
    graph.print_header();

    let mut bcc_vec: Vec<GraphHash> = vec![];

    graph.find_bicon_component(&mut bcc_vec);

    let g_s: usize = graph.num_nodes() + graph.num_edges();
    let g_n: usize = graph.num_nodes();

    let mut rand_edge_vec: Vec<Edge> = vec![];

    debug!("Generating edges...");

    let mut rng = WyRand::default();

    for i in 0..num_edges_to_insert {

        let mut rand_edge: Edge = Edge::default();

        let mut src: NodeId = NodeId::default();
        let mut dst: NodeId = NodeId::default();

        loop {

            src = nodeid![rng.generate::<usize>() % graph.num_nodes()];
            dst = nodeid![rng.generate::<usize>() % graph.num_nodes()];

            rand_edge.src = src;
            rand_edge.dst = dst;

            if !graph.has_edge(&rand_edge) 
            || rand_edge_vec.iter().find(|x| **x == rand_edge).is_some() 
            {
                break;
            }
        }

        rand_edge_vec.push(rand_edge);
    }

    debug!("");

    let mut est_speedup_vec: Vec<f64> = default!();

    for i in 0..rand_edge_vec.len() {

        let edge = rand_edge_vec[i];

        let mut bcc: GraphHash = GraphHash::empty(&format!{"GraphHash for edge: {}",edge});

        graph.insert_edge(&edge);

        graph.find_edge_bcc_subgraph(&mut bcc, &edge);

        graph.remove_edge(&edge);

        let bcc_s: usize = bcc.num_nodes() + bcc.num_edges();
        let bcc_n: usize = bcc.num_nodes();

        debug!(
            "BiconnectedComponents#nodes:[{:8}]   BiconnectedComponents#edges:[{:8}]   BiconnectedComponentsSizeFrac:[{}]   BiconnectedComponentsNodeFrac:[{}]", 
            bcc_n, 
            bcc.num_edges(), 
            bcc_s as f64 / g_s as f64, 
            bcc_n as f64 / graph.num_nodes() as f64
        );

        let (d0,d1,d2) = bcc.find_pruning_counts_exp(
            edge.src, 
            edge.dst, 
        )?;

        let iter_frac: f64 = 1.0 - (d0 as f64 / bcc_n as f64);

        let est_speedup: f64 = {
            let p0 = bcc_n as f64 / g_n as f64;
            let p1 = bcc_s as f64 / g_s as f64;
            let t0 =  p0 * iter_frac * p1;
            1.0 / t0 
        };

        est_speedup_vec.push(est_speedup);

        debug!(
            "d0:[{:8}][{}]   d1:[{:8}]    d2:[{:8}] -- {:?}", 
            d0, 
            d0 as f64 / bcc_n as f64, 
            d1, 
            d2, 
            edge, 
        );

        debug!("");
    }

    let stats = SpeedupStats::from(&mut est_speedup_vec);

    // estimate of the time it takes to do my long delta iteration
    // to the time it takes the normal iteration
    let mut long_iter_perc: f64 = 0.0;

    long_iter_perc = 150.0;

    debug!("");

    debug!(
        "Estimate ideal speedups: (delta iter to orig iter [{}])", 
        long_iter_perc
    );

    debug!(
        "Mean[{:?}] Stddev[{:?}] Median[{:?}] Min[{:?}] Max[{:?}]", 
        stats.mean   * (100.0 / long_iter_perc), 
        stats.stddev * (100.0 / long_iter_perc), 
        stats.median * (100.0 / long_iter_perc), 
        stats.min    * (100.0 / long_iter_perc), 
        stats.max    * (100.0 / long_iter_perc)
    );

    let worst_mean: f64 = stats.mean * (100.0 / long_iter_perc);

    long_iter_perc = 100.0;

    debug!(
        "Estimate ideal speedups: (delta iter to orig iter [{}])", 
        long_iter_perc
    );

    debug!(
        "Mean[{:?}] Stddev[{:?}] Median[{:?}] Min[{:?}] Max[{:?}]", 
        stats.mean   * (100.0 / long_iter_perc), 
        stats.stddev * (100.0 / long_iter_perc), 
        stats.median * (100.0 / long_iter_perc), 
        stats.min    * (100.0 / long_iter_perc), 
        stats.max    * (100.0 / long_iter_perc)
    );

    let best_mean: f64 = stats.mean * (100.0 / long_iter_perc);

    debug!("");

    debug!("[[Average speedup (ideal): {} - {}]]", worst_mean, best_mean);

    Ok(())
}
