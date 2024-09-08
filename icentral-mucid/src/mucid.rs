crate::ix!();

pub trait MucIdForNode {

    fn mucid_for_node(&self, node: NodeId) -> MinimumUnionCycleId;
}

pub trait SetMucIdForNode {

    fn set_mucid_for_node(
        &mut self, 
        node:  NodeId, 
        mucid: MinimumUnionCycleId
    );
}

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Eq,Ord,Hash)] 
pub struct MinimumUnionCycleId(usize);

impl fmt::Display for MinimumUnionCycleId {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MinimumUnionCycleId({})", self.0)
    }
}

impl MinimumUnionCycleId {

    /// try to eliminate this if possible
    ///
    pub fn val(&self) -> usize {
        self.0
    }

    pub fn inf() -> Self {
        Self(usize::MAX)
    }
}

impl Default for MinimumUnionCycleId {

    fn default() -> Self {
        Self::inf()
    }
}

impl From<usize> for MinimumUnionCycleId {

    fn from(x: usize) -> MinimumUnionCycleId {
        Self(x)
    }
}

#[macro_export] macro_rules! mucid {
    ($id:expr) => {
        MinimumUnionCycleId::from($id)
    }
}
