crate::ix!();

#[derive(Clone,Copy,PartialEq,PartialOrd,Eq,Ord,Hash)] 
pub struct NodeId(usize);

impl NodeId {

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn bad() -> Self {
        Self(usize::MAX)
    }

    pub fn val(&self) -> usize {
        self.0
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node{}", self.0)
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node{}", self.0)
    }
}

impl From<usize> for NodeId {
    fn from(x: usize) -> NodeId {
        NodeId(x)
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::bad()
    }
}

#[macro_export] macro_rules! nodeid {
    ($id:expr) => {
        NodeId::from($id)
    }
}
