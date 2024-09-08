crate::ix!();

impl<GH> FindSingleSourceShortestPaths for Graph<GH> {

    fn find_single_source_shortest_paths(&self, s: NodeId) 
    -> Result<DistanceMap,BetweennessCentralityError> 
    {
        let distances_name = name!(self.name(), format!("sssp_distances_for_{}", s));

        let mut distances = DistanceMap::new(
            self.num_nodes(), 
            distances_name
        );

        let queue_name = name![
            self.name(), 
            format!("find_single_source_shortest_paths_for_{}::queue", s)
        ];

        let mut queue = NodeIdQueue::empty(queue_name);

        queue.enqueue(s);

        distances.set_zero_distance(s);

        while let Some(v) = queue.dequeue() {

            let nbr_vec = self.neighbors(v);

            for &nbr in nbr_vec.iter() {

                if distances.is_infinite(nbr) {

                    distances.set_one_step_away(nbr, v);

                    queue.enqueue(nbr);
                }
            }
        }

        Ok(distances)
    }
}
