crate::ix!();

pub trait FindArticulationPoints {

    fn find_articulation_points(&self, out_vec: &mut Vec<NodeId>);

    fn articulation_point_dfs_visitor<'a>(
        &self, 
        u:                      NodeId,
        ctx:                    &mut ArticulationPointFinderContext<'a>,
        articulation_point_vec: &mut Vec<NodeId>) 
    -> Result<(),BetweennessCentralityError>;

    fn articulation_point_dfs_visitor_step_tree_edge<'a>(
        &self, 
        v:                      NodeId,
        u:                      NodeId,
        ctx:                    &mut ArticulationPointFinderContext<'a>,
        articulation_point_vec: &mut Vec<NodeId>,
        tree_edge_cnt:          &mut i32) 
    -> Result<(),BetweennessCentralityError>;
}

impl<G> FindArticulationPoints for G 

where G
: NumNodes
+ GetNeighborsForNode
+ Named
{
    /**
      | This function will return a vector of
      | the articulation points.
      | 
      | If an articulation point appears in
      | more than two biconnected components
      | it will appear more than once (number
      | of bcc's it appears in -1)
      |
      */
    fn find_articulation_points(&self, out_vec: &mut Vec<NodeId>)  
    {
        debug!("initiated Graph::find_articulation_points");
        
        let mut time: f64 = 0.0;
        let mut u: NodeId = NodeId::zero();

        let num_nodes = self.num_nodes();

        let mut color_vec = ColorMap::new(
            num_nodes,
            name![self.name(), "find_articulation_points::color_vec"]
        );

        let mut pred_vec  = PredecessorMap::new(
            num_nodes,
            name![self.name(), "find_articulation_points::pred_vec"]
        );

        let mut distances = DistanceMap::new(num_nodes, "distances");
        let mut low_vec   = DistanceMap::new(num_nodes, "low_vec");

        let mut ctx = ArticulationPointFinderContext {
            color_vec: &mut color_vec, 
            low_vec:   &mut low_vec, 
            distances: &mut distances, 
            pred_vec:  &mut pred_vec, 
            time:      &mut time, 
        };

        self.articulation_point_dfs_visitor(
            u, 
            &mut ctx,
            out_vec
        );
    }

    fn articulation_point_dfs_visitor<'a>(
        &self, 
        u:                      NodeId,
        ctx:                    &mut ArticulationPointFinderContext<'a>,
        articulation_point_vec: &mut Vec<NodeId>) 
    -> Result<(),BetweennessCentralityError>
    {
        ctx.color_vec.set_color_for_node_grey(u);

        debug!("stepping time {} by one", *ctx.time);

        *ctx.time += 1.0;

        ctx.distances.set_distance_for_node(u, *ctx.time);

        ctx.low_vec.set_distance_for_node(u, ctx.distances.distance(u));

        let mut tree_edge_cnt: i32 = 0;

        let nbr_vec = self.neighbors(u);

        for &v in nbr_vec.iter() {

            let neighbor_color = ctx.color_vec.color_for_node(v);

            //  (u, v) is a tree edge
            if neighbor_color == Color::None {

                debug!("processing uncolored neighbor v={} -- supposedly (u={},v={}) is a tree edge", v, u, v);

                self.articulation_point_dfs_visitor_step_tree_edge(
                    v, 
                    u, 
                    ctx,
                    articulation_point_vec, 
                    &mut tree_edge_cnt
                )?;

            } else {

                debug!("processing colored neighbor v={}, of color={} -- supposedly (u={},v={}) is a back edge", v, neighbor_color, u, v);

                ctx.articulation_point_dfs_visitor_step_back_edge(v, u)?;
            }
        }

        Ok(())
    }

    fn articulation_point_dfs_visitor_step_tree_edge<'a>(
        &self, 
        v:                      NodeId,
        u:                      NodeId,
        ctx:                    &mut ArticulationPointFinderContext<'a>,
        articulation_point_vec: &mut Vec<NodeId>,
        tree_edge_cnt:          &mut i32) 
    -> Result<(),BetweennessCentralityError>
    {
        debug!("articulation_point_dfs_visitor_step_tree_edge");

        ctx.pred_vec.set_predecessor_for_node(v, u);

        *tree_edge_cnt += 1;

        debug!("incremented tree_edge_cnt to {}", *tree_edge_cnt);

        self.articulation_point_dfs_visitor(
            v, 
            ctx,
            articulation_point_vec
        );

        ctx.low_vec.set_distance_for_node(
            u, 
            min(
                FloatOrd(ctx.low_vec.distance(u)),
                FloatOrd(ctx.low_vec.distance(v))
            ).0
        );

        // this is the root of the tree
        //
        if ctx.pred_vec.is_tree_root(u) {

            debug!("u={} is the root of the tree", u);

            // if v is u's second child
            // then u is art vertex
            //
            if *tree_edge_cnt > 1 {
                articulation_point_vec.push(u);
            }

        } else {

            debug!("u={} is not the root of the tree", u);

            if ctx.low_vec.distance(v) >= ctx.distances.distance(u) {

                debug!("we have found an articulation point! the low_vec distance to v={} is greater than or equal to the low_vec distance to u={}", v, u);

                articulation_point_vec.push(u);
            }
        }

        Ok(())
    }
}
