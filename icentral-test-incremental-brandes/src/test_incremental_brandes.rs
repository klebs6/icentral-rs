crate::ix!();

pub fn incremental_brandes_test<GH>(
    graph:               Arc<Mutex<Graph<GH>>>,
    num_edges_to_insert: Option<usize>,
    rand_edge_vec:       Option<Vec<Edge>>

) -> Result<(),BetweennessCentralityError> 
where GH: BccGraphHashInterface ,
      Graph<GH>: NumNodes + HasEdge + GetNeighborsForNode + CreateScoresVector + GetLimitedNodeIdRange,
{
    let num_edges_to_insert: usize     = num_edges_to_insert.unwrap_or(1);
    let mut rand_edge_vec:   Vec<Edge> = rand_edge_vec.unwrap_or(vec![]);

    if let graph = graph.lock()? {
        graph.print_header();
    }

    let mut brandes_time = Duration::from_secs(0);

    let mut tm: Timer = default!();

    debug!("Brandes avg time per iteration:  ");

    tm.start();

    let mut scores = BetweennessScores::empty_mapped("incremental_brandes_test::scores");

    if let mut graph = graph.lock()? {

        scores = brandes_bc(&mut *graph,None)?;

        tm.stop();

        brandes_time = tm.interval();

        debug!("{:?}", brandes_time / graph.num_nodes() as u32);

        debug!("Brandes time: {:?}", brandes_time);

        if rand_edge_vec.len() == 0 {

            debug!("Generating edges...");

            let mut rng = WyRand::default();

            rand_edge_vec = gen_rand_edges(
                &mut rng,
                num_edges_to_insert, 
                &mut *graph
            )?;
        }
    }

    let mut edge_stat_vec: Vec<EdgeStat> = vec![];
    let mut cnt_vec2:      Vec<Arc<Mutex<Vec<usize>>>>    = vec![];
    let mut time_vec2:     Vec<Arc<Mutex<Vec<Duration>>>> = vec![];

    for i in 0..rand_edge_vec.len() {

        // debug!("Inserting edge [{}]...", i);
        let rand_edge: Edge = rand_edge_vec[i];;

        let mut cnt_vec:  Arc<Mutex<Vec<usize>>>    = arcmut![vec![ default![]; 3]];
        let mut time_vec: Arc<Mutex<Vec<Duration>>> = arcmut![vec![ default![]; 3]];

        incremental_brandes_experimental(
            graph.clone(), 
            rand_edge.src, 
            rand_edge.dst, 
            &mut scores, 
            time_vec.clone(), 
            cnt_vec.clone()
        );

        cnt_vec2.push(cnt_vec);
        time_vec2.push(time_vec);
    }

    let mut tot_cnt_arr: Vec<usize> = vec![0,0,0];
    let mut tot_time_arr = vec![Duration::from_secs(0); 3];
    let mut tot_avg_arr  = vec![Duration::from_secs(0); 3];

    for i in 0..cnt_vec2.len() {

        if let (time_vec2i, cnt_vec2i) = (time_vec2[i].lock()?, cnt_vec2[i].lock()?) {

            tot_cnt_arr[0]  += cnt_vec2i[0];
            tot_cnt_arr[1]  += cnt_vec2i[1];
            tot_cnt_arr[2]  += cnt_vec2i[2];
            tot_time_arr[0] += time_vec2i[0];
            tot_time_arr[1] += time_vec2i[1];
            tot_time_arr[2] += time_vec2i[2];
            tot_avg_arr[0]  += time_vec2i[0] / cnt_vec2i[0] as u32;
            tot_avg_arr[1]  += time_vec2i[1] / cnt_vec2i[1] as u32;
            tot_avg_arr[2]  += time_vec2i[2] / cnt_vec2i[2] as u32;
        }
    }

    let inc_tot_time: Duration = {
        tot_time_arr[0] + tot_time_arr[1] + tot_time_arr[2]
    };

    let inc_avg_time: Duration = {
        inc_tot_time / rand_edge_vec.len() as u32
    };

    debug!("Incremental Brandes time:  {:?}", inc_avg_time);

    debug!("");

    let avg_d1_tm: Duration = tot_time_arr[1] / tot_cnt_arr[1] as u32;
    let avg_d2_tm: Duration = tot_time_arr[2] / tot_cnt_arr[2] as u32;
    let avg_br_tm: Duration = brandes_time    / graph.lock()?.num_nodes() as u32;

    debug!("");

    debug!("Tot d0 BFS:  {}", tot_cnt_arr[0]);
    debug!("Tot d1 BFS:  {}", tot_cnt_arr[1]);
    debug!("Tot d2 BFS:  {}", tot_cnt_arr[2]);

    debug!("");

    debug!("Avg d1 BFS time:  {:?} ({})", avg_d1_tm, avg_d1_tm.div_duration_f64(avg_br_tm));
    debug!("Avg d2 BFS time:  {:?} ({})", avg_d2_tm, avg_d2_tm.div_duration_f64(avg_br_tm));

    debug!("Avg Brandes time:  {:?}", avg_br_tm);

    let avg_inc_iter: Duration = {
        let t0 = tot_time_arr[1] + tot_time_arr[2];
        let t1 = tot_cnt_arr[1]  + tot_cnt_arr[2];
        t0 / t1 as u32
    };

    let mem_dag_time: Duration = {
        let t0 = avg_d1_tm - avg_br_tm;
        let t1 = avg_d2_tm - avg_br_tm;
        let t2 = tot_cnt_arr[1] as u32;
        let t3 = tot_cnt_arr[2] as u32;
        t0 * t2 +  t1 * t3
    };

    let mem_dag_speedup: f64 = brandes_time.div_duration_f64(mem_dag_time);

    debug!("Ideal mem DAG speedup:  {}",          mem_dag_speedup);
    debug!("Avg inc BFS time:       {:?} ({:?})", avg_inc_iter, avg_inc_iter.div_duration_f64(avg_br_tm));
    debug!("[[Avg speedup:          {}]]",        brandes_time.div_duration_f64(inc_avg_time));

    Ok(())
}
