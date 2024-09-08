crate::ix!();

pub trait PairDependencyForNode {

    fn pair_dependency_for_node(&self, node: NodeId) -> f64;
}

pub trait SetPairDependencyForNode {

    fn set_pair_dependency_for_node(&mut self, node: NodeId, val: f64);
}

pub const NO_DEPENDENCY: f64 = 0.0;

#[derive(Debug)]
pub struct PairDependencies {
    name: String,
    data: MaybeIndexedMap<f64>,
}

impl PairDependencyForNode for PairDependencies {

    fn pair_dependency_for_node(&self, node: NodeId) -> f64 {
        let dep = self.data[node];

        debug!("in {}, pair_dependency for node {} is {}", self.name, node, dep);

        dep
    }
}

impl SetPairDependencyForNode for PairDependencies {

    fn set_pair_dependency_for_node(&mut self, node: NodeId, val: f64) {

        debug!("in {}, setting pair_dependency for node {} to {}", self.name, node, val);

        self.data[node] = val;
    }
}

impl PairDependencies {

    pub fn empty_indexed(name: &str) -> Self {

        debug!("creating new empty_indexed PairDependencies named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }

    pub fn empty_mapped(name: &str) -> Self {

        debug!("creating new empty_mapped PairDependencies named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }

    pub fn new(len: usize, name: &str) -> Self {

        debug!("creating new PairDependencies named {} of len: {}", name, len);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::new(len,NO_DEPENDENCY),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn fill(&mut self, val: f64) {

        debug!("filling PairDependencies {} with val: {}", self.name, val);

        self.data.fill(val);
    }

    pub fn increment_pair_dependency_for_node(&mut self, node: NodeId, val: f64) {

        debug!("in {}, incrementing pair_dependency for node {} by {}", self.name, node, val);

        self.data[node] += val;
    }

    pub fn update(&mut self, 
        v_p:   NodeId, 
        v_n:   NodeId, 
        sp_sn: f64) 
    {
        let t0 = self.data[v_p];
        let t1 = self.data[v_n];

        self.data[v_p] = t0 + sp_sn * (1.0 + t1);
    }

    pub fn reinit(&mut self, len: usize) {

        debug!("in {}, reinitializing PairDependencies to len {}", self.name, len);

        self.data.refill(len, NO_DEPENDENCY);
    }
}
