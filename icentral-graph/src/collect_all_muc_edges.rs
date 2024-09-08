crate::ix!();

impl<GH> CollectAllMucEdges for Graph<GH> 

where GH:
GetConnectedComponentSizes
+ ExtendWith<GH>
+ GetEdges
+ GetNeighborsForNode
+ GetNodeIdRange
+ HasMapForNode
+ InsertEdge
+ InsertNode
+ MappedNodes
+ NumEdges
+ NumNodes
{

    fn collect_all_edges_in_mucs_in_one_set(&mut self) 
    -> Result<Edges,BetweennessCentralityError> 
    {
        let all_muc_edges_name = name![
            self.name(), 
            "collect_all_edges_in_mucs_in_one_set::all_muc_edges"
        ];

        let mut all_muc_edges = Edges::empty(all_muc_edges_name);

        for i in 0..self.mucs.len() {

            if self.mucs[i].is_valid() {

                all_muc_edges.extend(
                    &self.mucs[i].edges()
                );
            }
        }

        Ok(all_muc_edges)
    }
}
