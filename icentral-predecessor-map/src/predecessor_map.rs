crate::ix!();


/// 1 means grey
///
#[derive(Clone,Debug)]
pub struct PredecessorMap {
    name: String,
    data: MaybeIndexedMap<NodeId>,
}

impl PredecessorMap {

    pub fn new_from_nodes(nodes: Vec<NodeId>, name: &str) -> Self {

        debug!("creating new PredecessorMap named {} from {} nodes", name, nodes.len());

        let mut builder = Self::empty_mapped(name);

        for node in nodes {

            builder.set_predecessor_for_node(
                node,
                NodeId::bad()
            );
        }

        builder
    }

    pub fn empty_indexed(name: &str) -> Self {

        debug!("creating new empty_indexed PredecessorMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }

    pub fn empty_mapped(name: &str) -> Self {

        debug!("creating new empty_mapped PredecessorMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }

    pub fn new(len: usize, name: &str) -> Self {

        debug!("creating new PredecessorMap named {} of len {}", name, len);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::new(len,NodeId::bad()),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn has_predecessor(&self, node: NodeId) -> bool {

        let has_predecessor = self.predecessor_for_node(node) != NodeId::bad();

        debug!("in {}, checking whether node {} has predecessor in the map -- has? {}", self.name, node, has_predecessor);

        has_predecessor 
    }

    pub fn is_tree_root(&self, node: NodeId) -> bool {

        let is_root = self.predecessor_for_node(node) == NodeId::bad();

        debug!("in {}, checking whether node {} is the tree_root -- is? {}", self.name, node, is_root);

        is_root
    }

    pub fn predecessor_for_node(&self, node: NodeId) -> NodeId {

        debug!("in {}, getting predecessor for node {}", self.name, node);

        self.data[node]
    }

    pub fn set_predecessor_for_node(&mut self, node: NodeId, val: NodeId) {

        debug!("in {}, setting predecessor for node {} to {}", self.name, node, val);

        self.data.set(node, val);
    }

    pub fn reinit(&mut self, len: usize) {

        debug!("reinitializing PredecessorMap {} with len {}", self.name, len);

        self.data.refill(len,NodeId::bad());
    }
}
