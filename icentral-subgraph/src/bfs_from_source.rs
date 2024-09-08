crate::ix!();

impl BfsFromSource for SubGraph {

    /// do a BFS from s, count the number of
    /// vertices, and mark the visited
    ///
    fn do_bfs_from_source_count_vertices_and_mark_visited(
        &self, 
        source:        NodeId, 
        visit_markers: &mut VisitMarkers,
        out_vec:       &mut Vec<i32>)
    -> Result<(),BetweennessCentralityError>
    {
        let queue_name = name![
            self.name(), 
            "do_bfs_from_source_count_vertices_and_mark_visited::queue"
        ];

        let mut queue = NodeIdQueue::empty(queue_name);

        queue.enqueue(source);

        while let Some(node) = queue.dequeue() {

            let nbrs = self.neighbors(node).clone();

            for &nbr in nbrs.iter() {

                if visit_markers.unvisited(nbr) {

                    visit_markers.visit(nbr);

                    queue.enqueue(nbr);

                    *out_vec.iter_mut().last().unwrap() += 1;
                }
            }
        }

        Ok(())
    }
}
