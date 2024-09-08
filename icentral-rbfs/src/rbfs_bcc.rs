crate::ix!();


pub fn rbfs_bcc(
    delta_bc_of_vertices: &mut BetweennessScores,
    component:            &mut Component,
    source:               NodeId,
    workspace:            &mut ICentralWorkspace,
    op:                   &Option<RbfsOperation>) 
-> Result<(),BetweennessCentralityError> 
{
    info!("running rbfs_bcc");

    // BiconnectedComponents or MinimumUnionCycle
    // case: involved case, with external pairs
    // contribution
    workspace.capital_deltas_deltas_fill_to_len(component.num_nodes(), 0.0);

    for idx in (0..=workspace.stack_len() - 1).rev() {

        let v_n: NodeId = workspace.stack_node_at_index(idx);

        rbfs_bcc_step(
            v_n,
            delta_bc_of_vertices,
            component,
            source,
            workspace,
            op,
        );
    }

    Ok(())
}

fn rbfs_bcc_step(
    v_n:                  NodeId,
    delta_bc_of_vertices: &mut BetweennessScores,
    component:            &mut Component,
    source:               NodeId,
    workspace:            &mut ICentralWorkspace,
    op:                   &Option<RbfsOperation>) 
-> Result<(),BetweennessCentralityError> 
{
    rbfs_bcc_step_head(
        v_n, 
        delta_bc_of_vertices, 
        component, 
        source, 
        workspace, 
        op
    )?;

    rbfs_bcc_step_parents(
        v_n, 
        delta_bc_of_vertices, 
        component, 
        source, 
        workspace, 
        op
    )?;

    if source != v_n {

        rbfs_bcc_step_update_score_with_half_delta(
            v_n, 
            delta_bc_of_vertices, 
            component, 
            source, 
            workspace, 
            op
        )?;
    }

    rbfs_bcc_step_tail(
        v_n, 
        delta_bc_of_vertices, 
        component, 
        source, 
        workspace, 
        op
    )?;

    Ok(())
}

//------------------------------------------------
fn rbfs_bcc_step_head(
    v_n:                  NodeId,
    delta_bc_of_vertices: &mut BetweennessScores,
    component:            &mut Component,
    source:               NodeId,
    workspace:            &mut ICentralWorkspace,
    op:                   &Option<RbfsOperation>) 
-> Result<(),BetweennessCentralityError> 
{
    if component.has_both_articulation_points(source,v_n) 
    {
        let c_t = component.subgraphs_product_through_articulation_points(source, v_n);

        workspace.capital_deltas_increment_delta_value_for_node(
            v_n, 
            c_t
        );
    }

    Ok(())
}

//------------------------------------------------
fn rbfs_bcc_step_parent(
    v_p:                  NodeId,
    v_n:                  NodeId,
    delta_bc_of_vertices: &mut BetweennessScores,
    component:            &mut Component,
    source:               NodeId,
    workspace:            &mut ICentralWorkspace,
    op:                   &Option<RbfsOperation>) 
-> Result<(),BetweennessCentralityError> 
{
    let sp_sn = workspace.sigma_ratio(v_p,v_n);

    workspace.increment_delta_value_for_node(
        v_p, 
        sp_sn * (1.0 + workspace.delta_value_for_node(v_n))
    );

    if component.has_articulation_point(source) {

        workspace.capital_deltas_increment_delta_value_for_node(
            v_p, 
            workspace.capital_deltas_delta_value_for_node(v_n) * sp_sn
        );
    }

    Ok(())
}

fn rbfs_bcc_step_parents(
    v_n:                  NodeId,
    delta_bc_of_vertices: &mut BetweennessScores,
    component:            &mut Component,
    source:               NodeId,
    workspace:            &mut ICentralWorkspace,
    op:                   &Option<RbfsOperation>) 
-> Result<(),BetweennessCentralityError> 
{
    for &v_p in workspace.parents_for_node(v_n).iter() {

        rbfs_bcc_step_parent(
            v_p, 
            v_n, 
            delta_bc_of_vertices, 
            component, 
            source, 
            workspace, 
            op
        )?;
    }

    Ok(())
}

//------------------------------------------------
fn rbfs_bcc_step_update_score_with_half_delta(
    v_n:                  NodeId,
    delta_bc_of_vertices: &mut BetweennessScores,
    component:            &mut Component,
    source:               NodeId,
    workspace:            &mut ICentralWorkspace,
    op:                   &Option<RbfsOperation>) 
-> Result<(),BetweennessCentralityError> 
{
    if source != v_n {

        match op {
            Some(RbfsOperation::Addition) => {
                delta_bc_of_vertices.increase_score_for_node(
                    v_n, 
                    workspace.delta_value_for_node(v_n) / 2.0
                );
            }
            Some(RbfsOperation::Subtraction) => {
                delta_bc_of_vertices.decrease_score_for_node(
                    v_n, 
                    workspace.delta_value_for_node(v_n) / 2.0
                );
            }
            _ => {}
        }
    }

    Ok(())
}

//------------------------------------------------
/// called "tail" because it is the final part of
/// the step
///
fn rbfs_bcc_step_tail(
    v_n:                  NodeId,
    delta_bc_of_vertices: &mut BetweennessScores,
    component:            &mut Component,
    source:               NodeId,
    workspace:            &mut ICentralWorkspace,
    op:                   &Option<RbfsOperation>) 
-> Result<(),BetweennessCentralityError> 
{
    if component.has_articulation_point(source) {

        let vg_s: f64 = component.subgraph_micentraltude_through_articulation_point(source);

        match op {
            Some(RbfsOperation::Addition) 
                => rbfs_bcc_step_tail_add(
                    vg_s,
                    v_n, 
                    delta_bc_of_vertices, 
                    component, 
                    source, 
                    workspace, 
                    op
                )?,
            Some(RbfsOperation::Subtraction) 
                => rbfs_bcc_step_tail_subtract(
                    vg_s,
                    v_n, 
                    delta_bc_of_vertices, 
                    component, 
                    source, 
                    workspace, 
                    op
                )?,
            _ => {}
        }
    }

    Ok(())
}

fn rbfs_bcc_step_tail_add(
    vg_s:                 f64,
    v_n:                  NodeId,
    delta_bc_of_vertices: &mut BetweennessScores,
    component:            &mut Component,
    source:               NodeId,
    workspace:            &mut ICentralWorkspace,
    op:                   &Option<RbfsOperation>) 
-> Result<(),BetweennessCentralityError> 
{
    delta_bc_of_vertices.increase_score_for_node(
        v_n, 
        workspace.delta_value_for_node(v_n) * vg_s
    );

    delta_bc_of_vertices.increase_score_for_node(
        v_n, 
        workspace.capital_deltas_delta_value_for_node(v_n) / 2.0
    );

    Ok(())
}

fn rbfs_bcc_step_tail_subtract(
    vg_s:                 f64,
    v_n:                  NodeId,
    delta_bc_of_vertices: &mut BetweennessScores,
    component:            &mut Component,
    source:               NodeId,
    workspace:            &mut ICentralWorkspace,
    op:                   &Option<RbfsOperation>) 
-> Result<(),BetweennessCentralityError> 
{
    delta_bc_of_vertices.decrease_score_for_node(
        v_n, 
        workspace.delta_value_for_node(v_n) * vg_s
    );

    delta_bc_of_vertices.decrease_score_for_node(
        v_n, 
        workspace.capital_deltas_delta_value_for_node(v_n) / 2.0
    );

    Ok(())
}
