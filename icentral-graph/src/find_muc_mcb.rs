crate::ix!();

impl<GH> FindMucMcb<GH> for Graph<GH> 
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
    fn find_muc_mcb(&mut self) 
    -> Result<(),BetweennessCentralityError>
    {
        self.mcb_find();

        let mcb_cycle_vec_len = self.mcb.num_cycles();

        let visit_markers_name = name![self.name(), "find_muc_mcb::visit_markers"];

        let mut visit_markers = VisitMarkers::new(mcb_cycle_vec_len,visit_markers_name);

        let cycle_graph_name = name![
            self.name(), 
            "find_muc_mcb::cycle_graph"
        ];

        let cycle_graph = GH::new_from_cycle_vec(
            self.mcb.cycles(), 
            cycle_graph_name
        );

        debug!("{:?}", cycle_graph);

        for idx in NodeIdRange::new(0,self.mcb.num_cycles()) {

            if visit_markers.unvisited(idx) {

                self.find_muc_mcb_for_cycle(
                    idx, 
                    &cycle_graph, 
                    &mut visit_markers
                );
            }
        }

        self.create_single_vertex_mucs();

        self.find_conn_verts();

        self.find_all_muc_subgraphs();

        Ok(())
    }

    fn find_muc_mcb_for_cycle(
        &mut self, 
        src:           NodeId, 
        cycle_graph:   &GH, 
        visit_markers: &mut VisitMarkers) 
    -> Result<(),BetweennessCentralityError>
    {
        let mut muc = MinimumUnionCycle::<GH>::default();

        muc.set_id(self.mucs.len());

        let cycle = self.mcb.cycle(src).clone();

        self.merge_muc_cycle(
            &mut muc, 
            &cycle
        );

        visit_markers.visit(src);

        let queue_name = name![self.name(),"find_muc_mcb_for_cycle::queue"];

        let mut queue = NodeIdQueue::empty(queue_name);

        queue.enqueue(src);

        while let Some(node) = queue.dequeue() {

            let nbrs = cycle_graph.neighbors(node);

            for nbr in nbrs {

                if visit_markers.unvisited(nbr) {

                    visit_markers.visit(nbr);

                    let cycle = self.mcb.cycle(nbr).clone();

                    self.merge_muc_cycle(
                        &mut muc, 
                        &cycle
                    );

                    queue.enqueue(nbr);
                }
            }
        }

        self.mucs.push(muc);

        Ok(())
    }
}
