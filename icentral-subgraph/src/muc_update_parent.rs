crate::ix!();

pub trait MucUpdateParent {

    fn muc_update_parent(&mut self, 
        parent:              NodeId,
        v_n:                 NodeId, 
        source:              NodeId, 
        tmp_conn_vertex_map: &ConnVertexMap,
        scores:              &mut BetweennessScores);
}

impl MucUpdateParent for SubGraph {

    fn muc_update_parent(&mut self, 
        parent:              NodeId,
        v_n:                 NodeId, 
        source:              NodeId, 
        tmp_conn_vertex_map: &ConnVertexMap,
        scores:              &mut BetweennessScores)
    {
        let sp_sn = self.path_count_ratio(parent,v_n);

        self.update_pair_dependencies(parent,v_n);

        let new_sp_sn = self.new_path_counts_path_count_ratio(parent,v_n);

        self.update_new_pair_dependencies(parent,v_n);

        if tmp_conn_vertex_map.has_mapping_for_node(source) {

            self.update_all_sigmas(parent,v_n);

            let new_parent: NodeId = self.label_map_inout(parent);

            let new_val = {
                let map_val = scores.score_for_node(new_parent);
                let sigma_n = self.sigma_value_for_node(v_n);
                map_val - sigma_n * sp_sn / 2.0
            };

            scores.set_score_for_node(
                new_parent, 
                new_val
            );

            let new_val = {
                let map_val = scores.score_for_node(new_parent);
                let sigma_n = self.new_sigmas_sigma_value_for_node(v_n);
                map_val + sigma_n * new_sp_sn / 2.0
            };

            scores.set_score_for_node(new_parent, new_val);
        }
    }
}
