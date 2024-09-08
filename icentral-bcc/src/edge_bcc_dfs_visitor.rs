crate::ix!();

pub fn edge_bcc_dfs_visitor<'a,G: GetNeighborsForNode>(
    graph:  &G,
    u:      NodeId,
    ctx:    &mut BccDfsVisitorContext<'a>

) -> Result<(),BetweennessCentralityError> {

    // 1 means grey
    ctx.set_color_for_node_grey(u);
    ctx.step_time_and_update_distances_for_node(u);

    let nbr_vec = graph.neighbors(u);

    let mut tree_edge_cnt: i32 = 0;

    for i in 0..nbr_vec.len() {

        let v: NodeId = nbr_vec[i];

        // (u, v) is a tree edge
        if ctx.color_for_node(v) == Color::None {

            edge_bcc_dfs_visitor_step_tree_edge(
                graph,
                v,
                u,
                ctx,
                &mut tree_edge_cnt
            )?;

        } else {

            ctx.bcc_dfs_visitor_step_colored(v, u)?;
        }
    }

    Ok(())
}

//-------------------------------------------[context]
/// (u, v) is a tree edge
fn edge_bcc_dfs_visitor_step_tree_edge<'a,G: GetNeighborsForNode>(
    graph:         &G, 
    v:             NodeId,
    u:             NodeId,
    ctx:           &mut BccDfsVisitorContext<'a>,
    tree_edge_cnt: &mut i32

) -> Result<(),BetweennessCentralityError> {

    ctx.edge_stack.push(Edge::new(u,v));

    ctx.pred_vec.set_predecessor_for_node(v, u);

    *tree_edge_cnt += 1;

    // f() starts
    edge_bcc_dfs_visitor(graph, v, ctx);

    // g() starts
    ctx.low_vec.set_distance_for_node(
        u, 
        min(
            FloatOrd(ctx.low_vec.distance(u)),
            FloatOrd(ctx.low_vec.distance(v))
        ).0
    );

    let has_predecessor = ctx.pred_vec.has_predecessor(u);

    let mut flush_edges: bool 
    = ctx.low_vec.distance(v) >= ctx.distances.distance(u) 
    && has_predecessor;

    flush_edges |= ctx.pred_vec.is_tree_root(u) && *tree_edge_cnt >= 2;

    if flush_edges {

        // u is an articulation point,
        // ready to output a bcc
        //
        let last_edge = Edge::new(u,v);;

        let mut e: Edge = Edge::default();

        while let Some(edge) = ctx.edge_stack.pop() {

            if e == last_edge {
                break;
            }
        }
    }

    Ok(())
}
