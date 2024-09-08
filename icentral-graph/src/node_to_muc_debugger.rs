crate::ix!();

pub struct NodeToMinimumUnionCycleDebugger<'g,G> {
    host: &'g G,
}

impl<GH> Graph<GH> {

    pub fn node_to_muc_debugger<'g>(&'g self) -> NodeToMinimumUnionCycleDebugger<'g,Self> 
    {
        NodeToMinimumUnionCycleDebugger {
            host: self,
        }
    }
}

impl<'g,G: GetNodesToMucs> fmt::Debug for NodeToMinimumUnionCycleDebugger<'g,G> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        f.debug_list()
            .entries(self.host.get_nodes_to_mucs().iter())
            .finish()
    }
}

