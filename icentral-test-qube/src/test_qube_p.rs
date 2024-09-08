crate::ix!();


pub fn exp_qube_p_step<GH>(
    tot_speedup:   &mut f64,
    tot_time:      &mut Duration,
    speedup_vec:   &mut Vec<f64>,
    rand_edge:     Edge,
    graph:         &mut Graph<GH>,
    num_iter:      Option<usize>,
    rand_edge_vec: &Vec<Edge>,
    brandes_time:  Duration, 
    edge_stat_vec: &mut Vec<EdgeStat>) 
-> Result<(),BetweennessCentralityError> 
where GH
: GraphHashMucInterface 
+ InsertEdgeUpdateMuc 
+ GetConnectedComponentSizes 
+ GetNeighborsForNode 
+ GetNodeIdRange,
Graph<GH>
: InsertEdgeUpdateMuc
{
    let mut tm: Timer = Timer::default();

    let mut edge_stat: EdgeStat = EdgeStat::default();

    // debug!("Adding edge ({}, {})", rand_edge.src, rand_edge.dst);
    tm.start();

    graph.insert_edge_update_muc(&rand_edge);

    tm.stop();

    edge_stat.edge_ins_time = tm.interval();

    let muc_id: MinimumUnionCycleId = graph.nodeid_to_mucid(rand_edge.src);

    edge_stat.muc_num_edges = graph.muc(muc_id).num_edges();
    edge_stat.muc_num_nodes = graph.muc(muc_id).num_nodes();

    let mut scores = BetweennessScores::empty_mapped("exp_qube_p_step::scores");

    tm.start();

    graph.muc_mut(muc_id).compute_bc(
        &mut scores, 
        num_iter
    );

    tm.stop();

    edge_stat.muc_bc_update_time = match num_iter {
        Some(num_iter) => {
            (tm.interval() / num_iter as u32) * edge_stat.muc_num_nodes as u32
        }
        None => {
            tm.interval()
        }
    };

    // TMP CODE XXX FIX THIS SHIT
    edge_stat.muc_bc_update_time += edge_stat.edge_ins_time;

    let speedup: f64 = brandes_time.div_duration_f64(edge_stat.muc_bc_update_time);

    *tot_speedup += speedup;

    speedup_vec.push(speedup);

    *tot_time += edge_stat.muc_bc_update_time;

    //         debug!("MinimumUnionCycle # nodes: {} ({} G_n)",
    //                 edge_stat.muc_num_nodes,
    //                 edge_stat.muc_num_nodes/(double)graph.len());
    //         debug!("MinimumUnionCycle # edges: {} ({} G_M)",
    //                 edge_stat.muc_num_edges,
    //                 edge_stat.muc_num_edges/(double)graph.num_edges());
    //         debug!("");
    //         debug!("Time to update BC: {}", edge_stat.muc_bc_update_time);
    //         debug!("Speedup: {}", speedup);
    //         debug!("");
    //         debug!("QUBE avg iter time: {}",
    //                 edge_stat.muc_bc_update_time/edge_stat.muc_num_nodes);
    //         debug!("Time to update MinimumUnionCycles: {}", edge_stat.edge_ins_time);
    debug!(
        "e({:6},{:6})  tm[{:.2?}]  sup[{:.2}]",
        rand_edge.src,
        rand_edge.dst,
        edge_stat.muc_bc_update_time,
        speedup
    );

    edge_stat_vec.push(edge_stat);

    Ok(())
}

pub fn exp_qube_p<GH>(
    graph:         &mut Graph<GH>,
    num_iter:      Option<usize>,
    rand_edge_vec: &Vec<Edge>,
    brandes_time:  Duration) 
-> Result<(),BetweennessCentralityError> 

    where GH
    : GraphHashMucInterface
    + GetNeighborsForNode
    + GetNodeIdRange
    + InsertEdgeUpdateMuc
    + GetConnectedComponentSizes, 

      Graph<GH>
      : InsertEdgeUpdateMuc 
      + GetLimitedNodeIdRange 
      + MappedNodes 
      + FindMucs
{
    graph.print_header();

    debug!("Brandes_tm[{:.2?}]", brandes_time);

    let mut muc_find_time = Duration::from_secs(0);

    let mut tm: Timer = Timer::default();

    let scores: BetweennessScores 
    = brandes_bc_hash_out(
        graph,
        Some(1)
    )?;

    debug!("# MinimumUnionCycles: ");

    tm.start();

    graph.find_mucs();

    tm.stop();

    muc_find_time = tm.interval();

    debug!("{}", graph.get_num_mucs());

    debug!("Time to find MinimumUnionCycles: {:?}", muc_find_time);

    let mut edge_stat_vec: Vec<EdgeStat> = vec![];

    let mut tot_speedup: f64 = 0.0;

    let mut tot_time: Duration = Duration::from_secs(0);

    let mut speedup_vec: Vec<f64> = vec![];

    for i in 0..rand_edge_vec.len() {

        let rand_edge: Edge = rand_edge_vec[i];

        exp_qube_p_step(
            &mut tot_speedup,
            &mut tot_time,
            &mut speedup_vec,
            rand_edge, 
            graph, 
            num_iter, 
            rand_edge_vec, 
            brandes_time, 
            &mut edge_stat_vec
        );
    }

    let avg_speedup: f64 = tot_speedup / rand_edge_vec.len() as f64;

    // debug!("[[Avg. speedup (qube): {}]]", avg_speedup);
    let avg_time = tot_time / rand_edge_vec.len() as u32;

    debug!("Avg.tm[{:.2?}]  Avg.sup[{:.2}]", avg_time, avg_speedup);

    Ok(())
}
