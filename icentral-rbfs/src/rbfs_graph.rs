crate::ix!();

pub fn rbfs_graph(
    delta_bc_of_vertices: &mut BetweennessScores,
    source:               NodeId,
    workspace:            &mut ICentralWorkspace,
    op:                   &Option<RbfsOperation>) 
-> Result<(),BetweennessCentralityError> 
{
    info!(
        "running rbfs from source: {}, op: {:?}", 
        source, 
        op
    );

    while let Some(w) = workspace.stack_pop() {

        debug!("rbfs_graph, processing item popped from stack, {}", w);

        rbfs_graph_step(
            w, 
            delta_bc_of_vertices, 
            source, 
            workspace, 
            op
        )?;
    }

    debug!(
        "updated delta_bc_of_vertices: {:?}",
        delta_bc_of_vertices
    );

    Ok(())
}

fn rbfs_graph_step(
    w:                    NodeId,
    delta_bc_of_vertices: &mut BetweennessScores,
    source:               NodeId,
    workspace:            &mut ICentralWorkspace,
    op:                   &Option<RbfsOperation>) 
-> Result<(),BetweennessCentralityError> 
{
    rbfs_graph_step_parents(
        w, 
        delta_bc_of_vertices, 
        source, 
        workspace, 
        op
    )?;

    debug!("workspace.deltas: {:?}", workspace.deltas());

    rbfs_graph_step_tail(
        w, 
        delta_bc_of_vertices, 
        source, 
        workspace, 
        op
    )?;

    Ok(())
}

//------------------------------------------------
fn rbfs_graph_step_parents(
    w:                    NodeId,
    delta_bc_of_vertices: &mut BetweennessScores,
    source:               NodeId,
    workspace:            &mut ICentralWorkspace,
    op:                   &Option<RbfsOperation>) 
-> Result<(),BetweennessCentralityError> 
{
    let parents = workspace.parents_for_node(w);

    debug!("found parents for {}: {:?}", w, parents);

    for &v in parents.iter() {

        debug!("rbfs_graph_step_parents, processing parent={}", v);

        rbfs_graph_step_parent(
            v, 
            w, 
            delta_bc_of_vertices, 
            source, 
            workspace, 
            op
        )?;
    }

    Ok(())
}

fn rbfs_graph_step_parent(
    v:                    NodeId,
    w:                    NodeId,
    delta_bc_of_vertices: &mut BetweennessScores,
    source:               NodeId,
    workspace:            &mut ICentralWorkspace,
    op:                   &Option<RbfsOperation>) 
-> Result<(),BetweennessCentralityError> 
{
    let p0 = workspace.sigma_value_for_node(v) as f64;
    let p1 = workspace.sigma_value_for_node(w) as f64;

    let t0 = p0 / p1;

    let t1 = workspace.delta_value_for_node(w);

    let res = t0 * (1.0 + t1);

    debug!(
        "updating deltas[v] with {} -- inputs: v={}, w={}, p0={}, p1={}, t1={}", 
        res,
        v, 
        w, 
        p0, 
        p1, 
        t1
    );

    workspace.increment_delta_value_for_node(v, res);

    Ok(())
}

//------------------------------------------------
fn rbfs_graph_step_tail(
    w:                    NodeId,
    delta_bc_of_vertices: &mut BetweennessScores,
    source:               NodeId,
    workspace:            &mut ICentralWorkspace,
    op:                   &Option<RbfsOperation>) 
-> Result<(),BetweennessCentralityError> 
{
    if w != source {

        match op {

            Some(RbfsOperation::Addition) 
                => rbfs_graph_step_tail_add(
                    w, 
                    delta_bc_of_vertices, 
                    source, 
                    workspace, 
                    op
                )?,

            Some(RbfsOperation::Subtraction) 
                => rbfs_graph_step_tail_subtract(
                    w, 
                    delta_bc_of_vertices, 
                    source, 
                    workspace, 
                    op
                )?,

            _ => {
                warn!("rbfs_graph_step_tail, found noop={:?} ", op);
                warn!("This may not actually be a problem, but we still ought to know why it is happening and where it is coming from");
            }
        }
    } else {

        debug!(
            "rbfs_graph_step_tail, found that w={} equals source={}", 
            w, 
            source
        );
    }

    Ok(())
}

fn rbfs_graph_step_tail_add(
    w:                    NodeId,
    delta_bc_of_vertices: &mut BetweennessScores,
    source:               NodeId,
    workspace:            &mut ICentralWorkspace,
    op:                   &Option<RbfsOperation>) 
-> Result<(),BetweennessCentralityError> 
{
    let addend = workspace.delta_value_for_node(w) / 2.0;

    debug!("increasing delta_bc_of_vertices.score_for_node w={} by {}", w, addend);

    delta_bc_of_vertices.increase_score_for_node(
        w, 
        addend
    ); 

    Ok(())
}

fn rbfs_graph_step_tail_subtract(
    w:                    NodeId,
    delta_bc_of_vertices: &mut BetweennessScores,
    source:               NodeId,
    workspace:            &mut ICentralWorkspace,
    op:                   &Option<RbfsOperation>) 
-> Result<(),BetweennessCentralityError> 
{
    let subtrahend = workspace.delta_value_for_node(w) / 2.0;

    debug!(
        "decreasing delta_bc_of_vertices.score_for_node w={} by {}", 
        w, 
        subtrahend
    );

    delta_bc_of_vertices.decrease_score_for_node(
        w, 
        subtrahend
    );

    Ok(())
}
