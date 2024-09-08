crate::ix!();

impl<GH> GetNeighborsForNode for Graph<GH> {

    fn neighbors(&self, node: NodeId) -> Neighbors {
        self.nodes_map.neighbors(node)
    }
}
