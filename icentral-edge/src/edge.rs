crate::ix!();

///-------------------------------------------------
/// src and dst are intentionally public
///
#[derive(Copy,Clone,Eq,Hash,PartialOrd,Ord,PartialEq)]
pub struct Edge {

    pub src: NodeId,
    pub dst: NodeId,
}

impl fmt::Debug for Edge {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "Edge({} <-> {})", self.src, self.dst)
    }
}

impl fmt::Display for Edge {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "({} <-> {})", self.src, self.dst)
    }
}

impl Edge {

    pub fn connects_nodes(&self, 
        v_i: NodeId, 
        v_n: NodeId) -> bool 
    {
        let connects: bool = 
        v_i == self.src && v_n == self.dst 
        || 
        v_i == self.dst && v_n == self.src ;

        debug!(
            "checking whether edge {} connects nodes {} and {} -- connects? {}", 
            self, 
            v_i, 
            v_n,
            connects
        );

        connects
    }

    /// This constructor is used in the case where
    /// we have numeric values only
    ///
    pub fn new_with_ids(src: usize, dst: usize) -> Self {

        Edge::new(nodeid![src], nodeid![dst])
    }

    pub fn new(src: NodeId, dst: NodeId) -> Self {

        let edge = Self { src, dst };

        debug!("created new edge: {}", edge);

        edge
    }

    pub fn random(node_max: usize) -> Self {

        let mut rng = WyRand::new();

        let n0 = rng.generate_range(0..node_max);
        let n1 = rng.generate_range(0..node_max);

        Self::new_with_ids(n0, n1)
    }

    pub fn reversed(&self) -> Self {
        Self {
            src: self.dst,
            dst: self.src,
        }
    }
}

impl Default for Edge {

    fn default() -> Self {
        Self {
            src: NodeId::bad(),
            dst: NodeId::bad(),
        }
    }
}
