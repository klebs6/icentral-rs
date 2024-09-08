crate::ix!();

pub trait SetMucDebug {

    fn set_muc_debug(&self, val: bool);
}

pub trait GetMucDebuggerWithoutNodes: Sized {

    fn muc_debugger_without_nodes<'g>(&'g self) -> MinimumUnionCycleDebugger<'g,Self>;
}

pub trait GetMucDebuggerWithNodes: Sized {

    fn muc_debugger_with_nodes<'g>(&'g self) -> MinimumUnionCycleDebugger<'g,Self>;
}

//----------------------------
pub struct MinimumUnionCycleDebugger<'g,G> {
    host:       &'g G,
    with_nodes: bool,
}

impl<'g,GH> fmt::Debug for MinimumUnionCycleDebugger<'g,Graph<GH>> 
where 
    Graph<GH>
    : SetMucDebug 
    + GetMucs<GH> 
    + SetPrintNodes 
    + Debug,
    GH
    : ExtendWith<GH,Error=BetweennessCentralityError> 
    + Debug
    + GetConnectedComponentSizes
    + GetEdges 
    + GetNeighborsForNode
    + GetNodeIdRange
    + HasMapForNode
    + InsertEdge
    + InsertNode
    + MappedNodes
    + NumEdges
    + NumNodes
    + SetPrintNodes
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        self.host.set_muc_debug(self.with_nodes);

        f.debug_list()
            .entries(self.host.get_mucs().iter())
            .finish()
    }
}

impl<GH> SetMucDebug for Graph<GH> 
where GH: GraphHashMucInterface
{
    fn set_muc_debug(&self, val: bool) {

        for muc in self.mucs.iter() {
            muc.set_print_nodes(val);
        }
    }
}

impl<GH> GetMucDebuggerWithoutNodes for Graph<GH> {

    fn muc_debugger_without_nodes<'g>(&'g self) -> MinimumUnionCycleDebugger<'g,Self> 
    {
        MinimumUnionCycleDebugger {
            host:       self,
            with_nodes: false,
        }
    }
}

impl<GH> GetMucDebuggerWithNodes for Graph<GH> {

    fn muc_debugger_with_nodes<'g>(&'g self) -> MinimumUnionCycleDebugger<'g,Self> 
    {
        MinimumUnionCycleDebugger {
            host:       self,
            with_nodes: true,
        }
    }
}
