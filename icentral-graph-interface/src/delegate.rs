crate::ix!();

#[macro_export] macro_rules! delegate_to_graphhash {
    ($tag:ident) => {
        paste::item!{
            delegate!{
                to self.$tag {

                    #[call(parents_for_node)]
                    pub fn [<$tag _parents_for_node>](&self, n: NodeId) -> Vec<NodeId>;

                    #[call(insert_edge)]
                    pub fn [<$tag _insert_edge>](&mut self, e: &Edge);

                    #[call(init_dbg_iteration)]
                    pub fn [<$tag _init_dbg_iteration>](&mut self, s: NodeId);

                    #[call(dbg_iteration_step)]
                    pub fn [<$tag _dbg_iteration_step>](
                        &mut self, 
                        v_s: &mut Vec<NodeId>) 
                        -> Result<(),BetweennessCentralityError>;

                    #[call(remove_edge)]
                    pub fn [<$tag _remove_edge>](&mut self, edge: &Edge)
                        -> Result<(),BetweennessCentralityError>;

                    #[call(num_nodes)]
                    pub fn [<$tag _num_nodes>](&self) -> usize;

                    #[call(num_edges)]
                    pub fn [<$tag _num_edges>](&self) -> usize;

                    #[call(find_pruning_counts_exp)]
                    pub fn [<$tag _find_pruning_counts_exp>](&mut self, 
                        src:    NodeId,
                        dst:    NodeId) 
                        -> Result<(i32,i32,i32),BetweennessCentralityError>;

                    #[call(sigma_value_for_node)]
                    pub fn [<$tag _sigma_value_for_node>](&self, node: NodeId) -> f64;

                    #[call(pair_dependency_for_node)]
                    pub fn [<$tag _pair_dependency_for_node>](&self, node: NodeId) -> f64;

                    #[call(mapped_nodes)]
                    pub fn [<$tag _mapped_nodes>](&self) -> Vec<NodeId>;

                    #[call(has_edge)]
                    pub fn [<$tag _has_edge>](&self, e: &Edge) -> bool;
                }
            }
        }
    }
}
