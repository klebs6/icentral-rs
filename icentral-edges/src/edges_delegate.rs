crate::ix!();

#[macro_export] macro_rules! delegate_to_edges {

    () => {

        delegate!{

            to self.edges {

                #[call(len)]
                pub fn num_edges(&self) -> usize;

                #[call(clear)]
                pub fn clear_edges(&mut self);

                pub fn has_edge(&self, edge: &Edge) -> bool;

                pub fn connects(&self, src: NodeId, dst: NodeId) -> bool;

                #[call(unlink_all)]
                pub fn unlink_all_edges_between(&mut self, src: NodeId, dst: NodeId);

                pub fn edges_to_node(&self, src: NodeId) -> Vec<Edge>;

                pub fn edges_from_node(&self, src: NodeId) -> Vec<Edge>;
            }
        }
    }
}
