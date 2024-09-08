crate::ix!();
   
pub fn timing_update_bc_mem_graph<GH>(
    graph:        &mut Graph<GH>,
    edge_vec:     &mut Vec<Edge>,
    num_sources:  i32,
    algo_flag:    i32,
    do_brandes:   Option<bool>,
    brandes_time: Option<f64>) 
-> Result<(),BetweennessCentralityError> 
where Graph<GH>: MappedNodes + GetNodeIdRange + CreateScoresVector + GetLimitedNodeIdRange
{
    let mut brandes_time = Duration::default();

    let do_brandes:      bool = do_brandes.unwrap_or(true);

    let mut tm: Timer = Timer::default();

    let mut scores = graph.create_scores_vector();
    let mut tm_vec:      Vec<Duration> = vec![];
    let mut speedup_vec: Vec<f64>      = vec![];

    if do_brandes {

        tm.start();

        // fast_brandes_BC(graph, scores);
        scores = brandes_bc(graph,None)?;

        tm.stop();

        brandes_time = tm.interval();
    }

    let num_nodes = graph.num_nodes();

    let mut component  = Component::new_from_graph_ref(&*graph, "timing_update_bc_mem_graph::component");

    let mut workspaces = WorkspaceMap::new(num_nodes, "timing_update_bc_mem_graph::workspaces");

    workspaces.bbfs(None, &component);

    graph.print_header();

    debug!("Brandes_tm[{:.2?}]", brandes_time);

    let mut bc_mem_workspace = BcMemWorkspace::empty("timing_update_bc_mem_graph::bc_mem_workspace");

    for i in 0..edge_vec.len() {

        let e: Edge = edge_vec[i];

        tm.start();

        update_bc_mem(
            &mut bc_mem_workspace,
            &mut scores, 
            graph, 
            e, 
            &mut workspaces
        );

        tm.stop();

        let e_time = tm.interval();

        tm_vec.push(e_time);

        let e_speedup: f64 = brandes_time.div_duration_f64(e_time);

        speedup_vec.push(e_speedup);

        debug!(
            "e({:.6},{:.6})  tm[{:.2?}]  sup[{:.2}]", 
            e.src, 
            e.dst, 
            e_time, 
            e_speedup
        );
    }

    let tm_stats      = SimpleStats::from(&mut tm_vec);
    let speedup_stats = SpeedupStats::from(&mut speedup_vec);

    debug!(
        "Avg.tm[{:.2?}]  Avg.sup[{:.2?}]", 
        tm_stats.mean, 
        speedup_stats.mean
    );

    Ok(())
}
