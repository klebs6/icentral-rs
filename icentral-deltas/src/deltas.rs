crate::ix!();

pub const DELTA_ZERO: f64 = 0.0;

#[derive(Clone,Debug)]
pub struct DeltaMap {
    name: String,
    data: MaybeIndexedMap<f64>,
}

impl DeltaMap {

    pub fn empty_indexed(name: &str) -> Self {

        debug!("creating new empty_indexed DeltaMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }

    pub fn empty_mapped(name: &str) -> Self {

        debug!("creating new empty_mapped DeltaMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }

    pub fn new(len: usize, name: &str) -> Self {

        debug!("creating new DeltaMap named {} of len: {}", name, len);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::new(len,DELTA_ZERO),
        }
    }

    pub fn clear(&mut self) {

        debug!("clearing DeltaMap {} of len {}", self.name, self.len());

        self.data.clear();
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

        self.data[v_p] = t0 + t1 * sp_sn;
    }

    pub fn ratio(&self, v_p: NodeId, v_n: NodeId) -> f64 {

        let ratio = self.data[v_p] / self.data[v_n];

        debug!("in {}, computed delta ratio {} for nodes {} and {}", self.name, ratio, v_p, v_n);

        ratio
    }

    pub fn set_zero(&mut self, node: NodeId) {

        debug!("in {}, setting delta value for node {} to zero", self.name, node);

        self.data.set(node, 0.0);
    }

    pub fn set_one(&mut self, node: NodeId) {

        debug!("in {}, setting delta value for node {} to one", self.name, node);

        self.data[node] = 1.0;
    }

    pub fn get(&self, node: NodeId) -> f64 {
        self.data[node]
    }

    pub fn set(&mut self, node: NodeId, val: f64) {
        self.data.set(node, val);
    }

    pub fn increment_delta(&mut self, v_n: NodeId, c_t: f64) {

        debug!("in {}, increasing delta value for node={} by {}", self.name, v_n, c_t);

        self.data[v_n] += c_t;
    }

    pub fn attenuate_delta(&mut self, v_n: NodeId, c_t: f64) {

        debug!("in {}, decreasing delta value for node: {} by {}", self.name, v_n, c_t);

        self.data[v_n] -= c_t;
    }

    pub fn reinit(&mut self, len: usize) {

        self.fill_to_len(len, DELTA_ZERO);
    }

    pub fn fill_to_len(&mut self, len: usize, val: f64) {

        debug!("in {}, refilling DeltaMap to len: {} with val: {}" , self.name, len, val);

        self.data.refill(len,val);
    }
}
