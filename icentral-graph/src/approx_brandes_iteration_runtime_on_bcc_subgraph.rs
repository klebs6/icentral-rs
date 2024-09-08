crate::ix!();

impl<GH> ApproxBrandesIterationRuntimeOnBccSubgraph for Graph<GH> 

where GH
: CreateNamedEmpty
+ DebugIterationStep 
+ FindPruningCounts 
+ GetEdges 
+ GetNeighborsForNode 
+ GetNodeIdRange
+ GetSigmaValueForNode
+ HasEdge
+ InitDebugIteration
+ InsertEdge
+ MappedNodes
+ NumEdges
+ NumNodes
+ PairDependencyForNode
+ ParentsForNode
+ RemoveEdge
+ BccGraphHashInterface
{

    /**
      | approximates the runtime of Brandes
      | iteration on a bcc subgraph by doing
      | @num_iter iteration and taking the average
      | if @num_iter is -1 the number of iteration
      | is just going to be the number of nodes in
      | the graph
      |
      */
    fn approx_bcc_iter_tm(&mut self, 
        src:           NodeId,
        dst:           NodeId,
        avg_iter_time: &mut Duration,
        num_iter:      Option<usize>) 
    -> Result<(),BetweennessCentralityError> 
    {
        let mut tm: Timer = Timer::default();

        let bcc_name = name![self.name(), "approx_bcc_iter_tm::bcc"];

        let mut bcc = BiconnectedComponentsScratch::empty(bcc_name);

        let num_iter = num_iter.unwrap_or(bcc.bcc_subgraph_num_nodes());

        self.find_edge_bcc_with_scratch(&mut bcc, &Edge::new(src,dst));

        let scores_name = name![self.name(), "approx_bcc_iter_tm::scores"];

        let mut scores = BetweennessScores::new_from_nodeids(
            bcc.bcc_subgraph_mapped_nodes(),
            scores_name
        );

        tm.start();

        bcc.compute_bc(&mut scores, Some(num_iter));

        tm.stop();

        *avg_iter_time = tm.interval() / num_iter as u32;

        Ok(())
    }
}
