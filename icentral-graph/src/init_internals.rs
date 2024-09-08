crate::ix!();

impl<GH> InitInternals for Graph<GH> {

    type Error = BetweennessCentralityError;

    fn init_internals(&mut self) 
    -> Result<(),Self::Error> 
    {
        debug!("in {}, initializing graph internals", self.name());

        self.visit_markers.reinit(self.num_nodes());

        // scores.resize(size());
        self.bc_computed = false;;

        Ok(())
    }
}
