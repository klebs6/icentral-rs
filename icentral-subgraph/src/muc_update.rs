crate::ix!();

pub trait MucUpdate {

    fn muc_update(&mut self, 
        v_n:                 NodeId, 
        source:              NodeId, 
        tmp_conn_vertex_map: &ConnVertexMap,
        scores:              &mut BetweennessScores);
}

impl MucUpdate for SubGraph {

    fn muc_update(&mut self, 
        v_n:                 NodeId, 
        source:              NodeId, 
        tmp_conn_vertex_map: &ConnVertexMap,
        scores:              &mut BetweennessScores)
    {
        let parents = self.parents_for_node(v_n);

        for &parent in parents.iter() {

            self.muc_update_parent(
                parent,
                v_n,
                source,
                tmp_conn_vertex_map,
                scores
            );
        }
    }
}
