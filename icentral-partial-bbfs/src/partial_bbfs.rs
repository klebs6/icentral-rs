crate::ix!();

pub fn update_workspace_for_partial_bbfs_addition(
    workspace: &mut ICentralWorkspace,
    component: &mut Component,
    queue:     &mut NodeIdQueue,
)
-> Result<(),BetweennessCentralityError>
{
    while let Some(v) = queue.dequeue() {

        let nbrs = component.neighbors(v);

        for &w in nbrs.iter() {

            workspace.update_for_partial_bbfs_addition(
                queue, 
                w, 
                v
            );
        }
    }

    Ok(())
}

pub fn partial_bbfs_addition(
    workspace:      &mut ICentralWorkspace,
    component:      &mut Component,
    source:         NodeId,
    inserted_edge:  Edge,
)
-> Result<(),BetweennessCentralityError>
{
    let (src,dst) = {

        let inserted_src_distance = workspace.distance(inserted_edge.src);
        let inserted_dst_distance = workspace.distance(inserted_edge.dst);

        let e = match inserted_src_distance > inserted_dst_distance {
            true  => inserted_edge.reversed(),
            false => inserted_edge
        };

        (e.src, e.dst)
    };

    workspace.compute_new_path_counts_and_paths(src,dst);

    let mut queue = NodeIdQueue::empty("partial_bbfs_addition::queue");

    queue.enqueue(dst);

    workspace.set_distance_one_step_away(dst, src);

    workspace.inc_sigmas_set_sigma_value_for_node(
        dst, 
        workspace.sigma_value_for_node(src)
    );

    workspace.visit(dst);

    update_workspace_for_partial_bbfs_addition(
        workspace,
        component,
        &mut queue
    )?;

    workspace.fix_order_of_workspace_stack();

    Ok(())
}

/**
  | TMP: this uses Brandes, full BFS, TODO:
  | make efficient
  |
  */
pub fn partial_bbfs_deletion(
    workspace:    &mut ICentralWorkspace,
    component:    &Component,
    source:       NodeId,
    deleted_edge: &Edge)

-> Result<(),BetweennessCentralityError>
{
    let mut queue = NodeIdQueue::empty("partial_bbfs_deletion::queue");

    // for now everything is computed from
    // scratch, so old values are irrelevant
    //
    workspace.init_all(component.num_nodes());

    // Assumes workspace is initialized
    //
    // IMP: careful bitch, graph_t is not thread
    // safe!
    //
    // component.subgraph.remove_edge(deleted_edge.src,
    // deleted_edge.dst);
    //
    workspace.sigma_set_node_to_one(source);
    workspace.set_distance_zero(source);

    queue.enqueue(source);

    while let Some(v_i) = queue.dequeue() {

        workspace.stack_push(v_i);

        let nbrs = component.neighbors(v_i);

        for &v_n in nbrs.iter() {

            if deleted_edge.connects_nodes(v_i,v_n) {
                continue;
            }

            if workspace.distance_is_infinite(v_n) {

                queue.enqueue(v_n);

                workspace.set_distance_one_step_away(v_n, v_i);
            }

            if workspace.distance_is_one_step_away(v_n, v_i) {

                let sn = workspace.sigma_value_for_node(v_n);
                let si = workspace.sigma_value_for_node(v_i);

                workspace.set_sigma_value_for_node(
                    v_n, 
                    sn + si
                );

                workspace.add_parent(v_n, v_i);
            }
        }
    }

    //component.subgraph.insert_edge(deleted_edge.src, deleted_edge.dst);
    Ok(())
}
