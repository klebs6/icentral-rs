crate::ix!();

pub fn insertion_test_qube_step<GH>(
    brandes_time:        &Duration,
    edge_stat_vec:       &mut Vec<EdgeStat>,
    tm:                  &mut Timer,
    rand_edge:           Edge,
    graph:               &mut Graph<GH>,
    num_edges_to_insert: u32) 
-> Result<(),BetweennessCentralityError> 
where 
    Graph<GH>
    : InsertEdgeUpdateMuc 
    + NodeIdToMucId 
    + GetMuc<GH>,
    MinimumUnionCycle<GH>
    : NumNodes 
    + ExtendWith<GH,Error=BetweennessCentralityError> 
    + GetConnectedComponentSizes 
    + GetEdges 
    + GetNeighborsForNode 
    + GetNodeIdRange 
    + HasMapForNode 
    + InsertEdge 
    + InsertNode 
    + MappedNodes 
    + NumEdges,
    GH
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
    let mut edge_stat: EdgeStat = EdgeStat::default();

    debug!("Adding edge ({}, {})", rand_edge.src, rand_edge.dst);

    tm.start();

    graph.insert_edge_update_muc(&rand_edge);

    tm.stop();

    edge_stat.edge_ins_time = tm.interval();

    debug!(
        "Time to update MinimumUnionCycles: {:?}", 
        edge_stat.edge_ins_time
    );

    let muc_id: MinimumUnionCycleId = graph.nodeid_to_mucid(rand_edge.src);

    edge_stat.muc_num_edges = graph.muc(muc_id).num_edges();

    edge_stat.muc_num_nodes = graph.muc(muc_id).num_nodes();

    let mut scores = BetweennessScores::empty_mapped("insertion_test_qube_step::scores");

    tm.start();

    graph.muc_mut(muc_id).compute_bc(&mut scores, None);

    tm.stop();

    edge_stat.muc_bc_update_time = tm.interval();

    debug!(
        "Time to update BC: {:?}", 
        edge_stat.muc_bc_update_time
    );

    debug!("Speedup: {:?}",                  brandes_time.div_duration_f64(edge_stat.muc_bc_update_time));
    debug!("MinimumUnionCycle # nodes: {}",  edge_stat.muc_num_nodes);
    debug!("MinimumUnionCycle # edges: {}",  edge_stat.muc_num_edges);

    edge_stat_vec.push(edge_stat);

    Ok(())
}

/**
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
pub fn insertion_test_qube<GH>(
        graph:               &mut Graph<GH>,
        num_edges_to_insert: Option<u32>) 
-> Result<(),BetweennessCentralityError> 
where 
    Graph<GH>
    : GetNeighborsForNode 
    + NumNodes 
    + CreateScoresVector 
    + FindMucs,
    GH
    : BccGraphHashInterface
    + GetNeighborsForNode
    + GetConnectedComponentSizes
    + GetNodeIdRange
    + HasMapForNode
    + InsertNode
    + MappedNodes
    + ClearMucs
    + CreateNamedEmpty
    + IsValid
    + ExtendWith<GH,Error=BetweennessCentralityError>,
    MinimumUnionCycle<GH>
    : ExtendWith<MinimumUnionCycle<GH>,Error=BetweennessCentralityError> 
    + ExtendWith<GH,Error=BetweennessCentralityError> 
    + GetConnectedComponentSizes 
    + GetNeighborsForNode
    + GetNodeIdRange
{
    let num_edges_to_insert: u32 = num_edges_to_insert.unwrap_or(1);

    graph.print_header();

    let mut brandes_time  = Duration::from_secs(0);
    let mut muc_find_time = Duration::from_secs(0);

    let mut tm: Timer = Timer::default();

    debug!("Brandes runtime: ");

    tm.start();

    let mut scores = brandes_betweenness_centrality(graph)?;

    tm.stop();

    brandes_time = tm.interval();

    debug!("{:?}", brandes_time);

    debug!("# MinimumUnionCycles: ");

    tm.start();

    graph.find_mucs();

    tm.stop();

    muc_find_time = tm.interval();

    debug!("{}", graph.get_num_mucs());

    debug!("Time to find MinimumUnionCycles: {:?}", muc_find_time);

    let mut rand_edge_vec: Vec<Edge> = default!();

    debug!("Generating edges..n");

    let mut rng = WyRand::default();

    let mut edge_stat_vec: Vec<EdgeStat> = vec![];

    for i in 0..num_edges_to_insert {

        let mut rand_edge: Edge = Edge::default();
        let mut src:     NodeId = NodeId::default();
        let mut dst:     NodeId = NodeId::default();

        loop {

            src = nodeid![rng.generate::<usize>() % graph.num_nodes()];
            dst = nodeid![rng.generate::<usize>() % graph.num_nodes()];

            rand_edge.src = src;

            rand_edge.dst = dst;

            if !graph.has_edge(&rand_edge) {
                break;
            }
        }

        rand_edge_vec.push(rand_edge);
    }

    //==========================================================
    //    node_id_t nd = rand_edge_vec[0].0;
    //    muc_id_t muc_id = graph.nodes_to_mucs[nd];
    //    graph.mucs[muc_id].muc_subgraph.print_graph(false);
    //    return;
    //==========================================================
    for i in 0..rand_edge_vec.len() {

        let rand_edge: Edge = rand_edge_vec[i];

        insertion_test_qube_step(
            &brandes_time,
            &mut edge_stat_vec,
            &mut tm, 
            rand_edge, 
            graph, 
            num_edges_to_insert
        )?;
    }

    let mut total_muc_update_time = Duration::from_secs(0);

    for i in 0..edge_stat_vec.len() {
        total_muc_update_time += edge_stat_vec[i].muc_bc_update_time;
    }

    let avg_speedup: f64 = {
        let t0 = num_edges_to_insert as u32 * brandes_time;
        t0.div_duration_f64(total_muc_update_time)
    };

    debug!("[[Average speedup: {}]]", avg_speedup);


    Ok(())
}
