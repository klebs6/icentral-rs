crate::ix!();


#[test] fn test_rustworkx() {

    use rustworkx_core::petgraph;
    use rustworkx_core::centrality::betweenness_centrality;

    let g = petgraph::graph::UnGraph::<usize, ()>::from_edges(&[
        (0, 4), (1, 2), (2, 3), (3, 4), (1, 4)
    ]);

    // Calculate the betweenness centrality
    let output = betweenness_centrality(
        &g, 
        true, 
        true, 
        50
    );

    debug!("output: {:?}", output);

    assert_eq!(
        vec![Some(0.4), Some(0.5), Some(0.45), Some(0.5), Some(0.75)],
        output
    );
}
