crate::ix!();

pub fn brandes_bc_hash<GH: SpawnScores + MappedNodes + BrandesIterUpdatePairDependenciesAndFill + BrandesIterUpdateDistancesAndPathForNeighbors + BrandesIterInit>(
    graph:    &mut GH,
    max_iter: Option<usize>

) -> Result<BetweennessScores,BetweennessCentralityError> {

    let mut scores = graph.spawn_scores();

    let mut cnt: usize = 0;

    let ids: Vec<NodeId> = graph.mapped_nodes();

    for id in ids {

        cnt += 1;

        brandes_iter_with_graphhash(
            graph, 
            id, 
            &mut scores
        );

        if let Some(max_iter) = max_iter {

            if cnt == max_iter {
                break;
            }
        }
    }

    if max_iter.is_none() {
        scores.halve();
    }

    Ok(scores)
}

// increments bc values in @scores, with the pair
// dependency of @s
//
pub fn brandes_iter_with_graphhash<GH>(
    graph:  &mut GH,
    s:      NodeId,
    scores: &mut BetweennessScores

) -> Result<(),BetweennessCentralityError> 

where GH: BrandesIterInit 
        + BrandesIterUpdateDistancesAndPathForNeighbors 
        + BrandesIterUpdatePairDependenciesAndFill
{

    graph.brandes_iter_init(s);

    graph.brandes_iter_update_dist_and_path_for_neighbors(s);

    graph.brandes_iter_update_pair_dependencies_and_fill(
        s, 
        scores
    )?;

    Ok(())
}
