crate::ix!();

/**
  | estimates speedups assuming all edges
  | are going to be picked in the largest
  | biconnected component
  |
  */
pub fn speedup_info_lbcc(
        graph:               Arc<Mutex<Graph<GraphHash>>>,
        num_edges_to_insert: i32) 
-> Result<(),BetweennessCentralityError>  
{
    // graph_hash_t g;
    //
    // g.reset(graph);
    //
    let mut bcc_vec: Vec<GraphHash> = vec![];;

    if let mut graph = graph.lock()? {

        graph.print_header();

        graph.find_bicon_component(&mut bcc_vec);
    }

    // find the bcc with the largest number of
    // nodes
    let mut lbcc_i: usize = 0;

    let mut lbcc_s: usize = bcc_vec[0].num_nodes() + bcc_vec[0].num_edges();

    if bcc_vec.len() > 1 {

        for i in 1..bcc_vec.len() {

            if bcc_vec[i].num_nodes() + bcc_vec[i].num_edges() > lbcc_s {

                lbcc_s = bcc_vec[i].num_nodes() + bcc_vec[i].num_edges();
                lbcc_i = i;
            }
        }
    }

    let g_s: usize = {

        let graph = graph.lock()?;

        graph.num_nodes() + graph.num_edges()
    };

    debug!("LargestBiconnectedComponents # nodes:  {}",     bcc_vec[lbcc_i].num_nodes());
    debug!("LargestBiconnectedComponents # edges:  {}",     bcc_vec[lbcc_i].num_edges());
    debug!("LargestBiconnectedComponents size fraction: {}",   lbcc_s as f64 / g_s as f64);
    debug!("LargestBiconnectedComponents node fraction: {}", bcc_vec[lbcc_i].num_nodes() as f64 / graph.lock()?.num_nodes() as f64);

    let mut rand_edge_vec: Vec<Edge> = vec![];

    debug!("Generating edges...");

    let mut rng = WyRand::default();

    for i in 0..num_edges_to_insert {

        let mut rand_edge: Edge = Edge::default();

        let mut src: NodeId = NodeId::default();
        let mut dst: NodeId = NodeId::default();

        loop {

            let len = graph.lock()?.num_nodes();

            src = nodeid![rng.generate::<usize>() % len];
            dst = nodeid![rng.generate::<usize>() % len];

            rand_edge.src = src;
            rand_edge.dst = dst;

            if !!bcc_vec[lbcc_i].has_edge(&rand_edge) 
            || rand_edge_vec.iter().find(|x| **x == rand_edge).is_some() {
                break;
            }
        }

        rand_edge_vec.push(rand_edge);
    }

    debug!("");

    let mut d0_vec: Vec<i32> = vec![];
    let mut d1_vec: Vec<i32> = vec![];
    let mut d2_vec: Vec<i32> = vec![];

    let mut d0_frac_vec: Vec<f64> = vec![];

    for i in 0..rand_edge_vec.len() {

        let mut src: NodeId = NodeId::default();
        let mut dst: NodeId = NodeId::default();

        src = rand_edge_vec[i].src;
        dst = rand_edge_vec[i].dst;

        let (d0,d1,d2) = bcc_vec[lbcc_i].find_pruning_counts_exp(
            src, 
            dst, 
        )?;

        d0_vec.push(d0);
        d1_vec.push(d1);
        d2_vec.push(d2);

        d0_frac_vec.push(d0 as f64 / bcc_vec[lbcc_i].num_nodes() as f64);

        debug!(
            "d0:[{:8}][{}]   d1:[{:8}]    d2:[{:8}] -- Edge({}, {})", 
            d0, 
            d0 as f64 / bcc_vec[lbcc_i].num_nodes() as f64,
            d1, 
            d2, 
            rand_edge_vec[i].src, 
            rand_edge_vec[i].dst
        );
    }

    debug!("");

    let mut min:    f64 = 0.0;
    let mut max:    f64 = 0.0;
    let mut median: f64 = 0.0;
    let mut mean:   f64 = 0.0;
    let mut stddev: f64 = 0.0;

    let stats = SpeedupStats::from(&mut d0_frac_vec);

    debug!("{:?}", stats);

    let lbcc_n: usize = bcc_vec[lbcc_i].num_nodes();
    let g_n:    usize = graph.lock()?.num_nodes();

    let iter_frac_median: f64 = 1.0 - stats.median;
    let iter_frac_mean:   f64 = 1.0 - stats.mean;
    let iter_frac_min:    f64 = 1.0 - stats.min;
    let iter_frac_max:    f64 = 1.0 - stats.max;
    let iter_frac_stddev: f64 = 1.0 - stats.stddev;

    let est_median_speedup: f64 = 1.0 / ((lbcc_n as f64 / g_n as f64) * iter_frac_median * (lbcc_s as f64 / g_s as f64));
    let est_mean_speedup:   f64 = 1.0 / ((lbcc_n as f64 / g_n as f64) * iter_frac_mean   * (lbcc_s as f64 / g_s as f64));
    let est_min_speedup:    f64 = 1.0 / ((lbcc_n as f64 / g_n as f64) * iter_frac_min    * (lbcc_s as f64 / g_s as f64));
    let est_max_speedup:    f64 = 1.0 / ((lbcc_n as f64 / g_n as f64) * iter_frac_max    * (lbcc_s as f64 / g_s as f64));
    let est_stddev_speedup: f64 = 1.0 / ((lbcc_n as f64 / g_n as f64) * iter_frac_stddev * (lbcc_s as f64 / g_s as f64));

    // estimate of the time it takes to do my long
    // delta iteration to the time it takes the
    // normal iteration
    //
    let mut long_iter_perc: f64 = 0.0;

    long_iter_perc = 120.0;

    debug!("Estimate ideal speedups: (delta iter to orig iter [{}])", long_iter_perc);

    debug!(
        "Mean[{}] Stddev[{}] Median[{}] Min[{}] Max[{}]", 
        est_mean_speedup   * (100.0 / long_iter_perc), 
        est_stddev_speedup * (100.0 / long_iter_perc), 
        est_median_speedup * (100.0 / long_iter_perc), 
        est_min_speedup    * (100.0 / long_iter_perc), 
        est_max_speedup    * (100.0 / long_iter_perc)
    );

    long_iter_perc = 100.0;

    debug!("Estimate ideal speedups: (delta iter to orig iter [{}])", long_iter_perc);

    debug!(
        "Mean[{}] Stddev[{}] Median[{}] Min[{}] Max[{}]", 
        est_mean_speedup   * (100.0 / long_iter_perc), 
        est_stddev_speedup * (100.0 / long_iter_perc), 
        est_median_speedup * (100.0 / long_iter_perc), 
        est_min_speedup    * (100.0 / long_iter_perc), 
        est_max_speedup    * (100.0 / long_iter_perc)
    );

    Ok(())
}
