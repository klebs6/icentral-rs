crate::ix!();

//-------------------------------------------[icentral/src/qube.cc]

/**
  | returns true is muc and cycle share a
  | vertex
  |
  */
pub fn shared_vertex_with_muc_and_cycle<GH>(
    muc:   &MinimumUnionCycle<GH>,
    cycle: &Cycle

) -> Result<bool,BetweennessCentralityError> 
where GH: GraphHashMucInterface
{
    for i in 0..cycle.num_edges() {

        if muc.has_map_for_node(cycle[i].src)
        || muc.has_map_for_node(cycle[i].dst)
        {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn shared_vertex(
    c1: &Cycle,
    c2: &Cycle

) -> bool {

    for i in 0..c1.num_edges() {

        for j in 0..c2.num_edges() {

            if c1[i].src == c2[j].src 
            || c1[i].src == c2[j].dst 
            || c1[i].dst == c2[j].src 
            || c1[i].dst == c2[j].dst 
            {
                return true;
            }
        }
    }

    false
}
