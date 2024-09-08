crate::ix!();

pub trait GetSigmaValueForNode {

    fn sigma_value_for_node(&self, node: NodeId) -> f64;
}

pub trait SetSigmaValueForNode {

    fn set_sigma_value_for_node(&mut self, node: NodeId, val: f64);
}

pub const SIGMA_ZERO: f64 = 0.0;

#[derive(Clone,Debug)]
pub struct SigmaMap {
    name: String,
    data: MaybeIndexedMap<f64>,
}

impl GetSigmaValueForNode for SigmaMap {

    fn sigma_value_for_node(&self, node: NodeId) -> f64 
    {
        self.data[node]
    }
}

impl SetSigmaValueForNode for SigmaMap {

    fn set_sigma_value_for_node(&mut self, node: NodeId, val: f64) 
    {
        debug!("setting sigma value for node {} to {}", node, val);

        self.data.set(node, val);
    }
}

impl SigmaMap {

    pub fn empty_indexed(name: &str) -> Self {

        debug!("creating new empty_indexed SigmaMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }

    pub fn empty_mapped(name: &str) -> Self {

        debug!("creating new empty_mapped SigmaMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }

    pub fn new(len: usize, name: &str) -> Self {

        debug!("creating new SigmaMap named {} of len: {}", name, len);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::new(len,SIGMA_ZERO),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn update(&mut self, 
        v_p:   NodeId, 
        v_n:   NodeId, 
        sp_sn: f64) 
    {
        let t0 = self.data[v_p];
        let t1 = self.data[v_n];

        let val = t0 + t1 * sp_sn;

        debug!(
            "updating sigma value for node {} to {} = {} + {} * {}", 
            v_p, 
            val, 
            t0, 
            t1, 
            sp_sn
        );

        self.data[v_p] = val;
    }

    pub fn ratio(&self, v_p: NodeId, v_n: NodeId) -> f64 
    {
        let ratio = self.data[v_p] / self.data[v_n];

        debug!("computed sigma ratio {} for nodes {} and {}", ratio, v_p, v_n);

        ratio
    }

    pub fn set_node_to_zero(&mut self, node: NodeId) 
    {
        debug!("setting sigma value for node {} to zero", node);

        self.data.set(node, 0.0);
    }

    pub fn fill(&mut self, val: f64) 
    {
        debug!("filling SigmaMap with value: {}", val);

        self.data.fill(val);
    }

    pub fn set_node_to_one(&mut self, node: NodeId) 
    {
        debug!("setting sigma value for node {} to one", node);

        self.data.set(node, 1.0);
    }

    pub fn increment_sigma_value_for_node(&mut self, v_n: NodeId, c_t: f64) 
    {
        debug!("increasing SigmaMap value for node: {} by {}", v_n, c_t);

        self.data[v_n] += c_t;
    }

    pub fn reinit(&mut self, len: usize)
    {
        debug!("refilling SigmaMap to len: {}", len);

        self.data.refill(len, SIGMA_ZERO);
    }
}
