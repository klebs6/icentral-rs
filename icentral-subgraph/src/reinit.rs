crate::ix!();

impl ReinitMaps for SubGraph {

    fn reinit_maps(&mut self)  {

        let len = self.nodes_map.len();
        
        self.parents.reinit(len);
        self.distances.reinit(len);
        self.path_counts_reinit(len);
        self.new_path_counts_reinit(len);
        self.pair_dependencies.reinit(len);
        self.new_pair_dependencies.reinit(len);
        self.sigmas.reinit(len);
        self.new_sigmas.reinit(len);
        self.inc_path_counts_reinit(len);
        self.visit_markers.reinit(len);
    }
}
