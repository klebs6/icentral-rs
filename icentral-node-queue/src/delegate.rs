crate::ix!();

#[macro_export] macro_rules! delegate_to_bfs_queue {
    ($ty:ty) => {

        impl $ty {
            delegate! {
                to self.queue {

                    #[call(len)]
                    pub fn queue_len(&self) -> usize;

                    pub fn enqueue(&mut self, n: NodeId);

                    pub fn dequeue(&mut self) -> Option<NodeId>;
                }
            }
        }
    }
}
