crate::ix!();

impl<GH> FindAllMucSubgraphs for Graph<GH> 

where GH
: GetEdges
+ ClearMucs
+ CreateNamedEmpty
+ Debug
+ ExtendWith<GH,Error=BetweennessCentralityError>
+ FindConnectedComponents<GH,Error=BetweennessCentralityError>
+ GetConnectedComponentSizes
+ GetNeighborsForNode
+ GetNodeIdRange
+ HasMapForNode
+ InsertEdge
+ InsertNode
+ IsValid
+ MappedNodes
+ NewFromCycleVec
+ NewFromGraphRef<Self>
+ NumEdges
+ NumNodes
+ RemoveBridges
{
    /// find the disconnected subgraphs that will
    /// result from the deletion of connection
    /// vertices
    ///
    fn find_all_muc_subgraphs(&mut self) {

        debug!("finding MinimumUnionCycle subgraphs...");

        for m in 0..self.mucs.len() {
            self.find_muc_subgraphs(mucid![m]);
        }
    }
}
