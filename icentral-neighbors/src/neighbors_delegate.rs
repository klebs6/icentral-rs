crate::ix!();

#[macro_export] macro_rules! delegate_to_neighbors_map {

    () => {
        delegate!{

            to self.nodes_map {

                pub fn nodeid_range(&self) 
                    -> Vec<NodeId>;

                pub fn limited_nodeid_range(&self, cap: Option<usize>) 
                    -> Vec<NodeId>;

                pub fn mapped_nodes(&self) -> Vec<NodeId>;

                pub fn set_neighbors(
                    &mut self, 
                    node: NodeId, 
                    nbrs: Vec<NodeId>);

                pub fn add_neighbor(
                    &mut self, 
                    node: NodeId, 
                    nbr:  NodeId);

                pub fn has_map_for_node(&self, node: NodeId) -> bool;

                pub fn remove_node_and_neighbors(&mut self, node: NodeId);

                pub fn neighbors(&self, node: NodeId) -> Neighbors;

                #[call(extend_with)]
                pub fn extend_mapped_nodes(&mut self, other: &NeighborsMap);

                #[call(unlink_all)]
                pub fn nodes_map_unlink_all(&mut self, src: NodeId, dst: NodeId);

                #[call(add_edge)]
                pub fn nodes_map_add_edge(&mut self, e: &Edge);

                #[call(unlink_edge)]
                pub fn nodes_map_unlink_edge(&mut self, e: &Edge);

                #[call(reinit)]
                pub fn nodes_map_reinit(&mut self, len: usize);

                #[call(clear)]
                pub fn nodes_map_clear(&mut self);

                #[call(add_isolated_node)]
                pub fn nodes_map_add_isolated_node(
                    &mut self, 
                    node: NodeId);
            }
        }
    }
}
