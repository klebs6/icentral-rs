crate::ix!();

pub fn paper_exp_main<GH>() 
-> Result<(),BetweennessCentralityError>
where GH
: BccGraphHashInterface 
+ ClearMucs 
+ GetSigmaValueForNode
+ HasMapForNode
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
+ InitDebugIteration
+ RemoveEdge
+ ResetWith<GH>
+ HasEdge
+ SetPairDependencyForNode
+ SetSigmaValueForNode
+ CreateNamedEmpty
+ Debug
+ DebugIterationStep
+ GetNodeIdRange
+ ExtendWith<GH,Error=BetweennessCentralityError>
+ FindConnectedComponents<GH,Error=BetweennessCentralityError>
+ FindPruningCounts
+ FindSingleSourceShortestPaths
+ GetConnectedComponentSizes
+ GetNeighborsForNode
, Graph<GH>
: HasEdge
+ MappedNodes
+ GetLimitedNodeIdRange
{
    let mut args = std::env::args();

    let mut num_edges: usize = 0;
    let mut num_iter:  usize = 0;
    let mut rand_seed: i32 = 0;

    let mut t1:        i32 = 0;
    let mut t2:        i32 = 0;
    let mut t3:        i32 = 0;
    let mut t4:        i32 = 0;

    let mut do_inc_brandes: bool = false;
    let mut do_qube:        bool = false;
    let mut do_inc_qube:    bool = false;
    let mut do_fuad:        bool = false;

    let mut max_time_secs = 0;

    let mut path_vec: Vec<String> = vec![];

    if args.len() != 2 {

        debug!("Pass one parameter, path with experiment details");

        debug!("num_edges, num_iter, max_time, rand_seed");

        debug!("do_inc_brandes, do_qube, do_inc_qube, do_fuad");

        debug!("list of graph paths");

        return Ok(());

    } else {

        unsafe {

            let path = args.nth(1).unwrap();

            let mut fin = File::open(&path)
                .unwrap()
                .bytes()
                .map(|ch| ch.unwrap());

            text_io::scan!(fin => "{}, {}, {}, {}", num_edges, num_iter, max_time_secs, rand_seed);
            text_io::scan!(fin => "{}, {}, {}, {}", t1, t2 ,t3, t4);

            do_inc_brandes = (t1 != 0);
            do_qube        = (t2 != 0);
            do_inc_qube    = (t3 != 0);
            do_fuad        = (t4 != 0);

            while let Ok(path) = try_read!("{}", fin) {

                path_vec.push(path);
            }
        }
    }

    do_paper_exp::<GH>(
        num_edges, 
        Some(num_iter), 
        Duration::from_secs(max_time_secs), 
        rand_seed, 
        path_vec, 
        do_inc_brandes, 
        do_qube, 
        do_inc_qube, 
        do_fuad
    );

    Ok(())
}
