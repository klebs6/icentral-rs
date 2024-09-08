crate::ix!();

//-------------------------------------------[icentral/src/_bc_mem.cc]

/*************************************************
 * EXPERIMENTAL
 *************************************************/
pub fn update_bc_mem<GH>(
    bc_mem:     &mut BcMemWorkspace,
    scores:     &mut BetweennessScores,
    graph:      &Graph<GH>,
    edge:       Edge,
    workspaces: &mut WorkspaceMap) 
-> Result<(), BetweennessCentralityError> 
where Graph<GH>: MappedNodes + GetNodeIdRange
{
    let component  = Component::new_from_graph_ref(graph, "update_bc_mem::component");

    let delta_bc_of_vertices = icentral_mem(
        bc_mem,
        component, 
        edge, 
        workspaces
    )?;

    for node in delta_bc_of_vertices.nodeid_range() {

        scores.increase_score_for_node(
            node,  
            delta_bc_of_vertices.score_for_node(node)
        );
    }

    Ok(())
}

fn icentral_mem(
    bc_mem:     &mut BcMemWorkspace,
    component:  Component,
    edge:       Edge,
    workspaces: &mut WorkspaceMap) 
-> Result<BetweennessScores,BetweennessCentralityError> 
{
    let subgraph_len = component.num_nodes();

    let mut delta_bc_of_vertices = BetweennessScores::new(subgraph_len, "icentral_mem::delta_bc_of_vertices");

    let (src_distances, dst_distances) = component.create_distance_maps(&edge)?;

    let component = arcmut![component];

    for source in NodeIdRange::new(0,subgraph_len) {

        if src_distances.distance(source) != dst_distances.distance(source) {

            let workspace = workspaces.workspace_mut(source);

            compute_bc_increments_and_decrements_of_a_subgraph_in_component(
                bc_mem,
                &mut delta_bc_of_vertices, 
                component.clone(), 
                source, 
                edge, 
                workspace
            );
        }
    }

    Ok(delta_bc_of_vertices)
}

fn update_deltas_and_distances_for_parents(
    bc_mem:    &mut BcMemWorkspace,
    workspace: &mut ICentralWorkspace, 
    src:       NodeId, 
    dst:       NodeId) 
-> Result<(),BetweennessCentralityError> 
{
    let parents = workspace.parents_for_node(dst);

    for &parent in parents.iter() {

        workspace.attenuate_deltas(parent,dst);

        let parent_dist: usize = workspace.distance(parent) as usize;
        let src_dist:    usize = workspace.distance(src)    as usize;

        if parent_dist == src_dist + 1 {
            bc_mem.level_push(parent_dist,parent);
        }
    }

    workspace.clear_node_parents(dst);

    Ok(())
}

fn compute_new_path_counts_and_paths(
    bc_mem:    &mut BcMemWorkspace,
    workspace: &mut ICentralWorkspace, 
    src:       NodeId, 
    dst:       NodeId) 
-> Result<(),BetweennessCentralityError> 
{
    bc_mem.new_parents_add_parent(dst,src);

    // Compute new path counts and paths
    if !workspace.distance_is_one_step_away(dst,src) {

        // lost_parents[dst] = parents[dst];
        update_deltas_and_distances_for_parents(
            bc_mem,
            workspace, 
            src, 
            dst
        )?;
    }

    // parents[dst].push_back(src);
    bc_mem.new_sigmas_increment_sigma_value_for_node(
        dst, 
        bc_mem.new_sigmas_sigma_value_for_node(src)
    );

    Ok(())
}

pub fn construct_queue_for_bc_increment_computation(
    workspace: &mut ICentralWorkspace, 
    src:       NodeId, 
    dst:       NodeId)
-> Result<NodeIdQueue,BetweennessCentralityError>
{
    let mut queue = NodeIdQueue::empty("construct_queue_for_bc_increment_computation::queue");

    queue.enqueue(dst);

    workspace.set_distance_one_step_away(dst,src);

    workspace.inc_sigmas_set_sigma_value_for_node(
        dst, 
        workspace.sigma_value_for_node(src)
    );

    workspace.visit(dst);

    Ok(queue)
}

pub fn maybe_initialize_new_sigmas_for_bc_increment_computation(
    bc_mem:    &mut BcMemWorkspace,
    workspace: &mut ICentralWorkspace)
-> Result<(), BetweennessCentralityError>  
{
    use std::sync::{Once, ONCE_INIT};

    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            bc_mem.set_new_sigmas_from(workspace.new_sigmas());
        });
    }

    bc_mem.set_new_sigmas_from(workspace.sigmas());

    Ok(())
}

pub fn orient_edge_for_distance(
    workspace: &mut ICentralWorkspace, 
    edge:      &Edge) -> Edge 
{
    let src_distance = workspace.distance(edge.src);
    let dst_distance = workspace.distance(edge.dst);

    match src_distance > dst_distance {
        true  => edge.reversed(),
        false => *edge,
    }
}

pub fn update_level_vec(
    bc_mem:    &mut BcMemWorkspace,
    workspace: &mut ICentralWorkspace, 
    v:         NodeId) 
-> Result<(), BetweennessCentralityError>  
{
    let v_dist: usize = workspace.distance(v) as usize;

    if v_dist >= bc_mem.level_vec_len() {

        let mut qq: Vec<NodeId> = default!();

        bc_mem.push_level(qq);
    }

    bc_mem.level_push(v_dist,v);

    Ok(())
}

pub fn bc_increment_process_neighbor(
    bc_mem:    &mut BcMemWorkspace,
    neighbor:  NodeId,
    queue:     &mut NodeIdQueue,
    workspace: &mut ICentralWorkspace, 
    v:         NodeId) 
-> Result<(), BetweennessCentralityError>  
{
    let neighbor_dist = workspace.distance(neighbor);
    let v_dist        = workspace.distance(v);

    match neighbor_dist {
        _ if neighbor_dist > (v_dist + 1.0) => workspace.process_outer_layer_neighbor(
            bc_mem,
            neighbor, 
            queue, 
            v
        )?,
        _ if neighbor_dist == (v_dist + 1.0) => workspace.process_first_layer_neighbor(
            bc_mem,
            neighbor, 
            queue, 
            v
        )?,
        _ => {
            unreachable!();
        }
    }

    Ok(())
}

pub fn bc_increment_process_neighbors(
    bc_mem:    &mut BcMemWorkspace,
    queue:     &mut NodeIdQueue,
    workspace: &mut ICentralWorkspace, 
    nbr_vec:   &Vec<NodeId>, 
    v:         NodeId) 
-> Result<(), BetweennessCentralityError>  
{
    for &neighbor in nbr_vec.iter() {

        bc_increment_process_neighbor(
            bc_mem,
            neighbor, 
            queue, 
            workspace, 
            v
        )?;
    }

    Ok(())
}

pub fn bc_increment_drain_queue(
    bc_mem:    &mut BcMemWorkspace,
    workspace: &mut ICentralWorkspace, 
    component: Arc<Mutex<Component>>,
    queue:     &mut NodeIdQueue) 
-> Result<(), BetweennessCentralityError>  
{
    while let Some(v) = queue.dequeue() {

        update_level_vec(bc_mem,workspace,v)?;

        let component = component.lock()?;

        let nbr_vec = component.neighbors(v);

        bc_increment_process_neighbors(bc_mem, queue, workspace,&nbr_vec,v)?;
    }

    Ok(())
}

pub fn maybe_update_level_vec(
    bc_mem: &mut BcMemWorkspace,
    level:  usize,
    v:      NodeId) 
-> Result<(),BetweennessCentralityError>
{
    if level > 0 {
        bc_mem.level_push(level - 1,v);
    }

    Ok(())
}

/**
  | Computes the increments/decrements
  | to BC of a subgraph in @component
  | 
  | This function deals with nodes indexed
  | from 0 to N-1 in the passed subgraph and
  | knows nothing about the original graph,
  | the caller must add the deltas to the
  | BC vector of the original graph
  | 
  | Assumes @workspace has shortest path
  | info, so no BBFS is done
  | 
  | IMP: for now works for Graph component only
  |
  */
pub fn compute_bc_increments_and_decrements_of_a_subgraph_in_component(
    bc_mem:               &mut BcMemWorkspace,
    delta_bc_of_vertices: &mut BetweennessScores,
    component:            Arc<Mutex<Component>>,
    source:               NodeId,
    inserted_edge:        Edge,
    workspace:            &mut ICentralWorkspace) 

-> Result<(), BetweennessCentralityError>  
{
    let len = component.lock()?.num_nodes();

    //    RBFS(delta_bc_of_vertices, component, source, workspace, false, true);
    //    partial_BBFS(workspace, component, source, inserted_edge);
    //    RBFS(delta_bc_of_vertices, component, source, workspace, true, false);
    //    fill(workspace.inc_sigmas.begin(), workspace.inc_sigmas.end(), 0);
    //    fill(workspace.visit_markers.begin(), workspace.visit_markers.end(), false);

    bc_mem.new_parents_fill_to_len(len,vec![]);

    maybe_initialize_new_sigmas_for_bc_increment_computation(
        bc_mem,
        workspace
    )?;

    let edge = orient_edge_for_distance(workspace,&inserted_edge);

    let (src,dst) = (edge.src, edge.dst);

    // S.clear();
    bc_mem.level_vec_resize_default(
        (workspace.distance(dst) + 1.0) as usize, 
    );

    compute_new_path_counts_and_paths(
        bc_mem,
        workspace,
        src,
        dst
    )?;

    let mut queue = construct_queue_for_bc_increment_computation(
        workspace,
        src,
        dst
    )?;

    bc_increment_drain_queue(
        bc_mem, 
        workspace, 
        component.clone(), 
        &mut queue
    )?;

    let len = component.lock()?.num_nodes();

    workspace.fill_visit_markers_to_len(len,false);

    for level in (0..=bc_mem.level_vec_len() - 1).rev() {

        while let Some(w) = bc_mem.level_pop(level) {

            increment_computation_process_unvisited(
                bc_mem,
                workspace, 
                w, 
                level, 
                source, 
                delta_bc_of_vertices
            )?;
        }
    }

    workspace.set_deltas_from_new_deltas();

    workspace.set_sigmas_from_other(bc_mem.new_sigmas());

    workspace.inc_sigmas_fill_sigmas(0.0);

    workspace.fill_visit_markers(false);

    Ok(())
}

pub fn increment_computation_process_unvisited(
    bc_mem:               &mut BcMemWorkspace,
    workspace:            &mut ICentralWorkspace, 
    w:                    NodeId, 
    level:                usize,
    source:               NodeId,
    delta_bc_of_vertices: &mut BetweennessScores) 
-> Result<(),BetweennessCentralityError>
{
    if workspace.unvisited(w) {

        workspace.visit(w);

        let parents = workspace.parents_for_node(w);

        for &v in parents.iter() {

            let attenuation_step = workspace.calculate_deltas_step(v,w);

            workspace.attenuate_delta_value_for_node(
                v, 
                attenuation_step
            );

            let sigma_ratio = bc_mem.new_sigmas_sigma_ratio(v,w);

            let augmentation_step = workspace.calculate_deltas_step_with_sigma_ratio(
                sigma_ratio,
                w
            );

            workspace.increment_delta_value_for_node(
                v, 
                augmentation_step
            );

            maybe_update_level_vec(bc_mem,level,v)?;
        }

        for v in bc_mem.new_parents_parents_for_node(w) {

            let sigma_ratio = bc_mem.new_sigmas_sigma_ratio(v,w);

            workspace.increment_delta_value_for_node(
                v, 
                sigma_ratio * (1.0 + workspace.delta_value_for_node(w))
            );

            if level > 0 {
                bc_mem.level_push(level - 1,v);
            }
        }

        // has to update parents with new_parents
        // for the next edge insertions
        //
        workspace.set_parents_for_node(
            w, 
            bc_mem.new_parents_parents_for_node(w)
        );

        if w != source {

            delta_bc_of_vertices.decrease_score_for_node(
                w, 
                workspace.delta_value_for_node(w) / 2.0
            );

            delta_bc_of_vertices.increase_score_for_node(
                w, 
                workspace.delta_value_for_node(w) / 2.0
            );
        }
    }

    Ok(())
}
