crate::ix!();

/**
  | TODO remove
  |
  */
#[derive(Default,Debug)]
pub struct BiconnectedComponentsStat {

    /**
      | size of the bcc this edge insertion
      |
      */
    pub bcc_num_nodes:                   usize,
    pub bcc_num_edges:                   usize,

    /**
      | time it takes to do Brandes straight
      | forward iteration
      |
      */
    pub avg_iter_tm_brandes:             f64,

    pub num_d0_iter:                     i32,
    pub num_d1_iter:                     i32,
    pub num_d2_iter:                     i32,
    pub tot_d0_tm:                       Duration,
    pub tot_d1_tm:                       Duration,
    pub tot_d2_tm:                       Duration,
    pub bcc_find_time:                   Duration,
    pub bc_update_time:                  Duration,
    pub single_source_shortest_paths_tm: Duration,
    pub tot_d0_iter:                     i32,
    pub tot_d1_iter:                     i32,
    pub tot_d2_iter:                     i32,
}

impl BiconnectedComponentsStat {

    pub fn update(
        &mut self, 
        cnt_arr: &Vec<i32>, 
        tot_arr: &Vec<Duration>) 
    {
        self.num_d0_iter = cnt_arr[0];
        self.num_d1_iter = cnt_arr[1];
        self.num_d2_iter = cnt_arr[2];
        self.tot_d0_tm   = tot_arr[0];
        self.tot_d1_tm   = tot_arr[1];
        self.tot_d2_tm   = tot_arr[2];
    }
}
