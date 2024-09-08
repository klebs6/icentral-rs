crate::ix!();


#[macro_export] macro_rules! delegate_to_path_counts {
    ($ty:ty; $tag:ident) => {

        paste::item!{
            impl $ty {
                delegate! {
                    to self.$tag {

                        #[call(increment_path_count_for_node)]
                        pub fn [<$tag _increment_path_count_for_node>](&mut self, node: NodeId, val: usize);

                        #[call(increment_path_count_for_node_from)]
                        pub fn [<$tag _increment_path_count_for_node_from>](
                            &mut self, 
                            node:  NodeId, 
                            other: NodeId);

                        #[call(update_path_counts)]
                        pub fn [<$tag _update_path_counts>](
                            &mut self, 
                            dst: NodeId,
                            src: NodeId);

                        #[call(path_count_for_node)]
                        pub fn [<$tag _path_count_for_node>](&self, node: NodeId) -> usize;

                        #[call(set_path_count_for_node)]
                        pub fn [<$tag _set_path_count_for_node>](
                            &mut self, 
                            node:  NodeId, 
                            count: usize);

                        #[call(path_count_ratio)]
                        pub fn [<$tag _path_count_ratio>](&self, v_p: NodeId, v_n: NodeId) -> f64;

                        #[call(set_path_count_to_one)]
                        pub fn [<$tag _set_path_count_to_one>](&mut self, source: NodeId);

                        #[call(set_path_count_to_zero)]
                        pub fn [<$tag _set_path_count_to_zero>](&mut self, source: NodeId);

                        #[call(reinit)]
                        pub fn [<$tag _reinit>](&mut self, len: usize);
                    }
                }
            }
        }
    };
    ($ty:ty) => {

        impl $ty {
            delegate! {
                to self.path_counts {

                    pub fn increment_path_count_for_node(&mut self, node: NodeId, val: usize);

                    pub fn increment_path_count_for_node_from(
                        &mut self, 
                        node:  NodeId, 
                        other: NodeId);

                    pub fn update_path_counts(
                        &mut self, 
                        dst: NodeId,
                        src: NodeId);

                    pub fn path_count_for_node(&self, node: NodeId) -> usize;

                    pub fn set_path_count_for_node(
                        &mut self, 
                        node:  NodeId, 
                        count: usize);

                    pub fn path_count_ratio(&self, v_p: NodeId, v_n: NodeId) -> f64;

                    pub fn set_path_count_to_one(&mut self, source: NodeId);

                    pub fn set_path_count_to_zero(&mut self, source: NodeId);

                    #[call(reinit)]
                    pub fn path_counts_reinit(&mut self, len: usize);
                }
            }
        }
    }
}
