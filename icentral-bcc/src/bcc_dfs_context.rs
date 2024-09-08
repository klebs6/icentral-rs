crate::ix!();

pub struct BccDfsVisitorContext<'a> {
    pub(crate) color_vec:  &'a mut ColorMap,
    pub(crate) low_vec:    &'a mut DistanceMap,
    pub(crate) distances:  &'a mut DistanceMap,
    pub(crate) pred_vec:   &'a mut PredecessorMap,
    pub(crate) edge_stack: &'a mut Stack<Edge>,
    pub(crate) time:       &'a mut f64,
}

impl<'a> BccDfsVisitorContext<'a> {

    pub(crate) fn bcc_dfs_visitor_step_colored(
        &mut self,
        v:          NodeId,
        u:          NodeId,

    ) -> Result<(),BetweennessCentralityError> {

        if v != self.pred_vec.predecessor_for_node(u) 
            && self.distances.v_closer_than_u(v, u) {

            // (u, v) is a back edge
            self.edge_stack.push(Edge::new(u,v));

            self.low_vec.set_distance_for_node(
                u, 
                min(
                    FloatOrd(self.low_vec.distance(u)),
                    FloatOrd(self.distances.distance(v))
                ).0
            );
        }

        Ok(())
    }

    pub(crate) fn step_time_and_update_distances_for_node(&mut self, u: NodeId) {

        *self.time += 1.0;

        self.distances.set_distance_for_node(u, *self.time);

        self.low_vec.set_distance_for_node(
            u, 
            self.distances.distance(u)
        );
    }

    delegate!{
        to self.color_vec {
            pub(crate) fn set_color_for_node_grey(&mut self, u: NodeId);
            pub(crate) fn color_for_node(&self, u: NodeId) -> Color;
        }
    }
}
