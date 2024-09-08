crate::ix!();

impl<GH> BuildGraphHashMappingForConnVertex<GH> for Graph<GH> 
where GH: InsertNode + InsertEdge + CreateNamedEmpty
{
    fn build_graphhash_mapping_for_conn_vertex_step(
        &self, 
        gh:         &mut GH,
        bfs_source: NodeId,
        conn_vert:  NodeId,
        item:       (NodeId, &Vec<NodeId>),
        muc_id:     MinimumUnionCycleId) 
    -> Result<(),BetweennessCentralityError> 
    {
        let visit_markers_name = name![self.name(), "build_graphhash_mapping_for_conn_vertex_step::visit_markers"];

        let mut visit_markers = VisitMarkers::new(
            self.num_nodes(), 
            visit_markers_name
        );

        visit_markers.visit(bfs_source);
        visit_markers.visit(conn_vert);

        let queue_name = name![
            self.name(),
            "build_graphhash_mapping_for_conn_vertex_step::queue"
        ];

        let mut queue = NodeIdQueue::empty(queue_name);

        queue.enqueue(bfs_source);

        while let Some(node) = queue.dequeue() {

            gh.insert_node(node);

            let nbrs = self.neighbors(node);

            for &bfs_nbr in nbrs.iter() {

                if visit_markers.unvisited(bfs_nbr) {

                    queue.enqueue(bfs_nbr);

                    visit_markers.visit(bfs_nbr);

                    if bfs_nbr != conn_vert {
                        gh.insert_edge(&Edge::new(node, bfs_nbr));
                    }
                }
            }
        }

        Ok(())
    }

    fn build_graphhash_mapping_for_conn_vertex(
        &self, 
        item:   (NodeId, &Vec<NodeId>),
        muc_id: MinimumUnionCycleId) 
    -> Result<(NodeId, Arc<GH>),BetweennessCentralityError> 
    {
        let conn_vert: NodeId = item.0;

        let gh_name = name![
            self.name(), 
            "build_graphhash_mapping_for_conn_vertex::gh"
        ];

        let mut gh: GH = GH::empty(gh_name);

        for &bfs_source in item.1.iter() {

            self.build_graphhash_mapping_for_conn_vertex_step(
                &mut gh, 
                bfs_source,
                conn_vert,
                item, 
                muc_id
            )?;
        }

        Ok((conn_vert,Arc::new(gh)))
    }
}


