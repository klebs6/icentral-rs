crate::ix!();

pub struct MucSpeedupStats {
    pub muc_num_nodes:    usize,
    pub muc_num_edges:    usize,
    pub graph_len:        usize,
    pub graph_num_edges:  usize,
    pub ideal_speedup:    f64,
    pub edge_ins_time:    Duration,
}

impl fmt::Debug for MucSpeedupStats {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let binding = f.debug_struct("MucSpeedupStats");

        let mut builder = binding;

        builder.field("muc_num_nodes",  &self.muc_num_nodes);
        builder.field("G_n",            &(self.muc_num_nodes as f64 / self.graph_len as f64));

        builder.field("muc_num_edges",  &self.muc_num_edges);
        builder.field("G_M",            &(self.muc_num_edges as f64 / self.graph_num_edges as f64));

        builder.field("speedup_ideal",  &self.ideal_speedup);
        builder.field("time_to_update", &self.edge_ins_time);

        builder.finish()
    }
}
