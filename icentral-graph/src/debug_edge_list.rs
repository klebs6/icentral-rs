crate::ix!();

pub struct EdgeListDebugger<'g,G> {
    host: &'g G,
}

pub trait GetEdgeListDebugger: Sized {

    fn edgelist_debugger<'g>(&'g self) -> EdgeListDebugger<'g,Self>;
}

impl<GH> GetEdgeListDebugger for Graph<GH> {

    fn edgelist_debugger<'g>(&'g self) -> EdgeListDebugger<'g,Self> 
    {
        EdgeListDebugger {
            host: self,
        }
    }
}

impl<'g,G: GetEdges> fmt::Debug for EdgeListDebugger<'g,G> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(self.host.edges().iter())
            .finish()
    }
}
