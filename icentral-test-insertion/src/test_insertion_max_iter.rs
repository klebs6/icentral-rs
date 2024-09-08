crate::ix!();

pub fn insertion_test_fuad_max_iter<GH>(
    graph:                &mut Graph<GH>,
    num_edges_to_insert:  usize,
    num_iter:             usize,
    time_to_run_all_iter: Option<Duration>,
    rand_edge_vec:        Option<Vec<Edge>>) 
-> Result<(),BetweennessCentralityError>
where 
GH
: NumNodes 
+ NumEdges 
+ GraphHashMucInterface
+ GetEdges 
+ InsertEdge 
+ CreateNamedEmpty
+ GraphHashMucInterface
+ ResetWith<GH>
+ RemoveEdge
+ FindSingleSourceShortestPaths
+ GetNodeIdRange
+ GetNeighborsForNode
+ ParentsForNode
+ InitDebugIteration
+ DebugIterationStep
+ FindPruningCounts
+ HasEdge
+ PathCountForNode
+ PairDependencyForNode
+ SetPairDependencyForNode
+ SetSigmaValueForNode
+ GetSigmaValueForNode


, Graph<GH>
: MappedNodes
+ GetLimitedNodeIdRange
+ NumNodes
+ HasEdge
{
    let time_to_run_all_iter: Duration  = time_to_run_all_iter.unwrap_or(Duration::from_secs(0));
    let mut rand_edge_vec:    Vec<Edge> = rand_edge_vec.unwrap_or(vec![]);

    graph.print_header();

    let mut brandes_time = Duration::from_secs(0);

    let mut tm: Timer = default!();

    debug!("Brandes runtime:  ");

    tm.start();

    let scores: BetweennessScores = brandes_bc_hash_out(
        graph,
        Some(num_iter)
    )?;

    tm.stop();

    brandes_time = tm.interval() * graph.num_nodes() as u32 / num_iter as u32;

    let mut do_all_iter: bool = false;

    if brandes_time < time_to_run_all_iter {
        do_all_iter = true;
    }

    if do_all_iter {

        tm.start();

        let scores: BetweennessScores = brandes_bc_hash_out(
            graph,
            None
        )?;

        tm.stop();

        brandes_time = tm.interval();

        debug!("{:?}", brandes_time);

        debug!("Brandes avg iter time:  {:?}", tm.interval() / graph.num_nodes() as u32);

        debug!("WILL DO ALL ITERATIONS...");

    } else {

        debug!("{:?}", brandes_time);

        debug!("Brandes avg iter time:  {:?}", tm.interval() / num_iter as u32);
    }

    // vector<edge_t> rand_edge_vec;
    if rand_edge_vec.len() == 0 {

        debug!("Generating edges...");

        let mut rng = WyRand::default();

        rand_edge_vec = gen_rand_edges(
            &mut rng, 
            num_edges_to_insert, 
            graph
        )?;
    }

    let mut tot_fuad_time = Duration::from_secs(0);
    let mut tot_speedup:               f64 = 0.0;
    let mut tot_ideal_speedup_perfect: f64 = 0.0;
    let mut tot_ideal_speedup:         f64 = 0.0;

    let mut bcc_stat_vec: Vec<BiconnectedComponentsStat> = default!();

    let mut speedup_vec:  Vec<f64>     = default!();

    for i in 0..rand_edge_vec.len() {

        let mut bcc_stat: BiconnectedComponentsStat = BiconnectedComponentsStat::default();

        let rand_edge: Edge = rand_edge_vec[i];

        let mut est_time = Duration::from_secs(0);

        debug!(
            "Inserting edge ({}, {}) and updating bc...", 
            rand_edge.src, 
            rand_edge.dst
        );

        tm.start();

        graph.insert_edge_update_bc_experimental(
            &rand_edge, 
            &mut bcc_stat, 
            Some(num_iter), 
            Some(num_iter)
        );

        tm.stop();

        if do_all_iter {

            est_time = bcc_stat.bc_update_time + bcc_stat.bcc_find_time;

            //  TMP XXX FIX THIS
            // est_time = bcc_stat.bc_update_time;
        }

        let mut avg_iter_time_naive = Duration::from_secs(0);

        graph.approx_bcc_iter_tm(
            rand_edge.src, 
            rand_edge.dst, 
            &mut avg_iter_time_naive, 
            Some(num_iter)
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

        if !do_all_iter {

            est_time += {
                bcc_stat.tot_d1_tm * bcc_stat.tot_d1_iter as u32 / bcc_stat.num_d1_iter as u32
            };

            est_time += {
                bcc_stat.tot_d2_tm * bcc_stat.tot_d2_iter as u32 / bcc_stat.num_d2_iter as u32
            };
        }

        tot_speedup += brandes_time.div_duration_f64(est_time);

        speedup_vec.push(brandes_time.div_duration_f64(est_time));

        // ideal speedup estimation
        let mut ideal_speedup_perfect: f64 = 0.0;
        let mut ideal_speedup:         f64 = 0.0;

        let g_n:   f64 = graph.num_nodes() as f64;
        let g_m:   f64 = graph.num_edges() as f64;
        let bcc_n: f64 = bcc_stat.bcc_num_nodes as f64;
        let bcc_m: f64 = bcc_stat.bcc_num_edges as f64;

        // PARAMETER!! IMP make a #define
        let inc_iter_ratio: f64 = 150.0;

        let iter_frac: f64 = 1.0 - (bcc_stat.tot_d0_iter as f64 / bcc_n);

        ideal_speedup_perfect = fuad_ideal_speedup(
            iter_frac,
            bcc_n,
            bcc_m,
            g_n,
            g_m,
            100.0
        );

        ideal_speedup = fuad_ideal_speedup(
            iter_frac,
            bcc_n,
            bcc_m,
            g_n,
            g_m,
            inc_iter_ratio
        );

        tot_ideal_speedup_perfect += ideal_speedup_perfect;

        tot_ideal_speedup += ideal_speedup;

        // speedup if BFS DAGS were to be stored
        // in memory
        //
        let mut in_mem_speedup: f64 = 0.0;
        let mut in_mem_time = Duration::from_secs(0);

        in_mem_time += {
            let t0 = bcc_stat.tot_d1_tm / bcc_stat.num_d1_iter as u32 - avg_iter_time_naive;
            t0 * bcc_stat.tot_d1_iter as u32
        };

        in_mem_time += {
            let t0 = bcc_stat.tot_d2_tm / bcc_stat.num_d2_iter as u32 - avg_iter_time_naive;
            t0 * bcc_stat.tot_d2_iter as u32
        };

        in_mem_speedup = brandes_time.div_duration_f64(in_mem_time);

        debug!(
            "Stored DAGS ideal speedup:  {}", 
            in_mem_speedup
        );

        debug!(
            "BiconnectedComponents num nodes:    {} ({} G_n)", 
            bcc_stat.bcc_num_nodes, 
            bcc_stat.bcc_num_nodes as f64 / graph.num_nodes() as f64
        );

        debug!(
            "BiconnectedComponents num edges:    {} ({} G_m)", 
            bcc_stat.bcc_num_edges, 
            bcc_stat.bcc_num_edges as f64 / graph.num_edges() as f64
        );

        debug!("");

        debug!("Time to update BC:   {:?}", est_time);

        debug!(
            "Speedup:    {} (Ideal: {} - {})", 
            brandes_time.div_duration_f64(est_time), 
            ideal_speedup, 
            ideal_speedup_perfect
        );

        debug!("");

        debug!(
            "BiconnectedComponents d1 time:    {:?} ({:?} to naive iter)", 
            bcc_stat.tot_d1_tm / bcc_stat.num_d1_iter as u32, 
            (bcc_stat.tot_d1_tm / bcc_stat.num_d1_iter as u32).div_duration_f64(avg_iter_time_naive)
        );

        debug!(
            "BiconnectedComponents d2 time:    {:?} ({:?} to naive iter)", 
             bcc_stat.tot_d2_tm / bcc_stat.num_d2_iter as u32, 
            (bcc_stat.tot_d2_tm / bcc_stat.num_d2_iter as u32).div_duration_f64(avg_iter_time_naive)
        );

        debug!(
            "avg iter time naive:   {:?} ({:?} to naive iter)", 
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

        debug!("");

        debug!("BiconnectedComponents bcc find time: {:?}", bcc_stat.bcc_find_time);
        debug!("BiconnectedComponents single_source_shortest_paths time:     {:?}", bcc_stat.single_source_shortest_paths_tm);

        bcc_stat_vec.push(bcc_stat);

        // dont remove the edge, to have fair comparison to QUBE
        // in my QUBE implementation the MinimumUnionCycle update after edge removal is not
        // implemented
        // graph.remove_edge(rand_edge.src, rand_edge.dst);
    }

    // double avg_speedup = (num_edges_to_insert*brandes_time)/tot_fuad_time;
    let avg_speedup:               f64 = tot_speedup               / num_edges_to_insert as f64;
    let avg_ideal_speedup_perfect: f64 = tot_ideal_speedup_perfect / num_edges_to_insert as f64;
    let avg_ideal_speedup:         f64 = tot_ideal_speedup         / num_edges_to_insert as f64;

    debug!(
        "[[Average speedup: {} (Ideal: {} - {})]]", 
        avg_speedup, 
        avg_ideal_speedup, 
        avg_ideal_speedup_perfect
    );

    // calculating 95% confidence interval
    let stats = SpeedupStats::from(&mut speedup_vec);

    let diff: f64 = 1.96 * stats.stddev / num_edges_to_insert.sqrt() as f64;
    let v1:   f64 = stats.mean - diff;
    let v2:   f64 = stats.mean + diff;

    debug!("[[Speedup 95 CI: {} - {}]]", v1, v2);

    Ok(())
}
