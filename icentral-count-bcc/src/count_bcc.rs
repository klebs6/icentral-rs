crate::ix!();

/**
  | counts the number of biconnected components
  | in the graph that have at least @num_edges
  | edges
  |
  */
pub fn count_bcc<G, GH>(
    graph:     &mut G,
    num_edges: i32

) -> Result<(),BetweennessCentralityError> 

where G: PrintHeader + NumEdges + FindBiconnectedComponent,
      GH: BccGraphHashInterface + NumEdges
{
    graph.print_header();

    let mut bcc_vec: Vec<GH> = default!();

    graph.find_bicon_component(&mut bcc_vec);

    let mut bcc_sz_vec: Vec<i32> = vec![];

    let mut num_bcc: i32 = 0;

    if bcc_vec.len() > 1 {

        for i in 0..bcc_vec.len() {

            bcc_sz_vec.push(bcc_vec[i].num_edges().try_into()?);

            if bcc_vec[i].num_edges() >= num_edges.try_into()? {

                num_bcc += 1;
            }
        }
    }

    debug!("");

    if bcc_sz_vec.len() > 2 {

        bcc_sz_vec.sort();

        debug!(
            "Num edges in LargestBiconnectedComponents: [{}]", 
            bcc_sz_vec.iter().last().unwrap()
        );

        debug!(
            "Num edges in 2nd largest BiconnectedComponents: [{}]", 
            bcc_sz_vec[bcc_sz_vec.len() - 2]
        );
    }

    debug!("");

    let frac_edges_in_lbcc: f64 = {
        let t0 = *bcc_sz_vec.iter().last().unwrap() as f64;
        let t1 = graph.num_edges() as f64;
        t0 / t1
    };

    debug!(
        "Frac edges in LargestBiconnectedComponents:  [{}]", 
        frac_edges_in_lbcc
    );

    debug!("");

    debug!(
        "Num BiconnectedComponents:   [{}]", 
        bcc_vec.len()
    );

    debug!(
        "Num BiconnectedComponents with # edges >= {}: [{}]", 
        num_edges, 
        num_bcc
    );

    Ok(())
}
