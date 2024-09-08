crate::ix!();

pub const EMPTY_PARENTS: Vec<NodeId> = vec![];

#[derive(Clone,Debug)]
pub struct ParentsMap {
    name: String,
    data: MaybeIndexedMap<Vec<NodeId>>,
}

impl ParentsMap {

    pub fn new(len: usize, name: &str) -> Self {

        debug!("creating new ParentsMap named {} of len: {}", name, len);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::new(len,EMPTY_PARENTS),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl CreateEmptyIndexed for ParentsMap {

    fn empty_indexed(name: &str) -> Self {

        debug!("creating new empty_indexed ParentsMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }
}

impl CreateEmptyMapped for ParentsMap {

    fn empty_mapped(name: &str) -> Self {

        debug!("creating new empty_mapped ParentsMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }
}

impl NumParentsForNode for ParentsMap {

    fn num_parents_for_node(&self, node: NodeId) -> usize {

        let num_parents = self.data[node].len();

        debug!("computing num_parents_for_node of node {} to be {}", node, num_parents);

        num_parents
    }
}

impl ParentsForNode for ParentsMap {

    /// Probably best if we don't have to clone
    /// these...
    ///
    fn parents_for_node(&self, v_n: NodeId) 
    -> Vec<NodeId> 
    {
        let parents = self.data[v_n].clone();

        debug!("returning {} parents for node {}", parents.len(), v_n);

        parents
    }
}

impl ClearParents for ParentsMap {

    fn clear_parents(&mut self) 
    {
        debug!("clearing all parents of all nodes");

        self.data.clear()
    }
}

impl ClearNodeParents for ParentsMap {

    fn clear_node_parents(&mut self, node: NodeId) 
    {
        debug!("clearing parents for single node {}", node);

        self.data[node].clear()
    }
}

impl SetParentsForNode for ParentsMap {

    fn set_parents_for_node(
        &mut self, 
        node:    NodeId, 
        parents: Vec<NodeId>) 
    {
        self.data.set(node, parents);
    }
}

impl FillToLenWithItems for ParentsMap {

    type Item = NodeId;

    fn fill_to_len(&mut self, len: usize, val: Vec<Self::Item>) {

        debug!("filling ParentsForNode to len {} with val {:?}", len, val);

        self.data[NodeId::zero()..NodeId::from(len)].fill(val);
    }
}

impl SetSingleParent for ParentsMap {

    fn set_single_parent(&mut self, 
        node:   NodeId, 
        parent: NodeId) 
    {

        debug!("setting single parents {} for node {}", parent, node);

        self.clear_node_parents(node);
        self.add_parent(node, parent);
    }
}

impl HasParent for ParentsMap {

    fn has_parent(&self, 
        node:      NodeId, 
        candidate: NodeId) -> bool 
    {
        let does_have = self.data[node].iter().find(|x| **x == candidate).is_some();

        debug!("computing whether node {} has parent {} -- has? {}", node, candidate, does_have);

        does_have
    }
}

impl AddParent for ParentsMap {

    /// NOTE: there might be a way to remove the
    /// gated check `has_parent` first
    ///
    fn add_parent(
        &mut self,
        node:   NodeId,
        parent: NodeId)
    {
        debug!("adding parent {} to node {}", parent, node);

        if !self.has_parent(node,parent) {
            self.data[node].push(parent);
        }
    }
}

impl ReinitWithLen for ParentsMap {

    fn reinit(&mut self, len: usize) {

        debug!("reinitializing ParentsMap to len {}", len);

        self.data.refill(len,EMPTY_PARENTS);
    }
}
