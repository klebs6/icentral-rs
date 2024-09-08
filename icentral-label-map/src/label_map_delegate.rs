crate::ix!();


#[macro_export] macro_rules! delegate_to_label_map {

    ($ty:ty) => {
        impl $ty {

            delegate!{

                to self.label_map {

                    #[call(clear)]
                    pub fn label_map_clear(&mut self);

                    #[call(inout)]
                    pub fn label_map_inout(&self, node: NodeId) -> NodeId;

                    #[call(outin)]
                    pub fn label_map_outin(&self, node: NodeId) -> NodeId;

                    #[call(mapped_edge)]
                    pub fn label_map_mapped_edge(&self, theirs: &Edge) -> Edge;

                    #[call(projected_edge)]
                    pub fn label_map_projected_edge(&self, mine: &Edge) -> Edge;

                    #[call(insert_outin)]
                    pub fn label_map_insert_outin(&mut self, src: NodeId, dst: NodeId);

                    #[call(resize_inout)]
                    pub fn label_map_resize_inout(&mut self, len: usize, default: NodeId);
                }
            }
        }
    }
}
