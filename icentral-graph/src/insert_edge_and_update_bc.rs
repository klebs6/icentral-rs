crate::ix!();

impl<GH> InsertEdgeUpdateBc for Graph<GH>
where GH
: GraphHashMucInterface
+ ResetWith<GH>
+ RemoveEdge
+ CreateNamedEmpty
+ FindSingleSourceShortestPaths
+ GetNodeIdRange
+ GetNeighborsForNode
+ ParentsForNode
+ InitDebugIteration
+ DebugIterationStep
+ FindPruningCounts
+ HasEdge
+ PathCountForNode
+ PairDependencyForNode
+ SetPairDependencyForNode
+ SetSigmaValueForNode
+ GetSigmaValueForNode
+ BccGraphHashInterface

{
    // incremental bc with bcc stuff
    //
    fn insert_edge_update_bc_experimental(&mut self, 
        edge:        &Edge,
        bcc_stat:    &mut BiconnectedComponentsStat,
        max_iter_d1: Option<usize>,
        max_iter_d2: Option<usize>) 
    -> Result<(),BetweennessCentralityError>  
    {
        /*
         | 1. edge must be in the graph so that bcc
         | extraction works properly
         |
         | 2. the subgraph in the bcc must not have
         | the edge (done inside
         | bcc_delta_t::compute_bc)
         |
         */
        let mut tm: Timer = Timer::default();

        self.insert_edge(&edge);

        let bcc_name = name![self.name(), "insert_edge_update_bc_experimental::bcc"];

        let mut bcc = BiconnectedComponentsDelta::empty(bcc_name);

        tm.start();

        self.find_edge_bcc_with_delta(&mut bcc,&edge);

        tm.stop();

        bcc_stat.bcc_num_nodes = bcc.bcc_subgraph_num_nodes();
        bcc_stat.bcc_num_edges = bcc.bcc_subgraph_num_edges();

        bcc_stat.bcc_find_time = tm.interval();

        tm.start();

        if max_iter_d1.is_some() || max_iter_d2.is_some() {

            bcc.compute_bc_maxiter_exp(
                &mut self.scores, 
                edge.src, 
                edge.dst, 
                bcc_stat, 
                max_iter_d1, 
                max_iter_d2
            );

        } else {

            bcc.compute_bc_exp(
                &mut self.scores, 
                edge.src, 
                edge.dst, 
                bcc_stat, 
                max_iter_d1, 
                max_iter_d2
            );
        }

        tm.stop();

        bcc_stat.bc_update_time = tm.interval();

        bcc.bcc_subgraph_remove_edge(&edge);

        let (d0,d1,d2) = bcc.bcc_subgraph_find_pruning_counts_exp(
            edge.src, 
            edge.dst, 
        )?;

        bcc_stat.tot_d0_iter = d0; 
        bcc_stat.tot_d1_iter = d1; 
        bcc_stat.tot_d2_iter = d2;

        bcc.bcc_subgraph_insert_edge(&edge);

        Ok(())
    }
}
