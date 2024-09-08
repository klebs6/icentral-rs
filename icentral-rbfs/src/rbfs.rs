crate::ix!();


#[derive(Debug)]
pub enum RbfsOperation {
    Addition,
    Subtraction,
}

pub fn rbfs(
    delta_bc_of_vertices:    &mut BetweennessScores,

    // component could be BiconnectedComponents,
    // MinimumUnionCycle, or just a graph
    component:               &mut Component,
    source_of_the_iteration: NodeId,
    workspace:               &mut ICentralWorkspace,
    op:                      Option<RbfsOperation>) 

-> Result<(),BetweennessCentralityError> 
{
    debug!("running rbfs from source={}", source_of_the_iteration);

    let len = component.num_nodes();

    workspace.deltas_fill_to_len(len, 0.0);

    match component.ty() {

        CompType::Graph => rbfs_graph(
            delta_bc_of_vertices,
            source_of_the_iteration,
            workspace,
            &op,
        )?,

        CompType::BiconnectedComponent => rbfs_bcc(
            delta_bc_of_vertices,
            component,
            source_of_the_iteration,
            workspace,
            &op,
        )?,
        _ => { 
            unreachable!();
        }
    }

    Ok(())
}
