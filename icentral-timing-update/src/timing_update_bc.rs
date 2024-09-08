crate::ix!();

pub fn generate_random_edges_for_graphs_from_files<GH>(
    rand_seed:           i32,
    num_random_edges:    usize, 
    paths_of_the_graphs: Vec<String>)
-> Result<Vec<(Graph<GH>,Vec<Edge>)>,BetweennessCentralityError>
where Graph<GH>: HasEdge,
      GH: BccGraphHashInterface
{
    let mut rng = WyRand::new_seed(rand_seed.try_into()?);

    let mut work_items: Vec<(Graph<GH>,Vec<Edge>)> = vec![];

    // generate random edges
    //
    for parents in 0..paths_of_the_graphs.len() {

        let graph = Graph::from_filename(&paths_of_the_graphs[parents]);

        let edge_vec = gen_rand_edges(
            &mut rng,
            num_random_edges, 
            &graph
        )?;

        work_items.push((graph,edge_vec));
    }

    Ok(work_items)
}

/**
  | Does timing test for graphs in @paths_of_the_graphs
  |
  */
pub fn timing_update_bc<GH>(
    paths_of_the_graphs:     Vec<String>,
    num_random_edges:        usize,
    rand_seed:               i32,
    num_sources:             usize,
    algorithms_to_evaluate:  Option<CompType>,
    num_threads:             Option<usize>,
    op:                      Option<Operation>)
-> Result<(), BetweennessCentralityError> 

where Graph<GH>
      : HasEdge,

      GH
      : BccGraphHashInterface
{
    let algorithms_to_evaluate:  CompType = algorithms_to_evaluate.unwrap_or(CompType::BiconnectedComponent);
    let op:                     Operation = op.unwrap_or(Operation::Insertion);

    let mut work_items = generate_random_edges_for_graphs_from_files(
        rand_seed,
        num_random_edges,
        paths_of_the_graphs
    )?;

    for (graph,edge_vec) in work_items.iter_mut() {

        let config = TimingUpdateConfig {
            limit_sources: Some(num_sources),
            do_brandes:    false,
            num_threads:   num_threads.unwrap_or(1),
            del_edge:      false,
            op,
        };

        timing_update_bc_graph(
            graph, 
            edge_vec, 
            &algorithms_to_evaluate, 
            Some(Duration::from_secs(1)), 
            config
        );
    }

    Ok(())
}
