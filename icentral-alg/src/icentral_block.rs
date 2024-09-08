crate::ix!();

pub fn icentral_block(
    delta_bc_of_vertices: Arc<Mutex<BetweennessScores>>,
    component:            Arc<Mutex<Component>>,
    edge:                 Edge,
    source_vec:           Arc<Mutex<Vec<NodeId>>>,
    op:                   Operation) 
-> Result<(),BetweennessCentralityError> 
{
    let len = component.lock()?.num_nodes();

    delta_bc_of_vertices.lock()?.reinit(len);

    let mut workspace = ICentralWorkspace::new_init_all(len, "icentral_block::workspace");

    let source_vec_size = source_vec.lock()?.len();

    for i in 0..source_vec_size {

        let s: NodeId = source_vec.lock()?[i];

        let op = Some(op.clone());

        if let mut component = component.lock()? {

            if let mut delta_bc_of_vertices = delta_bc_of_vertices.lock()? {

                icentral_iter(
                    &mut delta_bc_of_vertices, 
                    &mut component, 
                    s, 
                    edge, 
                    &mut workspace, 
                    Some(-1.0), 
                    Some(false), 
                    op
                );
            }
        }
    }

    Ok(())
}
