crate::ix!();

impl GetConnectedComponentSizes for SubGraph {

    /**
      | the vector @out_vec will have sizes
      | of the connected components in the graph
      |
      */
    fn conn_comp_sizes(&self) 
    -> Result<Vec<i32>,BetweennessCentralityError>
    {
        let mut out_vec = vec![];

        let mut visit_markers = VisitMarkers::new(
            self.nodes_map.len(), 
            "conn_comp_sizes.visit_markers"
        );

        for node in visit_markers.iter_unvisited() {

            visit_markers.visit(node);

            out_vec.push(1);

            self.do_bfs_from_source_count_vertices_and_mark_visited(
                node, 
                &mut visit_markers, 
                &mut out_vec
            );
        }

        Ok(out_vec)
    }
}
