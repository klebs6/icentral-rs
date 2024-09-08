crate::ix!();

impl<GH> InitBetweennessCentrality for Graph<GH> 
where GH: GetLimitedNodeIdRange,
      Self: GetLimitedNodeIdRange
{
    fn init_bc(&mut self) 
    -> Result<(),BetweennessCentralityError>  
    {
        if !self.bc_computed {

            self.scores = brandes_bc(self,None)?;

            for id in NodeIdRange::new(0,self.scores.len()) {

                let score = self.scores.score_for_node(id);

                self.scores.set_score_for_node(id, score);
            }

            self.bc_computed = true;
        }

        Ok(())
    }
}
