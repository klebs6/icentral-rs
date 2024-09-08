crate::ix!();

impl FindPruningCounts for SubGraph {

    fn find_pruning_counts_exp(&mut self, 
        src:    NodeId,
        dst:    NodeId) 
    -> Result<(i32,i32,i32),BetweennessCentralityError> 
    {
        let src_distances = self.find_single_source_shortest_paths(src)?;
        let dst_distances = self.find_single_source_shortest_paths(dst)?;

        let mut d0 = 0;
        let mut d1 = 0;
        let mut d2 = 0;

        for node in self.nodes_map.nodeid_range() {

            let diff:     f64 = src_distances.distance(node) - dst_distances.distance(node);
            let abs_diff: f64 = diff.abs();

            match abs_diff {
                0.0  => d0 += 1,
                1.0  => d1 += 1,
                _    => d2 += 1,
            }
        }

        Ok((d0,d1,d2))
    }
}
