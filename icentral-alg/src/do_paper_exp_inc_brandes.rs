crate::ix!();

pub fn maybe_do_paper_exp_inc_brandes<GH>(
    brandes_time_vec: &Vec<Duration>,
    edge_vec2:        &Vec<Vec<Edge>>,
    num_iter:         Option<usize>,
    path_vec:         &Vec<String>,
    do_inc_brandes:   bool) 
-> Result<(),BetweennessCentralityError> 
where GH: BccGraphHashInterface
{
    if do_inc_brandes {

        info!("Starting Incremental Brandes");

        for i in 0..path_vec.len() {

            let mut graph = arcmut![Graph::<GH>::from_filename(&path_vec[i])];

            let brandes_time = brandes_time_vec[i];

            exp_inc_brandes_p(
                graph.clone(), 
                num_iter, 
                &edge_vec2[i], 
                brandes_time
            )?;
        }
    }

    Ok(())
}


