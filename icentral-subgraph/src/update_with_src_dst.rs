crate::ix!();

impl UpdateWithSrcDst for SubGraph {

    fn update(&mut self, src: NodeId, dst: NodeId)
    {
        self.enqueue(dst);

        self.set_distance_one_step_away(dst,src);

        // self.bcc_fast_subgraph.parents[dst].clear();
        // self.bcc_fast_subgraph.parents[dst].push_back(src);
        self.inc_path_counts_update_path_counts(dst,src);

        // self.bcc_fast_subgraph.path_counts.path_count_for_node(&dst) = self.bcc_fast_subgraph.inc_path_counts.path_count_for_node(&dst);
        self.visit(dst);
    }
}
