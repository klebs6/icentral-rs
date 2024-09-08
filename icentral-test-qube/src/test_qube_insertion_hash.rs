crate::ix!();

pub fn insertion_test_qube_hash_step<GH>(
    brandes_time:         &Duration,
    tot_speedup:          &mut f64,
    tot_ideal_speedup:    &mut f64,
    speedup_vec:          &mut Vec<f64>,
    rand_edge:            Edge,
    graph:                &mut Graph<GH>,
    num_edges_to_insert:  usize,
    use_incremental_algo: bool,
    max_iter:             Option<usize>,
    edge_stat_vec:        &mut Vec<EdgeStat>) 
-> Result<(),BetweennessCentralityError> 

where GH
      : BccGraphHashInterface
      + GetConnectedComponentSizes
      + ExtendWith<GH,Error=BetweennessCentralityError>
      + GetNeighborsForNode
      + GetNodeIdRange
      + HasMapForNode
      + InsertNode
      + MappedNodes,

      Graph<GH>
      : InsertEdgeUpdateMuc,

      MinimumUnionCycle<GH>
      : GetConnectedComponentSizes 
      + ExtendWith<GH,Error=BetweennessCentralityError>
      + GetEdges
      + GetNeighborsForNode
      + GetNodeIdRange
      + HasMapForNode
      + InsertEdge
      + InsertNode
      + MappedNodes
      + NumEdges
      + NumNodes
{
    let mut tm: Timer = default!();

    let mut edge_stat: EdgeStat = EdgeStat::default();

    debug!("Adding edge {:?}", rand_edge);

    tm.start();

    graph.insert_edge_update_muc(&rand_edge);

    tm.stop();

    edge_stat.edge_ins_time = tm.interval();

    let muc_id: MinimumUnionCycleId = graph.nodeid_to_mucid(rand_edge.src);

    edge_stat.muc_num_edges = graph.muc(muc_id).num_edges();
    edge_stat.muc_num_nodes = graph.muc(muc_id).num_nodes();

    let mut scores = BetweennessScores::empty_mapped("insertion_test_qube_hash_step::scores");

    tm.start();

    if !use_incremental_algo {

        graph.muc_mut(muc_id).compute_bc(
            &mut scores, 
            max_iter
        );

    } else {

        graph.muc_mut(muc_id).compute_bc_inc(
            &mut scores, 
            rand_edge.src, 
            rand_edge.dst, 
            None
        );
    }

    tm.stop();

    edge_stat.muc_bc_update_time = match max_iter {
        Some(max_iter) => {
            let t0 = tm.interval() / max_iter as u32;
            t0 * edge_stat.muc_num_nodes as u32
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

    // estimating ideal speedup using the node and edge proportions
    let mut ideal_speedup: f64 = f64::default();;

    let v:  f64 = graph.num_nodes()             as f64;
    let e:  f64 = graph.num_edges()    as f64;
    let vm: f64 = edge_stat.muc_num_nodes as f64;
    let em: f64 = edge_stat.muc_num_edges as f64;

    ideal_speedup = ((v + e) / (vm + em)) * (v / vm);

    *tot_ideal_speedup += ideal_speedup;

    debug!(
        " MinimumUnionCycle # nodes:   {} ({} G_n)", 
        edge_stat.muc_num_nodes, 
        edge_stat.muc_num_nodes / graph.num_nodes()
    );

    debug!(
        " MinimumUnionCycle # edges:   {} ({} G_M)", 
        edge_stat.muc_num_edges, 
        edge_stat.muc_num_edges / graph.num_edges()
    );

    debug!("");

    debug!(
        " Time to update BC:  {:?}", 
        edge_stat.muc_bc_update_time
    );

    debug!(
        " Speedup:   {} (Ideal: {})", 
        speedup, 
        ideal_speedup
    );

    debug!(
        " QUBE avg iter time:  {:?}", 
        edge_stat.muc_bc_update_time / edge_stat.muc_num_nodes as u32
    );

    debug!(
        " Time to update MinimumUnionCycles:  {:?}", 
        edge_stat.edge_ins_time
    );

    edge_stat_vec.push(edge_stat);

    Ok(())
}

/**
  | TODO change name.. not hash anymore
  |
  | 1. prepare structures to store the results for
  | each edge
  |
  | 2. generate random edges
  |
  | 3. run Brandes and find time
  |
  | 4. run update after each edge insertion and
  | record results
  |
  | 5. print the results progressively
  */
pub fn insertion_test_qube_hash<GH>(
    graph:                &mut Graph<GH>,
    num_edges_to_insert:  Option<usize>,
    rand_edge_vec:        Option<Vec<Edge>>,
    use_incremental_algo: Option<bool>,
    max_iter:             Option<usize>) 
-> Result<(),BetweennessCentralityError> 

where GH
    : NewFromGraphRef<Graph<GH>> 
    + BccGraphHashInterface
    + ClearMucs
    + CreateNamedEmpty
    + ExtendWith<GH,Error=BetweennessCentralityError>
    + GetConnectedComponentSizes
    + GetNeighborsForNode
    + GetNodeIdRange
    + HasMapForNode
    + IsValid
    + MappedNodes
    + InsertNode,

    Graph<GH>
    : FindMucs 
    + GetLimitedNodeIdRange 
    + GetNeighborsForNode 
    + HasEdge 
    + MappedNodes 
    + NumNodes,

    MinimumUnionCycle<GH>
    : GetNodeIdRange 
    + GetNeighborsForNode
    + GetConnectedComponentSizes
    + ExtendWith<GH,Error=BetweennessCentralityError>
{
    let num_edges_to_insert            = num_edges_to_insert.unwrap_or(2);
    let mut rand_edge_vec:   Vec<Edge> = rand_edge_vec.unwrap_or(vec![]);
    let use_incremental_algo:     bool = use_incremental_algo.unwrap_or(false);
    
    graph.print_header();

    let mut g = GH::new_from_graph_ref(&*graph, "insertion_test_qube_hash::graph_hash");

    let mut brandes_time = Duration::from_secs(0);

    let mut muc_find_time = Duration::from_secs(0);

    let mut tm: Timer = default!();

    debug!("Brandes runtime:  ");

    tm.start();

    let mut scores: BetweennessScores = brandes_bc_hash_out(
        graph,
        max_iter
    )?;

    tm.stop();

    brandes_time = match max_iter {
        Some(max_iter) => {
            let t0 = tm.interval() / max_iter as u32;
            t0 * graph.num_nodes() as u32
        }
        None => {
            tm.interval()
        }
    };

    debug!("{:?}", brandes_time);

    debug!(
        "Brandes avg iter time:  {:?}", 
        brandes_time / graph.num_nodes() as u32
    );

    debug!("# MinimumUnionCycles:   ");

    tm.start();

    graph.find_mucs();

    tm.stop();

    muc_find_time = tm.interval();

    debug!("{}", graph.get_num_mucs());

    debug!("Time to find MinimumUnionCycles:  {:?}", muc_find_time);

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

    let mut edge_stat_vec: Vec<EdgeStat> = vec![];

    let mut tot_speedup:       f64 = 0.0;
    let mut tot_ideal_speedup: f64 = 0.0;

    let mut speedup_vec: Vec<f64> = vec![];

    for i in 0..rand_edge_vec.len() {

        let rand_edge: Edge = rand_edge_vec[i].clone();

        insertion_test_qube_hash_step(
            &brandes_time,
            &mut tot_speedup,
            &mut tot_ideal_speedup,
            &mut speedup_vec,
            rand_edge,
            graph, 
            num_edges_to_insert, 
            use_incremental_algo, 
            max_iter,
            &mut edge_stat_vec
        );
    }

    let avg_speedup:       f64 = tot_speedup       / num_edges_to_insert as f64;
    let avg_ideal_speedup: f64 = tot_ideal_speedup / num_edges_to_insert as f64;

    debug!(
        "[[Average speedup: {} (Ideal: {})]]", 
        avg_speedup, 
        avg_ideal_speedup
    );

    // calculating 95% confidence interval
    let stats = SpeedupStats::from(&mut speedup_vec);

    let diff: f64 = 1.96 * stats.stddev / num_edges_to_insert.sqrt() as f64;
    let v1:   f64 = stats.mean - diff;
    let v2:   f64 = stats.mean + diff;

    debug!("[[Speedup 95 CI: {} - {}]]", v1, v2);

    Ok(())
}
