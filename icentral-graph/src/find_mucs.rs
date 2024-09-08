crate::ix!();

impl<GH> FindMucs for Graph<GH> 

where GH
: GetEdges
+ ClearMucs
+ CreateNamedEmpty
+ Debug
+ BccGraphHashInterface
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
    /// 1. find bridges
    ///
    /// 2. delete bridges
    ///
    /// 3. find connected components of size >=3
    /// (these are MinimumUnionCycles)
    ///
    fn find_mucs_fast(&mut self) 
    -> Result<(),BetweennessCentralityError> 
    {
        let mut bridge_vec: Vec<Edge> = self.find_bridge_edges::<GH>();

        let mut gh: GH = GH::new_from_graph_ref(&*self, name![self.name(),"find_mucs_fast::gh"]);

        gh.remove_bridges(bridge_vec);

        let mut conn_comp_vec: Vec<GH> = gh.find_conn_comp()?;

        self.construct_mucs(conn_comp_vec);

        self.construct_single_node_mucs();

        self.find_conn_verts();

        self.find_all_muc_subgraphs();

        debug!("DONE");

        Ok(())
    }

    fn find_mucs(&mut self)  {
        
        //find_muc_mcb();
        self.find_mucs_fast();
    }
}
