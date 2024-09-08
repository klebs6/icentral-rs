crate::ix!();

impl<GH> GetNumMucs for Graph<GH> {

    fn get_num_mucs(&mut self) -> usize {
        
        self.mucs.iter().filter(|x| x.is_valid()).count()
    }
}
