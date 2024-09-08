crate::ix!();

impl UpdatePairDependencies for SubGraph {

    fn update_pair_dependencies(&mut self, v_p: NodeId, v_n: NodeId)
    {
        let sp_sn = self.path_count_ratio(v_p,v_n);

        self.pair_dependencies.update(
            v_p,
            v_n,
            sp_sn
        );
    }

    fn update_new_pair_dependencies(&mut self, v_p: NodeId, v_n: NodeId)
    {
        let new_sp_sn = self.new_path_counts_path_count_ratio(v_p,v_n);

        self.new_pair_dependencies.update(
            v_p,
            v_n,
            new_sp_sn
        );
    }
}
