crate::ix!();

pub const DEFAULT_SCORE: f64 = 0.0;

#[derive(Clone,Debug)]
pub struct BetweennessScores {
    name: String,
    data: MaybeIndexedMap<f64>,
}

impl GetNodeIdRange for BetweennessScores {

    fn nodeid_range(&self) 
    -> Vec<NodeId>
    {
        self.data.nodeid_range()
    }
}

impl GetLimitedNodeIdRange for BetweennessScores {

    fn limited_nodeid_range(&self, cap: Option<usize>) 
    -> Vec<NodeId>
    {
        self.data.limited_nodeid_range(cap)
    }
}

impl BetweennessScores {

    pub fn new_from_graph_ref<G: NumNodes + MappedNodes>(gh: &G, name: &str) -> Self {

        debug!("creating new BetweennessScores named {} from Graph of len {}", name, gh.num_nodes());

        BetweennessScores::new_from_nodeids(gh.mapped_nodes(), name)
    }

    pub fn new_from_nodeids(nodes: Vec<NodeId>, name: &str) -> Self {

        debug!("creating new BetweennessScores named {} from {} nodes", name, nodes.len());

        let mut res = Self::empty_mapped(name);

        for node in nodes {
            res.set_score_for_node(node, 0.0);
        }

        res
    }

    pub fn empty_indexed(name: &str) -> Self {

        debug!("creating new empty_indexed BetweennessScores named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }

    pub fn empty_mapped(name: &str) -> Self {

        debug!("creating new empty_mapped BetweennessScores named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }

    pub fn new(len: usize, name: &str) -> Self {

        debug!("creating new BetweennessScores named {} of len {}", name, len);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::new(len, DEFAULT_SCORE),
        }
    }

    pub fn reinit(&mut self, len: usize) {

        debug!("reinitializing BetweennessScores {} to len {}", self.name, len);

        self.data.refill(len, DEFAULT_SCORE);
    }

    pub fn halve(&mut self) {

        self.data.halve();

        debug!("halved BetweennessScores {} -- new values {:?}", self.name, self.data);
    }

    pub fn clear(&mut self) {

        debug!("clearing BetweennessScores {}", self.name);

        self.data.clear();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn increase_score_for_node(
        &mut self, 
        node: NodeId, 
        val:  f64) 
    {
        debug!("in {}, increasing betweenness score for {} by {}", self.name, node, val);

        self.data[node] += val;
    }

    pub fn decrease_score_for_node(
        &mut self, 
        node: NodeId, 
        val:  f64) 
    {
        debug!("in {}, decreasing betweenness score for {} by {}", self.name, node, val);

        self.data[node] -= val;
    }

    pub fn score_for_node(&self, node: NodeId) -> f64 {
        self.data[node]
    }

    pub fn set_score_for_node(
        &mut self, 
        node: NodeId, 
        val:  f64) 
    {
        debug!("in {}, setting betweenness score for {} to {}", self.name, node, val);

        self.data[node] = val;
    }
}
