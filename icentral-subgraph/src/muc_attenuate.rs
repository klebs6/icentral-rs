crate::ix!();

pub trait MucAttenuate {

    fn muc_attenuate_no_new(&mut self, 
        v_n:                 NodeId, 
        source:              NodeId, 
        tmp_conn_vertex_map: &ConnVertexMap,
        scores:              &mut BetweennessScores);
}

impl MucAttenuate for SubGraph {

    fn muc_attenuate_no_new(&mut self, 
        v_n:                 NodeId, 
        source:              NodeId, 
        tmp_conn_vertex_map: &ConnVertexMap,
        scores:              &mut BetweennessScores)
    {
        let parents = self.parents_for_node(v_n);

        for &parent in parents.iter() {

            self.muc_attenuate_parent_no_new(
                parent,
                v_n,
                source,
                tmp_conn_vertex_map,
                scores
            );
        }
    }
}
