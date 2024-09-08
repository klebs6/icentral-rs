crate::ix!();

#[derive(Debug,Clone)]
pub struct ConnVertexMap {
    name: String,
    data: MaybeIndexedMap<Vec<NodeId>>,
}

impl ConnVertexMap {

    pub fn vertices_for_node(&self, node: NodeId) -> &Vec<NodeId> {
        &self.data[node]
    }

    pub fn vertices_for_node_mut(&mut self, node: NodeId) -> Option<&mut Vec<NodeId>> {

        if self.data.contains(node) {
            Some(&mut self.data[node])
        } else {
            None
        }
    }

    pub fn set_vertex_map_for_node(&mut self, node: NodeId, vertex_map: Vec<NodeId>) {
        self.data.set(node,vertex_map)
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn has_mapping_for_node(&self, node: NodeId) -> bool {
        self.data.contains(node)
    }

    pub fn iter(&self) -> MaybeIndexedMapIterator<'_,Vec<NodeId>> {
        self.data.iter()
    }

    pub fn empty_indexed(name: &str) -> Self {

        debug!("creating new empty_indexed ConnVertexMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }

    pub fn empty_mapped(name: &str) -> Self {

        debug!("creating new empty_mapped ConnVertexMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }
}
