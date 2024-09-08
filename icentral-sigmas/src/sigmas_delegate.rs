crate::ix!();


#[macro_export] macro_rules! delegate_to_sigmas {
    ($ty:ty; $tag:ident) => {

        paste::item!{
            impl $ty {
                delegate! {
                    to self.$tag {

                        #[call(update)]
                        pub fn [<$tag _sigma_update>](&mut self, 
                            v_p:   NodeId, 
                            v_n:   NodeId, 
                            sp_sn: f64);

                        #[call(set_node_to_zero)]
                        pub fn [<$tag _sigma_set_node_to_zero>](&mut self, node: NodeId);

                        #[call(set_node_to_one)]
                        pub fn [<$tag _sigma_set_node_to_one>](&mut self, node: NodeId);

                        #[call(sigma_value_for_node)]
                        pub fn [<$tag _sigma_value_for_node>](&self, node: NodeId) -> f64;

                        #[call(set_sigma_value_for_node)]
                        pub fn [<$tag _set_sigma_value_for_node>](&mut self, node: NodeId, val: f64);

                        #[call(increment_sigma_value_for_node)]
                        pub fn [<$tag _increment_sigma_value_for_node>](&mut self, v_n: NodeId, c_t: f64);

                        #[call(reinit)]
                        pub fn [<reinit_ $tag>](&mut self, len: usize);

                        #[call(ratio)]
                        pub fn [< $tag _sigma_ratio >](&self, 
                            v_p: NodeId, 
                            v_n: NodeId) -> f64;

                        #[call(fill)]
                        pub fn [< $tag _fill_sigmas >](&mut self, val: f64);
                    }
                }
            }
        }
    };

    ($ty:ty) => {

        impl SetSigmaValueForNode for $ty {

            delegate! {
                to self.sigmas {
                    fn set_sigma_value_for_node(&mut self, node: NodeId, val: f64);
                }
            }
        }

        impl GetSigmaValueForNode for $ty {

            delegate! {
                to self.sigmas {
                    fn sigma_value_for_node(&self, node: NodeId) -> f64;
                }
            }
        }

        impl $ty {
            delegate! {
                to self.sigmas {

                    #[call(update)]
                    pub fn sigma_update(&mut self, 
                        v_p:   NodeId, 
                        v_n:   NodeId, 
                        sp_sn: f64);

                    #[call(set_node_to_zero)]
                    pub fn sigma_set_node_to_zero(&mut self, node: NodeId);

                    #[call(set_node_to_one)]
                    pub fn sigma_set_node_to_one(&mut self, node: NodeId);

                    pub fn increment_sigma_value_for_node(
                        &mut self, 
                        v_n: NodeId, 
                        c_t: f64
                    );

                    #[call(reinit)]
                    pub fn reinit_sigmas(&mut self, len: usize);

                    #[call(ratio)]
                    pub fn sigma_ratio(&self, 
                        v_p: NodeId, 
                        v_n: NodeId) -> f64;

                    #[call(fill)]
                    pub fn fill_sigmas(&mut self, val: f64);
                }
            }
        }
    };
}

