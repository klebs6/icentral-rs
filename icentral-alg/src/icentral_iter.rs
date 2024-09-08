crate::ix!();

pub fn icentral_iter_insertion(
    subgraph_len:          usize,
    delta_bc_of_vertices:  &mut BetweennessScores,
    component:             &mut Component,
    s:                     NodeId,
    edge:                  Edge,
    workspace:             &mut ICentralWorkspace,
    dd:                    f64,
    use_d_1:               bool,
    op:                    Operation) 
-> Result<(),BetweennessCentralityError>  
{
    bbfs(
        None,
        workspace, 
        component, 
        s
    );

    rbfs(
        delta_bc_of_vertices, 
        component, 
        s, 
        workspace, 
        Some(RbfsOperation::Subtraction),
    );

    partial_bbfs_addition(
        workspace, 
        component, 
        s, 
        edge
    );

    rbfs(
        delta_bc_of_vertices,
        component, 
        s, 
        workspace, 
        Some(RbfsOperation::Addition),
    );

    Ok(())
}

pub fn icentral_iter_deletion(
    delta_bc_of_vertices: &mut BetweennessScores,
    component:            &mut Component,
    s:                    NodeId,
    edge:                 Edge,
    workspace:            &mut ICentralWorkspace) 
-> Result<(),BetweennessCentralityError>  
{
    bbfs(
        None,
        workspace, 
        component, 
        s
    );

    rbfs(
        delta_bc_of_vertices, 
        component, 
        s, 
        workspace, 
        Some(RbfsOperation::Subtraction),
    );

    partial_bbfs_deletion(
        workspace, 
        component, 
        s, 
        &edge
    );

    rbfs(
        delta_bc_of_vertices, 
        component, 
        s, 
        workspace, 
        Some(RbfsOperation::Addition),
    );

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
  */
pub fn icentral_iter(

    delta_bc_of_vertices:   &mut BetweennessScores,

    // component could be BiconnectedComponents,
    // MinimumUnionCycle, or just a graph
    component: &mut Component,

    // source of the iteration
    s:         NodeId,

    // inserted edge
    edge:      Edge,

    //TODO to be used later?
    workspace: &mut ICentralWorkspace,

    dd:        Option<f64>,
    use_d_1:   Option<bool>,
    op:        Option<Operation>) 
-> Result<(),BetweennessCentralityError>  
{
    let mut dd:   f64 = dd.unwrap_or(-1.0);

    let edge = match dd > 0.0 {
        true  => edge.reversed(),
        false => edge
    };

    dd = dd.abs();

    let use_d_1: bool = use_d_1.unwrap_or(true);

    let op: Operation = op.unwrap_or(Operation::Insertion);

    let len = component.num_nodes();

    workspace.init_all(len);

    match op {

        Operation::Insertion => icentral_iter_insertion(
            len,
            delta_bc_of_vertices, 
            component, 
            s, 
            edge, 
            workspace, 
            dd, 
            use_d_1, 
            op
        )?,

        Operation::Deletion  => icentral_iter_deletion(
            delta_bc_of_vertices, 
            component, 
            s, 
            edge, 
            workspace
        )?,
    }

    Ok(())
}
