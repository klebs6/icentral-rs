crate::ix!();

pub fn timing_update_bc_mem<GH>(
    paths_of_the_graphs: Vec<String>,
    num_random_edges:    usize,
    rand_seed:           i32,
    num_sources:         usize,
    algorithms_to_eval:  i32) 

-> Result<(),BetweennessCentralityError>  
where Graph<GH>: HasEdge + FromFilename + MappedNodes + GetNodeIdRange + GetLimitedNodeIdRange
{
    let mut rng = WyRand::new_seed(rand_seed.try_into()?);
    
    let mut edge_vec2: Vec<Vec<Edge>> = vec![];

    // generate random edges
    //
    for parents in 0..paths_of_the_graphs.len() {

        let mut graph = Graph::<GH>::from_filename(&paths_of_the_graphs[parents]);

        let mut edge_vec: Vec<Edge> = vec![];

        edge_vec = gen_rand_edges(
            &mut rng,
            num_random_edges, 
            &mut graph
        )?;

        edge_vec2.push(edge_vec);
    }

    for parents in 0..paths_of_the_graphs.len() {

        let mut graph = Graph::from_filename(&paths_of_the_graphs[parents]);

        timing_update_bc_mem_graph(
            &mut graph, 
            &mut edge_vec2[parents], 
            -1, 
            1,
            None,
            None
        );
    }

    Ok(())
}
