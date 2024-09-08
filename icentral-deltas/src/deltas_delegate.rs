crate::ix!();


#[macro_export] macro_rules! delegate_to_deltas {
    ($ty:ty; $tag:ident) => {

        paste::item!{
            impl $ty {
                delegate! {
                    to self.$tag {

                        #[call(update)]
                        pub fn [<$tag _update_delta_value>](&mut self, 
                            v_p:   NodeId, 
                            v_n:   NodeId, 
                            sp_sn: f64);

                        #[call(set_zero)]
                        pub fn [<$tag _set_delta_value_to_zero>](&mut self, node: NodeId);

                        #[call(set_one)]
                        pub fn [<$tag _set_delta_value_to_one>](&mut self, node: NodeId);

                        #[call(get)]
                        pub fn [<$tag _delta_value_for_node>](&self, node: NodeId) -> f64;

                        #[call(set)]
                        pub fn [<$tag _set_delta_value_for_node>](&mut self, node: NodeId, val: f64);

                        #[call(increment_delta)]
                        pub fn [<$tag _increment_delta_value_for_node>](&mut self, v_n: NodeId, c_t: f64);

                        #[call(attenuate_delta)]
                        pub fn [<$tag _attenuate_delta_value_for_node>](&mut self, v_n: NodeId, c_t: f64);

                        #[call(reinit)]
                        pub fn [<$tag _delta_reinit>](&mut self, len: usize);

                        #[call(ratio)]
                        pub fn [< $tag _delta_ratio >](&self, v_p: NodeId, v_n: NodeId) -> f64;

                        #[call(fill_to_len)]
                        pub fn [< $tag _deltas_fill_to_len >](&mut self, len: usize, val: f64);
                    }
                }
            }
        }
    };

    ($ty:ty) => {

        impl $ty {
            delegate! {
                to self.deltas {

                    #[call(update)]
                    pub fn update_delta_value(&mut self, 
                        v_p:   NodeId, 
                        v_n:   NodeId, 
                        sp_sn: f64);

                    #[call(set_zero)]
                    pub fn set_delta_value_to_zero(&mut self, node: NodeId);

                    #[call(set_one)]
                    pub fn set_delta_value_to_one(&mut self, node: NodeId);

                    #[call(get)]
                    pub fn delta_value_for_node(&self, node: NodeId) -> f64;

                    #[call(set)]
                    pub fn set_delta_value_for_node(&mut self, node: NodeId, val: f64);

                    #[call(increment_delta)]
                    pub fn increment_delta_value_for_node(&mut self, v_n: NodeId, c_t: f64);

                    #[call(attenuate_delta)]
                    pub fn attenuate_delta_value_for_node(&mut self, v_n: NodeId, c_t: f64);

                    #[call(reinit)]
                    pub fn delta_reinit(&mut self, len: usize);

                    #[call(ratio)]
                    pub fn delta_ratio(&self, v_p: NodeId, v_n: NodeId) -> f64;

                    #[call(fill_to_len)]
                    pub fn deltas_fill_to_len(&mut self, len: usize, val: f64);
                }
            }
        }
    }
}

