crate::ix!();

//-------------------------------------------[icentral/src/paper_exp.cc]

pub fn exp_fuad_p<GH>(
    graph:         &mut Graph<GH>,
    num_iter:      Option<usize>,
    rand_edge_vec: &Vec<Edge>,
    brandes_time:  Duration) 
-> Result<(),BetweennessCentralityError> 
where Graph<GH>: MappedNodes + GetLimitedNodeIdRange + InsertEdgeUpdateBc + ApproxBrandesIterationRuntimeOnBccSubgraph,
      GH: BccGraphHashInterface + GraphHashMucInterface
{
    graph.print_header();

    let scores: BetweennessScores
    = brandes_bc_hash_out(
        graph,
        Some(1)
    )?;

    let mut tm: Timer = Timer::default();

    let mut tot_fuad_time = Duration::from_secs(0);
    let mut tot_speedup:   f64 = 0.0;

    match num_iter {
        Some(num_iter) => {
            debug!("WILL DO {} ITERATIONS...", num_iter);
        }
        None => {
            debug!("WILL DO ALL ITERATIONS...");
        }
    }

    let mut bcc_stat_vec: Vec<BiconnectedComponentsStat> = vec![];
    let mut speedup_vec:  Vec<f64>     = vec![];
    
    for i in 0..rand_edge_vec.len() {

        let mut bcc_stat: BiconnectedComponentsStat = BiconnectedComponentsStat::default();

        let rand_edge: Edge = rand_edge_vec[i];

        let mut est_time = Duration::from_secs(0);

        debug!("Inserting edge {:?} and updating bc...", rand_edge);

        tm.start();

        graph.insert_edge_update_bc_experimental(
            &rand_edge, 
            &mut bcc_stat, 
            num_iter, 
            num_iter
        );

        tm.stop();

        if num_iter.is_none() {
            est_time = bcc_stat.bc_update_time + bcc_stat.bcc_find_time;
        }

        let mut avg_iter_time_naive = Duration::from_secs(0);

        graph.approx_bcc_iter_tm(
            rand_edge.src, 
            rand_edge.dst, 
            &mut avg_iter_time_naive, 
            Some(10)
        );

        tot_fuad_time += tm.interval();

        // fix counts if will produce errors:
        if bcc_stat.num_d0_iter == 0 {

            bcc_stat.num_d0_iter = 1;
            bcc_stat.tot_d0_tm   = Duration::from_secs(0);
        }

        if bcc_stat.num_d1_iter == 0 {

            bcc_stat.num_d1_iter = 1;
            bcc_stat.tot_d1_tm   = Duration::from_secs(0);
        }

        if bcc_stat.num_d2_iter == 0 {

            bcc_stat.num_d2_iter = 1;
            bcc_stat.tot_d2_tm   = Duration::from_secs(0);
        }

        if num_iter.is_some() {

            est_time += {
                bcc_stat.tot_d1_tm * bcc_stat.tot_d1_iter as u32 / bcc_stat.num_d1_iter as u32
            };

            est_time += {
                bcc_stat.tot_d2_tm * bcc_stat.tot_d2_iter as u32 / bcc_stat.num_d2_iter as u32
            };
        }

        tot_speedup += brandes_time.div_duration_f64(est_time);

        speedup_vec.push(brandes_time.div_duration_f64(est_time));

        debug!(
            " BiconnectedComponents num nodes:    {} ({} G_n)", 
            bcc_stat.bcc_num_nodes, 
            bcc_stat.bcc_num_nodes as f64 / graph.num_nodes() as f64
        );

        debug!(
            " BiconnectedComponents num edges:    {} ({} G_m)", 
            bcc_stat.bcc_num_edges, 
            bcc_stat.bcc_num_edges as f64 / graph.num_edges() as f64
        );

        debug!("");

        debug!("Time to update BC: {:?}", est_time);

        debug!(
            "Speedup: {}", 
            brandes_time.div_duration_f64(est_time)
        );

        debug!(
            "BiconnectedComponents d1 time: {:?} ({:?} to naive iter)", 
            bcc_stat.tot_d1_tm / bcc_stat.num_d1_iter as u32, 
            (bcc_stat.tot_d1_tm / bcc_stat.num_d1_iter as u32).div_duration_f64(avg_iter_time_naive)
        );

        debug!(
            "BiconnectedComponents d2 time: {:?} ({:?} to naive iter)", 
            bcc_stat.tot_d2_tm / bcc_stat.num_d2_iter as u32, 
            (bcc_stat.tot_d2_tm / bcc_stat.num_d2_iter as u32).div_duration_f64(avg_iter_time_naive)
        );

        debug!(
            "avg iter time naive: {:?} ({} to naive iter)", 
            avg_iter_time_naive, 
            avg_iter_time_naive.div_duration_f64(avg_iter_time_naive)
        );

        debug!("");

        debug!(
            "BiconnectedComponents num/tot d0 BFS:   {:8}/{:8} ({} to BiconnectedComponents_n)", 
            bcc_stat.num_d0_iter, 
            bcc_stat.tot_d0_iter, 
            bcc_stat.tot_d0_iter as f64 / bcc_stat.bcc_num_nodes as f64
        );

        debug!(
            "BiconnectedComponents num/tot d1 BFS:   {:8}/{:8} ({} to BiconnectedComponents_n)", 
            bcc_stat.num_d1_iter, 
            bcc_stat.tot_d1_iter, 
            bcc_stat.tot_d1_iter as f64 / bcc_stat.bcc_num_nodes as f64
        );

        debug!(
            "BiconnectedComponents num/tot d2 BFS:   {:8}/{:8} ({} to BiconnectedComponents_n)", 
            bcc_stat.num_d2_iter, 
            bcc_stat.tot_d2_iter, 
            bcc_stat.tot_d2_iter as f64 / bcc_stat.bcc_num_nodes as f64
        );

        debug!("BiconnectedComponents bcc find time: {:?}", bcc_stat.bcc_find_time);

        debug!("BiconnectedComponents single_source_shortest_paths time:     {:?}", bcc_stat.single_source_shortest_paths_tm);

        bcc_stat_vec.push(bcc_stat);

        // dont remove the edge, to have fair comparison to QUBE
        // in my QUBE implementation the MinimumUnionCycle update after edge removal is not
        // implemented
        // graph.remove_edge(rand_edge.src, rand_edge.dst);
    }

    // double avg_speedup = (num_edges_to_insert*brandes_time)/tot_fuad_time;
    let avg_speedup: f64 = tot_speedup / rand_edge_vec.len() as f64;

    debug!("[[Avg. speedup (fuad): {}]]", avg_speedup);

    Ok(())
}
