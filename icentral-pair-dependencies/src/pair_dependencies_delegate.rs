crate::ix!();

#[macro_export] macro_rules! delegate_to_pair_dependencies {

    ($ty:ty; $tag:ident) => {

        paste::item!{

            impl $ty {
                delegate! {
                    to self.$tag {

                        #[call(pair_dependency_for_node)]
                        pub fn [<$tag _pair_dependency_for_node>](&self, node: NodeId) -> f64;

                        #[call(set_pair_dependency_for_node)]
                        pub fn [<$tag _set_pair_dependency_for_node>](&mut self, node: NodeId, val: f64);

                        #[call(reinit)]
                        pub fn [<$tag _reinit>](&mut self, len: usize);

                        #[call(increment_pair_dependency_for_node)]
                        pub fn [< $tag _increment_pair_dependency_for_node >](
                            &mut self, 
                            node: NodeId, 
                            val:  f64
                        );
                    }
                }
            }
        }
    };

    ($ty:ty) => {

        impl $ty {
            delegate! {
                to self.pair_dependencies {

                    pub fn pair_dependency_for_node(&self, node: NodeId) -> f64;

                    pub fn set_pair_dependency_for_node(&mut self, node: NodeId, val: f64);

                    #[call(reinit)]
                    pub fn reinit_pair_dependencies(&mut self, len: usize);

                    pub fn increment_pair_dependency_for_node(&mut self, 
                        node: NodeId, 
                        val:  f64
                    );
                }
            }
        }
    }
}
