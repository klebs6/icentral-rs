crate::ix!();

/**
  | Parallel Brandes functions
  | 
  | Ziyad territory
  |
  */
pub fn parallel_brandes<G>(
    graph:  &G,
    scores: &mut BetweennessScores

) -> Result<(),BetweennessCentralityError> 

where G
: NumNodes 
+ MappedNodes 
+ GetNodeIdRange 
+ GetNeighborsForNode 
+ GetEdges
{

    // 1. make a component (fill a subgraph)
    // 2. loop through all sources
    let num_nodes = graph.num_nodes();

    scores.reinit(num_nodes);

    let mut component: Component = Component::new_from_graph_ref(
        graph, 
        "parallel_brandes::component"
    );

    // we want to do brandes_iter from each node
    // in the graph each thread will be
    // responsible for doing brandes_iter from
    // a subset of the nodes in the graph
    //
    // 1. divide the nodes in the graph to vectors,
    //   each has a subset of nodes a thread is
    //   responsible for
    //
    // 2. call brandes_block to get the
    //   contribution of the thread's nodes to the
    //   final BC value (fork and join)
    //
    // 3. accumulate the results in scores

    Ok(())
}
