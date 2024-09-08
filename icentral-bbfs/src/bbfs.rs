crate::ix!();

/**
  | If dd=1 will compute both old values
  | and new values to eliminate need for
  | PartialBFS later
  | 
  | IMP: edge.src is assumed to be closer
  | to the source than edge.dst
  |
  */
pub fn bbfs(
    maybe_config:  Option<BBFSConfig>,
    workspace:     &mut ICentralWorkspace,
    component:     &Component,

    // source of the iteration
    source:        NodeId)
-> Result<(),BetweennessCentralityError> 
{
    let config = maybe_config.unwrap_or(default!());

    let mut queue = bbfs_initialize(&config, workspace, source)?;

    debug!(
        "running bbfs from source: {}, config: {:?}, initial_queue: {:#?}", 
        source, 
        config, 
        queue
    );

    while let Some(nodeid) = queue.dequeue() {

        debug!("bbfs processing nodeid {} from queue", nodeid);

        workspace.stack_push(nodeid);

        bbfs_step(
            workspace,
            nodeid, 
            &config, 
            component, 
            &mut queue
        )?;
    }

    Ok(())
}

fn bbfs_initialize(
    config:    &BBFSConfig,
    workspace: &mut ICentralWorkspace,
    source:    NodeId)
-> Result<NodeIdQueue,BetweennessCentralityError>
{
    let mut queue = NodeIdQueue::empty("bbfs_initialize::queue");

    debug!("bbfs_initialize -- initializing sigma and distance values for source node {}", source);

    //  Assumes workspace is initialized
    workspace.set_sigma_value_for_node(source, 1.0);
    workspace.set_distance_for_node(source, 0.0);

    if config.handle_new_sigmas() {

        debug!("also setting `new_sigmas` sigma value for source node {}", source);

        workspace.new_sigmas_set_sigma_value_for_node(source, 1.0);
    }

    debug!("adding source node to queue {}", queue.name());

    queue.enqueue(source);

    Ok(queue)
}

fn bbfs_rbfs_d1_comp_type_graph_handle_nodeid(
    w:                    NodeId,
    delta_bc_of_vertices: &mut BetweennessScores,
    workspace:            &mut ICentralWorkspace,
    source:               NodeId,
    edge:                 Edge) 
-> Result<(),BetweennessCentralityError> 
{
    let parents = workspace.parents_for_node(w);

    for &v in parents.iter() {

        bbfs_update(workspace,v,w);
    }

    bbfs_update_if_match_edge_dst(workspace, w, &edge);

    if w != source {

        workspace.update_delta_bc_of_vertices_for_node(
            w, 
            delta_bc_of_vertices
        );
    }

    Ok(())
}

fn bbfs_rbfs_d1_comp_type_graph(
    delta_bc_of_vertices: &mut BetweennessScores,
    workspace:            &mut ICentralWorkspace,
    source:               NodeId,
    edge:                 Edge) 
-> Result<(),BetweennessCentralityError> 
{
    for i in (0..=workspace.stack_len() - 1).rev() {

        let w: NodeId = workspace.stack_node_at_index(i);

        bbfs_rbfs_d1_comp_type_graph_handle_nodeid(
            w,
            delta_bc_of_vertices,
            workspace,
            source,
            edge
        )?;
    }

    Ok(())
}

fn bbfs_rbfs_d1_comp_type_bcc_step(
    v_n:                  NodeId,
    delta_bc_of_vertices: &mut BetweennessScores,
    workspace:            &mut ICentralWorkspace,
    component:            &Component,
    source:               NodeId,
    edge:                 Edge) 
-> Result<(),BetweennessCentralityError> 
{
    workspace.maybe_update_capital_deltas_for_component(component,source,v_n);
    workspace.update_deltas_for_each_p(component,source,v_n);

    // IMP: this is the only change that happens
    // to workspace.parents, @src should be added as
    // parent for dst
    if v_n == edge.dst {

        let v_p: NodeId = edge.src;

        workspace.update_new_deltas_with_new_delta_ratio(v_p, v_n);

        let new_sp_sn: f64 = workspace.get_new_delta_ratio(v_p,v_n);

        if component.has_articulation_point(source) {

            workspace.update_new_capital_deltas_with_new_delta_ratio(v_p,v_n);

            delta_bc_of_vertices.set_score_for_node(
                v_p,
                {
                    let t0 = delta_bc_of_vertices.score_for_node(v_p);
                    let t1 = workspace.new_capital_deltas_delta_value_for_node(v_n);
                    t0 + t1 * new_sp_sn / 2.0
                }
            );
        }

        if source != v_n {

            delta_bc_of_vertices.decrease_score_for_node(
                v_n, 
                workspace.delta_value_for_node(v_n) / 2.0
            );

            delta_bc_of_vertices.increase_score_for_node(
                v_n, 
                workspace.new_deltas_delta_value_for_node(v_n) / 2.0
            );
        }

        if component.has_articulation_point(source) {

            let vg_s: f64 = component.subgraph_micentraltude_through_articulation_point(source);

            delta_bc_of_vertices.decrease_score_for_node(
                v_n, 
                workspace.delta_value_for_node(v_n) * vg_s
            );

            delta_bc_of_vertices.decrease_score_for_node(
                v_n, 
                workspace.capital_deltas_delta_value_for_node(v_n) / 2.0
            );

            delta_bc_of_vertices.increase_score_for_node(
                v_n, 
                workspace.new_deltas_delta_value_for_node(v_n) * vg_s
            );

            delta_bc_of_vertices.increase_score_for_node(
                v_n, 
                workspace.new_capital_deltas_delta_value_for_node(v_n) / 2.0
            );
        }
    }

    Ok(())
}

/// BiconnectedComponents or MinimumUnionCycle
/// case: involved case, with external pairs
/// contribution
///
fn bbfs_rbfs_d1_comp_type_bcc(
    delta_bc_of_vertices:  &mut BetweennessScores,
    workspace:             &mut ICentralWorkspace,
    component:             &Component,
    source:                NodeId,
    edge:                  Edge) 
-> Result<(),BetweennessCentralityError> 
{
    for i in (0..=workspace.stack_len() - 1).rev() {

        let v_n: NodeId = workspace.stack_node_at_index(i);

        bbfs_rbfs_d1_comp_type_bcc_step(
            v_n,
            delta_bc_of_vertices,
            workspace,
            component,
            source,
            edge
        )?;
    }

    Ok(())
}

fn bbfs_rbfs_d1(
    delta_bc_of_vertices:  &mut BetweennessScores,
    workspace:             &mut ICentralWorkspace,
    component:             &Component,
    source:                NodeId,
    edge:                  Edge) 
-> Result<(),BetweennessCentralityError> 
{
    bbfs(
        Some(BBFSConfig::new_rbfs_d1(edge)),
        workspace,
        component,
        source
    )?;

    workspace.refill_deltass(component.num_nodes());

    match component.ty() {

        CompType::Graph => bbfs_rbfs_d1_comp_type_graph(
            delta_bc_of_vertices,
            workspace,
            source,
            edge
        )?,

        CompType::BiconnectedComponent => bbfs_rbfs_d1_comp_type_bcc(
            delta_bc_of_vertices,
            workspace,
            &component,
            source,
            edge
        )?,

        _ => { unimplemented!() }
    }

    Ok(())
}

fn bbfs_step(
    workspace: &mut ICentralWorkspace,
    nodeid:    NodeId,
    config:    &BBFSConfig, 
    component: &Component, 
    queue:     &mut NodeIdQueue

) -> Result<(),BetweennessCentralityError> {

    if config.handle_new_sigmas() {

        let edge = config.edge();

        if nodeid == edge.dst {

            let val = workspace.sigma_value_for_node(edge.src);

            debug!("updating sigmas for node={} with val={}", nodeid, val);

            workspace.new_sigmas_increment_sigma_value_for_node(nodeid, val);
        }
    }

    debug!(
        "component.subgraph {:#?}", 
        component.subgraph()
    );

    let nbrs = component.neighbors(nodeid);

    debug!("nbrs: {:#?}", nbrs);

    for &v_n in nbrs.iter() {

        debug!("bbfs from node={}, scanning neighbor={}", nodeid, v_n);

        if workspace.distance_is_infinite(v_n) {

            debug!(
                "found that neighbor={} is infinite distance away from node={}! enqueuing and setting as one step away", 
                v_n, 
                nodeid
            );

            queue.enqueue(v_n);

            workspace.set_distance_one_step_away(v_n,nodeid);

        } else {

            let dist = workspace.distance(v_n);

            debug!("found neighbor={}! {} steps away", v_n, dist);
        }

        if workspace.distance(v_n) == workspace.distance(nodeid) + 1.0 {

            debug!(
                "distance of neighbor={} is one step away from node={}, will increment sigma value", 
                v_n, 
                nodeid
            );

            workspace.increment_sigma_value_for_node(
                v_n, 
                workspace.sigma_value_for_node(nodeid)
            );

            if config.handle_new_sigmas() {

                workspace.new_sigmas_increment_sigma_value_for_node(
                    v_n, 
                    workspace.new_sigmas_sigma_value_for_node(nodeid)
                );
            }

            debug!("parenting {} to its neighbor {}", nodeid, v_n);

            workspace.add_parent(v_n, nodeid);
        }
    }

    debug!("bbfs_step, finished processing {} neighbors for node={}", nbrs.len(), nodeid);

    Ok(())
}

fn bbfs_update_deltas(
    workspace: &mut ICentralWorkspace, 
    v:         NodeId, 
    w:         NodeId

) {
    let step = workspace.calculate_deltas_step(v,w);

    workspace.increment_delta_value_for_node(v, step);
}

fn bbfs_update_new_deltas(
    workspace: &mut ICentralWorkspace, 
    v: NodeId, 
    w: NodeId) 
{
    let sigma_ratio = workspace.new_sigmas_sigma_ratio(v,w);

    let new_delta_w = workspace.new_deltas_delta_value_for_node(w);

    let update      = sigma_ratio * (1.0 + new_delta_w);

    workspace.new_deltas_increment_delta_value_for_node(
        v, 
        update
    );
}

fn bbfs_update(
    workspace: &mut ICentralWorkspace, 
    v: NodeId, 
    w: NodeId
) {
    bbfs_update_deltas(workspace,v,w);
    bbfs_update_new_deltas(workspace,v,w);
}

fn bbfs_update_if_match_edge_dst(
    workspace: &mut ICentralWorkspace, 
    w: NodeId, 
    e: &Edge) 
{
    if w == e.dst {

        let v_p: NodeId = e.src;

        let new_sp_sn: f64 = {
            let t0 = workspace.new_deltas_delta_value_for_node(v_p);
            let t1 = workspace.new_deltas_delta_value_for_node(w);
            t0 as f64 / t1 as f64
        };

        workspace.new_deltas_set_delta_value_for_node(
            v_p, 
            {
                let t0 = workspace.new_deltas_delta_value_for_node(v_p);
                let t1 = 1.0 + workspace.new_deltas_delta_value_for_node(w);
                t0 + new_sp_sn * t1
            }
        );
    }
}
