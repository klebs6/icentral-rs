crate::ix!();

#[derive(Debug,Clone)]
pub struct SubGraphMap<GH> {
    name: String,
    data: MaybeIndexedMap<Arc<GH>>,
}

impl<GH> SubGraphMap<GH> {

    pub fn set_subgraph_map_for_node(
        &mut self, 
        node:         NodeId, 
        subgraph_map: Arc<GH>) 
    {
        self.data.set(node,subgraph_map)
    }

    pub fn subgraph_for_node(&self, node: NodeId) -> Arc<GH> {
        self.data[node].clone()
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn has_mapping_for_node(&self, node: NodeId) -> bool {
        self.data.contains(node)
    }

    pub fn iter(&self) -> MaybeIndexedMapIterator<'_,Arc<GH>> {
        self.data.iter()
    }

    pub fn empty_indexed(name: &str) -> Self {

        debug!("creating new empty_indexed SubGraphMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }

    pub fn empty_mapped(name: &str) -> Self {

        debug!("creating new empty_mapped SubGraphMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }
}
