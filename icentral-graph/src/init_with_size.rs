crate::ix!();
   
impl<GH> InitWithSize for Graph<GH> {

    fn init_size(&mut self, num_nodes: usize)  {
        
        info!("in {}, init_size: {}", self.name(), num_nodes);

        self.nodes_map.reinit(num_nodes);

        self.nodes_to_mucs.reinit(num_nodes);

        self.init_internals();
    }
}
