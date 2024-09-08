crate::ix!();

use lib_dachshund::dachshund::simple_undirected_graph::SimpleUndirectedGraph;
use lib_dachshund::dachshund::graph_builder_base::GraphBuilderBase;
use lib_dachshund::dachshund::error::CLQResult;
use lib_dachshund::dachshund::simple_undirected_graph_builder::SimpleUndirectedGraphBuilder;
use lib_dachshund::dachshund::graph_base::GraphBase;
use lib_dachshund::dachshund::algorithms::betweenness::Betweenness;

#[test] fn test_golden_paper_betweenness() {

    setup_test_logger![];

    golden_betweenness_scores_for_paper_via_dachshund();

    /*
    {
        NodeId { id: 1 }: 1.5, 
        NodeId { id: 2 }: 1.5, 
        NodeId { id: 3 }: 12.5
        NodeId { id: 4 }: 11.5, 
        NodeId { id: 5 }: 0.0, 
        NodeId { id: 6 }: 6.0, 
        NodeId { id: 7 }: 0.0, 
        NodeId { id: 8 }: 0.0, 
    }
    */
}

#[test] fn test_betweenness_match_dachshund() 
-> Result<(), BetweennessCentralityError> 
{
    setup_test_logger![];

    let n_tests = 100;

    let mut rng = WyRand::new();

    for test_idx in 0..n_tests {

        info!("----------------------[running test #{}]", test_idx);

        debug!("testing betweenness_match_dachshund, iter {}", test_idx);

        let mut timer = Timer::default();

        timer.start();

        let n_vert  = rng.generate_range(10..1000);
        let n_edges = rng.generate_range(n_vert..30000);

        let graph = Graph::random_connected(n_vert, n_edges);

        let scores: BetweennessScores = brandes_bc_hash_out(
            &graph,
            None
        )?;

        let adjacency = graph.adjacency_list_for_dachshund();

        let dachshund_graph = graph_for_dachshund(adjacency).unwrap();

        let dachshund_bet = dachshund_graph.get_node_betweenness().unwrap();

        assert_dachshund_result_matches_betweenness_result(dachshund_bet, scores);

        timer.stop();

        let interval = timer.interval();

        info!("test #{} took {:?}", test_idx, interval);
    }

    Ok(())
}

pub fn graph_for_dachshund(adjacency_list: Vec<(usize,usize)>) -> CLQResult<SimpleUndirectedGraph> {

    fn _graph_for_dachshund<T, R>(mut builder: T, adjacency_list: Vec<(usize,usize)>) -> CLQResult<R>
    where
        R: GraphBase,
        T: GraphBuilderBase<GraphType = R, RowType = (i64, i64)>,
    {
        builder.from_vector(
            adjacency_list.into_iter()
                .map(|(x, y)| (x as i64, y as i64))
                .collect(),
        )
    }

    let builder = SimpleUndirectedGraphBuilder {};

    _graph_for_dachshund::<SimpleUndirectedGraphBuilder, _>(
        builder,
        adjacency_list
    )
}

pub fn golden_betweenness_scores_for_paper_via_dachshund()
{
    let adjacency_list = paper_edges_for_dachshund();

    let graph = graph_for_dachshund(adjacency_list).unwrap();

    let bet = graph.get_node_betweenness().unwrap();

    debug!("{}",   graph.nodes.len());
    debug!("{}",   graph.count_edges());
    debug!("{:?}", bet);
}

pub fn assert_dachshund_result_matches_betweenness_result(
    dachshund_bet: HashMap<lib_dachshund::NodeId,f64>, 
    scores:        BetweennessScores)
{
    for (node,dachshund_score) in dachshund_bet.iter() {

        let node = nodeid![usize::try_from(node.value()).unwrap()];

        let betweenness_score = scores.score_for_node(node);

        debug!(
            "node={}, dachshund_score={}, betweenness_score={}", 
            node, 
            dachshund_score, 
            betweenness_score
        );

        assert!((*dachshund_score - betweenness_score).abs() < 0.0001);
    }
}
