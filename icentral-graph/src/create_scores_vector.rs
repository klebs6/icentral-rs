crate::ix!();

impl<GH> CreateScoresVector for Graph<GH> {

    fn create_scores_vector(&self) -> BetweennessScores
    {
        debug!("in {}, creating scores vector", self.name());

        let name = name![self.name(), "create_scores_vector::scores"];

        BetweennessScores::new(self.num_nodes(), name)
    }
}
