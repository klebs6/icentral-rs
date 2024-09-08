crate::ix!();

pub fn insertion_test_fuad_hash(
    graph:               &mut Graph<GraphHash>,
    num_edges_to_insert: i32) 
-> Result<(),BetweennessCentralityError> 
{
    graph.print_header();

    let mut gh = GraphHash::new_from_graph_ref(
        &*graph, 
        "insertion_test_fuad_hash::graph"
    );

    let mut brandes_time = Duration::from_secs(0);

    let mut tm: Timer = default!();

    let num_iter: usize = gh.num_nodes();

    debug!("Brandes runtime: ");

    tm.start();

    let scores = brandes_bc_hash(&mut gh, None)?;

    tm.stop();

    brandes_time = tm.interval();

    debug!("{:?}", brandes_time);

    debug!("Brandes avg iter time: {:?}", brandes_time / num_iter as u32);

    let mut rand_edge_vec: Vec::<Edge> = vec![];

    debug!("Generating edges...");

    let mut rng = WyRand::default();

    for i in 0..num_edges_to_insert {

        let mut rand_edge: Edge = Edge::default();

        let mut src: NodeId = NodeId::default();
        let mut dst: NodeId = NodeId::default();

        loop {

            let len = graph.num_nodes();

            src = nodeid![rng.generate::<usize>() % len];
            dst = nodeid![rng.generate::<usize>() % len];

            rand_edge.src = src;
            rand_edge.dst = dst;

            if !graph.has_edge(&rand_edge) 
            || rand_edge_vec.iter().find(|x| **x == rand_edge).is_some() {
                break;
            }
        }

        rand_edge_vec.push(rand_edge);
    }

    let mut tot_fuad_time = Duration::from_secs(0);
    let mut tot_speedup:   f64 = 0.0;

    let mut bcc_stat_vec: Vec::<BiconnectedComponentsStat> = vec![];

    for i in 0..rand_edge_vec.len() {

        let mut bcc_stat: BiconnectedComponentsStat = default!();

        let rand_edge: Edge = rand_edge_vec[i];

        debug!(
            "Inserting edge ({}, {}) and updating bc...", 
            rand_edge.src, 
            rand_edge.dst
        );

        tm.start();

        graph.insert_edge_update_bc_experimental(
            &rand_edge, 
            &mut bcc_stat,
            None,
            None,
        );

        tm.stop();

        let mut avg_iter_time_naive = Duration::from_secs(0);

        graph.approx_bcc_iter_tm(
            rand_edge.src, 
            rand_edge.dst, 
            &mut avg_iter_time_naive,
            None,
        );

        tot_fuad_time += tm.interval();

        tot_speedup += brandes_time.div_duration_f64(tm.interval());

        debug!("BiconnectedComponents num nodes:  {}",    bcc_stat.bcc_num_nodes);
        debug!("BiconnectedComponents num edges:  {}",    bcc_stat.bcc_num_edges);
        debug!("Time to update BC:                {:?}",  tm.interval());
        debug!("Speedup:                          {:?}",  brandes_time.div_duration_f64(tm.interval()));
        debug!("BiconnectedComponents num d0 BFS: {}",    bcc_stat.num_d0_iter);
        debug!("BiconnectedComponents d0 time:    {:?}",  bcc_stat.tot_d0_tm / bcc_stat.num_d0_iter as u32);
        debug!("BiconnectedComponents num d1 BFS: {}",    bcc_stat.num_d1_iter);
        debug!("BiconnectedComponents d1 time:    {:?}",  bcc_stat.tot_d1_tm / bcc_stat.num_d1_iter as u32);
        debug!("BiconnectedComponents num d2 BFS: {}",    bcc_stat.num_d2_iter);
        debug!("BiconnectedComponents d2 time:    {:?}",  bcc_stat.tot_d2_tm / bcc_stat.num_d2_iter as u32);
        debug!("avg iter time naive:              {:?}",  avg_iter_time_naive);

        // debug!("BiconnectedComponents bcc find time: {}", bcc_stat.bcc_find_time);
        // debug!("BiconnectedComponents single_source_shortest_paths time: {}", bcc_stat.single_source_shortest_paths_tm);
        bcc_stat_vec.push(bcc_stat);
    }

    // double avg_speedup = (num_edges_to_insert*brandes_time)/tot_fuad_time;
    let avg_speedup: f64 = tot_speedup / num_edges_to_insert as f64;

    debug!("[[Average speedup: {}]]", avg_speedup);

    Ok(())
}
