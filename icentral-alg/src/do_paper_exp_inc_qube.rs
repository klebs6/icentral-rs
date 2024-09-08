crate::ix!();

pub fn maybe_do_paper_exp_inc_qube<GH>(
    brandes_time_vec: &Vec<Duration>,
    edge_vec2:        &Vec<Vec<Edge>>,
    num_iter:         Option<usize>,
    max_time:         Duration,
    path_vec:         &Vec<String>,
    do_qube:          bool) 
-> Result<(),BetweennessCentralityError> 
where 
Graph<GH>
    : GetLimitedNodeIdRange 
    + MappedNodes,
GH
    : ClearMucs
    + CreateNamedEmpty
    + HasMapForNode
    + Debug
    + InsertNode
    + MappedNodes
    + BccGraphHashInterface
    + InsertEdgeUpdateMuc
    + RemoveBridges
    + NewFromGraphRef<Graph<GH>>
    + GetNeighborsForNode
    + NewFromCycleVec
    + FindConnectedComponents<GH,Error=BetweennessCentralityError>
    + GetNodeIdRange
    + IsValid
    + GetConnectedComponentSizes
    + ExtendWith<GH,Error=BetweennessCentralityError>
{
    if do_qube {

        info!("Starting QUBE");

        for i in 0..path_vec.len() {

            let mut graph = Graph::from_filename(&path_vec[i]);

            let brandes_time = brandes_time_vec[i];

            let mut loc_num_iter: Option<usize> = num_iter;

            if num_iter.is_some() && brandes_time < max_time {
                loc_num_iter = None;
            }

            exp_qube_p(
                &mut graph,
                loc_num_iter, 
                &edge_vec2[i], 
                brandes_time
            );
        }
    }

    Ok(())
}

