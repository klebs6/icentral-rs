crate::ix!();

impl GetEdges for SubGraph {

    fn edges(&self) -> &Edges {
        &self.edges
    }
}

impl GetNodes for SubGraph {

    fn nodes(&self) -> &NeighborsMap {
        &self.nodes_map
    }
}

impl<'a> Named<'a> for SubGraph {

    type Name = &'a str;

    fn name(&'a self) -> &'a str {
        &self.name
    }
}

impl NumNodes for SubGraph {

    fn num_nodes(&self) -> usize {
        self.nodes_map.len()
    }
}

impl GetNeighborsForNode for SubGraph {

    fn neighbors(&self, node: NodeId) -> Neighbors {
        self.nodes_map.neighbors(node)
    }
}
