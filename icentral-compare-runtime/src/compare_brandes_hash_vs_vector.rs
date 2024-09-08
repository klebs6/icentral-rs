crate::ix!();

pub fn compare_brandes_hash_vs_vector<GH>(
    graph:    &mut Graph<GH>,
    num_iter: usize) 
-> Result<(),BetweennessCentralityError> 
where 
GH
: NewFromGraphRef<Graph<GH>> 
+ NumNodes 
+ SpawnScores 
+ MappedNodes 
+ BrandesIterInit
+ BrandesIterUpdatePairDependenciesAndFill
+ BrandesIterUpdateDistancesAndPathForNeighbors,

Graph<GH>
: PrintHeader 
+ GetLimitedNodeIdRange,
{
    graph.print_header();

    let mut graphhash = GH::new_from_graph_ref(&*graph, "compare_brandes_hash_vs_vector::graphhash");

    let mut brandes_time = Duration::default();

    let mut tm: Timer = Timer::default();

    debug!("Brandes runtime: ");

    tm.start();

    let scores = brandes_bc(graph,Some(num_iter))?;

    tm.stop();

    brandes_time = {
        tm.interval() * (graphhash.num_nodes() as u32) / num_iter as u32
    };

    debug!("{:?}", brandes_time);

    debug!(
        "Brandes avg iter time: {:?}", 
        tm.interval() / num_iter as u32
    );

    debug!("");

    debug!("Brandes hash runtime: ");

    tm.start();

    let scores: BetweennessScores = brandes_bc_hash(
        &mut graphhash,
        Some(num_iter)
    )?;

    tm.stop();

    brandes_time = {
        let interval = tm.interval();
        let g_len    = graphhash.num_nodes() as u32;
        interval * g_len / num_iter as u32
    };

    debug!("{:?}", brandes_time);

    debug!(
        "Brandes hash avg iter time: {:?}", 
        tm.interval() / num_iter as u32
    );

    Ok(())
}
