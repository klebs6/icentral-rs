crate::ix!();

pub trait MucComputeNewPathCountsAndPaths {

    fn muc_compute_new_path_counts_and_paths(
        &mut self, 
        src: NodeId, 
        dst: NodeId);
}

impl MucComputeNewPathCountsAndPaths for SubGraph {

    fn muc_compute_new_path_counts_and_paths(
        &mut self, 
        src: NodeId, 
        dst: NodeId)
    {
        self.enqueue(dst);

        self.set_distance_one_step_away(dst,src);

        self.set_single_parent(dst,src);

        self.inc_path_counts_set_path_count_for_node(
            dst, 
            self.path_count_for_node(src)
        );

        self.set_path_count_for_node(
            dst, 
            self.inc_path_counts_path_count_for_node(dst)
        );

        self.visit(dst);
    }
}
