crate::ix!();

pub fn do_paper_exp<GH>(
    num_edges:      usize,
    num_iter:       Option<usize>,
    max_time:       Duration,
    rand_seed:      i32,
    path_vec:       Vec<String>,
    do_inc_brandes: bool,
    do_qube:        bool,
    do_inc_qube:    bool,
    do_fuad:        bool) 
-> Result<(),BetweennessCentralityError> 
where GH
: BccGraphHashInterface 
+ ClearMucs
+ Debug
+ CreateNamedEmpty
+ DebugIterationStep
+ ExtendWith<GH,Error=BetweennessCentralityError>
+ FindConnectedComponents<GH,Error=BetweennessCentralityError>
+ FindPruningCounts
+ FindSingleSourceShortestPaths
+ GetConnectedComponentSizes
+ GetNeighborsForNode
+ GetNodeIdRange
+ GetSigmaValueForNode
+ GraphHashMucInterface
+ HasEdge
+ HasMapForNode
+ InitDebugIteration
+ InsertNode
+ InsertEdgeUpdateMuc
+ IsValid
+ MappedNodes
+ NewFromCycleVec
+ NewFromGraphRef<Graph<GH>>
+ PairDependencyForNode
+ ParentsForNode
+ PathCountForNode
+ RemoveBridges
+ RemoveEdge
+ ResetWith<GH>
+ SetPairDependencyForNode
+ SetSigmaValueForNode
, Graph<GH>
: HasEdge 
+ MappedNodes 
+ GetLimitedNodeIdRange
{
    // to make sure the exact same edges are used
    // for my system and QUBE I pass the same
    // edges to both systems
    //
    let mut brandes_time_vec: Vec<Duration>  = vec![];
    let mut edge_vec2:        Vec<Vec<Edge>> = vec![];

    info!("Reading graphs and generating edges");

    let mut rng = WyRand::default();

    for i in 0..path_vec.len() {

        let mut graph = Graph::<GH>::empty(&extract_graph_name(&path_vec[i]));

        let path: String = path_vec[i].to_string();

        graph.read_graph(&path);

        let mut edge_vec: Vec<Edge> = vec![];

        edge_vec = gen_rand_edges(
            &mut rng, 
            num_edges, 
            &graph
        )?;

        edge_vec2.push(edge_vec);
    }

    info!("Starting Brandes");

    for i in 0..path_vec.len() {

        let mut graph = Graph::from_filename(&path_vec[i]);

        let mut brandes_time = Duration::from_secs(0);

        brandes_time = exp_brandes_p(
            &graph,
            num_iter,
            Some(max_time)
        )?;

        brandes_time_vec.push(brandes_time);
    }

    maybe_do_paper_exp_inc_brandes::<GH>(
        &brandes_time_vec,
        &edge_vec2,
        num_iter, 
        &path_vec, 
        do_inc_brandes, 
    )?;

    maybe_do_paper_exp_inc_qube(
        &brandes_time_vec,
        &edge_vec2,
        num_iter, 
        max_time, 
        &path_vec, 
        do_qube, 
    )?;

    if do_inc_qube {

        info!("Starting Incremental QUBE");

        for i in 0..path_vec.len() {

            let mut graph = Graph::from_filename(&path_vec[i]);

            let brandes_time: Duration = brandes_time_vec[i];

            let mut loc_num_iter: Option<usize> = num_iter;

            if num_iter.is_some() && brandes_time < max_time {
                loc_num_iter = None;
            }

            exp_inc_qube_p(
                &mut graph, 
                loc_num_iter, 
                &edge_vec2[i], 
                brandes_time
            )?;
        }
    }

    if do_fuad {

        info!("Starting FUAD");

        for i in 0..path_vec.len() {

            let mut graph = Graph::from_filename(&path_vec[i]);

            let brandes_time: Duration = brandes_time_vec[i];

            let mut loc_num_iter: Option<usize> = num_iter;

            if num_iter.is_some() && brandes_time < max_time {
                loc_num_iter = None;
            }

            exp_fuad_p(
                &mut graph, 
                loc_num_iter, 
                &edge_vec2[i], 
                brandes_time
            );
        }
    }

    Ok(())
}
