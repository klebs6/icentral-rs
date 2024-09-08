crate::ix!();

#[test] fn test_icentral() {

    setup_test_logger![];

    todo!();
}

pub fn create_scores_for_threads(num_threads: usize) 
-> Vec<Arc<Mutex<BetweennessScores>>> 
{
    let mut delta_bc_of_vertices_vec = vec![];

    for t in 0..num_threads {

        let my_scores_name = format!["scores_on_thread_{}", t];

        let my_scores = BetweennessScores::empty_mapped(&my_scores_name);

        delta_bc_of_vertices_vec.push(arcmut![my_scores]);
    }

    delta_bc_of_vertices_vec
}

pub fn icentral_serial_iter(
    node:                 NodeId,
    workspace:            &mut ICentralWorkspace,
    src_distances:        &DistanceMap,
    dst_distances:        &DistanceMap,
    delta_bc_of_vertices: &mut BetweennessScores,
    component:            &mut Component,
    mut edge:             Edge,
    op:                   Operation)
{
    let d_src = src_distances.distance(node);
    let d_dst = dst_distances.distance(node);

    debug!("icentral_serial_iter, source node={}, distance_to_src={}, distance_to_dst={}", node, d_src, d_dst);

    if d_src != d_dst {

        debug!("d_src={}, not equal to d_dst={}", d_src, d_dst);

        // dd=d_v1-d_v2
        let dd: f64 = d_src - d_dst;

        icentral_iter(
            delta_bc_of_vertices, 
            component, 
            node, 
            edge, 
            workspace, 
            Some(dd), 
            Some(false), 
            Some(op.clone())
        );
    }
}

pub fn icentral_serial(
    delta_bc_of_vertices: &mut BetweennessScores,
    component:            &mut Component,
    mut edge:             Edge,
    op:                   Operation)
-> Result<(), BetweennessCentralityError> 
{
    let len = component.num_nodes();

    debug!("reinitializing delta_bc_of_vertices to len={}", len);

    delta_bc_of_vertices.reinit(len);

    let src_distances = component.find_single_source_shortest_paths(edge.src)?;
    let dst_distances = component.find_single_source_shortest_paths(edge.dst)?;

    debug!("creating workspace for iCentral, of len={}", len);

    let mut workspace = ICentralWorkspace::new_init_all(
        len, 
        "icentral_serial::workspace"
    );

    for node in NodeIdRange::new(0,len) {

        icentral_serial_iter(
            node,
            &mut workspace,
            &src_distances,
            &dst_distances,
            delta_bc_of_vertices, 
            component, 
            edge, 
            op
        )
    }

    Ok(())
}

/**
  | Computes increments to BC in in @component.subgraph
  | after edge @edge is inserted
  | 
  | TODO: should handle any kind of component
  | (graph/BiconnectedComponents/MinimumUnionCycle)
  |
  |     vector<double>& delta_bc_of_vertices,     //values will be updated in place
  */
pub fn icentral(
    num_threads:          usize,
    delta_bc_of_vertices: &mut BetweennessScores,
    component:            Arc<Mutex<Component>>,
    mut edge:             Edge,
    op:                   Option<Operation>)
-> Result<(), BetweennessCentralityError> 
{
    debug!("running icentral with {} threads", num_threads);

    let op: Operation = op.unwrap_or(Operation::Insertion);

    match num_threads {

        1 => {

            if let mut component = component.lock()? {

                debug!("locked component");

                debug!("will perform icentral_serial");

                icentral_serial(
                    delta_bc_of_vertices,
                    &mut component,
                    edge,
                    op
                )?;

                debug!("unlocking component");
            }
        }
        _ => {

            debug!("will perform icentral_parallel");
            todo!("implement me");

            /*
            icentral_parallel(
                num_threads,
                delta_bc_of_vertices,
                component,
                edge,
                op
            )?;
            */
        }
    }

    Ok(())
}

#[disable]
fn icentral_parallel(
    num_threads:          usize,
    delta_bc_of_vertices: &mut BetweennessScores,
    component:            Arc<Mutex<Component>>,
    mut edge:             Edge,
    op:                   Operation) 
-> Result<(),BetweennessCentralityError>
{
    #[cfg(target_feature = "mpi")]
    {
        assert!(num_threads > 1);

        let mut all_sources_vec: Vec<NodeId> = vec![];

        let subgraph_len = component.lock()?.num_nodes();

        delta_bc_of_vertices.reinit( subgraph_len );

        let (src_distances,dst_distances) = component.lock()?.create_distance_maps(&edge)?;

        for source in NodeIdRange::new(0,subgraph_len) {

            if src_distances.distance(source) != dst_distances.distance(source) {

                all_sources_vec.push(source);
            }
        }

        // MPI shit goes here:
        //
        // all_sources_vec at each machine must have
        // only its share of the sources then continue
        // normally, note that each process will
        // finish its shit and have it's contribution
        // in its delta_bc_of_vertices
        //
        let universe = mpi::initialize().unwrap();
        let world    = universe.world();
        let size     = world.size();
        let rank     = world.rank();

        if all_sources_vec.len() < size.try_into()? {
            return Ok(());
        }

        let mut machine_source_vec: Vec<NodeId> = vec![];

        let num_s_per_machine = i32::try_from(
            all_sources_vec.len() / usize::try_from(size)?
        )?;

        let mut start: i32 = 0;
        let mut end:   i32 = 0;

        start = rank * num_s_per_machine;

        end = start + num_s_per_machine;

        if rank == size - 1 {

            end = all_sources_vec.len().try_into()?;
        }

        for i in start..end {

            machine_source_vec.push(
                all_sources_vec[usize::try_from(i)?]
            );
        }

        all_sources_vec = machine_source_vec;

        // printf("RANK[%d] -- num sources [%d]", rank, all_sources_vec.len());
        // ////////////
        let mut thread_source_vec: Vec<Arc<Mutex<Vec<NodeId>>>> = vec![];

        thread_source_vec.resize(
            num_threads, 
            default!()
        );

        if all_sources_vec.len() < num_threads {
            return Ok(());
        }

        let num_s_per_thread = i32::try_from(
            all_sources_vec.len() / num_threads
        )?;

        let mut t: i32 = -1;

        for i in 0..all_sources_vec.len() {

            if i % usize::try_from(num_s_per_thread)? == 0 
                && t < i32::try_from(num_threads)? - 1
            {
                t += 1;
            }

            thread_source_vec[usize::try_from(t)?].lock().unwrap().push(all_sources_vec[i]);
        }

        let mut thread_vec: Vec::<JoinHandle<()>> = Vec::with_capacity(num_threads);

        let mut delta_bc_of_vertices_vec = create_scores_for_threads(num_threads);

        // start the threads
        for t in 0..num_threads {

            let op = op.clone();

            let thread_source_vec_item        = thread_source_vec[t].clone();
            let delta_bc_of_vertices_vec_item = delta_bc_of_vertices_vec[t].clone();
            let component                     = component.clone();

            thread_vec[t] = thread::spawn(
                move || {

                    match icentral_block(
                        delta_bc_of_vertices_vec_item,
                        component,
                        edge,
                        thread_source_vec_item,
                        op)
                    {
                        Ok(res) => { }
                        Err(e) => {
                            tracing::error!("thread hit error! {:?}", e);
                        }
                    }
            }
            );
        }

        // wait for the threads to finish, then
        // accumulate the delta_bc_of_vertices
        while let Some(item) = thread_vec.pop() {
            item.join();
        }

        for t in 0..num_threads {

            for v in delta_bc_of_vertices.nodeid_range() {

                let item = delta_bc_of_vertices_vec[t].lock().unwrap();

                delta_bc_of_vertices.increase_score_for_node(
                    v, 
                    item.score_for_node(v)
                );
            }
        }

        let mut fout = File::create("dBC")?;

        for node in delta_bc_of_vertices.nodeid_range() {

            fout.write_all(
                format!(
                    "{}", 
                    delta_bc_of_vertices.score_for_node(node)
                ).as_bytes()
            )?;
        }

        Ok(())
    }

    Err(BetweennessCentralityError::NoMPI)
}
