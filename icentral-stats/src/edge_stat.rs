crate::ix!();

#[derive(Default,Debug)]
pub struct EdgeStat {
    pub edge_ins_time:      Duration,
    pub muc_bc_update_time: Duration,
    pub muc_num_nodes:      usize,
    pub muc_num_edges:      usize,
}
