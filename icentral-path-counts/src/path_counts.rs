crate::ix!();


pub const NO_PATHS: usize = 0;

#[derive(Clone,Debug)]
pub struct PathCounts {
    name: String,
    data: MaybeIndexedMap<usize>,
}

impl Default for PathCounts {
    fn default() -> Self {
        Self::empty_indexed("default_path_counts")
    }
}

pub trait PathCountForNode {

    fn path_count_for_node(&self, node: NodeId) -> usize;

    fn path_count_for_node_ref(&self, node: NodeId) -> &usize;

    fn path_count_for_node_mut(&mut self, node: NodeId) -> &mut usize;
}

impl PathCountForNode for PathCounts {

    fn path_count_for_node(&self, node: NodeId) -> usize {
        self.data[node]
    }

    fn path_count_for_node_ref(&self, node: NodeId) -> &usize {
        &self.data[node]
    }

    fn path_count_for_node_mut(&mut self, node: NodeId) -> &mut usize {
        &mut self.data[node]
    }
}

impl PathCounts {

    pub fn empty_indexed(name: &str) -> Self {

        debug!("creating new empty_indexed PathCounts named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_indexed()
        }
    }

    pub fn empty_mapped(name: &str) -> Self {

        debug!("creating new empty_mapped PathCounts named {}", name);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::empty_mapped()
        }
    }

    pub fn new(len: usize, name: &str) -> Self {

        debug!("creating new PathCounts named {} of len: {}", name, len);

        Self {
            name: name.to_owned(),
            data: MaybeIndexedMap::new(len,NO_PATHS),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn increment_path_count_for_node(&mut self, node: NodeId, val: usize) {

        debug!("in {}, increasing PathCount for node: {} by {}", self.name, node, val);

        self.data[node] += val;
    }

    pub fn increment_path_count_for_node_from(
        &mut self, 
        node:  NodeId, 
        other: NodeId) 
    {
        let other_path_count = self.data[other];
        let node_path_count  = self.data[node];

        debug!(
            "in {}, increasing PathCount for node: {} (currently {}) by the path count from the {} (currently {})", 
            self.name,
            node, 
            node_path_count,
            other,
            other_path_count, 
        );

        self.data[node] += other_path_count;
    }

    pub fn update_path_counts(
        &mut self, 
        dst: NodeId,
        src: NodeId) 
    {
        self.set_path_count_for_node(dst, self.data[src]);
    }

    pub fn set_path_count_for_node(
        &mut self, 
        node:  NodeId, 
        count: usize) 
    {
        debug!("in {}, setting path count for node {} to {}", self.name, node, count);

        self.data[node] = count;
    }

    pub fn path_count_ratio(&self, v_p: NodeId, v_n: NodeId) -> f64 {

        let ratio = self.path_count_for_node(v_p) as f64 / self.path_count_for_node(v_n) as f64;

        debug!("in {}, computed path_count ratio {} for nodes {} and {}", self.name, ratio, v_p, v_n);

        ratio
    }

    /// Convenience method for the common case of
    /// setting the path count to *one*
    ///
    pub fn set_path_count_to_one(&mut self, source: NodeId) {

        self.set_path_count_for_node(source,1);
    }

    /// Convenience method for the common case of
    /// zeroing the path count
    ///
    pub fn set_path_count_to_zero(&mut self, source: NodeId) {

        self.set_path_count_for_node(source,0);
    }

    pub fn reinit(&mut self, len: usize) {

        debug!("reinitializing PathCounts {} to len: {}", self.name, len);

        self.data.clear();

        self.data.refill(
            len,
            NO_PATHS
        );
    }
}
