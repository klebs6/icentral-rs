crate::ix!();

pub trait BccGraphHashInterface
: NumNodes 
+ NumEdges 
+ GetEdges 
+ InsertEdge 
+ CreateNamedEmpty
{ }

impl<T> BccGraphHashInterface for T
where T: NumNodes + NumEdges + GetEdges + InsertEdge + CreateNamedEmpty {}

pub fn bcc_dfs_visitor<'a,G,GH>(
    graph:      &G, 
    u:          NodeId,
    ctx:        &mut BccDfsVisitorContext<'a>,
    bcc_vec:    &mut Vec<GH>

) -> Result<(),BetweennessCentralityError> 
where G: GetNeighborsForNode, GH: BccGraphHashInterface
{
    debug!("Graph::initiating bcc_dfs_visitor");

    // 1 means grey
    ctx.set_color_for_node_grey(u);
    ctx.step_time_and_update_distances_for_node(u);

    let nbr_vec = graph.neighbors(u);

    let mut tree_edge_cnt: i32 = 0;

    for i in 0..nbr_vec.len() {

        let v: NodeId = nbr_vec[i];

        //  (u, v) is a tree edge
        if ctx.color_for_node(v) == Color::None {

            bcc_dfs_visitor_step_tree_edge(
                graph,
                v,
                u,
                ctx,
                bcc_vec,
                &mut tree_edge_cnt
            )?;

        } else {

            ctx.bcc_dfs_visitor_step_colored(
                v,
                u,
            )?;
        }
    }

    Ok(())
}

fn bcc_dfs_visitor_step_tree_edge<'a,G,GH>(
    graph:         &G, 
    v:             NodeId,
    u:             NodeId,
    ctx:           &mut BccDfsVisitorContext<'a>,
    bcc_vec:       &mut Vec<GH>,
    tree_edge_cnt: &mut i32

) -> Result<(),BetweennessCentralityError> 
where G: GetNeighborsForNode, GH: BccGraphHashInterface
{
    ctx.edge_stack.push(Edge::new(u,v));

    ctx.pred_vec.set_predecessor_for_node(v, u);

    *tree_edge_cnt += 1;

    bcc_dfs_visitor(graph, v, ctx, bcc_vec);

    ctx.low_vec.set_distance_for_node(
        u, 
        min(
            FloatOrd(ctx.low_vec.distance(u)),
            FloatOrd(ctx.low_vec.distance(v))
        ).0
    );

    if ctx.low_vec.distance(v) >= ctx.distances.distance(u) {

        //  u is an articulation point, ready to output a bcc
        let last_edge = Edge::new(u,v);

        let mut bcc = GH::empty("bcc");

        loop {

            let e: Edge = ctx.edge_stack.pop().unwrap();

            bcc.insert_edge(&e);

            if e == last_edge {
                break;
            }
        }

        bcc_vec.push(bcc);
    }

    Ok(())
}
