crate::ix!();

pub trait FindBiconnectedComponent {

    fn find_bicon_component<GH: BccGraphHashInterface>(&mut self, out_vec: &mut Vec<GH>);
}

impl<T> FindBiconnectedComponent for T 
where T: Sized + NumNodes + for<'a> Named<'a> + GetNeighborsForNode
{
    fn find_bicon_component<GH: BccGraphHashInterface>(&mut self, out_vec: &mut Vec<GH>) {

        debug!("initiating Graph::find_bicon_component");

        let size = self.num_nodes();
        
        let mut time: f64 = 0.0;

        let mut u: NodeId = NodeId::zero();

        let color_vec_name = name![self.name(), "find_bicon_component::color_vec"];
        let pred_vec_name  = name![self.name(), "find_bicon_component::pred_vec"];
        let distances_name = name![self.name(), "find_bicon_component::distances"];
        let low_vec_name   = name![self.name(), "find_bicon_component::low_vec"];

        let mut color_vec = ColorMap::new(size, color_vec_name);
        let mut pred_vec  = PredecessorMap::new(size, pred_vec_name);

        let mut distances = DistanceMap::new(size, distances_name);
        let mut low_vec   = DistanceMap::new(size, low_vec_name);

        let mut edge_stack: Stack::<Edge> = default!();

        let mut ctx = BccDfsVisitorContext {
            color_vec:  &mut color_vec, 
            low_vec:    &mut low_vec, 
            distances:  &mut distances, 
            pred_vec:   &mut pred_vec, 
            edge_stack: &mut edge_stack, 
            time:       &mut time, 
        };

        bcc_dfs_visitor(
            self,
            u, 
            &mut ctx,
            out_vec
        );
    }
}
