crate::ix!();

pub fn merge_mucs<GH>(
    nodes_to_mucs: &mut MucIdMap,
    muc1:          &mut MinimumUnionCycle<GH>,
    muc2:          &mut MinimumUnionCycle<GH>) 
-> Result<(),BetweennessCentralityError> 
where GH: ExtendWith<GH,Error=BetweennessCentralityError> 
        + GetConnectedComponentSizes 
        + GetEdges 
        + GetNeighborsForNode 
        + GetNodeIdRange
        + HasMapForNode 
        + InsertEdge 
        + InsertNode 
        + MappedNodes 
        + NumEdges 
        + NumNodes 
{
    debug!("merging muc1 id={}, muc2 id={}", muc1.id(), muc2.id());

    muc1.extend_with(muc2)?;

    // mark muc2 as invalid
    muc2.invalidate();

    // ASSUME muc1.id is valid, and update
    // nodes_to_mucs for vertices in muc2 to
    // hold the id of muc1
    for conn_vert in muc2.mapped_nodes() {

        nodes_to_mucs.set_mucid_for_node(conn_vert, muc1.id());
    }

    Ok(())
}


