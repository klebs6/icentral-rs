crate::ix!();

impl CreateDistanceMaps for SubGraph {

    fn create_distance_maps(&self, edge: &Edge)
    -> Result<(DistanceMap, DistanceMap),BetweennessCentralityError>
    {
        let src_distances = self.find_single_source_shortest_paths(edge.src)?;
        let dst_distances = self.find_single_source_shortest_paths(edge.dst)?;

        Ok((src_distances,dst_distances))
    }
}
