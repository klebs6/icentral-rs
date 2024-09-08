crate::ix!();

pub trait MucAugment {

    fn muc_augment_no_new(&mut self, 
        v_n:                 NodeId, 
        source:              NodeId, 
        tmp_conn_vertex_map: &ConnVertexMap,
        scores:              &mut BetweennessScores);
}

impl MucAugment for SubGraph {

    fn muc_augment_no_new(&mut self, 
        v_n:                 NodeId, 
        source:              NodeId, 
        tmp_conn_vertex_map: &ConnVertexMap,
        scores:              &mut BetweennessScores)
    {
        let parents = self.parents_for_node(v_n);

        for &parent in parents.iter() {

            self.muc_augment_parent_no_new(
                parent,
                v_n,
                source,
                tmp_conn_vertex_map,
                scores
            );
        }
    }
}
