crate::ix!();

#[macro_export] macro_rules! delegate_to_visit_markers {
    ($ty:ty) => {

        impl $ty {
            delegate! {
                to self.visit_markers {

                    pub fn visited(&self, id: NodeId)  -> bool;

                    pub fn unvisited(&self, id: NodeId)  -> bool;

                    pub fn visit(&mut self, id: NodeId);

                    pub fn unvisit(&mut self, id: NodeId);

                    #[call(reinit)]
                    pub fn reinit_visit_markers(&mut self, len: usize);

                    #[call(fill)]
                    pub fn fill_visit_markers(&mut self, val: bool);

                    #[call(fill_to_len)]
                    pub fn fill_visit_markers_to_len(&mut self, len: usize, val: bool);
                }
            }
        }
    }
}
