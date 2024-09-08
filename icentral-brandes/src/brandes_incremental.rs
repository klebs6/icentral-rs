crate::ix!();

pub fn incremental_brandes<G>(
    graph:    Arc<Mutex<G>>,
    mut src:  NodeId,
    mut dst:  NodeId,
    scores:   &mut BetweennessScores
) -> Result<(), BetweennessCentralityError> 

where G: NumNodes + VisitMarkersHandle + ResetVisitMarkersAndVisitNode + GetNeighborsForNode + Debug + FindSingleSourceShortestPaths
{
    let mut src_distances = graph.lock()?.find_single_source_shortest_paths(src)?;
    let mut dst_distances = graph.lock()?.find_single_source_shortest_paths(dst)?;

    let num_nodes = graph.lock()?.num_nodes();

    // fill(scores.begin(), scores.end(), 0);
    for source in NodeIdRange::new(0,num_nodes) {

        if src_distances.distance(source) != dst_distances.distance(source) {

            brandes_delta_iter(
                graph.clone(), 
                source, 
                &mut src, 
                &mut dst, 
                src_distances.distance_mut(source), 
                dst_distances.distance_mut(source), 
                scores
            );
        }
    }

    Ok(())
}

pub fn incremental_brandes_experimental<G>(
    graph:    Arc<Mutex<G>>,
    mut src:  NodeId,
    mut dst:  NodeId,
    scores:   &mut BetweennessScores,
    time_vec: Arc<Mutex<Vec<Duration>>>,
    cnt_vec:  Arc<Mutex<Vec<usize>>>) 

-> Result<(),BetweennessCentralityError> 
where G: Debug 
       + VisitMarkersHandle 
       + NumNodes 
       + ResetVisitMarkersAndVisitNode 
       + GetNeighborsForNode 
       + FindSingleSourceShortestPaths
{

    let mut src_distances = graph.lock()?.find_single_source_shortest_paths(src)?;
    let mut dst_distances = graph.lock()?.find_single_source_shortest_paths(dst)?;

    let num_nodes = graph.lock()?.num_nodes();

    // fill(scores.begin(), scores.end(), 0);
    let mut cnt_arr: Vec<usize>    = vec!{0,0,0};
    let mut tot_arr: Vec<Duration> = vec![Duration::default(); 3];

    for source in NodeIdRange::new(0,num_nodes) {

        let mut tm: Timer = Timer::default();

        tm.start();

        if src_distances.distance(source) != dst_distances.distance(source) {

            brandes_delta_iter(
                graph.clone(), 
                source, 
                &mut src, 
                &mut dst, 
                src_distances.distance_mut(source), 
                dst_distances.distance_mut(source), 
                scores
            );
        }

        tm.stop();

        let diff:     f64 = src_distances.distance(source) - dst_distances.distance(source);
        let abs_diff: f64 = diff.abs();

        match abs_diff {
            0.0  => {
                cnt_arr[0] += 1;
                tot_arr[0] += tm.interval();
            },

            1.0  => {
                cnt_arr[1] += 1;
                tot_arr[1] += tm.interval();
            },

            _  => {
                cnt_arr[2] += 1;
                tot_arr[2] += tm.interval();
            },
        }
    }

    cnt_vec.lock()?.splice(0..0,cnt_arr);
    time_vec.lock()?.splice(0..0,tot_arr);

    Ok(())
}
