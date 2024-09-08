crate::ix!();

/**
  | generates @num_edges random edges
  | that are not in @graph
  |
  */
pub fn gen_rand_edges<G: NumNodes + HasEdge>(
    rng:       &mut WyRand,
    num_edges: usize,
    graph:     &G) 
-> Result<Vec<Edge>,BetweennessCentralityError> 
{
    let mut res = vec![];

    let graph_len = graph.num_nodes();

    for i in 0..num_edges {

        let mut rand_edge: Edge = Edge::default();

        loop {

            rand_edge.src = NodeId::from(rng.generate::<usize>() % graph_len);
            rand_edge.dst = NodeId::from(rng.generate::<usize>() % graph_len);

            if !graph.has_edge(&rand_edge) || res.contains(&rand_edge)
            {
                break;
            }
        }

        res.push(rand_edge);
    }

    Ok(res)
}

/**
  | generates @num_edges random edges
  | that are in @graph, but are not bridges
  |
  */
pub fn gen_rand_edges_deletions<G: FindBridgeEdges + NumEdges + GetEdges, GH: BccGraphHashInterface>(
    num_edges: usize,
    graph:     &mut G)
-> Vec<Edge> 
{
    let mut bridges_vec = graph.find_bridge_edges::<GH>();

    // NOTE:insert each edge twice -- e(v1, v2) e(v2, v1)
    let num_bridges: usize = bridges_vec.len();

    for i in 0..num_bridges {

        bridges_vec.push(
            Edge::new(bridges_vec[i].dst, bridges_vec[i].src)
        );
    }

    let mut res = vec![];

    let num_edges_graph: usize = graph.num_edges();

    let mut rng = WyRand::default();

    for i in 0..num_edges {

        let mut rand_edge: Edge = Edge::default();

        loop {

            // select an edge at random
            let idx = rng.generate::<usize>() % num_edges_graph;

            rand_edge = *graph.edges().iter().nth(idx).unwrap();

            if bridges_vec.iter().find(|x| **x == rand_edge).is_none() {
                break;
            }
        }

        res.push(rand_edge);
    }

    res
}
