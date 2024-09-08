crate::ix!();

pub struct TimingUpdateConfig {

    /// None means do all sources, not approx
    ///
    pub limit_sources: Option<usize>,

    /*algo flag*/

    pub do_brandes:    bool,

    /* brandes time */

    pub num_threads:   usize,

    /// in case the edges are already in the graph
    ///
    pub del_edge:      bool,

    pub op:            Operation,
}

impl Default for TimingUpdateConfig {

    fn default() -> Self {
        Self {
            limit_sources: None,
            do_brandes:    true,
            num_threads:   1,
            del_edge:      false,
            op:            Operation::Insertion,
        }
    }
}

/**
  | Actual timing details are done here
  | 
  */
pub fn timing_update_bc_graph<GH>(
    graph:         &mut Graph<GH>,
    edge_vec:      &mut Vec<Edge>,
    algo_flag:     &CompType,
    brandes_time: Option<Duration>,
    config:       TimingUpdateConfig) 
-> Result<(),BetweennessCentralityError> 
{
    #[cfg(target_feature = "mpi")]
    {
        let mut brandes_time:   Duration = brandes_time.unwrap_or(Duration::default());

        let del_edge:          bool = config.del_edge;
        let do_brandes:        bool = config.do_brandes;
        let num_threads:      usize = config.num_threads;
        let op:           Operation = config.op;

        let universe = mpi::initialize().unwrap();
        let world    = universe.world();

        let rank = world.rank();
        let size = world.size();

        //let mut status: MPI_Status = zeroed!();;

        let mut tm: Timer = Timer::default();

        let graph_len = graph.len();

        let mut scores = graph.create_scores_vector();

        let mut tm_vec:      Vec<Duration> = vec![];
        let mut speedup_vec: Vec::<f64> = vec![];

        if do_brandes {

            tm.start();

            // fast_brandes_BC(graph, scores);
            scores = brandes_bc(graph,None)?;

            tm.stop();

            brandes_time = tm.interval();
        }

        if rank == 0 {

            graph.print_header();

            debug!("Brandes_tm[{:.2?}]", brandes_time);
        }

        for i in 0..edge_vec.len() {

            let e: Edge = edge_vec[i];

            if del_edge {
                graph.remove_edge(&e);
            }

            tm.start();

            update_bc(
                &mut scores, 
                graph, 
                algo_flag.clone(), 
                e, 
                Some(num_threads), 
                Some(op.clone())
            );

            tm.stop();

            let e_time = tm.interval();

            tm_vec.push(e_time);

            let e_speedup: f64 = brandes_time.div_duration_f64(e_time);

            speedup_vec.push(e_speedup);

            if rank == 0 {

                debug!(
                    "e({:.6},{:.6})  tm[{:.2?}]  sup[{:.2}]", 
                    e.src, 
                    e.dst, 
                    e_time, 
                    e_speedup
                );
            }

            if del_edge {
                graph.insert_edge(&e);
            }

            // synchronization barrier so no one
            // starts next edge before others
            //
            world.barrier();
        }

        let tm_stats      = SimpleStats::from(&mut tm_vec);
        let speedup_stats = SpeedupStats::from(&mut speedup_vec);

        if rank == 0 {

            debug!(
                "Avg.tm[{:.2?}]  Avg.sup[{:.2?}]", 
                tm_stats.mean, 
                speedup_stats.mean
            );
        }

        Ok(())
    }

    Err(BetweennessCentralityError::NoMPI)
}
