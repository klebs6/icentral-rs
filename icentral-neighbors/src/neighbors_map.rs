crate::ix!();

#[derive(Debug,Clone)]
pub struct NeighborsMap {
    name: String,
    data: MaybeIndexedMap<Neighbors>,
}

impl NeighborsMap {

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn new(len: usize, name: &str) -> Self {

        debug!("creating new NeighborsMap named {} of len: {}", name, len);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::new(len,ISOLATED),
        }
    }

    pub fn new_from_graph_ref<G>(g: &G, name: &str) -> Self 
        where G: Sized + NumNodes + GetNodeIdRange + GetNeighborsForNode
    {
        let num_nodes = g.num_nodes();

        debug!("creating new NeighborsMap named {} from reference to Graph of len: {}", name, num_nodes);

        let mut nodes_map = NeighborsMap::new(num_nodes, name);

        for node in g.nodeid_range() {

            nodes_map.set_neighbors(
                node, 
                g.neighbors(node)
            );
        }

        nodes_map
    }
}

impl MappedNodes for NeighborsMap {

    fn mapped_nodes(&self) -> Vec<NodeId> 
    {
        self.data.keys()
    }
}

impl GetNodeIdRange for NeighborsMap {

    fn nodeid_range(&self) 
    -> Vec<NodeId>
    {
        self.data.nodeid_range()
    }
}

impl GetLimitedNodeIdRange for NeighborsMap {

    fn limited_nodeid_range(&self, cap: Option<usize>) -> Vec<NodeId> {

        self.data.limited_nodeid_range(cap)
    }
}

impl HasMapForNode for NeighborsMap {

    fn has_map_for_node(&self, node: NodeId) -> bool {

        debug!("checking whether NeighborsMap {} has mapping for node: {}", self.name, node);

        self.data.contains(node)
    }
}

impl CreateEmptyIndexed for NeighborsMap {

    fn empty_indexed(name: &str) -> Self {

        debug!("creating new empty_indexed NeighborsMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }
}

impl CreateEmptyMapped for NeighborsMap {

    fn empty_mapped(name: &str) -> Self {

        debug!("creating new empty_mapped NeighborsMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }
}

impl ExtendWith<NeighborsMap> for NeighborsMap {

    type Error = BetweennessCentralityError;

    fn extend_with(&mut self, other: &NeighborsMap) -> Result<(),Self::Error>
    {
        debug!("extending NeighborsMap {} of len: {} with other of len: {}", 
            self.name, 
            self.len(), other.len());

        self.data.extend(&other.data);

        Ok(())
    }
}

impl UnlinkAll for NeighborsMap {

    fn unlink_all(&mut self, src: NodeId, dst: NodeId) {

        debug!("unlinking all edges between src: {} and dst: {}", src, dst);

        self.data[src].retain(|x| *x != dst);
        self.data[dst].retain(|x| *x != src);
    }
}

impl AddEdge for NeighborsMap {

    fn add_edge(&mut self, e: &Edge) {

        debug!("adding edge {} to NeighborsMap", e);

        self.add_neighbor(e.src, e.dst);
        self.add_neighbor(e.dst, e.src);
    }
}

impl UnlinkEdge for NeighborsMap {

    fn unlink_edge(&mut self, e: &Edge) {
        self.unlink_all(e.src, e.dst);
    }
}

impl crate::imports::ReinitWithLen for NeighborsMap {

    fn reinit(&mut self, len: usize) {

        debug!("reinitializing NeighborsMap to len: {}", len);

        self.data.refill(len,ISOLATED);
    }
}

impl Clear for NeighborsMap {

    fn clear(&mut self) {

        debug!("clearing NeighborsMap of len {}", self.len());

        self.data.clear();
    }
}

impl AddIsolatedNode for NeighborsMap {

    fn add_isolated_node(
        &mut self, 
        node: NodeId) 
    {
        debug!("adding node {} to NeighborsMap {} (without specifying neighbors)", node, self.name);

        self.data.set(node, vec![]);
    }
}

impl SetNeighbors for NeighborsMap {

    fn set_neighbors(
        &mut self, 
        node: NodeId, 
        nbrs: Vec<NodeId>) 
    {
        debug!("setting neighbors to {:?} for node {}", nbrs, node);

        self.data.set(node, nbrs);
    }
}

impl AddNeighbor for NeighborsMap {

    fn add_neighbor(
        &mut self, 
        node: NodeId, 
        nbr:  NodeId) 
    {
        debug!("adding neighbor {} to node {}", nbr, node);

        self.data[node].push(nbr);
    }
}

impl RemoveNodeAndNeighbors for NeighborsMap {

    fn remove_node_and_neighbors(&mut self, node: NodeId) {

        debug!("removing node {} and neighbors from NeighborsMap {}", node, self.name);

        self.data.remove(node);
    }
}

impl GetNeighborsForNode for NeighborsMap {

    fn neighbors(&self, node: NodeId) -> Neighbors {
        //TODO: we probably want to get rid of the
        //clone
        self.data[node].clone()
    }
}
