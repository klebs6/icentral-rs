crate::ix!();

pub struct ArticulationPointFinderContext<'a> {
    pub(crate) color_vec: &'a mut ColorMap,
    pub(crate) low_vec:   &'a mut DistanceMap,
    pub(crate) distances: &'a mut DistanceMap,
    pub(crate) pred_vec:  &'a mut PredecessorMap,
    pub(crate) time:      &'a mut f64,
}

impl<'a> ArticulationPointFinderContext<'a> {

    pub(crate) fn articulation_point_dfs_visitor_step_back_edge(
        &mut self,
        v:             NodeId,
        u:             NodeId,
    ) -> Result<(),BetweennessCentralityError> {

        let predecessor_for_node_u = self.pred_vec.predecessor_for_node(u);

        debug!("articulation_point_dfs_visitor_step_back_edge, v={}, u={}, predecessor_for_node_u={}", v, u, predecessor_for_node_u);

        if v != predecessor_for_node_u {

            // (u, v) is a back edge
            self.low_vec.set_distance_for_node(
                u, 
                min(
                    FloatOrd(self.low_vec.distance(u)),
                    FloatOrd(self.distances.distance(v))
                ).0
            );

        } else {

            warn!("predecessor_for_node_u == v, is this actually a warning or is it expected?");
        }

        Ok(())
    }
}
