crate::ix!();

pub struct NaiveBetweennessCentrality {
    scores:                         BetweennessScores,
    all_pairs_distances:            AllPairsDistances,
    all_pairs_shortest_path_counts: AllPairsShortestPathCounts,
}

impl NaiveBetweennessCentrality {

    pub fn scores(&self) -> BetweennessScores {
        self.scores.clone()
    }

    pub fn scores_ref(&self) -> &BetweennessScores {
        &self.scores
    }

    fn update_distance_for_immediate_neighbors_step(
        &mut self, 
        graph: &Graph<GraphHash>, 
        k:     NodeId, 
        i:     NodeId, 
        j:     NodeId) 
    {
        let dij = self.all_pairs_distances[(i,j)];
        let dik = self.all_pairs_distances[(i,k)];
        let dkj = self.all_pairs_distances[(k,j)];

        if dij > dik + dkj {

            self.all_pairs_distances[(i,j)] = dik + dkj;
        }
    }

    fn update_distance_for_immediate_neighbors(&mut self, graph: &Graph<GraphHash>) {

        for i in graph.nodeid_range() {

            self.all_pairs_distances[(i,i)] = 0.0;

            for nbr_id in graph.neighbors(i) {

                self.all_pairs_distances[(i,nbr_id)] = 1.0;
            }
        }

        let n_distances = self.all_pairs_distances.len();

        for k in NodeIdRange::new(0,graph.num_nodes()) {

            for i in NodeIdRange::new(0,n_distances) {

                for j in NodeIdRange::new(0,n_distances) {

                    self.update_distance_for_immediate_neighbors_step(
                        graph,
                        nodeid![k],
                        nodeid![i],
                        nodeid![j]
                    );
                }
            }
        }
    }

    fn update_counts_for_immediate_neighbors_step(&mut self, 
        graph: &Graph<GraphHash>, 
        k:     NodeId, 
        i:     NodeId, 
        j:     NodeId) 
    {
        let dij = self.all_pairs_distances[(i,j)];
        let dik = self.all_pairs_distances[(i,k)];
        let dkj = self.all_pairs_distances[(k,j)];

        if dij == dik + dkj {

            let spik = self.all_pairs_shortest_path_counts[(i,k)];
            let spkj = self.all_pairs_shortest_path_counts[(k,j)];

            self.all_pairs_shortest_path_counts[(i,j)] += spik * spkj;
        }
    }

    fn update_counts_for_immediate_neighbors(&mut self, graph: &Graph<GraphHash>) {

        for i in NodeIdRange::new(0,graph.num_nodes()) {

            self.all_pairs_shortest_path_counts[(i,i)] = 0;

            for nbr_id in graph.neighbors(i).iter() {

                self.all_pairs_shortest_path_counts[(i,*nbr_id)] = 1;
            }
        }

        let n_path_counts = self.all_pairs_shortest_path_counts.len();

        for k in NodeIdRange::new(0,graph.num_nodes()) {

            for i in NodeIdRange::new(0,n_path_counts) {

                for j in NodeIdRange::new(0,n_path_counts) {

                    self.update_counts_for_immediate_neighbors_step(graph,k,i,j);
                }
            }
        }
    }

    fn betweenness_centrality_computation_step(
        &mut self, 
        graph: &Graph<GraphHash>, 
        s:     NodeId, 
        t:     NodeId, 
        v:     NodeId) 
    {
        assert!(s != v);
        assert!(v != t);
        assert!(s != t);

        let dst = self.all_pairs_distances[(s,t)];
        let dsv = self.all_pairs_distances[(s,v)];
        let dvt = self.all_pairs_distances[(v,t)];

        if dst == dsv + dvt {

            let t0 = self.all_pairs_shortest_path_counts[(s,v)] as f64;
            let t1 = self.all_pairs_shortest_path_counts[(v,t)] as f64;
            let t2 = self.all_pairs_shortest_path_counts[(s,t)] as f64;

            self.scores.increase_score_for_node(
                v, 
                t0 * t1 / t2
            );
        }
    }

    fn betweenness_centrality_computation(&mut self, graph: &Graph<GraphHash>) {

        for s in graph.nodeid_range() {

            for t in graph.nodeid_range() {

                for v in self.scores.nodeid_range() {

                    if s != v 
                    && v != t 
                    && s != t 
                    {
                        self.betweenness_centrality_computation_step(
                            graph,
                            nodeid![s],
                            nodeid![t],
                            nodeid![v]
                        );
                    }
                }
            }
        }
    }

    /// 1. find all-pair shortest paths and store
    /// in a n^2 array
    ///
    /// 2. find all-pair shortest paths count and
    /// store in a x^2 array
    ///
    /// 3. do the three nested loops to find BC of
    /// each node
    ///
    fn compute(&mut self, graph: &Graph<GraphHash>) {

        /*Floyd-Warshall to compute all-pair shortest paths*/
        self.update_distance_for_immediate_neighbors(graph);

        /*Floyd-Warshall like algorithm to compute all-pair shortest paths counts*/
        self.update_counts_for_immediate_neighbors(graph);

        self.betweenness_centrality_computation(graph);

        self.scores.halve();
    }
}

impl From<&MutexGuard<'_,Graph<GraphHash>>> for NaiveBetweennessCentrality {

    fn from(graph: &MutexGuard<'_,Graph<GraphHash>>) -> Self {

        let graph_len = graph.num_nodes();

        let mut result = Self {
            scores:                         BetweennessScores::new(graph_len,          "naive_betweenness_centrality.scores"),
            all_pairs_distances:            AllPairsDistances::new(graph_len,          "naive_betweenness_centrality.all_pairs_distances"),
            all_pairs_shortest_path_counts: AllPairsShortestPathCounts::new(graph_len, "naive_betweenness_centrality.all_pairs_shortest_path_counts"),
        };

        result.compute(graph);

        result 
    }
}

pub fn naive_betweenness_centrality(graph: Arc<Mutex<Graph<GraphHash>>>) 
-> Result<BetweennessScores,BetweennessCentralityError> 
{
    if let graph = graph.lock()? {

        let result = NaiveBetweennessCentrality::from(&graph);

        Ok(result.scores())

    } else {

        panic!("poisoned graph!");
    }
}
