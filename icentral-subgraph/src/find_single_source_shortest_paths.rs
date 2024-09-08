crate::ix!();

impl FindSingleSourceShortestPaths for SubGraph {

    fn find_single_source_shortest_paths(&self, s: NodeId) 
    -> Result<DistanceMap,BetweennessCentralityError> 
    {
        debug!("finding single source shortest paths from source {}", s);

        let mut distances = DistanceMap::new(
            self.nodes_map.len(), 
            &format!{"node{}_sssp_distance", s}
        );

        let queue_name = name![
            self.name(),
            "find_single_source_shortest_paths::queue"
        ];

        let mut queue = NodeIdQueue::empty(queue_name);

        queue.enqueue(s);

        distances.set_zero_distance(s);

        while let Some(v) = queue.dequeue() {

            let nbrs = self.neighbors(v).clone();

            for &nbr in nbrs.iter() {

                if distances.is_infinite(nbr) {

                    distances.set_one_step_away(nbr,v);

                    queue.enqueue(nbr);
                }
            }
        }

        Ok(distances)
    }
}
