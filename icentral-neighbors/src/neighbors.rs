crate::ix!();

pub const ISOLATED: Vec<NodeId> = vec![];

pub type Neighbors = Vec<NodeId>;

pub trait GetNeighborsForNode {

    fn neighbors(&self, node: NodeId) -> Neighbors;
}

/// In certain cases,
///
/// (edge.src, dst) needs to be the first edge
/// to recurse from
///
/// IMP: explain this clearly
///
/// Jun 16, 2014.. I have no idea why!
pub fn move_destination_edge_to_front(
    nbrs_vec: &mut Vec<NodeId>, 
    dst:      NodeId) 
{
    for i in 0..nbrs_vec.len() {

        if nbrs_vec[i] == dst {

            nbrs_vec[i] = nbrs_vec[0];

            nbrs_vec[0] = dst;
        }
    }
}
