crate::ix!();

pub trait UpdateSigmas {

    fn maybe_update_all_sigmas<GH: NumNodes>(
        &mut self, 
        v_n:             NodeId, 
        source:          NodeId, 
        subgraph_map:    &SubGraphMap<GH>, 
        conn_vertex_map: &ConnVertexMap)
    -> Result<(),BetweennessCentralityError>;

    fn maybe_update_all_sigmas_and_do_new<GH: NumNodes>(
        &mut self, 
        v_n:             NodeId, 
        source:          NodeId, 
        subgraph_map:    &SubGraphMap<GH>, 
        conn_vertex_map: &ConnVertexMap)
    -> Result<(),BetweennessCentralityError>;

    fn update_all_sigmas(&mut self, v_p: NodeId, v_n: NodeId);

    fn update_new_sigmas(&mut self, v_p: NodeId, v_n: NodeId);

    fn update_sigmas(&mut self, v_p: NodeId, v_n: NodeId);
}

impl UpdateSigmas for SubGraph {

    fn maybe_update_all_sigmas<GH: NumNodes>(
        &mut self, 
        v_n:             NodeId, 
        source:          NodeId, 
        subgraph_map:    &SubGraphMap<GH>, 
        conn_vertex_map: &ConnVertexMap)
    -> Result<(),BetweennessCentralityError> 
    {
        if conn_vertex_map.has_mapping_for_node(source)
        && conn_vertex_map.has_mapping_for_node(v_n)
        && source != v_n 
        {
            let mut vg_s: i32 = subgraph_map.subgraph_for_node(source).num_nodes().try_into()?;
            let mut vg_n: i32 = subgraph_map.subgraph_for_node(v_n).num_nodes().try_into()?;

            let c_t: i32 = vg_s * vg_n;

            self.increment_sigma_value_for_node(
                v_n, 
                c_t as f64
            );

            /*this guy must not change!*/

            // scores.score_for_node(&v_n) = scores.score_for_node(&v_n) - c_t;
            // scores.score_for_node(&v_n) = scores.score_for_node(&v_n) + c_t;
        }

        Ok(())
    }

    fn maybe_update_all_sigmas_and_do_new<GH: NumNodes>(
        &mut self, 
        v_n:             NodeId, 
        source:          NodeId, 
        subgraph_map:    &SubGraphMap<GH>, 
        conn_vertex_map: &ConnVertexMap)
    -> Result<(),BetweennessCentralityError> 
    {
        if conn_vertex_map.has_mapping_for_node(source)
        && conn_vertex_map.has_mapping_for_node(v_n)
        && source != v_n 
        {
            let mut vg_s: i32 = subgraph_map.subgraph_for_node(source).num_nodes().try_into()?;
            let mut vg_n: i32 = subgraph_map.subgraph_for_node(v_n).num_nodes().try_into()?;

            let c_t: i32 = vg_s * vg_n;

            self.increment_sigma_value_for_node(
                v_n, 
                c_t as f64
            );

            self.new_sigmas_increment_sigma_value_for_node(
                v_n, 
                c_t as f64
            );

            /*this guy must not change!*/

            // scores.score_for_node(&v_n) = scores.score_for_node(&v_n) - c_t;
            // scores.score_for_node(&v_n) = scores.score_for_node(&v_n) + c_t;
        }

        Ok(())
    }

    fn update_all_sigmas(&mut self, v_p: NodeId, v_n: NodeId)
    {
        self.update_sigmas(v_p,v_n);
        self.update_new_sigmas(v_p,v_n);
    }

    fn update_new_sigmas(&mut self, v_p: NodeId, v_n: NodeId)
    {
        let new_sp_sn = self.new_path_counts_path_count_ratio(v_p,v_n);

        self.new_sigmas.update(
            v_p,
            v_n,
            new_sp_sn
        );
    }

    fn update_sigmas(&mut self, v_p: NodeId, v_n: NodeId)
    {
        let sp_sn = self.path_count_ratio(v_p,v_n);

        self.sigmas.update(
            v_p,
            v_n,
            sp_sn
        );
    }
}
