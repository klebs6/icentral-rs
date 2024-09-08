crate::ix!();

pub trait CreateScoresVector {

    fn create_scores_vector(&self) -> BetweennessScores;
}

pub trait SpawnScores {

    fn spawn_scores(&self) -> BetweennessScores;
}

pub trait BrandesIterUpdatePairDependenciesAndFill {

    fn brandes_iter_update_pair_dependencies_and_fill(
        &mut self, 
        s:      NodeId, 
        scores: &mut BetweennessScores) 
    -> Result<(),BetweennessCentralityError>;
}
