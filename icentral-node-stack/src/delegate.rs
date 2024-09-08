crate::ix!();

#[macro_export] macro_rules! delegate_to_bfs_stack {
    ($ty:ty) => {

        impl $ty {
            delegate! {
                to self.stack {

                    #[call(push)]
                    pub fn stack_push(&mut self, n: NodeId);

                    #[call(pop)]
                    pub fn stack_pop(&mut self) -> Option<NodeId>;

                    #[call(len)]
                    pub fn stack_len(&self) -> usize;

                    #[call(node_at_index)]
                    pub fn stack_node_at_index(&self, idx: usize) -> NodeId;

                    #[call(set_node_at_index)]
                    pub fn stack_set_node_at_index(&mut self, idx: usize, n: NodeId);
                }
            }
        }
    }
}

