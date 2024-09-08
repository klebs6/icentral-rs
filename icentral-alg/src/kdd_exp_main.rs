crate::ix!();

pub fn kdd_exp_main<GH>(
    rank:  i32,
    size:  i32,
    world: &dyn Communicator) 
-> Result<(),BetweennessCentralityError> 
where GH: BccGraphHashInterface,
      Graph<GH>: HasEdge + NumEdges
{
    let mut args = std::env::args();

    // -----------------------------[these are for scanning]
    let mut num_edges:   usize = 0;
    let mut num_threads: usize = 0;
    let mut rand_seed:   i32 = 0;
    let mut t1:          i32 = 0;
    let mut t2:          i32 = 0;
    let mut t3:          i32 = 0;
    let mut t4:          i32 = 0;

    let mut do_icent:        bool = false;
    let mut do_bcc_icent:    bool = false;
    let mut ext_edges:       bool = false;
    let mut do_fast_brandes: bool = false;
    let mut do_brandes:      bool = false;
    let mut do_qube:         bool = false;
    let mut do_inc_qube:     bool = false;

    let mut op: Operation = Operation::default();

    // for qube and incremental qube, old
    // implementation will be used, and no
    // parallelization
    //
    let mut path_vec:       Vec<String>    = vec![];
    let mut edge_vec2:      Vec<Vec<Edge>> = vec![];
    let mut brandes_tm_vec: Vec<Duration>  = vec![];

    let universe = mpi::initialize().unwrap();
    let world    = universe.world();

    let mut status: MPI_Status = zeroed!();;
    
    if args.len() != 2 {

        if rank == 0 {

            debug!("Pass one parameter, path with experiment details");

            debug!("deletion");

            debug!("num_edges, num_threads, rand_seed");

            debug!("external_edges, do_icent, do_bcc_icent");

            debug!("do_fast_brandes, do_brandes, do_qube, do_inc_qube");

            debug!("list of graph paths");

            debug!("if external_edges is nonzero a file with graph_name.edges is expected");

            debug!("external edges are assumed to be in the graph");

            debug!("external edges should not be bridges");

            debug!("if deletion is 1, bcc_icent with deletions will be invoked");
        }

        return Ok(());

    } else {

        {
            let mut fin = std::fs::File::open(
                args.nth(1).unwrap()
            ).unwrap().bytes().map(|ch| ch.unwrap());

            let mut del_int: i32 = read!("{}",fin);

            if del_int == 1 {
                op = Operation::Deletion;
            } else {
                op = Operation::Insertion;
            }

            text_io::scan!(fin => "{}, {}, {}", num_edges, num_threads, rand_seed);
            text_io::scan!(fin => "{}, {}, {}", t1, t2, t3);

            ext_edges    = (t1 != 0);
            do_icent     = (t2 != 0);
            do_bcc_icent = (t3 != 0);

            text_io::scan!(fin => "{}, {}, {}, {}", t1, t2, t3, t4);

            do_fast_brandes = (t1 != 0);
            do_brandes      = (t2 != 0);
            do_qube         = (t3 != 0);
            do_inc_qube     = (t4 != 0);

            while let Ok(path) = try_read!("{}", fin) {
                path_vec.push(path);
            }
        }

        // fill edges, read from file if needed
        //
        if ext_edges {

            // these are edges in the graph, and
            // have to be removed then inserted in
            // order
            //
            for i in 0..path_vec.len() {

                let mut edge_vec: Vec<Edge> = vec![];

                let edge_file_path: String
                = format!("{}.edges", path_vec[i]);

                let fin = File::open(&edge_file_path)?;

                let mut v1: usize = 0;
                let mut v2: usize = 0;

                for line in io::BufReader::new(fin).lines() {

                    if let Ok(line) = line {

                        let mut parts = line.split(" ");
                        v1 = parts.nth(0).unwrap().parse::<usize>()?;
                        v2 = parts.nth(1).unwrap().parse::<usize>()?;
                        edge_vec.push(Edge::new_with_ids(v1,v2));
                    }
                }

                edge_vec2.push(edge_vec);
            }

        } else {

            for parents in 0..path_vec.len() {

                let mut graph = arcmut![Graph::<GH>::from_filename(&path_vec[parents])];

                let mut edge_vec: Vec<Edge> = vec![];

                // gen_rand_edges(num_edges, graph, edge_vec);
                // edge_vec2.push_back(edge_vec);
                if rank == 0 {

                    let mut rng = WyRand::new_seed(rand_seed.try_into()?);

                    if let mut graph = graph.lock()? {

                        // master generates random
                        // edges and sends to everyone
                        //
                        edge_vec = match op {
                            Operation::Insertion => gen_rand_edges(&mut rng, num_edges, &mut *graph)?,
                            Operation::Deletion  => gen_rand_edges_deletions::<Graph<GH>,GH>(num_edges, &mut *graph),
                        };

                        // TMP code to make sure new
                        // random edges are
                        // generated.. Safely remove!
                        //
                        // for(int re = 0; re < edge_vec.len(); ++re) {
                        //     debug!("e(%d, %d)", edge_vec[re].first, edge_vec[re].second);
                        // }
                        //
                        // debug!("RANK[%d] -- e[0]: %u %u", rank, edge_vec[4].first, edge_vec[4].second);
                        //
                        for parents in 1..size {

                            /*
                                world.process_at_rank(parents)
                                    .send_with_tag(&edge_vec,0);
                            */

                            unsafe {
                                MPI_Send(
                                    edge_vec.as_ptr() as *const libc::c_void, 
                                    (edge_vec.len() * size_of::<Edge>()).try_into()?, 
                                    RSMPI_UINT8_T, 
                                    parents, 
                                    0, 
                                    RSMPI_COMM_WORLD
                                );
                            }
                        }
                    }

                } else {

                    // slaves get random edges
                    // from the master
                    //
                    edge_vec.resize(
                        num_edges, 
                        Edge::default()
                    );

                    /*
                        world.process_at_rank(0)
                            .send_with_tag(&edge_vec, MPI_ANY_TAG);
                    */

                    unsafe {
                        MPI_Send(
                            edge_vec.as_ptr() as *const libc::c_void, 
                            (edge_vec.len() * size_of::<Edge>()).try_into()?, 
                            RSMPI_UINT8_T, 
                            0, 
                            MPI_ANY_TAG, 
                            RSMPI_COMM_WORLD
                        );
                    }

                    // debug!("RANK[%d] -- e[0]: %u %u", rank, edge_vec[4].first, edge_vec[4].second);
                }

                edge_vec2.push(edge_vec);
            }
        }
    }

    brandes_tm_vec.resize(
        path_vec.len(),
        Duration::from_secs(1)
    );

    let mut tm: Timer = default!();

    let mut scores: Vec<f64> = vec![];

    let mut brandes_time = Duration::from_secs(0);

    if do_bcc_icent {

        if rank == 0 {

            debug!("");

            debug!(
                "Starting BiconnectedComponents+iCentral [{} threads] [{}]...", 
                num_threads, 
                match op == Operation::Deletion {
                    true   => "Deletion",
                    false  => "Insertion"
                }
            );

            debug!("========================================");
        }

        for i in 0..path_vec.len() {

            let mut graph = Graph::<GH>::from_filename(&path_vec[i]);

            let config = TimingUpdateConfig {
                do_brandes: false,
                del_edge:   ext_edges,
                num_threads,
                op,
                limit_sources: None
            };

            timing_update_bc_graph(
                &mut graph, 
                &mut edge_vec2[i], 
                &CompType::BiconnectedComponent, 
                Some(brandes_tm_vec[i]), 
                config
            );

            // synchronization barrier so that no
            // one starts next graph before others
            //
            world.barrier();
        }
    }

    Ok(())
}
