crate::ix!();

impl<GH> CreateRandomConnected for Graph<GH> {

    /// TODO: This is probably inefficient, but
    /// maybe not that bad
    ///
    fn random_connected(n_vertices: usize, n_edges: usize) -> Self {

        assert!(n_vertices < n_edges);

        let mut rng = WyRand::new();

        let graph_name = format!(
            "random_graph_with_{}_vertices_and_{}_edges", 
            n_vertices, 
            n_edges
        );

        let mut graph = Graph::empty(&graph_name);

        let mut nodes = vec![];

        /// every node will at least show up once
        for node_idx in 0..n_vertices {
            nodes.push(node_idx);
        }

        for _ in n_vertices..n_edges {

            let node_idx = rng.generate_range(0..n_vertices);

            nodes.push(node_idx);
        }

        assert!(nodes.len() == n_edges);

        let mut rng = rand::thread_rng();

        nodes.shuffle(&mut rng);

        for pair in nodes.windows(2) {

            let rand_edge = Edge::new_with_ids(pair[0], pair[1]);

            graph.insert_edge(&rand_edge);
        }

        graph
    }
}
