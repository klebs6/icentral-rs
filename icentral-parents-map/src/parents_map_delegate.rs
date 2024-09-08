crate::ix!();


#[macro_export] macro_rules! delegate_to_parents {

    ($ty:ty) => {
        impl $ty {

            delegate!{

                to self.parents {

                    pub fn clear_parents(&mut self);

                    pub fn clear_node_parents(&mut self, node: NodeId);

                    pub fn parents_for_node(&self, v_n: NodeId) -> Vec<NodeId>;

                    pub fn num_parents_for_node(&self, v_n: NodeId) -> usize;

                    pub fn set_single_parent(&mut self, 
                        node:   NodeId, 
                        parent: NodeId);

                    pub fn set_parents_for_node(
                        &mut self, 
                        node:    NodeId, 
                        parents: Vec<NodeId>);

                    pub fn has_parent(&self, 
                        node:      NodeId, 
                        candidate: NodeId) -> bool;

                    pub fn add_parent(&mut self,
                        node:   NodeId,
                        parent: NodeId);

                    #[call(reinit)]
                    pub fn reinit_parents(&mut self, len: usize);

                    pub fn fill_to_len(&mut self, len: usize, val: Vec<NodeId>);
                }
            }
        }
    };

    ($ty:ty; $tag:ident) => {

        paste::item!{

            impl $ty {

                delegate!{

                    to self.$tag {

                        #[call(clear_parents)]
                        pub fn [<$tag _clear_parents>](&mut self);

                        #[call(clear_node_parents)]
                        pub fn [<$tag _clear_node_parents>](&mut self, node: NodeId);

                        #[call(parents_for_node)]
                        pub fn [<$tag _parents_for_node>](&self, v_n: NodeId) -> Vec<NodeId>;

                        #[call(num_parents_for_node)]
                        pub fn [<$tag _num_parents_for_node>](&self, v_n: NodeId) -> usize;

                        #[call(set_single_parent)]
                        pub fn [<$tag _set_single_parent>](&mut self, 
                            node:   NodeId, 
                            parent: NodeId);

                        #[call(set_parents_for_node)]
                        pub fn [<$tag _set_parents_for_node>](
                            &mut self, 
                            node:    NodeId, 
                            parents: Vec<NodeId>);

                        #[call(has_parent)]
                        pub fn [<$tag _has_parent>](&self, 
                            node:      NodeId, 
                            candidate: NodeId) -> bool;

                        #[call(add_parent)]
                        pub fn [<$tag _add_parent>](&mut self,
                            node:   NodeId,
                            parent: NodeId);

                        #[call(reinit)]
                        pub fn [<$tag _reinit_parents>](&mut self, len: usize);

                        #[call(fill_to_len)]
                        pub fn [<$tag _fill_to_len>](&mut self, len: usize, val: Vec<NodeId>);
                    }
                }
            }
        }
    }
}
