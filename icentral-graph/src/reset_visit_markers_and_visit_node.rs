crate::ix!();

impl<GH> ResetVisitMarkersAndVisitNode for Graph<GH> {

    fn reset_visit_markers_and_visit_node(
        &mut self, 
        node: NodeId)
    {
        debug!("in {}, resetting visit markers to *false*", self.name());

        self.visit_markers.fill(false);

        debug!("in {}, visiting single node {}", self.name(), node);

        self.visit_markers.visit(node);
    }
}
