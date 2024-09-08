crate::ix!();

impl<GH> ConnectedComponentSize for Graph<GH> {

    /// do a dfs from this guy to figure out the
    /// size of the connected component connected
    /// to the bcc through v and this edge
    ///
    fn connected_component_size_through_v_and_this_edge(
        &self, 
        visit_markers: &mut VisitMarkers, 
        u:             NodeId) -> usize 
    {
        let mut cnt: usize = 0;

        let mut stack: Stack<NodeId> = default!();

        stack.push(u);

        visit_markers.visit(u);

        while let Some(vv) = stack.pop() {

            cnt += 1;

            let nbr_vec = self.neighbors(vv);

            for &nbr in nbr_vec.iter() {

                if visit_markers.unvisited(nbr) {

                    visit_markers.visit(nbr);

                    stack.push(nbr);
                }
            }
        }

        cnt
    }
}
