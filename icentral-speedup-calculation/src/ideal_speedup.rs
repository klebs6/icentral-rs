crate::ix!();

pub fn fuad_ideal_speedup(
    iter_frac:     f64,
    bcc_n:         f64,
    bcc_m:         f64,
    g_n:           f64,
    g_m:           f64,
    inc_bfs_ratio: f64) -> f64 {
    
    let bcc_s: f64 = bcc_n + bcc_m;

    let g_s: f64 = g_n + g_m;

    let mut res: f64 = {
        let x0 = bcc_n / g_n;
        let x1 = bcc_s / g_s;
        let t0 = x0 * iter_frac * x1;
        1.0 / t0
    };

    res = res * 100.0 / inc_bfs_ratio;

    res
}

pub fn time_find_mucs<G: FindMucs + GetNumMucs>(graph: &mut G)
-> Result<Duration,BetweennessCentralityError> 
{
    let mut tm: Timer = Timer::default();

    tm.start();

    graph.find_mucs();

    tm.stop();

    let muc_find_time: Duration = tm.interval();

    debug!("# MinimumUnionCycles: {}", graph.get_num_mucs());

    debug!("Time to find MinimumUnionCycles:  {:?}", muc_find_time);

    Ok(muc_find_time)
}

pub fn generate_random_edges<G>(
    graph:               &G,
    rand_edge_vec:       Option<Vec<Edge>>, 
    num_edges_to_insert: usize)
-> Result<Vec<Edge>,BetweennessCentralityError>
where G: HasEdge + NumNodes
{
    let mut rand_edge_vec 
    = rand_edge_vec.unwrap_or(vec![]);

    if rand_edge_vec.len() == 0 {

        let mut rng = WyRand::default();

        rand_edge_vec = gen_rand_edges(
            &mut rng,
            num_edges_to_insert, 
            graph
        )?;
    }

    Ok(rand_edge_vec)
}

pub fn qube_ideal_speedup_step<GH>(
    edge_stat_vec:       &mut Vec<EdgeStat>,
    tm:                  &mut Timer,
    tot_ideal_speedup:   &mut f64,
    rand_edge:           Edge,
    graph:               &mut Graph<GH>,
    num_edges_to_insert: usize)
-> Result<(),BetweennessCentralityError>
where Graph<GH>
    : InsertEdgeUpdateMuc 
    + NodeIdToMucId 
    + GetMuc<GH> 
    + NumNodes 
    + NumEdges,
MinimumUnionCycle<GH>
    : NumEdges 
    + NumNodes
{
    let mut edge_stat: EdgeStat = EdgeStat::default();

    debug!("Adding edge {:?}", rand_edge);

    tm.start();

    graph.insert_edge_update_muc(&rand_edge);

    tm.stop();

    edge_stat.edge_ins_time = tm.interval();

    let muc_id: MinimumUnionCycleId = graph.nodeid_to_mucid(rand_edge.src);

    edge_stat.muc_num_edges = graph.muc(muc_id).num_edges();
    edge_stat.muc_num_nodes = graph.muc(muc_id).num_nodes();

    // estimating ideal speedup using the node and edge proportions
    let mut ideal_speedup: f64 = 0.0;

    let v:  f64 = graph.num_nodes() as f64;
    let e:  f64 = graph.num_edges() as f64;
    let vm: f64 = edge_stat.muc_num_nodes as f64;
    let em: f64 = edge_stat.muc_num_edges as f64;

    ideal_speedup = ((v + e) / (vm + em)) * (v / vm);

    *tot_ideal_speedup += ideal_speedup;

    let muc_speedup_stats = MucSpeedupStats {
        muc_num_nodes:    edge_stat.muc_num_nodes,
        muc_num_edges:    edge_stat.muc_num_edges,
        graph_len:        graph.num_nodes(),
        graph_num_edges:  graph.num_edges(),
        ideal_speedup,
        edge_ins_time:    edge_stat.edge_ins_time,
    };

    info!("{:?}", muc_speedup_stats);

    edge_stat_vec.push(edge_stat);

    Ok(())
}

pub fn qube_ideal_speedup<GH>(
    graph:               &mut Graph<GH>,
    num_edges_to_insert: usize,
    rand_edge_vec:       Option<Vec<Edge>>

) -> Result<(),BetweennessCentralityError>

where Graph<GH>
    : PrintHeader 
    + FindMucs 
    + GetNumMucs 
    + InsertEdgeUpdateMuc 
    + NumNodes 
    + NumEdges 
    + HasEdge 
    + NodeIdToMucId,
GH
    : NewFromGraphRef<Graph<GH>> 
    + ExtendWith<GH> 
    + NumNodes 
    + NumEdges 
    + GetEdges 
    + HasMapForNode 
    + MappedNodes 
    + InsertEdge 
    + InsertNode
{
    let mut g = GH::new_from_graph_ref(&*graph, "qube_ideal_speedup::g");

    graph.print_header();

    time_find_mucs(graph)?;

    debug!("Generating edges...");

    let rand_edge_vec = generate_random_edges(
        graph,
        rand_edge_vec,
        num_edges_to_insert
    )?;

    let mut edge_stat_vec: Vec<EdgeStat> = vec![];

    let mut tot_ideal_speedup: f64 = 0.0;

    let mut tm: Timer = Timer::default();

    for idx in 0..rand_edge_vec.len() {

        let rand_edge: Edge = rand_edge_vec[idx];

        qube_ideal_speedup_step(
            &mut edge_stat_vec,
            &mut tm,
            &mut tot_ideal_speedup,
            rand_edge, 
            graph, 
            num_edges_to_insert
        )?;
    }

    let avg_ideal_speedup: f64 = tot_ideal_speedup / num_edges_to_insert as f64;

    debug!("[[Average speedup (ideal): {}]]", avg_ideal_speedup);

    Ok(())
}
