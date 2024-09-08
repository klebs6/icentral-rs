crate::ix!();

pub const INFINITE_DISTANCE: f64 = f64::MAX;

#[derive(Clone,Debug)]
pub struct DistanceMap {
    name: String,
    data: MaybeIndexedMap<f64>,
}

impl Default for DistanceMap {
    fn default() -> Self {
        Self::empty("default_distance_map")
    }
}

impl CreateNamedEmpty for DistanceMap {

    fn empty(name: &str) -> Self 
    {
        Self::empty_mapped(name)
    }
}

impl DistanceMap {

    pub fn new_from_nodes(nodes: Vec<NodeId>, name: &str) -> Self 
    {
        debug!("creating new DistanceMap named {} from {} nodes", name, nodes.len());

        let mut builder = Self::empty_mapped(name);

        for node in nodes {

            builder.set_distance_for_node(
                node,
                INFINITE_DISTANCE
            );
        }

        builder
    }

    pub fn empty_indexed(name: &str) -> Self 
    {
        debug!("creating new empty_indexed DistanceMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }

    pub fn empty_mapped(name: &str) -> Self 
    {
        debug!("creating new empty_mapped DistanceMap named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }

    pub fn new(len: usize, name: &str) -> Self 
    {
        debug!("creating new DistanceMap named {} of len: {}", name, len);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::new(len,INFINITE_DISTANCE),
        }
    }

    pub fn v_closer_than_u(
        &self, 
        v: NodeId, 
        u: NodeId) -> bool 
    {
        debug!("in {}, checking if v={} is closer than u={}", self.name, v, u);

        self.distance(v) < self.distance(u)
    }

    pub fn len(&self) -> usize 
    {
        self.data.len()
    }

    pub fn is_farther_away(&self, v: NodeId, u: NodeId) -> bool 
    {
        debug!("in {}, checking if v={} is farther away than u={}", self.name, v, u);

        self.data[v] > self.data[u]
    }

    pub fn is_farther_than_one_away(&self, v: NodeId, u: NodeId) -> bool 
    {
        debug!("in {}, checking if v={} is farther than one away than u={}", self.name, v, u);

        self.data[v] > (self.data[u] + 1.0)
    }

    pub fn is_one_step_away(&self, v: NodeId, u: NodeId) -> bool 
    {
        debug!("in {}, checking if v={} is one step away from u={}", self.name, v, u);

        self.data[v] == (self.data[u] + 1.0)
    }

    pub fn is_infinite(&self, node: NodeId) -> bool 
    {
        let result = self.distance(node) == INFINITE_DISTANCE;

        debug!("in {}, checking if the distance to node {} is INFINITE (or possibly unmapped) -- found {}", self.name, node, result);

        result
    }

    pub fn distance(&self, node: NodeId) -> f64 
    {
        self.data[node]
    }

    pub fn distance_ref(&self, node: NodeId)
    -> &f64 
    {
        &self.data[node]
    }

    pub fn distance_mut(&mut self, node: NodeId)
    -> &mut f64 
    {
        &mut self.data[node]
    }

    pub fn set_distance_for_node(&mut self, node: NodeId, val: f64) 
    {
        debug!("in {}, setting distance for node {} to {}", self.name, node, val);

        self.data.set(node, val);
    }

    pub fn insert_node_at_infinite_distance(&mut self, node: NodeId) 
    {
        self.set_distance_for_node(node,INFINITE_DISTANCE);
    }

    pub fn set_one_step_away(
        &mut self, 
        dst: NodeId,
        src: NodeId) 
    {
        self.set_distance_for_node(dst, self.data[src] + 1.0);
    }

    pub fn set_zero_distance(&mut self, source: NodeId) 
    {
        self.set_distance_for_node(source, 0.0);
    }

    pub fn reinit(&mut self, len: usize) 
    {
        debug!("reinitializing DistanceMap {} to len {}", self.name, len);

        self.data.refill(len,INFINITE_DISTANCE);
    }
}
