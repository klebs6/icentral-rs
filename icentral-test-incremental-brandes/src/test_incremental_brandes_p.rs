crate::ix!();

pub fn exp_inc_brandes_p<GH>(
        graph:         Arc<Mutex<Graph<GH>>>,
        num_iter:      Option<usize>,
        rand_edge_vec: &Vec<Edge>,
        brandes_time:  Duration) 
-> Result<(),BetweennessCentralityError>
where GH: BccGraphHashInterface,
      //Graph<GH>: NumNodes
{
    if let graph = graph.lock()? {
        graph.print_header();
    }

    let mut tm: Timer = Timer::default();

    let mut edge_stat_vec: Vec<EdgeStat> = vec![];

    let mut scores = graph.lock()?.create_scores_vector();

    let mut cnt_vec2:  Vec<Arc<Mutex<Vec<usize>>>> = vec![];
    let mut time_vec2: Vec<Arc<Mutex<Vec<Duration>>>> = vec![];

    for i in 0..rand_edge_vec.len() {

        // debug!("Inserting edge [{}]...", i);
        let rand_edge: Edge = rand_edge_vec[i];;

        let mut cnt_vec:  Arc<Mutex<Vec<usize>>>      = default![];
        let mut time_vec: Arc<Mutex<Vec<Duration>>> = default![];

        cnt_vec.lock()?.resize(3,0);

        time_vec.lock()?.resize(3,Duration::from_secs(0));

        incremental_brandes_experimental(
            graph.clone(), 
            rand_edge.src, 
            rand_edge.dst, 
            &mut scores, 
            time_vec.clone(), 
            cnt_vec.clone()
        )?;

        cnt_vec2.push(cnt_vec);

        time_vec2.push(time_vec);
    }

    let mut tot_cnt_arr:  Vec<usize>      = vec!{0,0,0};
    let mut tot_time_arr: Vec<Duration> = vec![Duration::from_secs(0); 3];

    for i in 0..cnt_vec2.len() {

        if let (time_vec2i,cnt_vec2i) 
            = (time_vec2[i].lock()?, cnt_vec2[i].lock()?) 
        {
            tot_cnt_arr[0]  += cnt_vec2i[0];
            tot_cnt_arr[1]  += cnt_vec2i[1];
            tot_cnt_arr[2]  += cnt_vec2i[2];
            tot_time_arr[0] += time_vec2i[0];
            tot_time_arr[1] += time_vec2i[1];
            tot_time_arr[2] += time_vec2i[2];
        }
    }

    let mut tot_speedup: f64 = 0.0;

    for i in 0..cnt_vec2.len() {

        if let time_vec2i = time_vec2[i].lock()? {

            let inc_br_time: Duration = time_vec2i[0] + time_vec2i[1] + time_vec2i[2];

            tot_speedup += brandes_time.div_duration_f64(inc_br_time);
        }
    }

    let inc_tot_time: Duration = tot_time_arr[0] + tot_time_arr[1] + tot_time_arr[2];

    let inc_avg_time: Duration = inc_tot_time / rand_edge_vec.len() as u32;

    debug!("Incremental-Brandes avg. runtime: {:?}", inc_avg_time);

    let avg_d1_tm: Duration = tot_time_arr[1] / tot_cnt_arr[1] as u32;
    let avg_d2_tm: Duration = tot_time_arr[2] / tot_cnt_arr[2] as u32;
    let avg_br_tm: Duration = brandes_time    / graph.lock()?.num_nodes() as u32;

    // the prints below is for the total iterations for all the inserted edges
    // they don't make sense if not reported per edge
    //     debug!("Tot d0 BFS:    {}/{}", tot_cnt_arr[0], graph.len());
    //     debug!("Tot d1 BFS:    {}/{}", tot_cnt_arr[1], graph.len());
    //     debug!("Tot d2 BFS:    {}/{}", tot_cnt_arr[2], graph.len());
    debug!(
        "Avg d1 BFS time:   {:?} ({:?})", 
        avg_d1_tm, 
        avg_d1_tm.div_duration_f64(avg_br_tm)
    );

    debug!(
        "Avg d2 BFS time:   {:?} ({:?})", 
        avg_d2_tm, 
        avg_d2_tm.div_duration_f64(avg_br_tm)
    );

    debug!(
        "[[Avg. speedup (inc_brandes): {:?}]]", 
        brandes_time.div_duration_f64(inc_avg_time)
    );

    debug!(
        "[[Avg. speedup (inc_brandes)(proper): {:?}]]", 
        tot_speedup / rand_edge_vec.len() as f64
    );

    Ok(())
}
