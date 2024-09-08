crate::ix!();

impl ComputeNewPathCountsAndPaths for SubGraph {

    fn compute_new_path_counts_and_paths(
        &mut self, 
        src: NodeId, 
        dst: NodeId)
    {
        if self.distances.is_one_step_away(dst,src) {

            self.add_parent(dst,src);

            self.increment_path_count_for_node_from(dst,src);

        } else {

            self.set_single_parent(dst,src);

            self.update_path_counts(dst,src);
        }
    }
}
