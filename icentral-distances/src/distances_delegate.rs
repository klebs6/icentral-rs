crate::ix!();


#[macro_export] macro_rules! delegate_to_distances {

    ($ty:ty) => {
        impl $ty {

            delegate!{

                to self.distances {

                    #[call(is_farther_away)]
                    pub fn distance_is_farther_away(&self, x: NodeId, y: NodeId) -> bool;

                    #[call(is_farther_than_one_away)]
                    pub fn distance_is_farther_than_one_away(&self, x: NodeId, y: NodeId) -> bool;

                    #[call(is_one_step_away)]
                    pub fn distance_is_one_step_away(&self, x: NodeId, y: NodeId) -> bool;

                    #[call(is_infinite)]
                    pub fn distance_is_infinite(&self, node: NodeId) -> bool;

                    pub fn distance(&self, node: NodeId) -> f64;

                    pub fn set_distance_for_node(&mut self, node: NodeId, val: f64);

                    #[call(set_one_step_away)]
                    pub fn set_distance_one_step_away(
                        &mut self, 
                        dst: NodeId,
                        src: NodeId);

                    #[call(set_zero_distance)]
                    pub fn set_distance_zero(&mut self, source: NodeId);

                    #[call(reinit)]
                    pub fn distance_reinit(&mut self, len: usize);
                }
            }
        }
    }
}
